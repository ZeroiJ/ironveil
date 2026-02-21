mod map;
use map::{Map, Tile};
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

fn main() -> std::io::Result<()> {
    terminal::enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, cursor::Hide)?;

    let mut current_floor = 1;

    let mut last_key = None;
    let mut last_move_time = Instant::now();

    'outer: loop {
        let (term_width, term_height) = terminal::size()?;
        let map_width = term_width as usize;
        let map_height = (term_height as usize).saturating_sub(2); // Reserve 2 lines for UI

        let map = Map::new(map_width, map_height);
        let (mut player_x, mut player_y) = map.get_starting_position();

        execute!(stdout, Clear(ClearType::All))?;
        render_map(&mut stdout, &map)?;

        // Show floor number
        execute!(
            stdout,
            cursor::MoveTo(0, map_height as u16 + 1),
            SetForegroundColor(Color::White),
            Print(format!("Floor: {}", current_floor))
        )?;

        'inner: loop {
            // Redraw player
            execute!(
                stdout,
                cursor::MoveTo(player_x as u16, player_y as u16),
                SetForegroundColor(Color::Yellow),
                Print("@"),
            )?;
            stdout.flush()?;

            if let Event::Key(key_event) = event::read()? {
                let mut next_x = player_x;
                let mut next_y = player_y;

                // Move rate limiting: if it's the SAME key, apply a cooldown.
                // If it's a DIFFERENT key, move immediately.
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

                if map.is_walkable(next_x, next_y) {
                    // Restore tile at old position
                    execute!(stdout, cursor::MoveTo(player_x as u16, player_y as u16))?;
                    match map.tiles[player_x][player_y] {
                        Tile::Floor => {
                            execute!(stdout, SetForegroundColor(Color::DarkGrey), Print("."))?;
                        }
                        Tile::Stairs => {
                            execute!(stdout, SetForegroundColor(Color::White), Print(">"))?;
                        }
                        _ => {}
                    }

                    player_x = next_x;
                    player_y = next_y;

                    // Check for stairs
                    if map.tiles[player_x][player_y] == Tile::Stairs {
                        current_floor += 1;
                        break 'inner;
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
