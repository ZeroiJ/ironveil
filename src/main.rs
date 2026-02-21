mod map;
mod player;
mod monster;

use map::{Map, Tile};
use player::Player;
use monster::Monster;
use std::io::{stdout, Write};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal::{self, Clear, ClearType},
};
use std::time::{Duration, Instant};

fn render_map(stdout: &mut std::io::Stdout, map: &Map) -> std::io::Result<()> {
    execute!(stdout, cursor::MoveTo(0, 0))?;
    for y in 0..map.height {
        for x in 0..map.width {
            execute!(stdout, cursor::MoveTo(x as u16, y as u16))?;
            match map.tiles[x][y] {
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
        let map_height = (term_height as usize).saturating_sub(6); // 1 for Floor/HP, 3 for Log, 2 for buffers

        let map = Map::new(map_width, map_height);
        let (spawn_x, spawn_y) = map.get_starting_position();
        let mut player = Player::new(spawn_x, spawn_y);
        let mut monsters = map.spawn_monsters();

        execute!(stdout, Clear(ClearType::All))?;
        render_map(&mut stdout, &map)?;
        log.push(format!("Welcome to floor {}!", current_floor));

        'inner: loop {
            // UI Update (Floor/HP and Log)
            execute!(
                stdout,
                cursor::MoveTo(0, map_height as u16 + 1),
                SetForegroundColor(Color::White),
                Print(format!("Floor: {} | HP: {:2}/{:2}    ", current_floor, player.hp, player.max_hp))
            )?;

            // Render Message Log (last 3 lines)
            for (i, msg) in log.iter().rev().take(3).enumerate() {
                execute!(
                    stdout,
                    cursor::MoveTo(0, map_height as u16 + 2 + i as u16),
                    SetForegroundColor(Color::Grey),
                    Clear(ClearType::UntilNewLine),
                    Print(msg)
                )?;
            }

            // Redraw monsters
            render_monsters(&mut stdout, &monsters)?;

            // Redraw player
            execute!(
                stdout,
                cursor::MoveTo(player.x as u16, player.y as u16),
                SetForegroundColor(Color::Yellow),
                Print("@"),
            )?;
            stdout.flush()?;

            if let Event::Key(key_event) = event::read()? {
                let mut next_x = player.x;
                let mut next_y = player.y;

                let now = Instant::now();
                if Some(key_event.code) == last_key && now.duration_since(last_move_time) < Duration::from_millis(100) {
                    continue;
                }
                last_key = Some(key_event.code);
                last_move_time = now;

                match key_event.code {
                    KeyCode::Char('q') | KeyCode::Esc => break 'outer,
                    KeyCode::Up => { if next_y > 0 { next_y -= 1; } }
                    KeyCode::Down => { next_y += 1; }
                    KeyCode::Left => { if next_x > 0 { next_x -= 1; } }
                    KeyCode::Right => { next_x += 1; }
                    _ => continue,
                }

                // Check for monster at destination (combat)
                let mut monster_index = None;
                for (i, monster) in monsters.iter().enumerate() {
                    if monster.is_alive() && monster.x == next_x && monster.y == next_y {
                        monster_index = Some(i);
                        break;
                    }
                }

                if let Some(i) = monster_index {
                    // Attack monster
                    let damage = 5;
                    monsters[i].take_damage(damage);
                    log.push(format!("You hit the {} for {} damage!", monsters[i].name, damage));
                    if !monsters[i].is_alive() {
                        log.push(format!("The {} dies!", monsters[i].name));
                        // Clear dead monster from screen
                        execute!(stdout, cursor::MoveTo(monsters[i].x as u16, monsters[i].y as u16), SetForegroundColor(Color::DarkGrey), Print("."))?;
                    }
                    // Skip move
                    continue;
                }

                if map.is_walkable(next_x, next_y) {
                    // Restore tile at old position
                    execute!(stdout, cursor::MoveTo(player.x as u16, player.y as u16))?;
                    match map.tiles[player.x][player.y] {
                        Tile::Floor => {
                            execute!(stdout, SetForegroundColor(Color::DarkGrey), Print("."))?;
                        }
                        Tile::Stairs => {
                            execute!(stdout, SetForegroundColor(Color::White), Print(">"))?;
                        }
                        _ => {}
                    }

                    player.x = next_x;
                    player.y = next_y;

                    // Check for stairs
                    if map.tiles[player.x][player.y] == Tile::Stairs {
                        current_floor += 1;
                        break 'inner;
                    }
                }
            }

            // Monster Turn
            for i in 0..monsters.len() {
                if !monsters[i].is_alive() { continue; }

                let dx = (player.x as i32 - monsters[i].x as i32).abs();
                let dy = (player.y as i32 - monsters[i].y as i32).abs();

                if dx <= 1 && dy <= 1 {
                    // Attack player
                    let damage = monsters[i].attack;
                    player.take_damage(damage);
                    log.push(format!("The {} hits you for {} damage!", monsters[i].name, damage));
                } else if dx + dy < 10 {
                    // Move towards player
                    let mut next_mx = monsters[i].x;
                    let mut next_my = monsters[i].y;

                    if monsters[i].x < player.x { next_mx += 1; }
                    else if monsters[i].x > player.x { next_mx -= 1; }
                    else if monsters[i].y < player.y { next_my += 1; }
                    else if monsters[i].y > player.y { next_my -= 1; }

                    // Check if tile is walkable and not occupied by another monster
                    if map.is_walkable(next_mx, next_my) {
                        let mut occupied = false;
                        for (j, other) in monsters.iter().enumerate() {
                            if i != j && other.is_alive() && other.x == next_mx && other.y == next_my {
                                occupied = true;
                                break;
                            }
                        }
                        if !occupied {
                            // Erase old position
                            execute!(stdout, cursor::MoveTo(monsters[i].x as u16, monsters[i].y as u16))?;
                            match map.tiles[monsters[i].x][monsters[i].y] {
                                Tile::Floor => { execute!(stdout, SetForegroundColor(Color::DarkGrey), Print("."))?; }
                                Tile::Stairs => { execute!(stdout, SetForegroundColor(Color::White), Print(">"))?; }
                                _ => {}
                            }
                            monsters[i].x = next_mx;
                            monsters[i].y = next_my;
                        }
                    }
                }
            }

            // Update UI
            execute!(
                stdout,
                cursor::MoveTo(0, map_height as u16 + 1),
                SetForegroundColor(Color::White),
                Print(format!("Floor: {} | HP: {:2}/{:2}    ", current_floor, player.hp, player.max_hp))
            )?;

            if !player.is_alive() {
                execute!(stdout, cursor::MoveTo(player.x as u16, player.y as u16), Print("X"))?;
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
