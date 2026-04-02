# Ironveil — ASCII Art & UI Reference

**Purpose:** Complete ASCII art reference for terminal rendering.
**Rule:** Every piece of art here is tested to work in a monospace
terminal. No emoji, no Unicode beyond box-drawing characters.
All widths are measured in character cells.

---

## 1. Title Screen

Shown once on game launch before character creation.
Width: 62 chars. Center on terminal.

```
╔══════════════════════════════════════════════════════════╗
║                                                          ║
║   ██╗██████╗  ██████╗ ███╗   ██╗██╗   ██╗███████╗██╗    ║
║   ██║██╔══██╗██╔═══██╗████╗  ██║██║   ██║██╔════╝██║    ║
║   ██║██████╔╝██║   ██║██╔██╗ ██║██║   ██║█████╗  ██║    ║
║   ██║██╔══██╗██║   ██║██║╚██╗██║╚██╗ ██╔╝██╔══╝  ██║    ║
║   ██║██║  ██║╚██████╔╝██║ ╚████║ ╚████╔╝ ███████╗██║    ║
║   ╚═╝╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═══╝  ╚═══╝  ╚══════╝╚═╝    ║
║                                                          ║
║           A dungeon awaits. Will you survive?            ║
║                                                          ║
╠══════════════════════════════════════════════════════════╣
║                                                          ║
║                    [N]  New Game                         ║
║                    [L]  Load Game                        ║
║                    [Q]  Quit                             ║
║                                                          ║
╚══════════════════════════════════════════════════════════╝
```

---

## 2. Character Creation Screen

Width: 58 chars. Three class cards, one active at a time.
Active class uses double-line border. Inactive use single-line.

### Active Card (selected class)
```
╔════════════════════════════════════════════════════╗
║  WARRIOR                                   @ (red) ║
╠════════════════════════════════════════════════════╣
║                                                    ║
║    /\                                              ║
║   /  \    STR [████████░░]  14                     ║
║  / @@ \   DEX [█████░░░░░]  10                     ║
║  \ -- /   INT [████░░░░░░]   8                     ║
║   \  /    CON [████████░░]  14   HP: 24            ║
║    \/                                              ║
║                                                    ║
║  Abilities:  Power Attack  |  War Cry (Lv5)        ║
║  Gear:       Iron Shortsword (+2)  Leather (+1)    ║
║  Style:      Tanky brawler. Hit hard, take hits.   ║
║                                                    ║
╚════════════════════════════════════════════════════╝
```

### Inactive Card (unselected class)
```
┌────────────────────────────────────────────────────┐
│  ROGUE                                  @ (green)  │
│  STR [███░░░░░░░]  10   DEX [███████░░░]  14       │
│  Abilities:  Shadow Step  |  Poison Blade (Lv5)    │
└────────────────────────────────────────────────────┘
```

### Full Screen Layout
```
╔══════════════════════════════════════════════════════════╗
║                   CHOOSE YOUR CLASS                      ║
╠══════════════════════════════════════════════════════════╣
║                                                          ║
║  ╔════════════════════════════════════════════════════╗  ║
║  ║  WARRIOR                               @ (red)    ║  ║
║  ╠════════════════════════════════════════════════════╣  ║
║  ║  STR [████████░░] 14  CON [████████░░] 14         ║  ║
║  ║  DEX [█████░░░░░] 10  INT [████░░░░░░]  8         ║  ║
║  ║  HP: 24   Gear: Iron Shortsword / Leather Armor   ║  ║
║  ║  Power Attack | War Cry (Lv5)                     ║  ║
║  ╚════════════════════════════════════════════════════╝  ║
║                                                          ║
║  ┌────────────────────────────────────────────────────┐  ║
║  │  ROGUE                              @ (green)      │  ║
║  │  STR 10  DEX 14  INT 10  CON 10    HP: 20          │  ║
║  │  Shadow Step | Poison Blade (Lv5)                  │  ║
║  └────────────────────────────────────────────────────┘  ║
║                                                          ║
║  ┌────────────────────────────────────────────────────┐  ║
║  │  MAGE                                @ (blue)      │  ║
║  │  STR 8   DEX 10  INT 14  CON 8      HP: 18         │  ║
║  │  Chain Lightning | Frost Nova (Lv5)                │  ║
║  └────────────────────────────────────────────────────┘  ║
║                                                          ║
╠══════════════════════════════════════════════════════════╣
║    [1/2/3] Select    [Up/Down] Navigate    [Enter] OK    ║
╚══════════════════════════════════════════════════════════╝
```

---

## 3. HUD Layout

Bottom 6 lines of terminal. map_height = term_height - 7.

```
────────────────────────────────────────────────────────────────
Floor:3 | HP:18/24 [███████░░░] | Lv:4 | Warrior | Iron Sword
XP:45/120 [███░░░░░░░] | STR:14 DEX:10 INT:8 CON:14 | Def:3
[1]Power Attack (READY)  [2]War Cry (Lv5 locked)  | Tab:Inv
> You hit the Goblin for 8 damage!
> The Troll hits you for 5 damage!
> You pick up a Health Potion!
```

### HP Bar styles
```
Full:     [██████████]  24/24
Mid:      [██████░░░░]  14/24
Low:      [███░░░░░░░]   8/24   <- render in Red when < 30%
Critical: [█░░░░░░░░░]   3/24   <- render in DarkRed + blink
```

### XP Bar
```
XP:42/120 [███░░░░░░░]
```

---

## 4. Inventory Screen

Opens on Tab. Pauses monster tick.

```
╔══════════════════════════════════════════════════════════╗
║                   INVENTORY  (4/10)                      ║
╠══════════════════════════════════════════════════════════╣
║  EQUIPPED                                                ║
║  ┌──────────────────────────────────────────────────┐    ║
║  │ [W] * Ragefang *            / artifact weapon    │    ║
║  │     Kill = +1 ATK 3 ticks (max 5 stacks)         │    ║
║  ├──────────────────────────────────────────────────┤    ║
║  │ [A] Leather Armor (+1 def)  [ armor              │    ║
║  ├──────────────────────────────────────────────────┤    ║
║  │ [R] (empty ring slot)                            │    ║
║  └──────────────────────────────────────────────────┘    ║
║                                                          ║
║  BACKPACK                                                ║
║    a) Health Potion (heals 7)                            ║
║    b) Shortsword (+2 dmg)                                ║
║    c) Ring of Strength (+1 STR)                          ║
║                                                          ║
╠══════════════════════════════════════════════════════════╣
║  [a-z] Select item    [Tab] Close                        ║
╚══════════════════════════════════════════════════════════╝
```

### Item action prompt (after selecting)
```
╔══════════════════════════════════╗
║  Shortsword (+2 dmg)             ║
╠══════════════════════════════════╣
║  [E] Equip                       ║
║  [D] Drop                        ║
║  [Esc] Cancel                    ║
╚══════════════════════════════════╝
```

---

## 5. Boss Intro Screens

Drawn line by line on boss floor entry.
Each line prints with 120ms delay.
"Press any key" appears last.

### Goblin King — Floor 5 (Yellow)
```
         ___
        /   \
       | ^ ^ |      THE GOBLIN KING
       |  _  |
      /|     |\     "Your gold belongs
     / |     | \     to ME now."
    /__|_____|__\
       |  K  |
      /|     |\
     g . . . . g
```

### Bone Dragon — Floor 10 (DarkRed)
```
                   /\
      __           \/    __
     /  \    /\  /\/\  /\  \
    / /\ \  /  \/    \/  \ /\ \
   / /  \ \/   D      \   V  \ \
  /_/    \_\  /  \  /  \  /_\ \_\
             /____\/____\
                 |    |
              ~~~fire~~~
```

### Shadow Lord — Floor 15+ (Magenta)
```
      *          *
   *     \   /     *
      *   \ /   *
   ----====S====----
      *   / \   *
   *     /   \     *
      *          *

   darkness pulses...
```

---

## 6. Death Screen

Drawn animated, line by line with 80ms delay per skull row,
then 120ms delay per stat line.

```
╔══════════════════════════════════════════════════════════╗
║                                                          ║
║              . . . X . . .                               ║
║            . X X X X X X X .                             ║
║            X X X X X X X X X                             ║
║            . X X X X X X X .                             ║
║              . . X X X . .                               ║
║                  @ @ @                                   ║
║                                                          ║
║          ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~                     ║
║                * YOU HAVE DIED *                         ║
║          ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~                     ║
║                                                          ║
║    Class . . . . . . . . Warrior                         ║
║    Floor reached . . . . 7                               ║
║    Monsters slain  . . . 34                              ║
║    Damage dealt  . . . . 412                             ║
║    Damage taken  . . . . 187                             ║
║    Cause of death  . . . Troll (12 dmg)   <- Red        ║
║                                                          ║
║          ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~                     ║
║                                                          ║
║              [R] Play again    [Q] Quit                  ║
║                                                          ║
╚══════════════════════════════════════════════════════════╝
```

---

## 7. Level Up Notification

Shown in message log area, not a full screen.
Appears for 2 seconds then fades back to log.

```
╔══════════════════════════════════╗
║       *** LEVEL UP! ***          ║
║       You are now Level 5        ║
║                                  ║
║   Ability 2 unlocked!            ║
║   [2] War Cry now available      ║
╚══════════════════════════════════╝
```

---

## 8. Boss Health Bar

Rendered at top of screen when boss is alive.
Disappears when boss dies.

```
┌─────────────────────────────────────────────────────┐
│  GOBLIN KING   [████████████░░░░░░░░░░░░]  38/60 HP │
└─────────────────────────────────────────────────────┘
```

Enraged version (HP < 50%):
```
┌─────────────────────────────────────────────────────┐
│  GOBLIN KING * ENRAGED *  [██████░░░░░░░░░░░]  28/60│
└─────────────────────────────────────────────────────┘
```

---

## 9. Floor Transition Message

Shown briefly when stepping on stairs.
Replaces the generic "Welcome to floor X" log message.

```
Floor 1-3 (dungeon):
  You descend into the stone dungeon...

Floor 4-6 (cavern):
  The walls narrow. You enter the caverns...

Floor 7-9 (void):
  The air turns foul. Darkness seeps from the walls...

Floor 10 (bone pits):
  Ancient bones crunch underfoot...

Floor 15+ (shadow realm):
  Reality warps. You have gone too deep.
```

---

## 10. Minimap Border

Current minimap uses `+---+`. Upgrade to:

```
┌──────────────────────────────┐
│  . . # # # . . . . . . . .  │
│  . # # . # # . . @ . . . .  │
│  . # . . . # # . . . . . .  │
│  . # . . . . # # . . > . .  │
│  . # # # # # # # . . . . .  │
│  . . . . . . . . . . . . .  │
└──────────────────────────────┘
```

Symbols:
- `@` White — player
- `>` Yellow — stairs (only when explored)
- `#` Grey — visible wall
- `#` DarkGrey — explored wall
- `.` DarkGrey — explored floor
- ` ` Black — unexplored

---

## 11. Combat Message Format

Replace plain text log messages with structured format.
Keep it readable — no emoji, pure ASCII symbols.

```
Current:
  You hit the Goblin for 5 damage!
  The Troll hits you for 8 damage!

Upgraded:
  [>>] You strike the Goblin for 5 dmg!
  [<<] The Troll hits you for 8 dmg!
  [!!] CRITICAL STRIKE! 2x damage!
  [**] The Goblin dies!
  [~~] You dodge the Skeleton's arrow!
  [**] Ragefang pulses! (+2 ATK stacked)
  [!!] BOSS ENRAGED!
```

Color coding:
- `[>>]` Cyan — player attack
- `[<<]` Red — damage taken
- `[!!]` Yellow — critical / important
- `[**]` Grey — kill / event
- `[~~]` Green — dodge / positive

---

## 12. Save / Load Confirmation

Small non-intrusive notification in log area.

```
Save:
  [**] Game saved to save.json

Load:
  [**] Game loaded — Floor 3, Warrior, HP 18/24
```

---

## Implementation Notes for Agent

### Safe centering formula (no panic)
```rust
let start_x = term_width.saturating_sub(box_width) / 2;
let start_y = term_height.saturating_sub(box_height) / 2;
```

### Drawing a box
```rust
// Top border
execute!(stdout,
    cursor::MoveTo(x, y),
    Print(format!("╔{}╗", "═".repeat(w - 2)))
)?;
// Side borders
for row in 1..h-1 {
    execute!(stdout,
        cursor::MoveTo(x, y + row as u16),
        Print(format!("║{}║", " ".repeat(w - 2)))
    )?;
}
// Bottom border
execute!(stdout,
    cursor::MoveTo(x, y + h as u16 - 1),
    Print(format!("╚{}╝", "═".repeat(w - 2)))
)?;
```

### HP bar renderer
```rust
fn hp_bar(current: i32, max: i32, width: usize) -> String {
    let filled = (current * width as i32 / max)
        .max(0) as usize;
    let empty = width.saturating_sub(filled);
    format!("[{}{}]",
        "█".repeat(filled),
        "░".repeat(empty)
    )
}
```

### Color rules
- Never use emoji — they are double-width and break alignment
- Box drawing chars (╔═║╚╗╝┌─│└┐┘) are safe — single width
- Block chars (█░) are safe — single width
- Test every screen at 80x24 (minimum terminal size)
- Use saturating_sub() for ALL position calculations

### What NOT to do
- No emoji (🔥👻⚔) — breaks column alignment
- No Unicode arrows (↑↓←→) in game log — use [>>] style instead
- No hardcoded box widths wider than 70 chars
- No fixed position rendering without saturating_sub()
