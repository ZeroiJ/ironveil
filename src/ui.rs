use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal::{self, Clear, ClearType},
};
use std::io::{stdout, Write};

use crate::items::ItemType;
use crate::player::{Class, Player, INVENTORY_CAPACITY};

// --- Title Screen ---

#[derive(Clone, Copy, PartialEq)]
pub enum TitleChoice {
    NewGame,
    LoadGame,
    Quit,
}

pub fn title_screen() -> std::io::Result<TitleChoice> {
    let mut stdout = stdout();
    let mut selected: usize = 0;

    loop {
        let (term_w, term_h) = terminal::size()?;
        let tw = term_w as usize;
        let th = term_h as usize;

        execute!(stdout, Clear(ClearType::All))?;

        let box_w = 60usize;
        let box_h = 20usize;
        let safe_box_w = box_w.min(tw.saturating_sub(4));
        let safe_box_h = box_h.min(th.saturating_sub(4));
        let box_x = (tw.saturating_sub(safe_box_w) / 2) as u16;
        let box_y = (th.saturating_sub(safe_box_h) / 2) as u16;

        let draw = |stdout: &mut std::io::Stdout,
                    row: i32,
                    text: &str,
                    color: Color|
         -> std::io::Result<()> {
            execute!(
                stdout,
                cursor::MoveTo(box_x, box_y + row as u16),
                SetForegroundColor(color),
                Print(text)
            )
        };

        let top_border = format!("╔{}╗", "═".repeat(safe_box_w - 2));
        let mid_border = format!("╠{}╣", "─".repeat(safe_box_w - 2));
        let empty_line = format!("║{}║", " ".repeat(safe_box_w - 2));
        let bottom_border = format!("╚{}╝", "═".repeat(safe_box_w - 2));

        let mut row = 0i32;

        draw(&mut stdout, row, &top_border, Color::DarkGrey)?;
        row += 1;
        draw(&mut stdout, row, &empty_line, Color::DarkGrey)?;
        row += 1;

        // ASCII Logo
        let logo = [
            "  █████╗ ██╗      ██████╗  ██████╗ ██████╗  ",
            " ██╔══██╗██║     ██╔═══██╗██╔═══██╗██╔══██╗ ",
            " ███████║██║     ██║   ██║██║   ██║██████╔╝ ",
            " ██╔══██║██║     ██║   ██║██║   ██║██╔══██╗ ",
            " ██║  ██║███████╗╚██████╔╝╚██████╔╝██║  ██║ ",
            " ╚═╝  ╚═╝╚══════╝ ╚═════╝  ╚═════╝ ╚═╝  ╚═╝ ",
        ];

        for logo_line in &logo {
            let pad = (safe_box_w.saturating_sub(2).saturating_sub(logo_line.len())) / 2;
            draw(
                &mut stdout,
                row,
                &format!(
                    "║{}{}{}║",
                    " ".repeat(pad),
                    logo_line,
                    " ".repeat(
                        safe_box_w
                            .saturating_sub(2)
                            .saturating_sub(pad)
                            .saturating_sub(logo_line.len())
                    )
                ),
                Color::Yellow,
            )?;
            row += 1;
        }

        draw(&mut stdout, row, &empty_line, Color::DarkGrey)?;
        row += 1;
        draw(&mut stdout, row, &mid_border, Color::DarkGrey)?;
        row += 1;

        // Menu options
        let options = ["[N]ew Game", "[L]oad Game", "[Q]uit"];
        let descriptions = [
            "Start a new adventure",
            "Continue a saved game",
            "Exit to desktop",
        ];

        for (i, (opt, desc)) in options.iter().zip(descriptions.iter()).enumerate() {
            let is_selected = i == selected;
            let marker = if is_selected { "►" } else { " " };
            let fg = if is_selected {
                Color::White
            } else {
                Color::DarkGrey
            };

            draw(&mut stdout, row, &format!("║  {} {} ", marker, opt), fg)?;
            // Add description on the same line
            execute!(
                stdout,
                cursor::MoveTo(box_x + 30, box_y + row as u16),
                SetForegroundColor(if is_selected {
                    Color::Grey
                } else {
                    Color::DarkGrey
                }),
                Print(desc)
            )?;
            execute!(
                stdout,
                cursor::MoveTo(box_x + safe_box_w as u16 - 1, box_y + row as u16),
                Print("║")
            )?;
            row += 1;
        }

        draw(&mut stdout, row, &empty_line, Color::DarkGrey)?;
        row += 1;
        draw(&mut stdout, row, &bottom_border, Color::DarkGrey)?;

        // Version info
        execute!(
            stdout,
            cursor::MoveTo(box_x, box_y + row as u16 + 1),
            SetForegroundColor(Color::DarkGrey),
            Print(format!(
                "{:^width$}",
                "v0.3.2  │  A Terminal Roguelike",
                width = box_w
            ))
        )?;

        stdout.flush()?;

        // Input
        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Up => {
                    if selected > 0 {
                        selected -= 1;
                    }
                }
                KeyCode::Down => {
                    if selected < 2 {
                        selected += 1;
                    }
                }
                KeyCode::Enter => {
                    return Ok(match selected {
                        0 => TitleChoice::NewGame,
                        1 => TitleChoice::LoadGame,
                        _ => TitleChoice::Quit,
                    });
                }
                KeyCode::Char('n') => return Ok(TitleChoice::NewGame),
                KeyCode::Char('l') => return Ok(TitleChoice::LoadGame),
                KeyCode::Char('q') | KeyCode::Esc => return Ok(TitleChoice::Quit),
                _ => {}
            }
        }
    }
}

// --- Character Creation Screen ---

pub fn character_creation_screen() -> std::io::Result<Class> {
    let mut stdout = stdout();
    let classes = [Class::Warrior, Class::Rogue, Class::Mage];
    let mut selected: usize = 0;

    loop {
        let (term_w, term_h) = terminal::size()?;
        let tw = term_w as usize;
        let th = term_h as usize;

        execute!(stdout, Clear(ClearType::All))?;

        // Box dimensions
        let box_w = 50usize;
        let safe_box_w = box_w.min(tw.saturating_sub(4));
        let box_x = (tw.saturating_sub(safe_box_w) / 2) as u16;
        let box_y = (th.saturating_sub(30) / 2) as u16;

        let draw = |stdout: &mut std::io::Stdout,
                    row: u16,
                    text: &str,
                    color: Color|
         -> std::io::Result<()> {
            execute!(
                stdout,
                cursor::MoveTo(box_x, box_y + row),
                SetForegroundColor(color),
                Print(text)
            )
        };

        // Unicode box-drawing borders
        let top_border = format!("╔{}╗", "═".repeat(safe_box_w - 2));
        let mid_border = format!("╠{}╣", "─".repeat(safe_box_w - 2));
        let empty_line = format!("║{}║", " ".repeat(safe_box_w - 2));
        let bottom_border = format!("╚{}╝", "═".repeat(safe_box_w - 2));

        draw(&mut stdout, 0, &top_border, Color::DarkGrey)?;
        draw(
            &mut stdout,
            1,
            &format!("║{:^width$}║", " CHOOSE YOUR CLASS ", width = box_w - 2),
            Color::White,
        )?;
        draw(&mut stdout, 2, &mid_border, Color::DarkGrey)?;

        let mut row = 3u16;

        for (i, class) in classes.iter().enumerate() {
            let stats = class.base_stats();
            let is_selected = i == selected;

            let class_color = match class {
                Class::Warrior => Color::Red,
                Class::Rogue => Color::Green,
                Class::Mage => Color::Blue,
            };

            let class_icon = match class {
                Class::Warrior => "⚔",
                Class::Rogue => "🗡",
                Class::Mage => "✦",
            };

            let marker = if is_selected { "►" } else { " " };
            let highlight = if is_selected {
                Color::White
            } else {
                Color::DarkGrey
            };

            draw(&mut stdout, row, &empty_line, Color::DarkGrey)?;
            row += 1;

            // Class name with icon and marker
            let name_line = format!("║  {}{} [{}] {} ", marker, class_icon, i + 1, class.name());
            draw(&mut stdout, row, &name_line, highlight)?;
            // Re-draw class name in class color
            execute!(
                stdout,
                cursor::MoveTo(box_x + 10, box_y + row),
                SetForegroundColor(class_color),
                Print(format!(" {}", class.name()))
            )?;
            row += 1;

            if is_selected {
                // ASCII art
                let art = match class {
                    Class::Warrior => "      /|",
                    Class::Rogue => "      X",
                    Class::Mage => "      *",
                };
                let art2 = match class {
                    Class::Warrior => "     / |",
                    Class::Rogue => "     / \\",
                    Class::Mage => "     /|\\",
                };
                let art3 = match class {
                    Class::Warrior => "    /__|",
                    Class::Rogue => "    '   '",
                    Class::Mage => "      |",
                };

                draw(
                    &mut stdout,
                    row,
                    &format!("|  {:<width$}|", art, width = box_w - 4),
                    Color::DarkGrey,
                )?;
                execute!(
                    stdout,
                    cursor::MoveTo(box_x + 3, box_y + row),
                    SetForegroundColor(class_color),
                    Print(art)
                )?;
                row += 1;
                draw(
                    &mut stdout,
                    row,
                    &format!("|  {:<width$}|", art2, width = box_w - 4),
                    Color::DarkGrey,
                )?;
                execute!(
                    stdout,
                    cursor::MoveTo(box_x + 3, box_y + row),
                    SetForegroundColor(class_color),
                    Print(art2)
                )?;
                row += 1;
                draw(
                    &mut stdout,
                    row,
                    &format!("|  {:<width$}|", art3, width = box_w - 4),
                    Color::DarkGrey,
                )?;
                execute!(
                    stdout,
                    cursor::MoveTo(box_x + 3, box_y + row),
                    SetForegroundColor(class_color),
                    Print(art3)
                )?;
                row += 1;

                draw(&mut stdout, row, &empty_line, Color::DarkGrey)?;
                row += 1;

                // Stat bars
                let stat_bar = |val: i32| -> String {
                    let filled = ((val as f32 / 20.0) * 10.0) as usize;
                    let empty = 10usize.saturating_sub(filled);
                    format!("{}{}", "#".repeat(filled), ".".repeat(empty))
                };

                let stats_lines = [
                    format!(
                        "║      STR {} {:2}                    ║",
                        stat_bar(stats.str_),
                        stats.str_
                    ),
                    format!(
                        "║      DEX {} {:2}                    ║",
                        stat_bar(stats.dex),
                        stats.dex
                    ),
                    format!(
                        "║      INT {} {:2}                    ║",
                        stat_bar(stats.int),
                        stats.int
                    ),
                    format!(
                        "║      CON {} {:2}                    ║",
                        stat_bar(stats.con),
                        stats.con
                    ),
                ];

                for line in &stats_lines {
                    draw(&mut stdout, row, line, Color::Grey)?;
                    row += 1;
                }

                // HP
                let hp = stats.max_hp();
                let hp_line = format!("║      HP: {:<31}║", hp);
                draw(&mut stdout, row, &hp_line, Color::Grey)?;
                row += 1;

                // Starting gear
                let gear = match class {
                    Class::Warrior => vec!["Iron Shortsword (+2 dmg)", "Leather Armor (+1 def)"],
                    Class::Rogue => vec!["Twin Daggers (+1 dmg)"],
                    Class::Mage => vec!["Wooden Staff (+1 dmg)", "Ring of Intellect (+2 INT)"],
                };

                draw(
                    &mut stdout,
                    row,
                    &format!("║      Starts with:{:<24}║", ""),
                    Color::DarkGrey,
                )?;
                execute!(
                    stdout,
                    cursor::MoveTo(box_x + 6, box_y + row),
                    SetForegroundColor(Color::Grey),
                    Print("Starts with:")
                )?;
                row += 1;

                for g in &gear {
                    let gear_line = format!("║        {:<33}║", g);
                    draw(&mut stdout, row, &gear_line, Color::DarkYellow)?;
                    row += 1;
                }

                // Playstyle
                let playstyle = match class {
                    Class::Warrior => "Tanky brawler. Hit hard, take hits.",
                    Class::Rogue => "Dodge + crits. High risk/reward.",
                    Class::Mage => "INT-boosted healing. Future spells.",
                };
                draw(&mut stdout, row, &empty_line, Color::DarkGrey)?;
                row += 1;
                let play_line = format!("║    {:<width$}║", playstyle, width = box_w - 6);
                draw(&mut stdout, row, &play_line, Color::DarkGrey)?;
                execute!(
                    stdout,
                    cursor::MoveTo(box_x + 4, box_y + row),
                    SetForegroundColor(Color::DarkCyan),
                    Print(playstyle)
                )?;
                row += 1;
            }
        }

        // Footer
        draw(&mut stdout, row, &empty_line, Color::DarkGrey)?;
        row += 1;
        draw(&mut stdout, row, &mid_border, Color::DarkGrey)?;
        row += 1;
        let footer = format!(
            "║{:^width$}║",
            "Up/Down Navigate  │  Enter Select",
            width = box_w - 2
        );
        draw(&mut stdout, row, &footer, Color::Grey)?;
        row += 1;
        draw(&mut stdout, row, &bottom_border, Color::DarkGrey)?;

        stdout.flush()?;

        // Input
        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Up => {
                    if selected > 0 {
                        selected -= 1;
                    }
                }
                KeyCode::Down => {
                    if selected < 2 {
                        selected += 1;
                    }
                }
                KeyCode::Enter | KeyCode::Char(' ') => {
                    return Ok(classes[selected]);
                }
                KeyCode::Char('1') => return Ok(Class::Warrior),
                KeyCode::Char('2') => return Ok(Class::Rogue),
                KeyCode::Char('3') => return Ok(Class::Mage),
                KeyCode::Char('q') | KeyCode::Esc => {
                    // Default to Warrior if they quit creation
                    return Ok(Class::Warrior);
                }
                _ => {}
            }
        }
    }
}

// --- Inventory Screen ---

/// Show inventory overlay. Returns true if the player used/equipped/dropped something.
/// Pauses monster tick (caller is responsible for not ticking while this runs).
pub fn inventory_screen(player: &mut Player) -> std::io::Result<bool> {
    let mut stdout = stdout();
    let mut changed = false;

    loop {
        let (term_w, term_h) = terminal::size()?;
        let tw = term_w as usize;
        let th = term_h as usize;

        execute!(stdout, Clear(ClearType::All))?;

        let box_w = 50usize;
        let safe_box_w = box_w.min(tw.saturating_sub(4));
        let box_x = (tw.saturating_sub(safe_box_w) / 2) as u16;
        let box_y = (th.saturating_sub(28) / 2) as u16;

        let draw = |stdout: &mut std::io::Stdout,
                    row: u16,
                    text: &str,
                    color: Color|
         -> std::io::Result<()> {
            execute!(
                stdout,
                cursor::MoveTo(box_x, box_y + row),
                SetForegroundColor(color),
                Print(text)
            )
        };

        let top_border = format!("╔{}╗", "═".repeat(safe_box_w - 2));
        let mid_border = format!("╠{}╣", "─".repeat(safe_box_w - 2));
        let empty_line = format!("║{}║", " ".repeat(safe_box_w - 2));
        let bottom_border = format!("╚{}╝", "═".repeat(safe_box_w - 2));

        let mut row = 0u16;

        draw(&mut stdout, row, &top_border, Color::DarkGrey)?;
        row += 1;
        draw(
            &mut stdout,
            row,
            &format!("║{:^width$}║", " INVENTORY ", width = box_w - 2),
            Color::White,
        )?;
        row += 1;
        draw(&mut stdout, row, &mid_border, Color::DarkGrey)?;
        row += 1;

        // Equipment section
        draw(
            &mut stdout,
            row,
            &format!("║  {:<width$}║", "Equipment:", width = box_w - 4),
            Color::DarkYellow,
        )?;
        row += 1;

        let weapon_name = player
            .equipment
            .weapon
            .as_ref()
            .map_or("(empty)".to_string(), |w| w.display_name());
        let armor_name = player
            .equipment
            .armor
            .as_ref()
            .map_or("(empty)".to_string(), |a| a.display_name());
        let ring_name = player
            .equipment
            .ring
            .as_ref()
            .map_or("(empty)".to_string(), |r| r.display_name());

        let w_line = format!("║    Weapon: {:<width$}║", weapon_name, width = box_w - 14);
        let a_line = format!("║    Armor:  {:<width$}║", armor_name, width = box_w - 14);
        let r_line = format!("║    Ring:   {:<width$}║", ring_name, width = box_w - 14);

        draw(&mut stdout, row, &w_line, Color::Cyan)?;
        row += 1;
        draw(&mut stdout, row, &a_line, Color::DarkYellow)?;
        row += 1;
        draw(&mut stdout, row, &r_line, Color::Yellow)?;
        row += 1;

        draw(&mut stdout, row, &empty_line, Color::DarkGrey)?;
        row += 1;
        draw(&mut stdout, row, &mid_border, Color::DarkGrey)?;
        row += 1;

        // Backpack
        let count_str = format!(
            "Backpack ({}/{}):",
            player.inventory.len(),
            INVENTORY_CAPACITY
        );
        draw(
            &mut stdout,
            row,
            &format!("║  {:<width$}║", count_str, width = box_w - 4),
            Color::DarkYellow,
        )?;
        row += 1;

        if player.inventory.is_empty() {
            draw(
                &mut stdout,
                row,
                &format!("║    {:<width$}║", "(empty)", width = box_w - 6),
                Color::DarkGrey,
            )?;
            row += 1;
        } else {
            for (i, item) in player.inventory.iter().enumerate() {
                let letter = (b'a' + i as u8) as char;
                let item_color = item.rarity.color();
                let action_hint = match item.item_type {
                    ItemType::Potion => "[use]",
                    _ => "[equip]",
                };
                let line = format!(
                    "║    {}) {:<24} {:>8}║",
                    letter,
                    item.display_name(),
                    action_hint
                );
                draw(&mut stdout, row, &line, item_color)?;
                row += 1;
            }
        }

        draw(&mut stdout, row, &empty_line, Color::DarkGrey)?;
        row += 1;
        draw(&mut stdout, row, &mid_border, Color::DarkGrey)?;
        row += 1;

        let footer = format!(
            "║{:^width$}║",
            "[a-j] Use/Equip  │  [A-J] Drop  │  Esc Close",
            width = box_w - 2
        );
        draw(&mut stdout, row, &footer, Color::Grey)?;
        row += 1;
        draw(&mut stdout, row, &bottom_border, Color::DarkGrey)?;

        stdout.flush()?;

        // Input
        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Tab | KeyCode::Esc => {
                    return Ok(changed);
                }
                KeyCode::Char(c) if c >= 'a' && c <= 'j' => {
                    let index = (c as u8 - b'a') as usize;
                    if index < player.inventory.len() {
                        let item_type = player.inventory[index].item_type.clone();
                        match item_type {
                            ItemType::Potion => {
                                player.use_potion(index);
                                changed = true;
                            }
                            ItemType::Weapon | ItemType::Armor | ItemType::Ring => {
                                player.equip_from_inventory(index);
                                changed = true;
                            }
                        }
                    }
                }
                // Uppercase A-J: drop/destroy item from inventory
                KeyCode::Char(c) if c >= 'A' && c <= 'J' => {
                    let index = (c as u8 - b'A') as usize;
                    if index < player.inventory.len() {
                        player.inventory.remove(index);
                        changed = true;
                    }
                }
                _ => {}
            }
        }
    }
}
