mod map;
mod monster;
mod player;
mod projectile;

use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal::{self, Clear, ClearType},
};
use map::{Map, Tile};
use monster::{Monster, MonsterAction};
use player::Player;
use projectile::Projectile;
use std::io::{stdout, Write};
use std::time::{Duration, Instant};

const MONSTER_TICK_MS: u64 = 500;
const POLL_TIMEOUT_MS: u64 = 50;

fn render_map(stdout: &mut std::io::Stdout, map: &Map) -> std::io::Result<()> {
    execute!(stdout, cursor::MoveTo(0, 0))?;
    for y in 0..map.height {
        for x in 0..map.width {
            execute!(stdout, cursor::MoveTo(x as u16, y as u16))?;
            render_tile(stdout, map.tiles[x][y])?;
        }
    }
    Ok(())
}

fn render_tile(stdout: &mut std::io::Stdout, tile: Tile) -> std::io::Result<()> {
    match tile {
        Tile::Wall => {
            execute!(stdout, SetForegroundColor(Color::Grey), Print("#"))?;
        }
        Tile::Floor => {
            execute!(stdout, SetForegroundColor(Color::DarkGrey), Print("."))?;
        }
        Tile::Stairs => {
            execute!(stdout, SetForegroundColor(Color::White), Print(">"))?;
        }
        Tile::Potion => {
            execute!(stdout, SetForegroundColor(Color::Magenta), Print("!"))?;
        }
    }
    Ok(())
}

fn render_monsters(stdout: &mut std::io::Stdout, monsters: &[Monster]) -> std::io::Result<()> {
    for monster in monsters {
        if monster.is_alive() {
            execute!(
                stdout,
                cursor::MoveTo(monster.x as u16, monster.y as u16),
                SetForegroundColor(match monster.symbol {
                    'g' => Color::Green,
                    's' => Color::White,
                    'T' => Color::Red,
                    _ => Color::Magenta,
                }),
                Print(monster.symbol)
            )?;
        }
    }
    Ok(())
}

fn render_projectiles(
    stdout: &mut std::io::Stdout,
    projectiles: &[Projectile],
) -> std::io::Result<()> {
    for proj in projectiles {
        execute!(
            stdout,
            cursor::MoveTo(proj.x as u16, proj.y as u16),
            SetForegroundColor(Color::Yellow),
            Print(proj.symbol)
        )?;
    }
    Ok(())
}

/// Erase a projectile by redrawing the underlying tile.
fn erase_projectile(
    stdout: &mut std::io::Stdout,
    map: &Map,
    x: usize,
    y: usize,
) -> std::io::Result<()> {
    execute!(stdout, cursor::MoveTo(x as u16, y as u16))?;
    render_tile(stdout, map.tiles[x][y])?;
    Ok(())
}

/// Erase a monster position by restoring the underlying tile.
fn erase_entity(
    stdout: &mut std::io::Stdout,
    map: &Map,
    x: usize,
    y: usize,
) -> std::io::Result<()> {
    execute!(stdout, cursor::MoveTo(x as u16, y as u16))?;
    render_tile(stdout, map.tiles[x][y])?;
    Ok(())
}

/// Build list of occupied positions (living monsters), excluding index `skip`.
fn occupied_positions(monsters: &[Monster], skip: usize) -> Vec<(usize, usize)> {
    monsters
        .iter()
        .enumerate()
        .filter(|(j, m)| *j != skip && m.is_alive())
        .map(|(_, m)| (m.x, m.y))
        .collect()
}

/// All living monster positions.
fn all_monster_positions(monsters: &[Monster]) -> Vec<(usize, usize)> {
    monsters
        .iter()
        .filter(|m| m.is_alive())
        .map(|m| (m.x, m.y))
        .collect()
}

/// Process projectile movement and collisions. Returns true if player died from a projectile.
fn process_projectiles(
    stdout: &mut std::io::Stdout,
    projectiles: &mut Vec<Projectile>,
    monsters: &[Monster],
    player: &mut Player,
    map: &Map,
    log: &mut Vec<String>,
) -> std::io::Result<()> {
    let mut i = 0;
    while i < projectiles.len() {
        // Erase old position
        let old_x = projectiles[i].x;
        let old_y = projectiles[i].y;
        erase_projectile(stdout, map, old_x, old_y)?;

        let still_alive = projectiles[i].advance(map);

        if !still_alive {
            log.push("The arrow thuds into a wall.".to_string());
            projectiles.remove(i);
            continue;
        }

        let px = projectiles[i].x;
        let py = projectiles[i].y;

        // Check if projectile hit the player
        if px == player.x && py == player.y {
            let dmg = projectiles[i].damage;
            player.take_damage(dmg);
            log.push(format!("An arrow hits you for {} damage!", dmg));
            projectiles.remove(i);
            continue;
        }

        // Check if projectile hit a monster (friendly fire stops arrow)
        let mut hit_monster = false;
        for monster in monsters.iter() {
            if monster.is_alive() && monster.x == px && monster.y == py {
                hit_monster = true;
                break;
            }
        }
        if hit_monster {
            log.push("The arrow hits a creature.".to_string());
            projectiles.remove(i);
            continue;
        }

        i += 1;
    }
    Ok(())
}

/// Process all monster AI actions. Returns nothing; mutates monsters, player, projectiles, log.
fn process_monsters(
    stdout: &mut std::io::Stdout,
    monsters: &mut Vec<Monster>,
    player: &mut Player,
    projectiles: &mut Vec<Projectile>,
    map: &Map,
    log: &mut Vec<String>,
) -> std::io::Result<()> {
    let player_pos = (player.x, player.y);
    let all_positions = all_monster_positions(monsters);

    for i in 0..monsters.len() {
        if !monsters[i].is_alive() {
            continue;
        }

        let occupied = occupied_positions(monsters, i);

        // Check if this monster just went berserk (for log message)
        let was_berserk = monsters[i].is_berserk;

        let action = monsters[i].decide_action(player_pos, map, &occupied, &all_positions);

        // Log berserk activation
        if monsters[i].is_berserk && !was_berserk {
            log.push("The Troll flies into a rage!".to_string());
        }

        match action {
            MonsterAction::Nothing => {}
            MonsterAction::MeleeAttack { damage, ref name } => {
                player.take_damage(damage);
                log.push(format!("The {} hits you for {} damage!", name, damage));
            }
            MonsterAction::MoveTo(nx, ny) => {
                // Verify the move is still valid (another monster may have moved there)
                let mut blocked = false;
                for (j, other) in monsters.iter().enumerate() {
                    if i != j && other.is_alive() && other.x == nx && other.y == ny {
                        blocked = true;
                        break;
                    }
                }
                // Don't walk onto the player
                if nx == player.x && ny == player.y {
                    blocked = true;
                }

                if !blocked && map.is_walkable(nx, ny) {
                    erase_entity(stdout, map, monsters[i].x, monsters[i].y)?;
                    monsters[i].x = nx;
                    monsters[i].y = ny;

                    // Tier 3 Goblin: second move (speed 2)
                    if matches!(monsters[i].monster_type, monster::MonsterType::Goblin)
                        && monsters[i].floor_tier >= 3
                        && monsters[i].can_see_player
                    {
                        let occupied2 = occupied_positions(monsters, i);
                        if let Some(next2) = map.astar_next_step(
                            (monsters[i].x, monsters[i].y),
                            player_pos,
                            &occupied2,
                        ) {
                            let mut blocked2 = false;
                            for (j, other) in monsters.iter().enumerate() {
                                if i != j
                                    && other.is_alive()
                                    && other.x == next2.0
                                    && other.y == next2.1
                                {
                                    blocked2 = true;
                                    break;
                                }
                            }
                            if next2.0 == player.x && next2.1 == player.y {
                                // Second step would land on player — melee attack instead
                                let damage = monsters[i].attack;
                                player.take_damage(damage);
                                log.push(format!(
                                    "The Goblin dashes and strikes for {} damage!",
                                    damage
                                ));
                            } else if !blocked2 && map.is_walkable(next2.0, next2.1) {
                                erase_entity(stdout, map, monsters[i].x, monsters[i].y)?;
                                monsters[i].x = next2.0;
                                monsters[i].y = next2.1;
                            }
                        }
                    }
                }
            }
            MonsterAction::FireProjectile(proj) => {
                log.push(format!("The {} fires an arrow!", monsters[i].name));
                projectiles.push(proj);
            }
        }
    }
    Ok(())
}

fn render_ui(
    stdout: &mut std::io::Stdout,
    map_height: usize,
    current_floor: i32,
    player: &Player,
    log: &[String],
) -> std::io::Result<()> {
    execute!(
        stdout,
        cursor::MoveTo(0, map_height as u16 + 1),
        SetForegroundColor(Color::White),
        Print(format!(
            "Floor: {} | HP: {:2}/{:2}    ",
            current_floor, player.hp, player.max_hp
        ))
    )?;

    for (i, msg) in log.iter().rev().take(3).enumerate() {
        execute!(
            stdout,
            cursor::MoveTo(0, map_height as u16 + 2 + i as u16),
            SetForegroundColor(Color::Grey),
            Clear(ClearType::UntilNewLine),
            Print(msg)
        )?;
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    terminal::enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, cursor::Hide)?;

    let mut current_floor = 1;
    let mut last_key = None;
    let mut last_move_time = Instant::now();
    let mut log: Vec<String> = Vec::new();

    'outer: loop {
        let (term_width, term_height) = terminal::size()?;
        let map_width = term_width as usize;
        let map_height = (term_height as usize).saturating_sub(6);

        let mut map = Map::new(map_width, map_height);
        let (spawn_x, spawn_y) = map.get_starting_position();
        let mut player = Player::new(spawn_x, spawn_y);
        let mut monsters = map.spawn_monsters_for_floor(current_floor);
        let mut projectiles: Vec<Projectile> = Vec::new();
        let mut last_monster_tick = Instant::now();

        execute!(stdout, Clear(ClearType::All))?;
        render_map(&mut stdout, &map)?;
        log.push(format!("Welcome to floor {}!", current_floor));

        'inner: loop {
            // --- RENDER ---
            render_ui(&mut stdout, map_height, current_floor, &player, &log)?;
            render_monsters(&mut stdout, &monsters)?;
            render_projectiles(&mut stdout, &projectiles)?;

            execute!(
                stdout,
                cursor::MoveTo(player.x as u16, player.y as u16),
                SetForegroundColor(Color::Yellow),
                Print("@"),
            )?;
            stdout.flush()?;

            // --- NON-BLOCKING INPUT POLL ---
            // Poll for input with a short timeout so monsters can tick independently
            let mut player_acted = false;
            if event::poll(Duration::from_millis(POLL_TIMEOUT_MS))? {
                if let Event::Key(key_event) = event::read()? {
                    let mut next_x = player.x;
                    let mut next_y = player.y;

                    let now = Instant::now();
                    let repeat_too_fast = Some(key_event.code) == last_key
                        && now.duration_since(last_move_time) < Duration::from_millis(100);

                    last_key = Some(key_event.code);

                    if !repeat_too_fast {
                        last_move_time = now;

                        match key_event.code {
                            KeyCode::Char('q') | KeyCode::Esc => break 'outer,
                            KeyCode::Up => {
                                if next_y > 0 {
                                    next_y -= 1;
                                }
                            }
                            KeyCode::Down => {
                                next_y += 1;
                            }
                            KeyCode::Left => {
                                if next_x > 0 {
                                    next_x -= 1;
                                }
                            }
                            KeyCode::Right => {
                                next_x += 1;
                            }
                            _ => {} // Non-movement key — don't set player_acted
                        }

                        // Only process if position actually changed
                        if next_x != player.x || next_y != player.y {
                            // --- PLAYER COMBAT (bump-to-attack) ---
                            let mut monster_index = None;
                            for (i, monster) in monsters.iter().enumerate() {
                                if monster.is_alive() && monster.x == next_x && monster.y == next_y
                                {
                                    monster_index = Some(i);
                                    break;
                                }
                            }

                            if let Some(i) = monster_index {
                                let damage = 5;
                                monsters[i].take_damage(damage);
                                log.push(format!(
                                    "You hit the {} for {} damage!",
                                    monsters[i].name, damage
                                ));
                                if !monsters[i].is_alive() {
                                    log.push(format!("The {} dies!", monsters[i].name));
                                    erase_entity(&mut stdout, &map, monsters[i].x, monsters[i].y)?;
                                }
                                player_acted = true;
                            } else if map.is_walkable(next_x, next_y) {
                                // --- PLAYER MOVEMENT ---
                                erase_entity(&mut stdout, &map, player.x, player.y)?;

                                player.x = next_x;
                                player.y = next_y;

                                if map.tiles[player.x][player.y] == Tile::Stairs {
                                    current_floor += 1;
                                    break 'inner;
                                }

                                if map.tiles[player.x][player.y] == Tile::Potion {
                                    let heal_amount = 7;
                                    player.heal(heal_amount);
                                    map.tiles[player.x][player.y] = Tile::Floor;
                                    log.push(format!(
                                        "You drink a health potion! (+{} HP)",
                                        heal_amount
                                    ));
                                }
                                player_acted = true;
                            }
                        }
                    }
                }
            }

            // --- MONSTER TICK (independent of player input) ---
            let now = Instant::now();
            if now.duration_since(last_monster_tick) >= Duration::from_millis(MONSTER_TICK_MS) {
                last_monster_tick = now;

                // Process projectiles
                process_projectiles(
                    &mut stdout,
                    &mut projectiles,
                    &monsters,
                    &mut player,
                    &map,
                    &mut log,
                )?;

                // Process monster AI
                process_monsters(
                    &mut stdout,
                    &mut monsters,
                    &mut player,
                    &mut projectiles,
                    &map,
                    &mut log,
                )?;
            }

            // Suppress unused variable warning — player_acted reserved for future
            // hybrid tick logic (e.g. reset monster tick on player action)
            let _ = player_acted;

            // --- DEATH CHECK ---
            if !player.is_alive() {
                execute!(
                    stdout,
                    cursor::MoveTo(player.x as u16, player.y as u16),
                    SetForegroundColor(Color::Red),
                    Print("X")
                )?;
                execute!(
                    stdout,
                    cursor::MoveTo(0, map_height as u16 + 1),
                    SetForegroundColor(Color::Red),
                    Print("YOU DIED! Press 'r' to restart or 'q' to quit.          ")
                )?;
                stdout.flush()?;
                loop {
                    if let Event::Key(key_event) = event::read()? {
                        match key_event.code {
                            KeyCode::Char('q') | KeyCode::Esc => break 'outer,
                            KeyCode::Char('r') => {
                                current_floor = 1;
                                break 'inner;
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    execute!(stdout, cursor::Show)?;
    terminal::disable_raw_mode()?;
    println!("Exiting Ironveil...");
    Ok(())
}
