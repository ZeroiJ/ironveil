# Ironveil — Phase 3 Implementation Plan

## "You Have an Identity"

*Planned: 2026-02-21*

---

## Overview

Phase 3 transforms Ironveil from a generic dungeon crawler into a game where your character has a class, stats, equipment, and an inventory. Every run now starts with a meaningful choice, and loot found in the dungeon shapes your build. This is where identity and replayability come in.

---

## Design Decisions (Locked In)

| Decision | Choice |
|----------|--------|
| Classes | Warrior / Rogue / Mage (3 classes) |
| Creation screen | Visual with ASCII art + stat previews |
| Stats | Wired into gameplay immediately (not display-only) |
| Inventory | Full inventory + equipment slots |
| Potions | Go to inventory (no longer auto-use on walk) |
| Item spawning | Both room spawns AND monster drops |
| Drop rate | ~30% chance per monster kill |
| Item quality | Scales with floor depth |
| Starting gear | Class-specific starter equipment |
| Inventory key | `Tab` to open |

---

## Implementation Steps

### 3.1 — Stats & Class System (`player.rs`)

**New struct**: `Stats { str, dex, int, con }`

**Three classes** with different base stats:

| Class | Color | STR | DEX | INT | CON | Playstyle |
|-------|-------|-----|-----|-----|-----|-----------|
| Warrior | Red `@` | 14 | 10 | 8 | 14 | Tanky brawler. Hit hard, take hits. |
| Rogue | Green `@` | 10 | 14 | 10 | 10 | Dodge + crits. Glass cannon, high risk/reward. |
| Mage | Blue `@` | 8 | 10 | 14 | 8 | Low HP, future spell power. INT placeholder for Phase 4. |

**Stats affect gameplay**:
- **STR** → melee damage bonus: `(STR - 10) / 2` added to attack damage
- **DEX** → dodge chance: `(DEX - 10) * 3`% chance to completely avoid incoming damage
- **CON** → max HP: `base 20 + (CON - 10)` gives Warrior 24 HP, Mage 18 HP
- **INT** → potion healing bonus: `+1 per 2 INT above 10` (Mage gets better heals)

---

### 3.2 — Character Creation Screen (`ui.rs`)

Shown before the dungeon loads, on game start and on restart after death.

**Layout**:
- Title: "CHOOSE YOUR CLASS" in bold
- ASCII art for each class (sword, daggers, staff)
- Stat bars and starting equipment preview
- Playstyle description
- Arrow keys (Up/Down) to navigate, Enter to select

**Example**:
```
╔══════════════════════════════════════╗
║         CHOOSE YOUR CLASS            ║
╠══════════════════════════════════════╣
║                                      ║
║   [1]  WARRIOR        ⚔              ║
║        STR ████████░░  14            ║
║        DEX █████░░░░░  10            ║
║        INT ████░░░░░░   8            ║
║        CON ████████░░  14            ║
║        HP: 24                        ║
║        Starts with: Iron Shortsword  ║
║                     Leather Armor    ║
║                                      ║
║   [2]  ROGUE          ⚔⚔            ║
║   [3]  MAGE           ✦              ║
║                                      ║
║        ↑↓ Navigate  •  Enter Select  ║
╚══════════════════════════════════════╝
```

---

### 3.3 — Item System (`items.rs`)

**New module**: `src/items.rs`

**Item struct**:
```
Item {
    name: String,
    item_type: ItemType,       // Weapon, Armor, Ring, Potion
    damage_bonus: i32,         // for weapons
    defense_bonus: i32,        // for armor
    stat_bonus: (Stat, i32),   // for rings (e.g. +2 STR)
    heal_amount: i32,          // for potions
    floor_level: i32,          // quality tier
}
```

**Item types and examples**:

| Type | Symbol | Color | Examples |
|------|--------|-------|----------|
| Weapon | `/` | Cyan | Dagger (+1), Shortsword (+2), Longsword (+3), Greataxe (+5) |
| Armor | `[` | Brown/Yellow | Leather (+1), Chainmail (+2), Plate (+4) |
| Ring | `=` | Gold/Yellow | Ring of Strength (+2 STR), Ring of Vitality (+2 CON) |
| Potion | `!` | Magenta | Health Potion (heals 7 HP) |

**Equipment slots** (3 total):
- Weapon slot → affects damage
- Armor slot → reduces incoming damage
- Ring slot → gives a stat bonus

**Quality scaling with floor depth**:
- Floors 1-3: tier 1 items (Dagger, Leather, minor rings)
- Floors 4-6: tier 2 items (Shortsword, Chainmail, better rings)
- Floors 7+: tier 3 items (Longsword/Greataxe, Plate, strong rings)

---

### 3.4 — Inventory System (`main.rs` + `ui.rs`)

**Open with `Tab`**. Full-screen overlay on top of the map.

**Layout**:
```
╔═══════════════════════════════════╗
║           INVENTORY               ║
╠═══════════════════════════════════╣
║  Equipment:                       ║
║    Weapon: Iron Shortsword (+2)   ║
║    Armor:  Leather Armor (+1)     ║
║    Ring:   (empty)                ║
║                                   ║
║  Backpack:                        ║
║    a) Health Potion               ║
║    b) Ring of Strength (+2 STR)   ║
║    c) Chainmail (+2)              ║
║                                   ║
║  [a-z] Select  •  Tab Close       ║
╚═══════════════════════════════════╝
```

**Item actions** (after selecting with a letter key):
- **Equip** (weapons/armor/rings) — swaps with current equipment if slot is occupied
- **Use** (potions) — drinks potion, heals, removes from inventory
- **Drop** — places item on the ground at player's feet

**Inventory capacity**: 10 items max. If full, new items aren't picked up (log message: "Your inventory is full!")

---

### 3.5 — Item Spawning

**Room spawns**:
- ~25% chance per room to have a random item on the ground
- Item type weighted: 40% potion, 30% weapon, 20% armor, 10% ring
- Quality based on current floor depth
- Rendered on the map using item symbols (`/`, `[`, `=`, `!`)
- Avoid spawning on room center (monster spawn point) and stairs

**Monster drops**:
- ~30% chance when a monster is killed
- Drop appears on the monster's death tile
- Drop type weighted: 50% potion, 25% weapon, 15% armor, 10% ring
- Quality based on current floor depth

**Pickup**:
- Walking over a ground item picks it up into inventory
- Log message: "You pick up a Shortsword (+2)!"
- If inventory is full: "Your inventory is full!" — item stays on ground

---

### 3.6 — Starting Equipment

| Class | Weapon | Armor | Ring |
|-------|--------|-------|------|
| Warrior | Iron Shortsword (+2 dmg) | Leather Armor (+1 def) | (none) |
| Rogue | Twin Daggers (+1 dmg, implied fast) | (none) | (none) |
| Mage | Wooden Staff (+1 dmg) | (none) | Ring of Intellect (+2 INT) |

Starting equipment is pre-equipped, not in the backpack.

---

### 3.7 — Refactoring Existing Systems

**Potion change**: `Tile::Potion` removed. Potions become items on the ground (part of a ground-item layer or `Tile::Item` variant). Walking over them adds to inventory instead of instant-healing.

**Player color**: Driven by class selection (Red/Green/Blue `@`).

**Combat formulas updated**:
- Player attack: `base_damage + weapon_bonus + STR_modifier`
  - `STR_modifier = (STR - 10) / 2`
  - `base_damage = 1` (fist)
- Incoming damage: `monster_attack - armor_defense`
  - Minimum 1 damage (can't reduce to 0)
- Dodge check: `rand(0..100) < (DEX - 10) * 3`
  - If dodge succeeds: "You dodge the Goblin's attack!"

---

### 3.8 — HUD Updates

**Status bar** (bottom of screen):
```
Floor: 3 | HP: 18/24 | STR:14 DEX:10 INT:8 CON:14 | Warrior | Weapon: Iron Shortsword
```

**Message log** stays as-is (last 3 messages), below the status bar.

---

## Implementation Order

| Step | What | Files Touched | Status |
|------|------|---------------|--------|
| 1 | Stats struct + class enum + wire into Player | `player.rs` | ✅ |
| 2 | Item struct + item generation functions | `items.rs` (new) | ✅ |
| 3 | Character creation screen | `ui.rs` (new), `main.rs` | ✅ |
| 4 | Player color based on class | `main.rs` | ✅ |
| 5 | Ground items (replace Tile::Potion, add item layer) | `map.rs`, `main.rs` | ✅ |
| 6 | Item pickup into inventory | `player.rs`, `main.rs` | ✅ |
| 7 | Inventory screen (Tab to open, equip/use/drop) | `ui.rs`, `main.rs` | ✅ |
| 8 | Equipment affects stats (damage, defense, bonuses) | `player.rs`, `main.rs` | ✅ |
| 9 | Monster drops on kill | `main.rs` | ✅ |
| 10 | Wire STR/DEX/CON/INT into combat | `main.rs` | ✅ |
| 11 | Starting equipment per class | `player.rs`, `items.rs` | ✅ |
| 12 | HUD updates (stats, weapon, class name) | `main.rs` | ✅ |
| 13 | Update CHANGELOG.md | `CHANGELOG.md` | ✅ |

---

## New File Structure After Phase 3

```
ironveil/
├── Cargo.toml
└── src/
    ├── main.rs         # Game loop, rendering, input handling
    ├── map.rs          # Map generation, tiles, room spawning
    ├── player.rs       # Player struct, stats, class, inventory, equipment
    ├── monster.rs      # Monster types, AI, combat
    ├── items.rs        # Item definitions, generation, equipment slots (NEW)
    └── ui.rs           # Character creation, inventory screen (NEW)
```
