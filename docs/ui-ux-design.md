# Ironveil UI/UX Design Document — Full Visual Overhaul

**Date:** 2026-03-26  
**Status:** Discussion / Planning Phase  
**Goal:** Complete visual overhaul of Ironveil's terminal UI

---

## 1. Current State Analysis

### What Exists (v0.3.2)

| Component | Implementation | Quality |
|-----------|----------------|---------|
| **Character Creation** | ASCII boxes with `+ - =` | Basic |
| **HUD** | Simple text + borders | Basic |
| **Inventory** | List view with boxes | Basic |
| **Death Screen** | Skull ASCII art (new) | Medium |
| **Box Borders** | `+---+` style | Basic |
| **Colors** | 8-color crossterm palette | Limited |

### Current Code Pattern
```rust
// src/ui.rs - Current approach
let top_border = format!("+{}+", "=".repeat(box_w - 2));
let mid_border = format!("+{}+", "-".repeat(box_w - 2));
let empty_line = format!("|{}|", " ".repeat(box_w - 2));
```

---

## 2. Visual Overhaul Scope

The user requested **full visual overhaul** covering ALL elements:
- Title screen with game logo
- Detailed death screen expansion
- Floor transition art
- Monster/boss portraits
- Item/gear visual enhancements
- UI borders and panels
- Map tiles and terrain
- Effects and particles

---

## 3. Unicode Box-Drawing Characters

### Standard Box (Recommended for most UI)

| Character | Code | Use |
|-----------|------|-----|
| ┌ | U+250C | Top-left corner |
| ┐ | U+2510 | Top-right corner |
| └ | U+2514 | Bottom-left corner |
| ┘ | U+2518 | Bottom-right corner |
| ─ | U+2500 | Horizontal line |
| │ | U+2502 | Vertical line |
| ├ | U+251C | T-junction left |
| ┤ | U+2524 | T-junction right |
| ┬ | U+252C | T-junction top |
| ┴ | U+2534 | T-junction bottom |
| ┼ | U+253C | Cross junction |

### Heavy Box (Emphasized borders)

| Character | Code | Use |
|-----------|------|-----|
| ┏ | U+250F | Top-left corner |
| ┓ | U+2513 | Top-right corner |
| ┗ | U+2517 | Bottom-left corner |
| ┛ | U+251B | Bottom-right corner |
|━ | U+2501 | Horizontal line |
┃ | U+2503 | Vertical line |

### Double Box (Elegant/Important UI)

| Character | Code | Use |
|-----------|------|-----|
| ╔ | U+2550 | Top-left corner |
| ╗ | U+2552 | Top-right corner |
| ╚ | U+2554 | Bottom-left corner |
| ╝ | U+2558 | Bottom-right corner |
| ═ | U+2550 | Horizontal line |
| ║ | U+2551 | Vertical line |

### Rounded (Friendly UI)

| Character | Code | Use |
|-----------|------|-----|
| ╭ | U+252D | Top-left corner |
| ╮ | U+252F | Top-right corner |
| ╯ | U+2531 | Bottom-right corner |
| ╰ | U+2533 | Bottom-left corner |
| ─ | U+2500 | Horizontal line |
| │ | U+2502 | Vertical line |

---

## 4. Shading Characters (Texture/Depth)

### Light to Dark Gradient

| Character | Name | Use |
|-----------|------|-----|
|   (space) | Empty | Background |
| . | Dot | Very light |
| ░ | Light shade | Light texture |
| ▒ | Medium shade | Medium texture |
| ▓ | Dark shade | Dark texture |
| █ | Full block | Solid/active |

### Line Shading

| Character | Name | Use |
|-----------|------|-----|
| ─ | Light horizontal | Subtle dividers |
| ━ | Heavy horizontal | Emphasized dividers |
| │ | Light vertical | Subtle separators |
| ┃ | Heavy vertical | Emphasized separators |

---

## 5. Specialized Characters

### Arrows & Direction

| Character | Code | Use |
|-----------|------|-----|
| ↑ | U+2191 | Up arrow |
| ↓ | U+2193 | Down arrow |
| ← | U+2190 | Left arrow |
| → | U+2192 | Right arrow |
| ↔ | U+2194 | Horizontal double |
| ↕ | U+2195 | Vertical double |

### Symbols

| Character | Code | Use |
|-----------|------|-----|
| ♦ | U+2666 | Diamond/danger |
| ♠ | U+2660 | Spade |
| ♣ | U+2663 | Club |
| ♥ | U+2665 | Heart/positive |
| ★ | U+2605 | Star/important |
| ☆ | U+2606 | Star (outline) |
| ● | U+25CF | Circle filled |
| ○ | U+25CB | Circle outline |
| ■ | U+25A0 | Square filled |
| □ | U+25A1 | Square outline |

### RPG-Specific

| Character | Use |
|-----------|-----|
| @ | Player (standard) |
| # | Wall |
| . | Floor |
| > | Stairs down |
| < | Stairs up |
| + | Door |
| ^ | Trap |
| ! | Potion/scroll |
| / | Weapon |
| [ | Armor |
| ? | Magic item |

---

## 6. Color System Upgrade

### Current (8 colors)
```
Black, White, Grey, DarkGrey
Red, Green, Blue, Cyan, Yellow, Magenta
```

### Recommended (Extend with ANSI sequences)
```
// Keep existing colors
// Add intensity variants via Bold attribute
// Use background colors for panels
```

### Color Palette for Ironveil

| Element | Foreground | Background |
|---------|------------|------------|
| **UI Panel** | White | Black/DarkGrey |
| **Title** | Yellow/Gold | Black |
| **Selection** | White | DarkBlue |
| **Danger/HP** | Red | — |
| **Mana/MP** | Cyan | — |
| **XP/Level** | Yellow | — |
| **Success** | Green | — |
| **Info** | Grey | — |

---

## 7. Screen Designs

### 7.1 Title Screen (NEW)

**Goal:** Dramatic, atmospheric, sets mood

```
╔══════════════════════════════════════════════════════════════╗
║                                                              ║
║                      ███████╗ ██████╗ ██████╗                ║
║                      ╚══███╔╝██╔═══██╗██╔══██╗               ║
║                        ███╔╝ ██║   ██║██████╔╝               ║
║                       ███╔╝  ██║   ██║██╔══██╗               ║
║                      ███████╗╚██████╔╝██║  ██║               ║
║                      ╚══════╝ ╚═════╝ ╚═╝  ╚═╝               ║
║                                                              ║
║                ██████╗  █████╗  ██████╗ ██████╗               ║
║                ██╔══██╗██╔══██╗██╔════╝ ██╔══██╗              ║
║                ██████╔╝███████║██║  ███╗██████╔╝              ║
║                ██╔══██╗██╔══██║██║   ██║██╔══██╗              ║
║                ██║  ██║██║  ██║╚██████╔╝██║  ██║              ║
║                ╚═╝  ╚═╝╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═╝              ║
║                                                              ║
╠══════════════════════════════════════════════════════════════╣
║                          [N]ew Game                            ║
║                          [L]oad Game                           ║
║                          [Q]uit                                ║
╚══════════════════════════════════════════════════════════════╝
```

### 7.2 Character Creation Upgrade

**Goal:** Clear information, visual hierarchy

```
┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┃                        CHOOSE YOUR CLASS                      ┃
┣━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┫
┃                                                               ┃
┃  ┌─────────────────────────────────────────────────────────┐  ┃
┃  │  ⚔ WARRIOR                                              │  ┃
┃  │  ─────────────────────────────────────────────────────── │  ┃
┃  │  STR  DEX  INT  CON          HP: 34    Attack: +2      │  ┃
┃  │  ████ ██░  ██░  ████        ████████████░░░░░░░░       │  ┃
┃  │                                                          │  ┃
┃  │  Abilities:  [Power Attack]  [War Cry]                  │  ┃
┃  │  Style:     Melee Tank / Heavy Hitter                   │  ┃
┃  └─────────────────────────────────────────────────────────┘  ┃
┃                                                               ┃
┃  ┌─────────────────────────────────────────────────────────┐  ┃
┃  │  🗡 ROGUE                                                │  ┃
┃  │  ─────────────────────────────────────────────────────── │  ┃
┃  │  STR  DEX  INT  CON          HP: 26    Attack: +2      │  ┃
┃  │  ██░  ████  ██░  ██░        ████████░░░░░░░░░░░        │  ┃
┃  │                                                          │  ┃
┃  │  Abilities:  [Shadow Step]  [Poison Blade]              │  ┃
┃  │  Style:     Fast / Evasion / Burst                       │  ┃
┃  └─────────────────────────────────────────────────────────┘  ┃
┃                                                               ┃
┃  ┌─────────────────────────────────────────────────────────┐  ┃
┃  │  ✦ MAGE                                                 │  ┃
┃  │  ─────────────────────────────────────────────────────── │  ┃
┃  │  STR  DEX  INT  CON          HP: 22    Attack: +2      │  ┃
┃  │  ██░  ██░   ████ ██░        ██████░░░░░░░░░░░░░         │  ┃
┃  │                                                          │  ┃
┃  │  Abilities:  [Chain Lightning]  [Frost Nova]             │  ┃
┃  │  Style:     Ranged Caster / Area Damage                 │  ┃
┃  └─────────────────────────────────────────────────────────┘  ┃
┃                                                               ┃
┣━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┫
┃  ↑↓ Navigate                    Enter Select                 ┃
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
```

### 7.3 HUD Upgrade

**Goal:** Information-dense but readable

```
┌────────────────────────────────────────────────────────────────┐
│ ┌──────┐                                        ┌───────────┐  │
│ │  @   │  HP: ████████████░░░░ 28/40          │ LVL: 3    │  │
│ │  W   │  MP: ████████░░░░░░░ 20/26            │ XP: 45/50 │  │
│ └──────┘  STR: 14  DEX: 10  INT: 8  CON: 14    │ Floor: 3  │  │
├────────────────────────────────────────────────────────────────┤
│ #...........#                                        F3  ⚔ 2   │
│ #...........#     ░░░                                1  ✦     │
│ #...........#     ░░░   @                            ───────   │
│ ######.#####     ░░░░░                              [1] Ability  │
│ #......#                                            [2] Ability │
│ #..>..#                                            [I] Inv     │
│ ########                                            [?] Help   │
├────────────────────────────────────────────────────────────────┤
│ > You hit the Goblin for 8 damage!                            │
│ > The Goblin hits you for 3 damage!                          │
└────────────────────────────────────────────────────────────────┘
```

### 7.4 Inventory Screen

**Goal:** Organized, clear item hierarchy

```
┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┃           INVENTORY (5/10)                      ┃
┣━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┫
┃  [1] ┌────────────────────────────────────────┐ ┃
┃      │ ⚔ Iron Sword          [Equip] [Drop]   │ ┃
┃      │     Damage: 5-8     Type: Melee         │ ┃
┃      │     Status: Equipped                    │ ┃
┃      └────────────────────────────────────────┘ ┃
┃  [2] ┌────────────────────────────────────────┐ ┃
┃      │ ✦ Potion of Healing    [Use] [Drop]    │ ┃
┃      │     Heal: 15 HP       Qty: 2            │ ┃
┃      └────────────────────────────────────────┘ ┃
┃  [3] ┌────────────────────────────────────────┐ ┃
┃      │ ? Scroll of Fire        [Use] [Drop]     │ ┃
┃      │     Damage: 20 Fire    Qty: 1            │ ┃
┃      └────────────────────────────────────────┘ ┃
┃                                                  ┃
┃  [E]quip  [U]se  [D]rop  [Esc]Close             ┃
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
```

### 7.5 Death Screen (Expanded)

**Goal:** Memorable, respect the run

```
                      . . . X . . .
                    . X X X X X X X X .
                    X X X X X X X X X X
                    . X X X X X X X X .
                      . . X X X . .
                          @ @ @

        ════════════════════════════════════════════
                      * YOU HAVE DIED *
        ════════════════════════════════════════════

        ┌───────────────────────────────────────────┐
        │ Class.......... Warrior                   │
        │ Floor reached. 7                          │
        │ Monsters slain. 23                        │
        │ Damage dealt.... 487                      │
        │ Damage taken.... 312                      │
        │ Cause of death. Bone Dragon (45 dmg)       │
        └───────────────────────────────────────────┘

                    [R] Play Again
                    [Q] Quit
```

---

## 8. Implementation Strategy

### Phase 1: Foundation (Low Effort, High Impact)

1. **Replace box borders** — Unicode box-drawing chars throughout
2. **Upgrade color usage** — Better color choices for UI
3. **Add shading** — Use ░▒▓ for visual interest

### Phase 2: Screens (Medium Effort)

1. **Title screen** — New ASCII art logo
2. **Character creation** — Visual stat bars, ability icons
3. **HUD** — Better layout, visual hierarchy
4. **Inventory** — Clear item display

### Phase 3: Content (High Effort)

1. **Monster portraits** — ASCII art for key enemies
2. **Boss art** — Special ASCII for bosses
3. **Item visuals** — Enhanced item representations
4. **Floor transitions** — ASCII art between floors
5. **Effects** — ASCII particles

---

## 9. Technical Considerations

### Font Requirements
- Monospace font required for alignment
- Must support Unicode block characters
- Recommend: **Fira Code**, **JetBrains Mono**, **Source Code Pro**, or terminal default

### Terminal Compatibility
| Terminal | Unicode Support | Notes |
|----------|-----------------|-------|
| Windows Terminal | ✅ Full | Recommended on Windows |
| iTerm2 | ✅ Full | macOS default |
| Alacritty | ✅ Full | Fast, modern |
| Kitty | ✅ Full | Feature-rich |
| Windows CMD | ⚠️ Limited | May have issues |

### Fallback Strategy
- Store ASCII fallback for each Unicode element
- Detect terminal capabilities at startup
- Graceful degradation if Unicode unavailable

---

## 10. Open Questions

1. **Terminal detection:** Should we detect and adapt, or assume full Unicode support?

2. **ASCII fallback:** For terminals without Unicode, should we provide pure ASCII alternatives?

3. **Animation:** Should we add simple ASCII animations (pulse, blink, scroll)?

4. **Sound:** Beyond visual, should we add terminal bell or audio cues?

---

## 11. Next Steps

1. **Decision:** Confirm implementation phases
2. **Art creation:** Generate ASCII art for each screen
3. **Code refactor:** Update UI functions to use new characters
4. **Testing:** Verify renders correctly in target terminals

---

*Document for Ironveil development planning - visual overhaul discussion.*