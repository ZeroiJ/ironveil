mod items;
mod map;
mod monster;
mod player;
mod projectile;
mod ui;

use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal::{self, Clear, ClearType},
};
use items::{Item, ItemType};
use map::{Map, Tile};
use monster::{Monster, MonsterAction};
use player::{Class, Player};
use projectile::Projectile;
use rand::RngExt;
use std::collections::HashMap;
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

fn render_ground_items(
    stdout: &mut std::io::Stdout,
    ground_items: &HashMap<(usize, usize), Item>,
) -> std::io::Result<()> {
    for (&(x, y), item) in ground_items {
        let color = match item.item_type {
            ItemType::Weapon => Color::Cyan,
            ItemType::Armor => Color::DarkYellow,
            ItemType::Ring => Color::Yellow,
            ItemType::Potion => Color::Magenta,
        };
        execute!(
            stdout,
            cursor::MoveTo(x as u16, y as u16),
            SetForegroundColor(color),
            Print(item.symbol)
        )?;
    }
    Ok(())
}

/// Erase a position by restoring the underlying tile.
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

/// Player color based on class.
fn player_color(class: Class) -> Color {
    match class {
        Class::Warrior => Color::Red,
        Class::Rogue => Color::Green,
        Class::Mage => Color::Blue,
    }
}

/// Process projectile movement and collisions.
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
        let old_x = projectiles[i].x;
        let old_y = projectiles[i].y;
        erase_entity(stdout, map, old_x, old_y)?;

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
            // Dodge check
            if player.try_dodge() {
                log.push("You dodge the arrow!".to_string());
            } else {
                let raw_dmg = projectiles[i].damage;
                let dmg = player.reduce_damage(raw_dmg);
                player.take_damage(dmg);
                log.push(format!("An arrow hits you for {} damage!", dmg));
            }
            projectiles.remove(i);
            continue;
        }

        // Check if projectile hit a monster
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

/// Process all monster AI actions.
fn process_monsters(
    stdout: &mut std::io::Stdout,
    monsters: &mut Vec<Monster>,
    player: &mut Player,
    projectiles: &mut Vec<Projectile>,
    map: &Map,
    log: &mut Vec<String>,
    _ground_items: &mut HashMap<(usize, usize), Item>,
    _current_floor: i32,
) -> std::io::Result<()> {
    let player_pos = (player.x, player.y);
    let all_positions = all_monster_positions(monsters);

    for i in 0..monsters.len() {
        if !monsters[i].is_alive() {
            continue;
        }

        let occupied = occupied_positions(monsters, i);
        let was_berserk = monsters[i].is_berserk;
        let action = monsters[i].decide_action(player_pos, map, &occupied, &all_positions);

        if monsters[i].is_berserk && !was_berserk {
            log.push("The Troll flies into a rage!".to_string());
        }

        match action {
            MonsterAction::Nothing => {}
            MonsterAction::MeleeAttack { damage, ref name } => {
                // Player dodge check
                if player.try_dodge() {
                    log.push(format!("You dodge the {}'s attack!", name));
                } else {
                    let dmg = player.reduce_damage(damage);
                    player.take_damage(dmg);
                    log.push(format!("The {} hits you for {} damage!", name, dmg));
                }
            }
            MonsterAction::MoveTo(nx, ny) => {
                let mut blocked = false;
                for (j, other) in monsters.iter().enumerate() {
                    if i != j && other.is_alive() && other.x == nx && other.y == ny {
                        blocked = true;
                        break;
                    }
                }
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
                                let damage = monsters[i].attack;
                                if player.try_dodge() {
                                    log.push("You dodge the Goblin's dash attack!".to_string());
                                } else {
                                    let dmg = player.reduce_damage(damage);
                                    player.take_damage(dmg);
                                    log.push(format!(
                                        "The Goblin dashes and strikes for {} damage!",
                                        dmg
                                    ));
                                }
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
    let stats = player.effective_stats();
    let weapon_name = player
        .equipment
        .weapon
        .as_ref()
        .map_or("Fists".to_string(), |w| w.display_name());

    // Status line 1: Floor, HP, Class, Weapon
    let status = format!(
        "Floor: {} | HP: {:2}/{:2} | {} | {}",
        current_floor,
        player.hp,
        player.max_hp,
        player.class.name(),
        weapon_name
    );
    execute!(
        stdout,
        cursor::MoveTo(0, map_height as u16 + 1),
        SetForegroundColor(Color::White),
        Clear(ClearType::UntilNewLine),
        Print(&status)
    )?;

    // Status line 2: Stats
    let stat_line = format!(
        "STR:{} DEX:{} INT:{} CON:{} | Def:{} | Tab:Inventory",
        stats.str_,
        stats.dex,
        stats.int,
        stats.con,
        player.equipment.armor_defense()
    );
    execute!(
        stdout,
        cursor::MoveTo(0, map_height as u16 + 2),
        SetForegroundColor(Color::DarkGrey),
        Clear(ClearType::UntilNewLine),
        Print(&stat_line)
    )?;

    // Message log (3 lines)
    for (i, msg) in log.iter().rev().take(3).enumerate() {
        execute!(
            stdout,
            cursor::MoveTo(0, map_height as u16 + 3 + i as u16),
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

    let mut log: Vec<String> = Vec::new();

    'outer: loop {
        // --- CHARACTER CREATION ---
        let chosen_class = ui::character_creation_screen()?;
        let p_color = player_color(chosen_class);

        // Create player ONCE — persists across all floors
        let (term_width, term_height) = terminal::size()?;
        let map_width = term_width as usize;
        let map_height = (term_height as usize).saturating_sub(7);
        let first_map = Map::new(map_width, map_height);
        let (spawn_x, spawn_y) = first_map.get_starting_position();

        let mut player = Player::new(spawn_x, spawn_y, chosen_class);

        // Give starting equipment based on class
        let (start_weapon, start_armor, start_ring) = match chosen_class {
            Class::Warrior => (
                Some(items::warrior_starting_weapon()),
                Some(items::warrior_starting_armor()),
                None,
            ),
            Class::Rogue => (Some(items::rogue_starting_weapon()), None, None),
            Class::Mage => (
                Some(items::mage_starting_weapon()),
                None,
                Some(items::mage_starting_ring()),
            ),
        };
        player.equip_starting_gear(start_weapon, start_armor, start_ring);

        let mut current_floor = 1i32;
        let mut last_key: Option<KeyCode> = None;
        let mut last_move_time = Instant::now();
        log.clear();

        // Store first map in an Option so we can take() it once, then generate new maps
        let mut cached_map: Option<Map> = Some(first_map);

        // Floor loop — player persists, map regenerates each floor
        'floor_loop: loop {
            let (term_width, term_height) = terminal::size()?;
            let map_width = term_width as usize;
            let map_height = (term_height as usize).saturating_sub(7);

            let map = if let Some(m) = cached_map.take() {
                // Floor 1: use the map we already generated (player spawn already set)
                m
            } else {
                // Subsequent floors: generate a new map and update player position
                let m = Map::new(map_width, map_height);
                let (sx, sy) = m.get_starting_position();
                player.x = sx;
                player.y = sy;
                m
            };

            let mut monsters = map.spawn_monsters_for_floor(current_floor);
            let mut projectiles: Vec<Projectile> = Vec::new();
            let mut ground_items = map.spawn_ground_items(current_floor);
            let mut last_monster_tick = Instant::now();

            execute!(stdout, Clear(ClearType::All))?;
            render_map(&mut stdout, &map)?;
            log.push(format!("Welcome to floor {}!", current_floor));

            'inner: loop {
                // --- RENDER ---
                render_ui(&mut stdout, map_height, current_floor, &player, &log)?;
                render_ground_items(&mut stdout, &ground_items)?;
                render_monsters(&mut stdout, &monsters)?;
                render_projectiles(&mut stdout, &projectiles)?;

                execute!(
                    stdout,
                    cursor::MoveTo(player.x as u16, player.y as u16),
                    SetForegroundColor(p_color),
                    Print("@"),
                )?;
                stdout.flush()?;

                // --- NON-BLOCKING INPUT POLL ---
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
                                KeyCode::Tab => {
                                    // Open inventory — PAUSES monster tick
                                    ui::inventory_screen(&mut player)?;
                                    // Redraw everything after closing inventory
                                    execute!(stdout, Clear(ClearType::All))?;
                                    render_map(&mut stdout, &map)?;
                                    // Reset monster tick so they don't all act immediately
                                    last_monster_tick = Instant::now();
                                    continue 'inner;
                                }
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
                                _ => {}
                            }

                            // Only process if position changed
                            if next_x != player.x || next_y != player.y {
                                // --- PLAYER COMBAT (bump-to-attack) ---
                                let mut monster_index = None;
                                for (i, monster) in monsters.iter().enumerate() {
                                    if monster.is_alive()
                                        && monster.x == next_x
                                        && monster.y == next_y
                                    {
                                        monster_index = Some(i);
                                        break;
                                    }
                                }

                                if let Some(i) = monster_index {
                                    let damage = player.melee_damage();
                                    monsters[i].take_damage(damage);
                                    log.push(format!(
                                        "You hit the {} for {} damage!",
                                        monsters[i].name, damage
                                    ));
                                    if !monsters[i].is_alive() {
                                        log.push(format!("The {} dies!", monsters[i].name));
                                        let death_pos = (monsters[i].x, monsters[i].y);
                                        erase_entity(&mut stdout, &map, death_pos.0, death_pos.1)?;

                                        // Monster drop (~30%)
                                        let mut rng = rand::rng();
                                        if rng.random_range(0..100) < 30 {
                                            let drop = items::random_drop(current_floor);
                                            log.push(format!(
                                                "The {} drops a {}!",
                                                monsters[i].name,
                                                drop.display_name()
                                            ));
                                            ground_items.insert(death_pos, drop);
                                        }
                                    }
                                } else if map.is_walkable(next_x, next_y) {
                                    // --- PLAYER MOVEMENT ---
                                    erase_entity(&mut stdout, &map, player.x, player.y)?;

                                    player.x = next_x;
                                    player.y = next_y;

                                    // Check stairs
                                    if map.tiles[player.x][player.y] == Tile::Stairs {
                                        current_floor += 1;
                                        continue 'floor_loop;
                                    }

                                    // Check ground item pickup
                                    let pos = (player.x, player.y);
                                    if let Some(item) = ground_items.get(&pos).cloned() {
                                        if player.add_to_inventory(item.clone()) {
                                            log.push(format!(
                                                "You pick up a {}!",
                                                item.display_name()
                                            ));
                                            ground_items.remove(&pos);
                                        } else {
                                            log.push("Your inventory is full!".to_string());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // --- MONSTER TICK (independent of player) ---
                let now = Instant::now();
                if now.duration_since(last_monster_tick) >= Duration::from_millis(MONSTER_TICK_MS) {
                    last_monster_tick = now;

                    process_projectiles(
                        &mut stdout,
                        &mut projectiles,
                        &monsters,
                        &mut player,
                        &map,
                        &mut log,
                    )?;

                    process_monsters(
                        &mut stdout,
                        &mut monsters,
                        &mut player,
                        &mut projectiles,
                        &map,
                        &mut log,
                        &mut ground_items,
                        current_floor,
                    )?;
                }

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
                        Clear(ClearType::UntilNewLine),
                        Print("YOU DIED! Press 'r' to restart or 'q' to quit.")
                    )?;
                    stdout.flush()?;
                    loop {
                        if let Event::Key(key_event) = event::read()? {
                            match key_event.code {
                                KeyCode::Char('q') | KeyCode::Esc => break 'outer,
                                KeyCode::Char('r') => {
                                    log.clear();
                                    continue 'outer; // Goes back to character creation
                                }
                                _ => {}
                            }
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
