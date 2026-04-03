# Ironveil

A terminal-based roguelike dungeon crawler written in Rust. Descend through procedurally generated dungeons, fight monsters, collect loot, and defeat bosses.

## Features

- **3 Character Classes**: Warrior, Rogue, Mage — each with 5 unique abilities
- **5 Abilities Per Class**: 4 active + 1 ultimate, unlocked at levels 1/5/10/15/20
- **Procedural Generation**: Every floor is randomly generated with rooms, corridors, and special rooms
- **30+ Monster Types**: Goblins, skeletons, trolls, wraiths, necromancers, demons, elementals, and more
- **Boss Fights**: Unique bosses every 5 floors (Goblin King, Bone Dragon, Shadow Lord)
- **Affix & Exotic Loot System**: Items roll prefixes/suffixes; 27 unique exotic items with trade-offs
- **Shop System**: Merchant NPC appears every 3 floors — buy and sell gear
- **Gold Economy**: Earn gold from monster kills, spend it at shops
- **Inventory System**: Equip weapons, armor, rings; use potions; sell unwanted gear
- **Fog of War**: Line-of-sight visibility with explored memory
- **Save/Load**: Ctrl+S to save, Ctrl+L to load your progress

## Controls

| Key | Action |
|-----|--------|
| Arrow Keys / WASD | Move / Attack |
| 1-5 | Use abilities (unlock at Lv1/5/10/15/20) |
| E | Interact with shop merchant |
| Space | Wait a turn |
| Tab | Open inventory |
| > | Descend stairs |
| Ctrl+S | Save game |
| Ctrl+L | Load game |
| q / Esc | Quit |

## How to Play

```bash
cd ironveil
cargo run
```

## Gameplay Tips

- Watch your HP — enemies can deal significant damage
- Use abilities strategically — they have cooldowns
- Explore every room before descending — there may be valuable items
- Some floors have bosses — defeat them to unblock the stairs
- Visit shops every 3 floors to buy better gear
- Exotic items are powerful but come with trade-offs — read their descriptions
- Gold drops from every monster kill — save up for expensive shop items

## Building

```bash
# Development build
cargo build

# Release build (faster)
cargo build --release
```

## Documentation

- [Classes](docs/features/CLASSES.md) — Character classes and abilities
- [Monsters](docs/features/MONSTERS.md) — Enemy encyclopedia
- [Items](docs/features/ITEMS.md) — Item database, affixes, and exotics
- [Controls](docs/features/CONTROLS.md) — Full control reference
- [Biomes](docs/features/BIOMES.md) — Floor theming
- [Algorithms](docs/algorithms/ALGORITHMS.md) — Technical breakdown of every algorithm used
- [Changelog](CHANGELOG.md) — Version history

## Algorithms Used

Want to know what makes Ironveil tick? Check the [Algorithms Reference](docs/algorithms/ALGORITHMS.md) for detailed explanations of:

- **A* Pathfinding** — Monster navigation around walls and obstacles
- **Bresenham's Line** — Fog of war and line-of-sight checks
- **Finite State Machines** — Monster AI behavior
- **Procedural Generation** — Room placement and tunnel carving
- **Delta Rendering** — Optimized terminal updates
- **Weighted RNG** — Loot drops and rarity scaling
- **D&D Combat Math** — Stat modifiers and damage calculation

Each algorithm includes layman's explanations, real-world equivalents, formulas, examples, and the actual source code.

## Tech Stack

- **Language**: Rust
- **Terminal**: crossterm
- **Random**: rand
- **Serialization**: serde + serde_json

## Project Structure

```
ironveil/
├── src/
│   ├── main.rs        # Game loop, rendering, input handling
│   ├── map.rs         # Procedural map generation, visibility
│   ├── monster.rs     # Monster AI, behaviors, types
│   ├── player.rs      # Player state, stats, abilities
│   ├── items.rs       # Item definitions, loot generation
│   ├── affixes.rs     # Prefix/suffix/exotic data and generation
│   ├── ui.rs          # Character creation, inventory, shop screens
│   ├── projectile.rs  # Arrow/projectile handling
│   └── save_load.rs   # Game save/load
├── docs/
│   ├── algorithms/    # Algorithm documentation
│   ├── features/      # Game feature docs
│   ├── planning/      # Implementation plans
│   └── research/      # Research notes
├── CHANGELOG.md
└── Cargo.toml
```
