mod items;
mod map;
mod monster;
mod player;
mod projectile;
mod save_load;
mod ui;

use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal::{self, Clear, ClearType},
};
use items::{Item, ItemType};
use map::{DecoObject, Map, RoomType, Tile};
use monster::{Monster, MonsterAction};
use player::{Ability, AbilityType, Class, Player};
use projectile::Projectile;
use rand::RngExt;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{stdout, Write};
use std::time::{Duration, Instant};

const MONSTER_TICK_MS: u64 = 500;
const POLL_TIMEOUT_MS: u64 = 16; // ~60fps loop
const PLAYER_PULSE_MS: u64 = 500; // Player blink/pulse speed

fn render_map(stdout: &mut std::io::Stdout, map: &Map, floor: i32) -> std::io::Result<()> {
    execute!(stdout, cursor::MoveTo(0, 0))?;
    for y in 0..map.height {
        for x in 0..map.width {
            execute!(stdout, cursor::MoveTo(x as u16, y as u16))?;
            let visible = map.current_visibility[x][y];
            let seen = map.visibility[x][y];
            render_tile(stdout, map.tiles[x][y], map, x, y, floor, visible, seen)?;
        }
    }
    Ok(())
}

fn render_map_delta(
    stdout: &mut std::io::Stdout,
    map: &Map,
    floor: i32,
    prev_visibility: &mut Vec<Vec<bool>>,
) -> std::io::Result<()> {
    for x in 0..map.width {
        for y in 0..map.height {
            let now_visible = map.current_visibility[x][y];
            let now_explored = map.visibility[x][y];
            let was_visible = prev_visibility[x][y];

            if now_visible != was_visible {
                execute!(stdout, cursor::MoveTo(x as u16, y as u16))?;
                render_tile(
                    stdout,
                    map.tiles[x][y],
                    map,
                    x,
                    y,
                    floor,
                    now_visible,
                    now_explored,
                )?;
                prev_visibility[x][y] = now_visible;
            }
        }
    }
    Ok(())
}

fn render_tile(
    stdout: &mut std::io::Stdout,
    tile: Tile,
    map: &Map,
    x: usize,
    y: usize,
    floor: i32,
    visible: bool,
    seen: bool,
) -> std::io::Result<()> {
    if !seen {
        execute!(stdout, SetForegroundColor(Color::Black), Print(" "))?;
        return Ok(());
    }

    let room_type = map.get_room_type_at(x, y);

    let mut wall_char = match floor {
        1..=3 => '#',
        4..=6 => '%',
        _ => '+',
    };
    let mut floor_char = match floor {
        1..=3 => '.',
        4..=6 => '.',
        _ => '~',
    };
    let corridor_char = match floor {
        1..=3 => '·',
        4..=6 => ',',
        _ => '~',
    };
    let mut wall_color = match floor {
        1..=3 => Color::Grey,
        4..=6 => Color::DarkYellow,
        _ => Color::DarkRed,
    };
    let mut floor_color = match floor {
        1..=3 => Color::DarkGrey,
        4..=6 => Color::DarkGrey,
        _ => Color::DarkMagenta,
    };

    match room_type {
        RoomType::Treasure => {
            wall_color = Color::Yellow;
            floor_color = Color::DarkYellow;
        }
        RoomType::Trap => {
            floor_char = '^';
        }
        RoomType::Shrine => {
            wall_char = '|';
            floor_char = ':';
            wall_color = Color::Cyan;
            floor_color = Color::DarkCyan;
        }
        RoomType::Secret => {
            wall_char = '?';
            floor_char = '*';
            wall_color = Color::Magenta;
            floor_color = Color::DarkMagenta;
        }
        _ => {}
    }

    if !visible {
        wall_color = Color::DarkGrey;
        floor_color = Color::Black;
    }

    match tile {
        Tile::Wall | Tile::SecretDoor => {
            execute!(stdout, SetForegroundColor(wall_color), Print(wall_char))?
        }
        Tile::Floor => {
            let floor_char = if map.is_corridor(x, y) {
                match floor {
                    1..=3 => '·',
                    4..=6 => ',',
                    _ => '-',
                }
            } else {
                floor_char
            };
            execute!(stdout, SetForegroundColor(floor_color), Print(floor_char))?;
        }
        Tile::Stairs => {
            let color = if visible {
                Color::Yellow
            } else {
                Color::DarkGrey
            };
            execute!(stdout, SetForegroundColor(color), Print('>'))?;
        }
    }
    Ok(())
}

fn render_monsters(
    stdout: &mut std::io::Stdout,
    map: &Map,
    monsters: &[Monster],
) -> std::io::Result<()> {
    for monster in monsters {
        if monster.is_alive() {
            if !map.current_visibility[monster.x][monster.y] {
                continue;
            }
            // Wraiths inside walls are invisible
            if monster.is_phasing {
                continue;
            }
            // Status effect colors override normal colors
            let color = if monster.stun_ticks > 0 {
                Color::DarkGrey
            } else if monster.freeze_ticks > 0 {
                Color::Cyan
            } else if monster.poison_ticks > 0 {
                Color::Green
            } else {
                match monster.symbol {
                    'g' => Color::Green,
                    's' => Color::White,
                    'T' => Color::Red,
                    'b' => Color::DarkYellow,
                    'x' => Color::DarkMagenta,
                    'W' => Color::DarkCyan,
                    'N' => Color::DarkRed,
                    'K' => Color::Yellow,
                    'D' => Color::DarkRed,
                    'S' => Color::Magenta,
                    'z' => Color::Grey,
                    'G' => Color::DarkYellow,
                    'p' => Color::DarkGrey,
                    'i' => Color::Red,
                    'f' => Color::DarkRed,
                    'M' => Color::DarkRed,
                    'B' => Color::Yellow,
                    'w' => Color::White,
                    'O' => Color::DarkYellow,
                    'a' => Color::DarkGrey,
                    'F' => Color::Red,
                    'E' => Color::DarkCyan,
                    'I' => Color::Cyan,
                    'o' => Color::DarkMagenta,
                    _ => Color::Magenta,
                }
            };
            execute!(
                stdout,
                cursor::MoveTo(monster.x as u16, monster.y as u16),
                SetForegroundColor(color),
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
    map: &Map,
    ground_items: &HashMap<(usize, usize), Item>,
) -> std::io::Result<()> {
    for (&(x, y), item) in ground_items {
        if !map.current_visibility[x][y] {
            continue;
        }
        let color = if item.is_artifact {
            Color::Yellow
        } else {
            match item.item_type {
                ItemType::Weapon => Color::Cyan,
                ItemType::Armor => Color::DarkYellow,
                ItemType::Ring => Color::Yellow,
                ItemType::Potion => Color::Magenta,
            }
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

fn render_deco_objects(
    stdout: &mut std::io::Stdout,
    map: &Map,
    pulse_on: bool,
) -> std::io::Result<()> {
    for (&(x, y), deco) in &map.deco_objects {
        if !map.current_visibility[x][y] {
            continue;
        }

        let (char, color) = match deco {
            DecoObject::Torch => {
                if pulse_on {
                    ('*', Color::White)
                } else {
                    ('*', Color::DarkYellow)
                }
            }
            DecoObject::Pillar => ('O', Color::Grey),
            DecoObject::Altar => {
                if map.shrine_used.contains(&(x, y)) {
                    ('&', Color::DarkGrey)
                } else {
                    ('&', Color::Yellow)
                }
            }
            DecoObject::Chest => ('$', Color::Yellow),
        };

        execute!(stdout, cursor::MoveTo(x as u16, y as u16))?;
        execute!(stdout, SetForegroundColor(color), Print(char))?;
    }
    Ok(())
}

fn render_minimap(
    stdout: &mut std::io::Stdout,
    map: &Map,
    player_x: usize,
    player_y: usize,
    term_width: u16,
    map_height: usize,
) -> std::io::Result<()> {
    let mini_w: usize = 30;
    let mini_h: usize = 15;

    let scale_x = map.width as f32 / mini_w as f32;
    let scale_y = map.height as f32 / mini_h as f32;

    let start_col = (term_width as usize).saturating_sub(mini_w + 2) as u16;
    let start_row = map_height.saturating_sub(mini_h + 1) as u16;

    execute!(
        stdout,
        cursor::MoveTo(start_col - 1, start_row - 1),
        SetForegroundColor(Color::DarkGrey),
        Print(format!("┌{}┐", "─".repeat(mini_w)))
    )?;

    for my in 0..mini_h {
        execute!(
            stdout,
            cursor::MoveTo(start_col - 1, start_row + my as u16),
            SetForegroundColor(Color::DarkGrey),
            Print("│")
        )?;
        execute!(
            stdout,
            cursor::MoveTo(start_col + mini_w as u16, start_row + my as u16),
            SetForegroundColor(Color::DarkGrey),
            Print("│")
        )?;
    }
    execute!(
        stdout,
        cursor::MoveTo(start_col - 1, start_row + mini_h as u16),
        SetForegroundColor(Color::DarkGrey),
        Print(format!("└{}┘", "─".repeat(mini_w)))
    )?;

    let mut stairs_mini: Option<(usize, usize)> = None;
    for x in 0..map.width {
        for y in 0..map.height {
            if map.tiles[x][y] == Tile::Stairs && map.visibility[x][y] {
                let mx = (x as f32 / scale_x) as usize;
                let my = (y as f32 / scale_y) as usize;
                stairs_mini = Some((mx.min(mini_w - 1), my.min(mini_h - 1)));
            }
        }
    }

    for my in 0..mini_h {
        for mx in 0..mini_w {
            let real_x = (mx as f32 * scale_x) as usize;
            let real_y = (my as f32 * scale_y) as usize;

            let col = start_col + mx as u16;
            let row = start_row + my as u16;

            execute!(stdout, cursor::MoveTo(col, row))?;

            let player_mx = (player_x as f32 / scale_x) as usize;
            let player_my = (player_y as f32 / scale_y) as usize;

            if mx == player_mx.min(mini_w - 1) && my == player_my.min(mini_h - 1) {
                execute!(stdout, SetForegroundColor(Color::White), Print("@"))?;
                continue;
            }

            if let Some((sx, sy)) = stairs_mini {
                if mx == sx && my == sy {
                    execute!(stdout, SetForegroundColor(Color::Yellow), Print(">"))?;
                    continue;
                }
            }

            if real_x < map.width && real_y < map.height && map.visibility[real_x][real_y] {
                let (glyph, color) = match map.tiles[real_x][real_y] {
                    Tile::Wall | Tile::SecretDoor => ("#", Color::DarkGrey),
                    Tile::Floor | Tile::Stairs => (".", Color::Black),
                };

                let color = if map.current_visibility[real_x][real_y] {
                    match map.tiles[real_x][real_y] {
                        Tile::Wall | Tile::SecretDoor => Color::Grey,
                        _ => Color::DarkGrey,
                    }
                } else {
                    color
                };

                execute!(stdout, SetForegroundColor(color), Print(glyph))?;
            } else {
                execute!(stdout, SetForegroundColor(Color::Black), Print(" "))?;
            }
        }
    }

    Ok(())
}

fn show_boss_intro(
    stdout: &mut std::io::Stdout,
    floor: i32,
    term_width: u16,
    term_height: u16,
) -> std::io::Result<()> {
    let cx = (term_width / 2) as u16;
    let cy = (term_height / 2) as u16;

    for row in cy - 8..cy + 6 {
        execute!(
            stdout,
            cursor::MoveTo(cx - 20, row),
            SetForegroundColor(Color::Black),
            Print(" ".repeat(40))
        )?;
    }

    match floor {
        5 => {
            let art = vec![
                ("    /\\  K  /\\    ", Color::Yellow),
                ("   /  \\_^_/  \\   ", Color::Yellow),
                ("  |  (o) (o)  |  ", Color::Yellow),
                ("   \\  \\___/  /   ", Color::Yellow),
                ("    |  |||  |    ", Color::DarkYellow),
                ("   _|__|_|__|_   ", Color::DarkYellow),
                ("  g . . . . . g  ", Color::Green),
            ];
            let title = "*** THE GOBLIN KING ***";
            let subtitle = "His crown gleams with stolen gold...";

            for (i, (line, color)) in art.iter().enumerate() {
                execute!(
                    stdout,
                    cursor::MoveTo(cx - 10, cy - 7 + i as u16),
                    SetForegroundColor(*color),
                    Print(line)
                )?;
                stdout.flush()?;
                std::thread::sleep(Duration::from_millis(120));
            }
            std::thread::sleep(Duration::from_millis(300));
            execute!(
                stdout,
                cursor::MoveTo(cx - 12, cy + 2),
                SetForegroundColor(Color::Yellow),
                Print(title)
            )?;
            stdout.flush()?;
            std::thread::sleep(Duration::from_millis(200));
            execute!(
                stdout,
                cursor::MoveTo(cx - 18, cy + 4),
                SetForegroundColor(Color::DarkGrey),
                Print(subtitle)
            )?;
        }
        10 => {
            let art = vec![
                ("    __        __   ", Color::DarkRed),
                ("   /  \\  D  /  \\  ", Color::DarkRed),
                ("--( )--^^^--( )--  ", Color::DarkRed),
                ("   \\__|___|__/     ", Color::DarkRed),
                ("       |||         ", Color::DarkRed),
                ("      /   \\        ", Color::DarkRed),
                (" ~~~~fire~~~~      ", Color::Red),
            ];
            let title = "*** THE BONE DRAGON ***";
            let subtitle = "Ancient bones creak in the darkness...";

            for (i, (line, color)) in art.iter().enumerate() {
                execute!(
                    stdout,
                    cursor::MoveTo(cx - 10, cy - 7 + i as u16),
                    SetForegroundColor(*color),
                    Print(line)
                )?;
                stdout.flush()?;
                std::thread::sleep(Duration::from_millis(120));
            }
            std::thread::sleep(Duration::from_millis(300));
            execute!(
                stdout,
                cursor::MoveTo(cx - 12, cy + 2),
                SetForegroundColor(Color::DarkRed),
                Print(title)
            )?;
            stdout.flush()?;
            std::thread::sleep(Duration::from_millis(200));
            execute!(
                stdout,
                cursor::MoveTo(cx - 20, cy + 4),
                SetForegroundColor(Color::DarkGrey),
                Print(subtitle)
            )?;
        }
        _ => {
            let art = vec![
                ("   *   \\|/   *    ", Color::Magenta),
                ("  *  --[S]--  *   ", Color::Magenta),
                ("   *   /|\\   *    ", Color::Magenta),
                ("  * *  | |  * *   ", Color::DarkMagenta),
                ("      (   )       ", Color::DarkMagenta),
                ("  ~shadow pools~  ", Color::DarkGrey),
                (" * * darkness * * ", Color::DarkGrey),
            ];
            let title = "*** THE SHADOW LORD ***";
            let subtitle = "Darkness pulses from every corner...";

            for (i, (line, color)) in art.iter().enumerate() {
                execute!(
                    stdout,
                    cursor::MoveTo(cx - 10, cy - 7 + i as u16),
                    SetForegroundColor(*color),
                    Print(line)
                )?;
                stdout.flush()?;
                std::thread::sleep(Duration::from_millis(120));
            }
            std::thread::sleep(Duration::from_millis(300));
            execute!(
                stdout,
                cursor::MoveTo(cx - 12, cy + 2),
                SetForegroundColor(Color::Magenta),
                Print(title)
            )?;
            stdout.flush()?;
            std::thread::sleep(Duration::from_millis(200));
            execute!(
                stdout,
                cursor::MoveTo(cx - 20, cy + 4),
                SetForegroundColor(Color::DarkGrey),
                Print(subtitle)
            )?;
        }
    }

    stdout.flush()?;
    std::thread::sleep(Duration::from_millis(400));
    execute!(
        stdout,
        cursor::MoveTo(cx - 14, cy + 6),
        SetForegroundColor(Color::DarkGrey),
        Print("[ press any key to face your fate ]")
    )?;
    stdout.flush()?;

    loop {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(_) = event::read()? {
                break;
            }
        }
    }

    Ok(())
}

fn render_webs(
    stdout: &mut std::io::Stdout,
    map: &Map,
    webs: &std::collections::HashSet<(usize, usize)>,
) -> std::io::Result<()> {
    for &(x, y) in webs {
        if !map.current_visibility[x][y] {
            continue;
        }
        execute!(
            stdout,
            cursor::MoveTo(x as u16, y as u16),
            SetForegroundColor(Color::White),
            Print(":")
        )?;
    }
    Ok(())
}

fn show_death_screen(
    stdout: &mut std::io::Stdout,
    player: &Player,
    current_floor: i32,
) -> std::io::Result<bool> {
    use crossterm::event::{Event, KeyCode};

    execute!(stdout, Clear(ClearType::All))?;

    let (term_width, term_height) = terminal::size()?;
    let cx = term_width / 2;
    let cy = term_height / 2;

    let skull = vec![
        "  . . . X . . .",
        ". X X X X X X X .",
        "X X X X X X X X X",
        ". X X X X X X X .",
        "  . . X X X . .",
        "      @ @ @",
    ];

    for (i, line) in skull.iter().enumerate() {
        execute!(
            stdout,
            cursor::MoveTo(cx - 10, cy - 12 + i as u16),
            SetForegroundColor(Color::DarkGrey),
            Print(line)
        )?;
        stdout.flush()?;
        std::thread::sleep(Duration::from_millis(80));
    }

    for _ in 0..3 {
        execute!(
            stdout,
            cursor::MoveTo(cx - 10, cy - 4),
            SetForegroundColor(Color::DarkRed),
            Print("  * YOU HAVE DIED *")
        )?;
        stdout.flush()?;
        std::thread::sleep(Duration::from_millis(200));
        execute!(
            stdout,
            cursor::MoveTo(cx - 10, cy - 4),
            SetForegroundColor(Color::Black),
            Print("  * YOU HAVE DIED *")
        )?;
        stdout.flush()?;
        std::thread::sleep(Duration::from_millis(200));
    }

    execute!(
        stdout,
        cursor::MoveTo(cx - 10, cy - 4),
        SetForegroundColor(Color::DarkRed),
        Print("  * YOU HAVE DIED *")
    )?;

    let stats_box_w: usize = 35;
    let stats = vec![
        ("Class", player.class.name().to_string(), Color::Grey),
        ("Floor reached", current_floor.to_string(), Color::Grey),
        (
            "Monsters slain",
            player.monsters_slain.to_string(),
            Color::Grey,
        ),
        ("Damage dealt", player.damage_dealt.to_string(), Color::Grey),
        ("Damage taken", player.damage_taken.to_string(), Color::Grey),
        (
            "Cause of death",
            player.cause_of_death.clone(),
            Color::DarkRed,
        ),
    ];

    let box_x = cx - (stats_box_w / 2) as u16;
    let box_y = cy - 2;
    let box_h = stats.len() as u16 + 2;

    // Top border
    execute!(
        stdout,
        cursor::MoveTo(box_x - 1, box_y),
        SetForegroundColor(Color::DarkGrey),
        Print(format!("╔{}╗", "═".repeat(stats_box_w)))
    )?;
    stdout.flush()?;
    std::thread::sleep(Duration::from_millis(80));

    // Stats lines with side borders
    for (i, (label, value, color)) in stats.iter().enumerate() {
        let row = box_y + 1 + i as u16;
        execute!(
            stdout,
            cursor::MoveTo(box_x - 1, row),
            SetForegroundColor(Color::DarkGrey),
            Print(format!("║ {:20} │ ", label))
        )?;
        execute!(
            stdout,
            cursor::MoveTo(box_x + 24, row),
            SetForegroundColor(*color),
            Print(value)
        )?;
        stdout.flush()?;
        std::thread::sleep(Duration::from_millis(80));
    }

    // Bottom border
    execute!(
        stdout,
        cursor::MoveTo(box_x - 1, box_y + box_h),
        SetForegroundColor(Color::DarkGrey),
        Print(format!("╚{}╝", "═".repeat(stats_box_w)))
    )?;

    stdout.flush()?;
    std::thread::sleep(Duration::from_millis(200));

    execute!(
        stdout,
        cursor::MoveTo(cx - 16, cy + stats.len() as u16 + 3),
        SetForegroundColor(Color::DarkGrey),
        Print("[R] Play again          [Q] Quit")
    )?;
    stdout.flush()?;

    loop {
        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Char('r') | KeyCode::Char('R') => return Ok(true),
                KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => return Ok(false),
                _ => {}
            }
        }
    }
}

/// Erase a position by restoring the underlying tile.
fn erase_entity(
    stdout: &mut std::io::Stdout,
    map: &Map,
    x: usize,
    y: usize,
    floor: i32,
) -> std::io::Result<()> {
    execute!(stdout, cursor::MoveTo(x as u16, y as u16))?;
    let visible = map.current_visibility[x][y];
    let seen = map.visibility[x][y];
    render_tile(stdout, map.tiles[x][y], map, x, y, floor, visible, seen)?;
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

/// Player base color - always bright white for visibility
fn player_color(_class: Class) -> Color {
    Color::White
}

/// Process projectile movement and collisions.
fn process_projectiles(
    stdout: &mut std::io::Stdout,
    projectiles: &mut Vec<Projectile>,
    monsters: &[Monster],
    player: &mut Player,
    map: &Map,
    log: &mut Vec<String>,
    floor: i32,
) -> std::io::Result<()> {
    let mut i = 0;
    while i < projectiles.len() {
        let old_x = projectiles[i].x;
        let old_y = projectiles[i].y;
        erase_entity(stdout, map, old_x, old_y, floor)?;

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
                player.last_damage_source = Some(("Arrow".to_string(), dmg));
                player.take_damage(dmg);
                player.damage_taken += dmg;
                log.push(format!("<< An arrow hits you for {} damage!", dmg));
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
    current_floor: i32,
    webs: &mut HashSet<(usize, usize)>,
) -> std::io::Result<()> {
    let player_pos = (player.x, player.y);
    let all_positions = all_monster_positions(monsters);

    // Collect dead monster indices for Necromancer
    let dead_indices: Vec<usize> = monsters
        .iter()
        .enumerate()
        .filter(|(_, m)| !m.is_alive() && m.death_pos.is_some())
        .map(|(i, _)| i)
        .collect();

    for i in 0..monsters.len() {
        if !monsters[i].is_alive() {
            continue;
        }

        // --- Status effect processing ---
        // Poison: deal 1 damage per tick
        if monsters[i].poison_ticks > 0 {
            monsters[i].poison_ticks -= 1;
            monsters[i].take_damage(1);
            if !monsters[i].is_alive() {
                player.monsters_slain += 1;
                log.push(format!("The {} dies from poison!", monsters[i].name));
                // XP reward for poison kill
                let xp = monsters[i].xp_value();
                let level_msgs = player.gain_xp(xp);
                log.push(format!("+{} XP", xp));
                for msg in level_msgs {
                    log.push(msg);
                }
                monsters[i].death_pos = Some((monsters[i].x, monsters[i].y));
                erase_entity(stdout, map, monsters[i].x, monsters[i].y, current_floor)?;
                continue;
            }
        }
        // Stun: skip turn
        if monsters[i].stun_ticks > 0 {
            monsters[i].stun_ticks -= 1;
            continue;
        }
        // Freeze: skip turn
        if monsters[i].freeze_ticks > 0 {
            monsters[i].freeze_ticks -= 1;
            continue;
        }

        let occupied = occupied_positions(monsters, i);
        let was_berserk = monsters[i].is_berserk;
        let action =
            monsters[i].decide_action(player_pos, map, &occupied, &all_positions, &dead_indices);

        if monsters[i].is_berserk && !was_berserk {
            log.push("The Troll flies into a rage!".to_string());
        }

        match action {
            MonsterAction::Nothing => {}
            MonsterAction::MeleeAttack { damage, ref name } => {
                // Player dodge check
                if player.try_dodge() {
                    log.push(format!("You dodge the {}'s attack!", name));
                    if player.has_wraithwalkers() {
                        player.activate_damage_buff();
                        log.push("Wraithwalkers! Next hit = 2x dmg!".to_string());
                    }
                } else {
                    let dmg = player.reduce_damage(damage);
                    player.last_damage_source = Some((name.to_string(), dmg));
                    player.take_damage(damage);
                    player.damage_taken += dmg;
                    log.push(format!("<< The {} hits you for {} damage!", name, dmg));

                    // Stonehide Plate: trigger below 30% HP
                    if !player.stonehide_triggered && player.has_stonehide() {
                        let hp_pct = player.hp * 100 / player.max_hp;
                        if hp_pct <= 30 {
                            player.stonehide_triggered = true;
                            player.stonehide_bonus = 3;
                            log.push("Stonehide Plate activates! +3 DEF permanently!".to_string());
                        }
                    }
                }
            }
            MonsterAction::PoisonAttack {
                damage,
                ref name,
                poison_ticks,
            } => {
                if player.try_dodge() {
                    log.push(format!("You dodge the {}'s venomous bite!", name));
                    if player.has_wraithwalkers() {
                        player.activate_damage_buff();
                        log.push("Wraithwalkers! Next hit = 2x dmg!".to_string());
                    }
                } else {
                    let dmg = player.reduce_damage(damage);
                    player.last_damage_source = Some((name.to_string(), dmg));
                    player.take_damage(dmg);
                    player.damage_taken += dmg;
                    player.poison_ticks = poison_ticks;
                    log.push(format!(
                        "The {} bites you for {} damage! Poison courses through your veins!",
                        name, dmg
                    ));
                }
            }
            MonsterAction::DrainAttack { damage, ref name } => {
                if player.try_dodge() {
                    log.push(format!("You dodge the {}'s spectral grasp!", name));
                    if player.has_wraithwalkers() {
                        player.activate_damage_buff();
                        log.push("Wraithwalkers! Next hit = 2x dmg!".to_string());
                    }
                } else {
                    let dmg = player.reduce_damage(damage);
                    player.last_damage_source = Some((name.to_string(), dmg));
                    player.take_damage(dmg);
                    player.damage_taken += dmg;
                    let heal = dmg / 2;
                    monsters[i].hp = (monsters[i].hp + heal).min(monsters[i].max_hp);
                    log.push(format!(
                        "The {} drains {} life from you! (heals {})",
                        name, dmg, heal
                    ));
                }
            }
            MonsterAction::PlaceWeb(wx, wy) => {
                if map.is_walkable(wx, wy) {
                    webs.insert((wx, wy));
                    // Don't log — spider places webs silently
                }
            }
            MonsterAction::Resurrect(dead_idx) => {
                if dead_idx < monsters.len() && !monsters[dead_idx].is_alive() {
                    if let Some((dx, dy)) = monsters[dead_idx].death_pos {
                        let restore_pct = if monsters[i].floor_tier >= 3 { 75 } else { 50 };
                        let restored_hp = (monsters[dead_idx].max_hp * restore_pct / 100).max(1);
                        monsters[dead_idx].hp = restored_hp;
                        monsters[dead_idx].x = dx;
                        monsters[dead_idx].y = dy;
                        monsters[dead_idx].death_pos = None;
                        monsters[dead_idx].stun_ticks = 0;
                        monsters[dead_idx].freeze_ticks = 0;
                        monsters[dead_idx].poison_ticks = 0;
                        monsters[dead_idx].behavior = monster::BehaviorState::Idle;
                        log.push(format!(
                            "The Necromancer raises the {} from the dead!",
                            monsters[dead_idx].name
                        ));
                    }
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
                    erase_entity(stdout, map, monsters[i].x, monsters[i].y, current_floor)?;
                    monsters[i].x = nx;
                    monsters[i].y = ny;
                    // Wraith: exiting wall -> no longer phasing
                    monsters[i].is_phasing = false;

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
                                    if player.has_wraithwalkers() {
                                        player.activate_damage_buff();
                                        log.push("Wraithwalkers! Next hit = 2x dmg!".to_string());
                                    }
                                } else {
                                    let dmg = player.reduce_damage(damage);
                                    player.last_damage_source = Some(("Goblin".to_string(), dmg));
                                    player.take_damage(dmg);
                                    player.damage_taken += dmg;
                                    log.push(format!(
                                        "The Goblin dashes and strikes for {} damage!",
                                        dmg
                                    ));
                                }
                            } else if !blocked2 && map.is_walkable(next2.0, next2.1) {
                                erase_entity(
                                    stdout,
                                    map,
                                    monsters[i].x,
                                    monsters[i].y,
                                    current_floor,
                                )?;
                                monsters[i].x = next2.0;
                                monsters[i].y = next2.1;
                            }
                        }
                    }
                }
            }
            MonsterAction::MoveToPhase(nx, ny) => {
                // Wraith phase movement — can go through walls
                if nx < map.width && ny < map.height {
                    // Don't move onto player
                    if nx == player.x && ny == player.y {
                        // Already handled by adjacent check in AI
                    } else {
                        erase_entity(stdout, map, monsters[i].x, monsters[i].y, current_floor)?;
                        monsters[i].x = nx;
                        monsters[i].y = ny;
                        // Check if now inside a wall
                        monsters[i].is_phasing = !map.is_walkable(nx, ny);
                    }
                }
            }
            MonsterAction::FireProjectile(proj) => {
                log.push(format!("The {} fires an arrow!", monsters[i].name));
                projectiles.push(proj);
            }
            MonsterAction::BossSummon => {
                // Goblin King summons goblin minions near himself
                let enraged = monsters[i].hp * 100 / monsters[i].max_hp < 50;
                let count = if enraged { 3 } else { 2 };
                let bx = monsters[i].x;
                let by = monsters[i].y;
                let floor = monsters[i].floor_tier * 3; // approximate floor from tier
                let mut spawned = 0;
                let offsets: [(i32, i32); 8] = [
                    (-1, -1),
                    (0, -1),
                    (1, -1),
                    (-1, 0),
                    (1, 0),
                    (-1, 1),
                    (0, 1),
                    (1, 1),
                ];
                for &(dx, dy) in &offsets {
                    if spawned >= count {
                        break;
                    }
                    let nx = bx as i32 + dx;
                    let ny = by as i32 + dy;
                    if nx >= 0 && ny >= 0 {
                        let (ux, uy) = (nx as usize, ny as usize);
                        if ux < map.width
                            && uy < map.height
                            && map.is_walkable(ux, uy)
                            && !(ux == player.x && uy == player.y)
                        {
                            let mut blocked = false;
                            for m in monsters.iter() {
                                if m.is_alive() && m.x == ux && m.y == uy {
                                    blocked = true;
                                    break;
                                }
                            }
                            if !blocked {
                                let mut goblin = monster::Monster::new(
                                    ux,
                                    uy,
                                    monster::MonsterType::Goblin,
                                    floor,
                                );
                                goblin.can_see_player = true;
                                goblin.behavior = monster::BehaviorState::Chase;
                                monsters.push(goblin);
                                spawned += 1;
                            }
                        }
                    }
                }
                if spawned > 0 {
                    log.push(format!(
                        "The Goblin King bellows! {} goblins rush to his aid!",
                        spawned
                    ));
                }
            }
            MonsterAction::BreathAttack {
                dx,
                dy,
                damage,
                range,
            } => {
                // Bone Dragon breath: line AoE in direction
                log.push(format!("The Bone Dragon unleashes a torrent of fire!"));
                let mut cx = monsters[i].x as i32;
                let mut cy = monsters[i].y as i32;
                for _ in 0..range {
                    cx += dx;
                    cy += dy;
                    if cx < 0 || cy < 0 || cx >= map.width as i32 || cy >= map.height as i32 {
                        break;
                    }
                    let (ux, uy) = (cx as usize, cy as usize);
                    if !map.is_walkable(ux, uy) {
                        break; // Breath stops at walls
                    }
                    // Hit player if in the line
                    if ux == player.x && uy == player.y {
                        if player.try_dodge() {
                            log.push("You duck under the dragon's breath!".to_string());
                            if player.has_wraithwalkers() {
                                player.activate_damage_buff();
                                log.push("Wraithwalkers! Next hit = 2x dmg!".to_string());
                            }
                        } else {
                            let dmg = player.reduce_damage(damage);
                            player.last_damage_source = Some(("Bone Dragon".to_string(), dmg));
                            player.take_damage(dmg);
                            player.damage_taken += dmg;
                            log.push(format!("[**] Dragon fire engulfs you for {} damage!", dmg));
                        }
                    }
                }
            }
            MonsterAction::ShadowPulse { damage, radius } => {
                // Shadow Lord AoE pulse centered on self
                let sx = monsters[i].x;
                let sy = monsters[i].y;
                let dist_to_player = Map::distance(sx, sy, player.x, player.y);
                log.push("The Shadow Lord releases a wave of dark energy!".to_string());
                if dist_to_player <= radius {
                    if player.try_dodge() {
                        log.push("You resist the shadow pulse!".to_string());
                        if player.has_wraithwalkers() {
                            player.activate_damage_buff();
                            log.push("Wraithwalkers! Next hit = 2x dmg!".to_string());
                        }
                    } else {
                        let dmg = player.reduce_damage(damage);
                        player.last_damage_source = Some(("Shadow Lord".to_string(), dmg));
                        player.take_damage(dmg);
                        player.damage_taken += dmg;
                        log.push(format!(
                            "[~~] Shadow energy tears at you for {} damage!",
                            dmg
                        ));
                    }
                }
            }
            MonsterAction::BossTeleport => {
                // Shadow Lord teleports to a random walkable tile near the player
                let mut rng = rand::rng();
                let px = player.x;
                let py = player.y;
                // Collect valid tiles within 3-6 range of player
                let mut candidates: Vec<(usize, usize)> = Vec::new();
                let search = 8;
                let min_x = (px as i32 - search).max(0) as usize;
                let max_x = ((px as i32 + search) as usize).min(map.width - 1);
                let min_y = (py as i32 - search).max(0) as usize;
                let max_y = ((py as i32 + search) as usize).min(map.height - 1);
                for tx in min_x..=max_x {
                    for ty in min_y..=max_y {
                        let d = Map::distance(tx, ty, px, py);
                        if d >= 2 && d <= 5 && map.is_walkable(tx, ty) && !(tx == px && ty == py) {
                            let mut occ = false;
                            for m in monsters.iter() {
                                if m.is_alive() && m.x == tx && m.y == ty {
                                    occ = true;
                                    break;
                                }
                            }
                            if !occ {
                                candidates.push((tx, ty));
                            }
                        }
                    }
                }
                if !candidates.is_empty() {
                    let idx = rng.random_range(0..candidates.len());
                    let (nx, ny) = candidates[idx];
                    erase_entity(stdout, map, monsters[i].x, monsters[i].y, current_floor)?;
                    monsters[i].x = nx;
                    monsters[i].y = ny;
                    log.push(
                        "The Shadow Lord vanishes and reappears in a swirl of darkness!"
                            .to_string(),
                    );
                }
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
    player_x: usize,
    player_y: usize,
    monsters: &[Monster],
) -> std::io::Result<()> {
    let stats = player.effective_stats();
    let weapon_name = player
        .equipment
        .weapon
        .as_ref()
        .map_or("Fists".to_string(), |w| w.display_name());

    // Status line 1: Floor, HP, Level, Class, Weapon
    let hp_bar_len = 12;
    let hp_pct = (player.hp as f32 / player.max_hp as f32).min(1.0);
    let hp_filled = (hp_pct * hp_bar_len as f32) as usize;
    let hp_empty = hp_bar_len - hp_filled;
    let hp_bar: String = format!("[{}{}]", "█".repeat(hp_filled), "░".repeat(hp_empty));
    let hp_color = if hp_pct > 0.5 {
        Color::Green
    } else if hp_pct > 0.25 {
        Color::Yellow
    } else {
        Color::Red
    };

    let status = format!(
        "Floor: {} │ HP {} │ Lv:{} {} │ Pos:({},{})",
        current_floor, hp_bar, player.level, weapon_name, player_x, player_y
    );
    execute!(
        stdout,
        cursor::MoveTo(0, map_height as u16 + 1),
        SetForegroundColor(Color::White),
        Clear(ClearType::UntilNewLine),
        Print(&status)
    )?;

    // Draw HP bar color overlay
    let bar_start = "Floor: ".len() + "│ HP ".len() + 1;
    execute!(
        stdout,
        cursor::MoveTo(bar_start as u16, map_height as u16 + 1),
        SetForegroundColor(hp_color),
        Print(&hp_bar)
    )?;

    // Status line 2: XP bar + Stats
    let xp_pct = if player.xp_to_next_level > 0 {
        (player.xp as f32 / player.xp_to_next_level as f32 * 100.0).min(100.0) as i32
    } else {
        100
    };
    let xp_bar_len: usize = 10;
    let filled = (xp_pct * xp_bar_len as i32 / 100) as usize;
    let xp_bar: String = format!(
        "[{}{}]",
        "█".repeat(filled),
        "░".repeat(xp_bar_len.saturating_sub(filled))
    );
    let stat_line = format!(
        "XP {}/{} │ {} │ STR:{} DEX:{} INT:{} CON:{} │ Def:{}",
        player.xp,
        player.xp_to_next_level,
        xp_bar,
        stats.str_,
        stats.dex,
        stats.int,
        stats.con,
        player.equipment.armor_defense()
    );
    execute!(
        stdout,
        cursor::MoveTo(0, map_height as u16 + 2),
        SetForegroundColor(Color::Cyan),
        Clear(ClearType::UntilNewLine),
        Print(&stat_line)
    )?;

    // Status line 3: Abilities
    let a1_text = player
        .ability_1
        .as_ref()
        .map(|a| format!("[1]{}", a.status_text()))
        .unwrap_or_default();
    let a2_text = player
        .ability_2
        .as_ref()
        .map(|a| format!("[2]{}", a.status_text()))
        .unwrap_or_else(|| {
            if player.level < 5 {
                "[2]Locked (Lv5)".to_string()
            } else {
                String::new()
            }
        });
    let a3_text = player
        .ability_3
        .as_ref()
        .map(|a| format!("[3]{}", a.status_text()))
        .unwrap_or_else(|| {
            if player.level < 10 {
                "[3]Locked (Lv10)".to_string()
            } else {
                String::new()
            }
        });
    let a4_text = player
        .ability_4
        .as_ref()
        .map(|a| format!("[4]{}", a.status_text()))
        .unwrap_or_else(|| {
            if player.level < 15 {
                "[4]Locked (Lv15)".to_string()
            } else {
                String::new()
            }
        });
    let a5_text = player
        .ability_5
        .as_ref()
        .map(|a| format!("[5]{}", a.status_text()))
        .unwrap_or_else(|| {
            if player.level < 20 {
                "[5]Locked (Lv20)".to_string()
            } else {
                String::new()
            }
        });
    let poison_text = if player.poison_ticks > 0 {
        format!(" | POISONED({})", player.poison_ticks)
    } else {
        String::new()
    };
    let mana_shield_text = if player.mana_shield_ticks > 0 {
        format!(" | M-SHIELD({})", player.mana_shield_ticks)
    } else {
        String::new()
    };
    let ability_line = format!(
        "{} {} {} {} {}{}{}",
        a1_text, a2_text, a3_text, a4_text, a5_text, poison_text, mana_shield_text
    );
    execute!(
        stdout,
        cursor::MoveTo(0, map_height as u16 + 3),
        SetForegroundColor(Color::Cyan),
        Clear(ClearType::UntilNewLine),
        Print(&ability_line)
    )?;

    // Message log (3 lines) - shifted down by 1 to make room for ability line
    for (i, msg) in log.iter().rev().take(3).enumerate() {
        execute!(
            stdout,
            cursor::MoveTo(0, map_height as u16 + 4 + i as u16),
            SetForegroundColor(Color::Grey),
            Clear(ClearType::UntilNewLine),
            Print(msg)
        )?;
    }

    // Boss health bar (if boss is nearby)
    for m in monsters.iter() {
        if m.is_boss && m.is_alive() {
            let dist = ((m.x as i32 - player_x as i32).abs() + (m.y as i32 - player_y as i32).abs())
                as usize;
            if dist <= 8 {
                let hp_pct = m.hp as f32 / m.max_hp as f32;
                let bar_len = 20;
                let filled = (hp_pct * bar_len as f32) as usize;
                let hp_bar = format!("[{}{}]", "█".repeat(filled), "░".repeat(bar_len - filled));

                execute!(
                    stdout,
                    cursor::MoveTo(0, map_height as u16 + 4),
                    SetForegroundColor(Color::DarkRed),
                    Clear(ClearType::UntilNewLine),
                    Print(format!("⚠ BOSS: {} │ {}", m.name, hp_bar))
                )?;
                break;
            }
        }
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    terminal::enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, cursor::Hide)?;

    let mut log: Vec<String> = Vec::new();

    'outer: loop {
        // --- TITLE SCREEN ---
        match ui::title_screen()? {
            ui::TitleChoice::Quit => {
                execute!(stdout, cursor::Show)?;
                terminal::disable_raw_mode()?;
                return Ok(());
            }
            ui::TitleChoice::LoadGame => {
                // TODO: Implement save/load
                let (tw, th) = terminal::size()?;
                let _ = execute!(stdout, Clear(ClearType::All));
                execute!(
                    stdout,
                    cursor::MoveTo(tw / 2 - 15, th / 2),
                    SetForegroundColor(Color::Yellow),
                    Print("No saved games found!")
                )?;
                stdout.flush()?;
                std::thread::sleep(std::time::Duration::from_secs(2));
                continue;
            }
            ui::TitleChoice::NewGame => {}
        }

        // --- CHARACTER CREATION ---
        let chosen_class = ui::character_creation_screen()?;
        let p_color = player_color(chosen_class);

        // Create player ONCE — persists across all floors
        let (term_width, term_height) = terminal::size()?;
        let map_width = term_width as usize;
        let map_height = (term_height as usize).saturating_sub(7);
        let mut first_map = Map::new(map_width, map_height);
        first_map.assign_room_types(1);
        first_map.generate_decorations();
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

            let mut map = if let Some(m) = cached_map.take() {
                m
            } else {
                let mut m = Map::new(map_width, map_height);
                m.assign_room_types(current_floor);
                m.generate_decorations();
                let (sx, sy) = m.get_starting_position();
                player.x = sx;
                player.y = sy;
                m
            };

            // Delta visibility tracking for efficient fog of war rendering
            let mut prev_visibility: Vec<Vec<bool>> = vec![vec![false; map.height]; map.width];

            let mut monsters = map.spawn_monsters_for_floor(current_floor);
            let mut projectiles: Vec<Projectile> = Vec::new();
            let mut ground_items = map.spawn_ground_items(current_floor);
            let mut webs: HashSet<(usize, usize)> = HashSet::new();
            let mut player_web_stuck: i32 = 0;
            let mut last_monster_tick = Instant::now();
            let mut pulse_start_time = Instant::now();

            // Progressive reveal animation - expand from player outward
            // Medium speed: ~1-2 seconds to fully reveal
            let max_reveal_radius = 12;
            let frames_per_ring = 3; // ~50ms per ring at 60fps

            execute!(stdout, Clear(ClearType::All))?;

            // Show empty map first
            map.reveal_all();
            render_map(&mut stdout, &map, current_floor)?;
            stdout.flush()?;

            // Hide map again for animation
            for y in 0..map.height {
                for x in 0..map.width {
                    map.current_visibility[x][y] = false;
                }
            }

            // Progressive reveal - ring by ring from player
            let mut frame_count = 0;
            for radius in 0..=max_reveal_radius {
                // Add delay to make it visible to eye
                if frame_count > 0 && frame_count % frames_per_ring == 0 {
                    std::thread::sleep(Duration::from_millis(50));
                }

                map.reveal_ring(player.x, player.y, radius);
                render_map(&mut stdout, &map, current_floor)?;
                stdout.flush()?;

                frame_count += 1;
            }

            log.push(format!("Welcome to floor {}!", current_floor));

            match current_floor {
                5 | 10 | 15 => {
                    execute!(stdout, Clear(ClearType::All))?;
                    let (tw, th) = terminal::size()?;
                    show_boss_intro(&mut stdout, current_floor, tw, th)?;
                    execute!(stdout, Clear(ClearType::All))?;
                    render_map(&mut stdout, &map, current_floor)?;
                    prev_visibility = vec![vec![false; map.height]; map.width];
                    log.push(
                        match current_floor {
                            5 => "The Goblin King awaits!",
                            10 => "The Bone Dragon stirs!",
                            _ => "The Shadow Lord has come!",
                        }
                        .to_string(),
                    );
                }
                _ => {}
            }

            'inner: loop {
                // --- RENDER ---
                render_ui(
                    &mut stdout,
                    map_height,
                    current_floor,
                    &player,
                    &log,
                    player.x,
                    player.y,
                    &monsters,
                )?;
                render_webs(&mut stdout, &map, &webs)?;
                render_deco_objects(&mut stdout, &map, true)?;
                render_ground_items(&mut stdout, &map, &ground_items)?;
                render_monsters(&mut stdout, &map, &monsters)?;
                render_projectiles(&mut stdout, &projectiles)?;
                render_minimap(
                    &mut stdout,
                    &map,
                    player.x,
                    player.y,
                    term_width,
                    map_height,
                )?;

                // Player pulse effect - toggle between White and Yellow
                let pulse_on =
                    (pulse_start_time.elapsed().as_millis() / PLAYER_PULSE_MS as u128) % 2 == 0;

                // Render player with buff-aware color + pulse effect
                let active_p_color = if player.has_damage_buff() {
                    Color::White
                } else if player.poison_ticks > 0 {
                    Color::Green
                } else if pulse_on {
                    Color::White // Bright pulse
                } else {
                    Color::Yellow // Dimmer pulse
                };

                execute!(
                    stdout,
                    cursor::MoveTo(player.x as u16, player.y as u16),
                    SetForegroundColor(active_p_color),
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
                            && now.duration_since(last_move_time) < Duration::from_millis(50);

                        last_key = Some(key_event.code);

                        if !repeat_too_fast {
                            last_move_time = now;
                            let mut ability_dx: i32 = 0;
                            let mut ability_dy: i32 = 0;

                            match key_event.code {
                                KeyCode::Char('q') | KeyCode::Esc => break 'outer,
                                KeyCode::Char('s')
                                    if key_event
                                        .modifiers
                                        .contains(crossterm::event::KeyModifiers::CONTROL) =>
                                {
                                    match save_load::save_game(
                                        &player,
                                        &map,
                                        &monsters,
                                        &ground_items,
                                        current_floor,
                                        &log,
                                    ) {
                                        Ok(_) => log.push("Game saved!".to_string()),
                                        Err(e) => log.push(format!("Save failed: {}", e)),
                                    }
                                }
                                KeyCode::Char('l')
                                    if key_event
                                        .modifiers
                                        .contains(crossterm::event::KeyModifiers::CONTROL) =>
                                {
                                    match save_load::load_game() {
                                        Ok(data) => {
                                            player = data.player;
                                            map = data.map;
                                            monsters = data.monsters;
                                            ground_items.clear();
                                            for (x, y, item) in data.ground_items {
                                                ground_items.insert((x, y), item);
                                            }
                                            current_floor = data.current_floor;
                                            log = data.log;
                                            map.update_visibility(player.x, player.y, 8);
                                            execute!(stdout, Clear(ClearType::All))?;
                                            render_map(&mut stdout, &map, current_floor)?;
                                            render_minimap(
                                                &mut stdout,
                                                &map,
                                                player.x,
                                                player.y,
                                                term_width,
                                                map_height,
                                            )?;
                                            log.push("Game loaded!".to_string());
                                        }
                                        Err(e) => log.push(format!("Load failed: {}", e)),
                                    }
                                }
                                KeyCode::Tab => {
                                    // Open inventory — PAUSES monster tick
                                    ui::inventory_screen(&mut player)?;
                                    // Redraw everything after closing inventory
                                    execute!(stdout, Clear(ClearType::All))?;
                                    render_map(&mut stdout, &map, current_floor)?;
                                    render_minimap(
                                        &mut stdout,
                                        &map,
                                        player.x,
                                        player.y,
                                        term_width,
                                        map_height,
                                    )?;
                                    // Reset monster tick so they don't all act immediately
                                    last_monster_tick = Instant::now();
                                    continue 'inner;
                                }
                                KeyCode::Char('1') => {
                                    if let Some(ref mut a) = player.ability_1 {
                                        if a.is_ready() && !a.is_active {
                                            match a.ability_type {
                                                AbilityType::PowerAttack => {
                                                    a.activate();
                                                    log.push("Power Attack ready! Next melee does 2x damage.".to_string());
                                                }
                                                AbilityType::ShadowStep
                                                | AbilityType::ChainLightning => {
                                                    // Needs directional input
                                                    player.pending_ability_direction = Some(1);
                                                    log.push(format!(
                                                        "{} — choose direction (arrow key).",
                                                        a.name
                                                    ));
                                                }
                                                _ => {
                                                    // Instant abilities handled in step 8
                                                }
                                            }
                                        } else if a.cooldown_remaining > 0 {
                                            log.push(format!(
                                                "{} on cooldown ({} ticks).",
                                                a.name, a.cooldown_remaining
                                            ));
                                        }
                                    }
                                    continue 'inner;
                                }
                                KeyCode::Char('2') => {
                                    if let Some(ref mut a) = player.ability_2 {
                                        if a.is_ready() && !a.is_active {
                                            match a.ability_type {
                                                AbilityType::PoisonBlade => {
                                                    a.activate();
                                                    log.push("Poison Blade active! Next 3 melee hits apply poison.".to_string());
                                                }
                                                AbilityType::WarCry => {
                                                    // Instant AoE stun — implemented in step 8
                                                    a.activate();
                                                    a.is_active = false; // instant, no buff
                                                                         // Stun all monsters within 4 tiles
                                                    let px = player.x;
                                                    let py = player.y;
                                                    let mut stunned_count = 0;
                                                    for m in monsters.iter_mut() {
                                                        if m.is_alive() {
                                                            let d = Map::distance(px, py, m.x, m.y);
                                                            if d <= 4 {
                                                                m.stun_ticks = 2;
                                                                stunned_count += 1;
                                                            }
                                                        }
                                                    }
                                                    log.push(format!(
                                                        "WAR CRY! {} monsters stunned!",
                                                        stunned_count
                                                    ));
                                                }
                                                AbilityType::FrostNova => {
                                                    // Instant AoE freeze + damage — implemented in step 8
                                                    a.activate();
                                                    a.is_active = false; // instant
                                                    let px = player.x;
                                                    let py = player.y;
                                                    let frost_dmg =
                                                        2 + player.effective_stats().potion_bonus(); // INT-based
                                                    let mut frozen_count = 0;
                                                    let mut frost_kills: Vec<(usize, usize, i32)> =
                                                        Vec::new(); // (x, y, xp)
                                                    for m in monsters.iter_mut() {
                                                        if m.is_alive() {
                                                            let d = Map::distance(px, py, m.x, m.y);
                                                            if d <= 3 {
                                                                m.freeze_ticks = 2;
                                                                m.take_damage(frost_dmg);
                                                                frozen_count += 1;
                                                                if !m.is_alive() {
                                                                    m.death_pos = Some((m.x, m.y));
                                                                    frost_kills.push((
                                                                        m.x,
                                                                        m.y,
                                                                        m.xp_value(),
                                                                    ));
                                                                    log.push(format!(
                                                                        "The {} shatters!",
                                                                        m.name
                                                                    ));
                                                                }
                                                            }
                                                        }
                                                    }
                                                    log.push(format!("FROST NOVA! {} monsters frozen for {} damage!", frozen_count, frost_dmg));
                                                    for (kx, ky, xp) in frost_kills {
                                                        erase_entity(
                                                            &mut stdout,
                                                            &map,
                                                            kx,
                                                            ky,
                                                            current_floor,
                                                        )?;
                                                        let level_msgs = player.gain_xp(xp);
                                                        log.push(format!("+{} XP", xp));
                                                        for msg in level_msgs {
                                                            log.push(msg);
                                                        }
                                                    }
                                                }
                                                _ => {}
                                            }
                                        } else if let Some(ref a) = player.ability_2 {
                                            if a.cooldown_remaining > 0 {
                                                log.push(format!(
                                                    "{} on cooldown ({} ticks).",
                                                    a.name, a.cooldown_remaining
                                                ));
                                            }
                                        }
                                    } else {
                                        log.push(
                                            "No second ability yet (unlocks at level 5)."
                                                .to_string(),
                                        );
                                    }
                                    continue 'inner;
                                }
                                KeyCode::Char('3') => {
                                    if let Some(ref mut a) = player.ability_3 {
                                        if a.is_ready() && !a.is_active {
                                            match a.ability_type {
                                                AbilityType::ShieldBash => {
                                                    a.activate();
                                                    a.is_active = false;
                                                    let px = player.x;
                                                    let py = player.y;
                                                    let mut nearest: Option<&mut Monster> = None;
                                                    let mut nearest_dist = 999;
                                                    for m in monsters.iter_mut() {
                                                        if m.is_alive() {
                                                            let d = Map::distance(px, py, m.x, m.y);
                                                            if d < nearest_dist && d <= 2 {
                                                                nearest_dist = d;
                                                                nearest = Some(m);
                                                            }
                                                        }
                                                    }
                                                    if let Some(m) = nearest {
                                                        m.stun_ticks = 2;
                                                        log.push(format!(
                                                            "SHIELD BASH! {} stunned!",
                                                            m.name
                                                        ));
                                                    } else {
                                                        log.push(
                                                            "Shield Bash - no target in range!"
                                                                .to_string(),
                                                        );
                                                    }
                                                }
                                                AbilityType::Backstab => {
                                                    player.pending_ability_direction = Some(3);
                                                    log.push(format!(
                                                        "{} — choose direction (arrow key).",
                                                        a.name
                                                    ));
                                                }
                                                AbilityType::ArcaneMissiles => {
                                                    a.activate();
                                                    a.is_active = false;
                                                    let px = player.x;
                                                    let py = player.y;
                                                    let missile_dmg =
                                                        3 + player.effective_stats().potion_bonus();
                                                    let mut missiles_fired = 0;
                                                    for m in monsters.iter_mut() {
                                                        if m.is_alive() && missiles_fired < 3 {
                                                            let d = Map::distance(px, py, m.x, m.y);
                                                            if d <= 6 {
                                                                m.take_damage(missile_dmg);
                                                                missiles_fired += 1;
                                                                if !m.is_alive() {
                                                                    m.death_pos = Some((m.x, m.y));
                                                                    log.push(format!("Arcane Missile hit {} for {}!", m.name, missile_dmg));
                                                                }
                                                            }
                                                        }
                                                    }
                                                    log.push(format!("ARCANE MISSILES! {} missiles fired for {} damage each.", missiles_fired, missile_dmg));
                                                }
                                                _ => {}
                                            }
                                        } else if a.cooldown_remaining > 0 {
                                            log.push(format!(
                                                "{} on cooldown ({} ticks).",
                                                a.name, a.cooldown_remaining
                                            ));
                                        }
                                    } else {
                                        log.push("Third ability unlocks at level 10.".to_string());
                                    }
                                    continue 'inner;
                                }
                                KeyCode::Char('4') => {
                                    if let Some(ref mut a) = player.ability_4 {
                                        if a.is_ready() && !a.is_active {
                                            match a.ability_type {
                                                AbilityType::BattleCry => {
                                                    a.activate();
                                                    a.is_active = false;
                                                    let px = player.x;
                                                    let py = player.y;
                                                    let mut affected = 0;
                                                    for m in monsters.iter_mut() {
                                                        if m.is_alive() {
                                                            let d = Map::distance(px, py, m.x, m.y);
                                                            if d <= 4 {
                                                                m.attack_reduction = 2;
                                                                affected += 1;
                                                            }
                                                        }
                                                    }
                                                    log.push(format!(
                                                        "BATTLE CRY! {} enemies weakened!",
                                                        affected
                                                    ));
                                                }
                                                AbilityType::FanOfKnives => {
                                                    a.activate();
                                                    a.is_active = false;
                                                    let px = player.x;
                                                    let py = player.y;
                                                    let knife_dmg =
                                                        2 + player.effective_stats().potion_bonus();
                                                    let mut hit_count = 0;
                                                    for m in monsters.iter_mut() {
                                                        if m.is_alive() {
                                                            let d = Map::distance(px, py, m.x, m.y);
                                                            if d <= 4 {
                                                                m.take_damage(knife_dmg);
                                                                hit_count += 1;
                                                                if !m.is_alive() {
                                                                    m.death_pos = Some((m.x, m.y));
                                                                    log.push(format!(
                                                                        "Fan of Knives hit {}!",
                                                                        m.name
                                                                    ));
                                                                }
                                                            }
                                                        }
                                                    }
                                                    log.push(format!("FAN OF KNIVES! {} enemies hit for {} damage!", hit_count, knife_dmg));
                                                }
                                                AbilityType::ManaShield => {
                                                    a.activate();
                                                    player.mana_shield_ticks = 5;
                                                    log.push("MANA SHIELD activated! Absorbing damage for 5 turns.".to_string());
                                                }
                                                _ => {}
                                            }
                                        } else if a.cooldown_remaining > 0 {
                                            log.push(format!(
                                                "{} on cooldown ({} ticks).",
                                                a.name, a.cooldown_remaining
                                            ));
                                        }
                                    } else {
                                        log.push("Fourth ability unlocks at level 15.".to_string());
                                    }
                                    continue 'inner;
                                }
                                KeyCode::Char('5') => {
                                    if let Some(ref mut a) = player.ability_5 {
                                        if a.is_ready() && !a.is_active {
                                            match a.ability_type {
                                                AbilityType::Earthquake => {
                                                    a.activate();
                                                    a.is_active = false;
                                                    let px = player.x;
                                                    let py = player.y;
                                                    let quake_dmg =
                                                        8 + player.effective_stats().potion_bonus();
                                                    let mut hit_count = 0;
                                                    let mut stun_count = 0;
                                                    for m in monsters.iter_mut() {
                                                        if m.is_alive() {
                                                            let d = Map::distance(px, py, m.x, m.y);
                                                            if d <= 5 {
                                                                m.take_damage(quake_dmg);
                                                                m.stun_ticks = 2;
                                                                hit_count += 1;
                                                                stun_count += 1;
                                                                if !m.is_alive() {
                                                                    m.death_pos = Some((m.x, m.y));
                                                                    log.push(format!(
                                                                        "Earthquake crushed {}!",
                                                                        m.name
                                                                    ));
                                                                }
                                                            }
                                                        }
                                                    }
                                                    log.push(format!("EARTHQUAKE! {} enemies hit for {} damage and stunned!", hit_count, quake_dmg));
                                                }
                                                AbilityType::Assassinate => {
                                                    player.pending_ability_direction = Some(5);
                                                    log.push(format!(
                                                        "{} — choose direction (arrow key).",
                                                        a.name
                                                    ));
                                                }
                                                AbilityType::Meteor => {
                                                    a.activate();
                                                    a.is_active = false;
                                                    let px = player.x;
                                                    let py = player.y;
                                                    let meteor_dmg = 10
                                                        + player.effective_stats().potion_bonus();
                                                    let mut nearest_idx: Option<usize> = None;
                                                    let mut nearest_dist = 999;
                                                    for (i, m) in monsters.iter_mut().enumerate() {
                                                        if m.is_alive() {
                                                            let d = Map::distance(px, py, m.x, m.y);
                                                            if d < nearest_dist && d <= 8 {
                                                                nearest_dist = d;
                                                                nearest_idx = Some(i);
                                                            }
                                                        }
                                                    }
                                                    if let Some(idx) = nearest_idx {
                                                        let (mx, my) =
                                                            (monsters[idx].x, monsters[idx].y);
                                                        monsters[idx].take_damage(meteor_dmg);
                                                        log.push(format!(
                                                            "METEOR strikes {} for {} damage!",
                                                            monsters[idx].name, meteor_dmg
                                                        ));
                                                        for sm in monsters.iter_mut() {
                                                            if sm.is_alive() {
                                                                let d = Map::distance(
                                                                    mx, my, sm.x, sm.y,
                                                                );
                                                                if d <= 2 {
                                                                    let splash =
                                                                        (meteor_dmg / 2).max(1);
                                                                    sm.take_damage(splash);
                                                                    if !sm.is_alive() {
                                                                        sm.death_pos =
                                                                            Some((sm.x, sm.y));
                                                                        log.push(format!(
                                                                            "Splash hit {}!",
                                                                            sm.name
                                                                        ));
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        if !monsters[idx].is_alive() {
                                                            monsters[idx].death_pos = Some((
                                                                monsters[idx].x,
                                                                monsters[idx].y,
                                                            ));
                                                        }
                                                    } else {
                                                        log.push(
                                                            "Meteor - no target in range!"
                                                                .to_string(),
                                                        );
                                                    }
                                                }
                                                _ => {}
                                            }
                                        } else if a.cooldown_remaining > 0 {
                                            log.push(format!(
                                                "{} on cooldown ({} ticks).",
                                                a.name, a.cooldown_remaining
                                            ));
                                        }
                                    } else {
                                        log.push(
                                            "Ultimate ability unlocks at level 20.".to_string(),
                                        );
                                    }
                                    continue 'inner;
                                }
                                KeyCode::Char('4') => {
                                    if let Some(ref mut a) = player.ability_4 {
                                        if a.is_ready() && !a.is_active {
                                            match a.ability_type {
                                                AbilityType::BattleCry => {
                                                    a.activate();
                                                    a.is_active = false;
                                                    let px = player.x;
                                                    let py = player.y;
                                                    let mut affected = 0;
                                                    for m in monsters.iter_mut() {
                                                        if m.is_alive() {
                                                            let d = Map::distance(px, py, m.x, m.y);
                                                            if d <= 4 {
                                                                m.attack_reduction = 2;
                                                                affected += 1;
                                                            }
                                                        }
                                                    }
                                                    log.push(format!(
                                                        "BATTLE CRY! {} enemies weakened!",
                                                        affected
                                                    ));
                                                }
                                                AbilityType::FanOfKnives => {
                                                    a.activate();
                                                    a.is_active = false;
                                                    let px = player.x;
                                                    let py = player.y;
                                                    let knife_dmg =
                                                        2 + player.effective_stats().potion_bonus();
                                                    let mut hit_count = 0;
                                                    for m in monsters.iter_mut() {
                                                        if m.is_alive() {
                                                            let d = Map::distance(px, py, m.x, m.y);
                                                            if d <= 4 {
                                                                m.take_damage(knife_dmg);
                                                                hit_count += 1;
                                                                if !m.is_alive() {
                                                                    m.death_pos = Some((m.x, m.y));
                                                                    log.push(format!(
                                                                        "Fan of Knives hit {}!",
                                                                        m.name
                                                                    ));
                                                                }
                                                            }
                                                        }
                                                    }
                                                    log.push(format!("FAN OF KNIVES! {} enemies hit for {} damage!", hit_count, knife_dmg));
                                                }
                                                AbilityType::ManaShield => {
                                                    a.activate();
                                                    player.mana_shield_ticks = 5;
                                                    log.push("MANA SHIELD activated! Absorbing damage for 5 turns.".to_string());
                                                }
                                                _ => {}
                                            }
                                        } else if a.cooldown_remaining > 0 {
                                            log.push(format!(
                                                "{} on cooldown ({} ticks).",
                                                a.name, a.cooldown_remaining
                                            ));
                                        }
                                    } else {
                                        log.push("Fourth ability unlocks at level 15.".to_string());
                                    }
                                    continue 'inner;
                                }
                                KeyCode::Char('5') => {
                                    if let Some(ref mut a) = player.ability_5 {
                                        if a.is_ready() && !a.is_active {
                                            match a.ability_type {
                                                AbilityType::Earthquake => {
                                                    a.activate();
                                                    a.is_active = false;
                                                    let px = player.x;
                                                    let py = player.y;
                                                    let quake_dmg =
                                                        8 + player.effective_stats().potion_bonus();
                                                    let mut hit_count = 0;
                                                    let mut stun_count = 0;
                                                    for m in monsters.iter_mut() {
                                                        if m.is_alive() {
                                                            let d = Map::distance(px, py, m.x, m.y);
                                                            if d <= 5 {
                                                                m.take_damage(quake_dmg);
                                                                m.stun_ticks = 2;
                                                                hit_count += 1;
                                                                stun_count += 1;
                                                                if !m.is_alive() {
                                                                    m.death_pos = Some((m.x, m.y));
                                                                    log.push(format!(
                                                                        "Earthquake crushed {}!",
                                                                        m.name
                                                                    ));
                                                                }
                                                            }
                                                        }
                                                    }
                                                    log.push(format!("EARTHQUAKE! {} enemies hit for {} damage and stunned!", hit_count, quake_dmg));
                                                }
                                                AbilityType::Assassinate => {
                                                    player.pending_ability_direction = Some(5);
                                                    log.push(format!(
                                                        "{} — choose direction (arrow key).",
                                                        a.name
                                                    ));
                                                }
                                                AbilityType::Meteor => {
                                                    a.activate();
                                                    a.is_active = false;
                                                    let px = player.x;
                                                    let py = player.y;
                                                    let meteor_dmg = 10
                                                        + player.effective_stats().potion_bonus();
                                                    let mut nearest_idx: Option<usize> = None;
                                                    let mut nearest_dist = 999;
                                                    for (i, m) in monsters.iter_mut().enumerate() {
                                                        if m.is_alive() {
                                                            let d = Map::distance(px, py, m.x, m.y);
                                                            if d < nearest_dist && d <= 8 {
                                                                nearest_dist = d;
                                                                nearest_idx = Some(i);
                                                            }
                                                        }
                                                    }
                                                    if let Some(idx) = nearest_idx {
                                                        let (mx, my) =
                                                            (monsters[idx].x, monsters[idx].y);
                                                        monsters[idx].take_damage(meteor_dmg);
                                                        log.push(format!(
                                                            "METEOR strikes {} for {} damage!",
                                                            monsters[idx].name, meteor_dmg
                                                        ));
                                                        for sm in monsters.iter_mut() {
                                                            if sm.is_alive() {
                                                                let d = Map::distance(
                                                                    mx, my, sm.x, sm.y,
                                                                );
                                                                if d <= 2 {
                                                                    let splash =
                                                                        (meteor_dmg / 2).max(1);
                                                                    sm.take_damage(splash);
                                                                    if !sm.is_alive() {
                                                                        sm.death_pos =
                                                                            Some((sm.x, sm.y));
                                                                        log.push(format!(
                                                                            "Splash hit {}!",
                                                                            sm.name
                                                                        ));
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        if !monsters[idx].is_alive() {
                                                            monsters[idx].death_pos = Some((
                                                                monsters[idx].x,
                                                                monsters[idx].y,
                                                            ));
                                                        }
                                                    } else {
                                                        log.push(
                                                            "Meteor - no target in range!"
                                                                .to_string(),
                                                        );
                                                    }
                                                }
                                                _ => {}
                                            }
                                        } else if a.cooldown_remaining > 0 {
                                            log.push(format!(
                                                "{} on cooldown ({} ticks).",
                                                a.name, a.cooldown_remaining
                                            ));
                                        }
                                    } else {
                                        log.push(
                                            "Ultimate ability unlocks at level 20.".to_string(),
                                        );
                                    }
                                    continue 'inner;
                                }
                                KeyCode::Up => {
                                    if player.pending_ability_direction.is_some() {
                                        ability_dx = 0;
                                        ability_dy = -1;
                                    } else if next_y > 0 {
                                        next_y -= 1;
                                    }
                                }
                                KeyCode::Down => {
                                    if player.pending_ability_direction.is_some() {
                                        ability_dx = 0;
                                        ability_dy = 1;
                                    } else {
                                        next_y += 1;
                                    }
                                }
                                KeyCode::Left => {
                                    if player.pending_ability_direction.is_some() {
                                        ability_dx = -1;
                                        ability_dy = 0;
                                    } else if next_x > 0 {
                                        next_x -= 1;
                                    }
                                }
                                KeyCode::Right => {
                                    if player.pending_ability_direction.is_some() {
                                        ability_dx = 1;
                                        ability_dy = 0;
                                    } else {
                                        next_x += 1;
                                    }
                                }
                                _ => {
                                    // Any other key cancels pending ability direction
                                    if player.pending_ability_direction.is_some() {
                                        player.pending_ability_direction = None;
                                        log.push("Ability cancelled.".to_string());
                                    }
                                }
                            }

                            // --- DIRECTIONAL ABILITY EXECUTION ---
                            if let Some(slot) = player.pending_ability_direction {
                                if ability_dx != 0 || ability_dy != 0 {
                                    player.pending_ability_direction = None;

                                    let ability_ref: Option<&Ability> = match slot {
                                        1 => player.ability_1.as_ref(),
                                        2 => player.ability_2.as_ref(),
                                        3 => player.ability_3.as_ref(),
                                        4 => player.ability_4.as_ref(),
                                        5 => player.ability_5.as_ref(),
                                        _ => None,
                                    };

                                    if let Some(a) = ability_ref {
                                        match a.ability_type {
                                            AbilityType::ShadowStep => {
                                                erase_entity(
                                                    &mut stdout,
                                                    &map,
                                                    player.x,
                                                    player.y,
                                                    current_floor,
                                                )?;
                                                let mut land_x = player.x as i32;
                                                let mut land_y = player.y as i32;
                                                for _ in 1..=4 {
                                                    let tx = land_x + ability_dx;
                                                    let ty = land_y + ability_dy;
                                                    if tx < 0
                                                        || ty < 0
                                                        || tx as usize >= map.width
                                                        || ty as usize >= map.height
                                                    {
                                                        break;
                                                    }
                                                    if !map.is_walkable(tx as usize, ty as usize) {
                                                        break;
                                                    }
                                                    land_x = tx;
                                                    land_y = ty;
                                                }
                                                player.x = land_x as usize;
                                                player.y = land_y as usize;

                                                // Check if any monster adjacent to landing = shadow strike buff
                                                let mut adjacent_monster = false;
                                                for m in monsters.iter() {
                                                    if m.is_alive() {
                                                        let d = Map::distance(
                                                            player.x, player.y, m.x, m.y,
                                                        );
                                                        if d == 1 {
                                                            adjacent_monster = true;
                                                            break;
                                                        }
                                                    }
                                                }

                                                // Activate the ability (puts it on cooldown + sets buff)
                                                if slot == 1 {
                                                    if let Some(ref mut a) = player.ability_1 {
                                                        a.activate();
                                                        if !adjacent_monster {
                                                            a.is_active = false;
                                                            // no buff if no adjacent enemy
                                                        }
                                                    }
                                                } else {
                                                    if let Some(ref mut a) = player.ability_2 {
                                                        a.activate();
                                                        if !adjacent_monster {
                                                            a.is_active = false;
                                                        }
                                                    }
                                                }

                                                if adjacent_monster {
                                                    log.push("Shadow Step! Shadow strike ready — next melee does 2x damage!".to_string());
                                                } else {
                                                    log.push("Shadow Step!".to_string());
                                                }
                                            }
                                            AbilityType::ChainLightning => {
                                                // Fire lightning up to 6 tiles, chain to nearby monsters
                                                let int_mod =
                                                    player.effective_stats().potion_bonus(); // INT-based modifier
                                                let damages =
                                                    [3 + int_mod, 2 + int_mod, 1 + int_mod];
                                                let mut hit_indices: Vec<usize> = Vec::new();

                                                // Find first monster in line
                                                let mut cx = player.x as i32;
                                                let mut cy = player.y as i32;
                                                for _ in 1..=6 {
                                                    cx += ability_dx;
                                                    cy += ability_dy;
                                                    if cx < 0
                                                        || cy < 0
                                                        || cx as usize >= map.width
                                                        || cy as usize >= map.height
                                                    {
                                                        break;
                                                    }
                                                    if map.tiles[cx as usize][cy as usize]
                                                        == Tile::Wall
                                                    {
                                                        break;
                                                    }
                                                    // Check for monster at this position
                                                    for (mi, m) in monsters.iter().enumerate() {
                                                        if m.is_alive()
                                                            && m.x == cx as usize
                                                            && m.y == cy as usize
                                                        {
                                                            hit_indices.push(mi);
                                                            break;
                                                        }
                                                    }
                                                    if !hit_indices.is_empty() {
                                                        break; // found first target
                                                    }
                                                }

                                                // Chain: find nearest monster within 3 tiles of last hit
                                                for chain in 1..=2 {
                                                    if hit_indices.len() < chain {
                                                        break;
                                                    }
                                                    let last_mi = hit_indices[chain - 1];
                                                    let lx = monsters[last_mi].x;
                                                    let ly = monsters[last_mi].y;

                                                    let mut best_mi: Option<usize> = None;
                                                    let mut best_dist = i32::MAX;
                                                    for (mi, m) in monsters.iter().enumerate() {
                                                        if m.is_alive()
                                                            && !hit_indices.contains(&mi)
                                                        {
                                                            let d = Map::distance(lx, ly, m.x, m.y);
                                                            if d <= 3 && d < best_dist {
                                                                best_dist = d;
                                                                best_mi = Some(mi);
                                                            }
                                                        }
                                                    }
                                                    if let Some(mi) = best_mi {
                                                        hit_indices.push(mi);
                                                    }
                                                }

                                                // Apply damage
                                                if hit_indices.is_empty() {
                                                    log.push(
                                                        "Chain Lightning fizzles — no targets hit."
                                                            .to_string(),
                                                    );
                                                } else {
                                                    for (i, &mi) in hit_indices.iter().enumerate() {
                                                        let dmg = damages[i.min(2)];
                                                        monsters[mi].take_damage(dmg);
                                                        log.push(format!(
                                                            "Lightning strikes {} for {} damage!",
                                                            monsters[mi].name, dmg
                                                        ));
                                                        if !monsters[mi].is_alive() {
                                                            player.monsters_slain += 1;
                                                            monsters[mi].death_pos = Some((
                                                                monsters[mi].x,
                                                                monsters[mi].y,
                                                            ));
                                                            log.push(format!(
                                                                "[**] The {} dies!",
                                                                monsters[mi].name
                                                            ));
                                                            // XP reward
                                                            let xp = monsters[mi].xp_value();
                                                            let level_msgs = player.gain_xp(xp);
                                                            log.push(format!("+{} XP", xp));
                                                            for msg in level_msgs {
                                                                log.push(msg);
                                                            }
                                                            erase_entity(
                                                                &mut stdout,
                                                                &map,
                                                                monsters[mi].x,
                                                                monsters[mi].y,
                                                                current_floor,
                                                            )?;
                                                        }
                                                    }
                                                }

                                                // Put on cooldown
                                                match slot {
                                                    1 => {
                                                        if let Some(ref mut a) = player.ability_1 {
                                                            a.activate();
                                                            a.is_active = false;
                                                        }
                                                    }
                                                    2 => {
                                                        if let Some(ref mut a) = player.ability_2 {
                                                            a.activate();
                                                            a.is_active = false;
                                                        }
                                                    }
                                                    3 => {
                                                        if let Some(ref mut a) = player.ability_3 {
                                                            a.activate();
                                                            a.is_active = false;
                                                        }
                                                    }
                                                    4 => {
                                                        if let Some(ref mut a) = player.ability_4 {
                                                            a.activate();
                                                            a.is_active = false;
                                                        }
                                                    }
                                                    5 => {
                                                        if let Some(ref mut a) = player.ability_5 {
                                                            a.activate();
                                                            a.is_active = false;
                                                        }
                                                    }
                                                    _ => {}
                                                }
                                            }
                                            AbilityType::Backstab => {
                                                let px = player.x as i32;
                                                let py = player.y as i32;
                                                let target_x = (px + ability_dx) as usize;
                                                let target_y = (py + ability_dy) as usize;
                                                let backstab_dmg =
                                                    4 + player.effective_stats().potion_bonus();
                                                for m in monsters.iter_mut() {
                                                    if m.is_alive()
                                                        && m.x == target_x
                                                        && m.y == target_y
                                                    {
                                                        m.take_damage(backstab_dmg);
                                                        log.push(format!(
                                                            "BACKSTAB! {} takes {} damage!",
                                                            m.name, backstab_dmg
                                                        ));
                                                        if !m.is_alive() {
                                                            m.death_pos = Some((m.x, m.y));
                                                            log.push(format!(
                                                                "[**] The {} dies!",
                                                                m.name
                                                            ));
                                                            let xp = m.xp_value();
                                                            let level_msgs = player.gain_xp(xp);
                                                            log.push(format!("+{} XP", xp));
                                                            for msg in level_msgs {
                                                                log.push(msg);
                                                            }
                                                            erase_entity(
                                                                &mut stdout,
                                                                &map,
                                                                m.x,
                                                                m.y,
                                                                current_floor,
                                                            )?;
                                                        }
                                                        break;
                                                    }
                                                }
                                                match slot {
                                                    1 => {
                                                        if let Some(ref mut a) = player.ability_1 {
                                                            a.activate();
                                                            a.is_active = false;
                                                        }
                                                    }
                                                    2 => {
                                                        if let Some(ref mut a) = player.ability_2 {
                                                            a.activate();
                                                            a.is_active = false;
                                                        }
                                                    }
                                                    3 => {
                                                        if let Some(ref mut a) = player.ability_3 {
                                                            a.activate();
                                                            a.is_active = false;
                                                        }
                                                    }
                                                    4 => {
                                                        if let Some(ref mut a) = player.ability_4 {
                                                            a.activate();
                                                            a.is_active = false;
                                                        }
                                                    }
                                                    5 => {
                                                        if let Some(ref mut a) = player.ability_5 {
                                                            a.activate();
                                                            a.is_active = false;
                                                        }
                                                    }
                                                    _ => {}
                                                }
                                            }
                                            AbilityType::Assassinate => {
                                                let px = player.x as i32;
                                                let py = player.y as i32;
                                                let target_x = (px + ability_dx) as usize;
                                                let target_y = (py + ability_dy) as usize;
                                                let assassinate_dmg =
                                                    8 + player.effective_stats().potion_bonus();
                                                for m in monsters.iter_mut() {
                                                    if m.is_alive()
                                                        && m.x == target_x
                                                        && m.y == target_y
                                                    {
                                                        let hp_percent =
                                                            m.hp as f32 / m.max_hp as f32;
                                                        let execute_bonus = if hp_percent < 0.3 {
                                                            2.0
                                                        } else {
                                                            1.0
                                                        };
                                                        let final_dmg = (assassinate_dmg as f32
                                                            * execute_bonus)
                                                            as i32;
                                                        m.take_damage(final_dmg);
                                                        log.push(format!(
                                                            "ASSASSINATE! {} takes {} damage!",
                                                            m.name, final_dmg
                                                        ));
                                                        if hp_percent < 0.3 {
                                                            log.push("EXECUTED!".to_string());
                                                        }
                                                        if !m.is_alive() {
                                                            m.death_pos = Some((m.x, m.y));
                                                            log.push(format!(
                                                                "[**] The {} dies!",
                                                                m.name
                                                            ));
                                                            let xp = m.xp_value();
                                                            let level_msgs = player.gain_xp(xp);
                                                            log.push(format!("+{} XP", xp));
                                                            for msg in level_msgs {
                                                                log.push(msg);
                                                            }
                                                            erase_entity(
                                                                &mut stdout,
                                                                &map,
                                                                m.x,
                                                                m.y,
                                                                current_floor,
                                                            )?;
                                                        }
                                                        break;
                                                    }
                                                }
                                                match slot {
                                                    1 => {
                                                        if let Some(ref mut a) = player.ability_1 {
                                                            a.activate();
                                                            a.is_active = false;
                                                        }
                                                    }
                                                    2 => {
                                                        if let Some(ref mut a) = player.ability_2 {
                                                            a.activate();
                                                            a.is_active = false;
                                                        }
                                                    }
                                                    3 => {
                                                        if let Some(ref mut a) = player.ability_3 {
                                                            a.activate();
                                                            a.is_active = false;
                                                        }
                                                    }
                                                    4 => {
                                                        if let Some(ref mut a) = player.ability_4 {
                                                            a.activate();
                                                            a.is_active = false;
                                                        }
                                                    }
                                                    5 => {
                                                        if let Some(ref mut a) = player.ability_5 {
                                                            a.activate();
                                                            a.is_active = false;
                                                        }
                                                    }
                                                    _ => {}
                                                }
                                            }
                                            _ => {}
                                        }
                                    }
                                    continue 'inner;
                                }
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
                                    // Wraith invulnerability: can't hit a phasing wraith
                                    if monsters[i].is_phasing {
                                        log.push(format!(
                                            "Your attack passes through the {}!",
                                            monsters[i].name
                                        ));
                                    // Bat Swarm dodge: 25% chance at tier 2+
                                    } else if monsters[i].can_dodge_attack() {
                                        log.push(format!(
                                            "The {} dodges your attack!",
                                            monsters[i].name
                                        ));
                                    } else {
                                        let mut damage = player.melee_damage();

                                        // Power Attack / Shadow Strike buff: 2x damage
                                        if player.has_damage_buff() {
                                            damage *= 2;
                                            player.consume_damage_buff();
                                            log.push("[!!] CRITICAL STRIKE! [!!]".to_string());
                                        }

                                        monsters[i].take_damage(damage);
                                        player.damage_dealt += damage;
                                        log.push(format!(
                                            ">> You hit the {} for {} damage!",
                                            monsters[i].name, damage
                                        ));

                                        // Poison Blade buff: apply poison on hit
                                        if player.has_poison_buff() {
                                            monsters[i].poison_ticks = 3;
                                            player.consume_poison_buff();
                                            log.push(format!(
                                                "The {} is poisoned!",
                                                monsters[i].name
                                            ));
                                        }

                                        if !monsters[i].is_alive() {
                                            player.monsters_slain += 1;
                                            log.push(format!(
                                                "[**] The {} dies!",
                                                monsters[i].name
                                            ));
                                            monsters[i].death_pos =
                                                Some((monsters[i].x, monsters[i].y));
                                            let dpos = (monsters[i].x, monsters[i].y);
                                            erase_entity(
                                                &mut stdout,
                                                &map,
                                                dpos.0,
                                                dpos.1,
                                                current_floor,
                                            )?;

                                            // XP reward
                                            let xp = monsters[i].xp_value();
                                            let level_msgs = player.gain_xp(xp);
                                            log.push(format!("+{} XP", xp));
                                            for msg in level_msgs {
                                                log.push(msg);
                                            }

                                            // Random artifact drop (3% chance, floor 6+)
                                            if player.has_ragefang() {
                                                if player.ragefang_stacks < 5 {
                                                    player.ragefang_stacks += 1;
                                                }
                                                player.ragefang_ticks = 3;
                                                log.push(format!(
                                                    "Ragefang pulses! (+{} ATK)",
                                                    player.ragefang_stacks
                                                ));
                                            }

                                            if player.has_mindfire() {
                                                player.mindfire_kill_count += 1;
                                                if player.mindfire_kill_count >= 10 {
                                                    player.mindfire_kill_count = 0;
                                                    player.mindfire_ready = true;
                                                    log.push(
                                                        "Mindfire Crown blazes! Next ability = 2x dmg!"
                                                            .to_string(),
                                                    );
                                                }
                                            }

                                            // Random artifact drop (3% chance, floor 6+)
                                            if current_floor >= 6 {
                                                let mut rng = rand::rng();
                                                if rng.random_range(0..100) < 3 {
                                                    let artifact =
                                                        items::random_artifact(player.class.name());
                                                    log.push(format!(
                                                        "A glowing artifact falls: {}!",
                                                        artifact.display_name()
                                                    ));
                                                    ground_items.insert(dpos, artifact);
                                                }
                                            }

                                            // Monster drop (~30%)
                                            let mut rng = rand::rng();
                                            if rng.random_range(0..100) < 30 {
                                                let drop = items::random_drop(current_floor);
                                                log.push(format!(
                                                    "The {} drops a {}!",
                                                    monsters[i].name,
                                                    drop.display_name()
                                                ));
                                                ground_items.insert(dpos, drop);
                                            }

                                            // Boss kill celebration + guaranteed loot
                                            if monsters[i].is_boss {
                                                log.push("*** BOSS DEFEATED! ***".to_string());
                                                log.push("The stairs are unsealed!".to_string());
                                                // Guaranteed high-tier drop with minimum rarity
                                                let mut boss_drop =
                                                    items::random_drop(current_floor + 3);
                                                // Boss drops: floor 5+ = Rare minimum, floor 10+ = Epic minimum
                                                if current_floor >= 10 {
                                                    boss_drop.rarity = items::Rarity::Epic;
                                                } else {
                                                    boss_drop.rarity = items::Rarity::Rare;
                                                }
                                                log.push(format!(
                                                    "The {} drops a powerful {}!",
                                                    monsters[i].name,
                                                    boss_drop.display_name()
                                                ));
                                                // Place adjacent to death spot if taken
                                                let bx = if dpos.0 + 1 < map.width {
                                                    dpos.0 + 1
                                                } else {
                                                    dpos.0.saturating_sub(1)
                                                };
                                                ground_items.insert((bx, dpos.1), boss_drop);

                                                // Boss always drops a class artifact
                                                let artifact =
                                                    items::random_artifact(player.class.name());
                                                log.push(format!(
                                                    "A legendary artifact appears: {}!",
                                                    artifact.display_name()
                                                ));
                                                let ba_x = if bx + 1 < map.width {
                                                    bx + 1
                                                } else {
                                                    bx.saturating_sub(1)
                                                };
                                                ground_items.insert((ba_x, dpos.1), artifact);
                                            }
                                        }
                                    }
                                } else if map.is_walkable(next_x, next_y) {
                                    // --- PLAYER MOVEMENT ---
                                    erase_entity(
                                        &mut stdout,
                                        &map,
                                        player.x,
                                        player.y,
                                        current_floor,
                                    )?;

                                    // Web stuck: skip movement if player is stuck
                                    if player_web_stuck > 0 {
                                        log.push(format!(
                                            "You struggle against the web! ({} ticks)",
                                            player_web_stuck
                                        ));
                                    } else {
                                        player.x = next_x;
                                        player.y = next_y;
                                        map.update_visibility(player.x, player.y, 8);
                                        render_map_delta(
                                            &mut stdout,
                                            &map,
                                            current_floor,
                                            &mut prev_visibility,
                                        )?;
                                        render_minimap(
                                            &mut stdout,
                                            &map,
                                            player.x,
                                            player.y,
                                            term_width,
                                            map_height,
                                        )?;

                                        // Check if player stepped on a web
                                        if webs.remove(&(player.x, player.y)) {
                                            player_web_stuck = 2;
                                            log.push(
                                                "You walk into a web! You're stuck!".to_string(),
                                            );
                                        }

                                        // Check if player stepped on a trap
                                        if let Some(trap_type) =
                                            map.trap_tiles.remove(&(player.x, player.y))
                                        {
                                            match trap_type {
                                                map::TrapType::Spike => {
                                                    let dmg = rand::rng().random_range(1..=4);
                                                    player.take_damage(dmg);
                                                    player.damage_taken += dmg;
                                                    log.push(format!(
                                                        "Spikes erupt from the floor! (-{} HP)",
                                                        dmg
                                                    ));
                                                }
                                                map::TrapType::Fire => {
                                                    let dmg = rand::rng().random_range(2..=3);
                                                    player.take_damage(dmg);
                                                    player.damage_taken += dmg;
                                                    player.poison_ticks = 2;
                                                    log.push(format!("Flames burst from the ground! (-{} HP, burning!)", dmg));
                                                }
                                                map::TrapType::Teleport => {
                                                    let (tx, ty) = map.get_starting_position();
                                                    player.x = tx;
                                                    player.y = ty;
                                                    map.update_visibility(player.x, player.y, 8);
                                                    log.push(
                                                        "The floor vanishes beneath you!"
                                                            .to_string(),
                                                    );
                                                }
                                                map::TrapType::Alarm => {
                                                    log.push("An alarm sounds!".to_string());
                                                    for m in monsters.iter_mut() {
                                                        if m.is_alive() {
                                                            m.can_see_player = true;
                                                        }
                                                    }
                                                }
                                            }
                                        }

                                        // Check shrine altar interaction
                                        if let Some(DecoObject::Altar) =
                                            map.deco_objects.get(&(player.x, player.y))
                                        {
                                            if !map.shrine_used.contains(&(player.x, player.y)) {
                                                map.shrine_used.insert((player.x, player.y));
                                                let mut rng = rand::rng();
                                                let buff_msg = match rng.random_range(0..6) {
                                                    0 => {
                                                        player.bonus_str += 2;
                                                        "Strength (+2 STR for this floor)"
                                                    }
                                                    1 => {
                                                        player.hp = player.max_hp;
                                                        "Vitality (fully healed)"
                                                    }
                                                    2 => {
                                                        player.bonus_dodge += 10;
                                                        "Swiftness (+10% dodge for this floor)"
                                                    }
                                                    3 => {
                                                        let bonus_xp = 50;
                                                        let msgs = player.gain_xp(bonus_xp);
                                                        for m in msgs {
                                                            log.push(m);
                                                        }
                                                        "Knowledge (+50 XP)"
                                                    }
                                                    4 => {
                                                        for x in 0..map.width {
                                                            for y in 0..map.height {
                                                                map.visibility[x][y] = true;
                                                            }
                                                        }
                                                        "Darkness (map revealed!)"
                                                    }
                                                    _ => {
                                                        player.warding_buff = true;
                                                        "Warding (next hit reduced by 5)"
                                                    }
                                                };
                                                log.push(format!(
                                                    "The shrine pulses... {}!",
                                                    buff_msg
                                                ));
                                            }
                                        }

                                        // Check stairs
                                        if map.tiles[player.x][player.y] == Tile::Stairs {
                                            // Block descent if a boss is alive
                                            let boss_alive =
                                                monsters.iter().any(|m| m.is_boss && m.is_alive());
                                            if boss_alive {
                                                log.push("The stairs are sealed by a dark power! Defeat the boss first!".to_string());
                                            } else {
                                                current_floor += 1;
                                                player.bonus_str = 0;
                                                player.bonus_dodge = 0;
                                                player.warding_buff = false;
                                                let floor_xp = current_floor * 5;
                                                let level_msgs = player.gain_xp(floor_xp);
                                                // Enhanced floor transition message
                                                let transition_msg = match current_floor {
                                                    5 => "▼ You descend to the Dungeons... A dark presence lurks below...",
                                                    10 => "▼ You descend to the Bone Pits... The air grows cold...",
                                                    15 => "▼ You descend to the Shadow Realm... The final battle awaits!",
                                                    _ => "▼ You descend deeper into the darkness...",
                                                };
                                                log.push(transition_msg.to_string());
                                                log.push(format!("+{} XP", floor_xp));
                                                for msg in level_msgs {
                                                    log.push(msg);
                                                }
                                                continue 'floor_loop;
                                            }
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
                }

                // --- MONSTER TICK (independent of player) ---
                let now = Instant::now();
                if now.duration_since(last_monster_tick) >= Duration::from_millis(MONSTER_TICK_MS) {
                    last_monster_tick = now;

                    // Tick player ability cooldowns
                    player.tick_abilities();

                    // Decrement web stuck counter
                    if player_web_stuck > 0 {
                        player_web_stuck -= 1;
                        if player_web_stuck == 0 {
                            log.push("You break free from the web!".to_string());
                        }
                    }

                    // Player poison tick
                    if player.poison_ticks > 0 {
                        player.poison_ticks -= 1;
                        player.take_damage(1);
                        player.damage_taken += 1;
                        log.push("Poison burns in your veins! (-1 HP)".to_string());
                    }

                    if player.mana_shield_ticks > 0 {
                        player.mana_shield_ticks -= 1;
                        if player.mana_shield_ticks == 0 {
                            log.push("Mana Shield fades.".to_string());
                        }
                    }

                    process_projectiles(
                        &mut stdout,
                        &mut projectiles,
                        &monsters,
                        &mut player,
                        &map,
                        &mut log,
                        current_floor,
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
                        &mut webs,
                    )?;
                }

                // --- DEATH CHECK ---
                if !player.is_alive() {
                    if let Some((source, dmg)) = &player.last_damage_source {
                        player.cause_of_death = format!("{} ({} dmg)", source, dmg);
                    }
                    let restart = show_death_screen(&mut stdout, &player, current_floor)?;
                    if restart {
                        log.clear();
                        continue 'outer;
                    } else {
                        break 'outer;
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
