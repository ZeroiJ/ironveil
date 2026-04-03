# PROJECT KNOWLEDGE BASE

**Generated:** 2026-04-03
**Commit:** (latest)
**Branch:** master

## OVERVIEW

Terminal-based roguelike game "Ironveil" written in Rust. Uses crossterm for terminal rendering. Turn-based dungeon crawler with procedural generation, combat, inventory, special rooms, shop system, and multiple character classes.

## STRUCTURE

```
ironveil/
├── src/
│   ├── main.rs       # Game loop, rendering, input handling
│   ├── map.rs        # Procedural map generation, visibility, room types
│   ├── monster.rs    # Monster AI, behaviors, types
│   ├── player.rs     # Player state, stats, abilities, shrine buffs, gold
│   ├── items.rs      # Item definitions, loot generation, shop inventory
│   ├── ui.rs         # Character creation, inventory screens, shop screen
│   ├── projectile.rs # Arrow/projectile handling
│   └── save_load.rs  # Game save/load
├── docs/
│   ├── design/       # Design documents
│   │   ├── IRONVEIL_MAP_DESIGN.md
│   │   ├── IRONVEIL_PROCGEN_DESIGN.md
│   │   └── IRONVEIL_ASCII_ART.md
│   ├── features/    # Game features
│   ├── planning/     # Implementation plans
│   ├── research/    # Research notes
│   └── reference/  # References (this file, improvements)
├── CHANGELOG.md     # Version history
├── Cargo.toml       # crossterm, rand, serde
└── target/          # Build output
```

## WHERE TO LOOK

| Task | Location | Notes |
|------|----------|-------|
| Game loop | `main.rs` | Entry point, main loop, input handling |
| Map gen | `map.rs` | Room placement, tunnels, stairs |
| Monster AI | `monster.rs` | Pathfinding, behaviors, combat |
| Player logic | `player.rs` | Stats, abilities, inventory, gold |
| Rendering | `main.rs` | `render_map`, `render_tile`, `render_ui` functions |
| Fog of war | `map.rs` | `reveal_area`, `visibility` arrays |
| Shop system | `ui.rs` | `shop_screen()` function |
| Shop items | `items.rs` | `generate_shop_inventory()`, `get_sell_price()` |

## CODE MAP

| Symbol | Type | Location | Role |
|--------|------|----------|------|
| `Map` | struct | map.rs:41 | Dungeon map, tiles, rooms, visibility |
| `Player` | struct | player.rs:389 | Player state, HP, stats, buffs, gold |
| `Monster` | struct | monster.rs:98 | Enemies with AI, gold_value |
| `Tile` | enum | map.rs:5 | Wall/Floor/Stairs/SecretDoor |
| `RoomType` | enum | map.rs:14 | Normal/Treasure/Trap/Shrine/Secret/Boss/Spawn/Shop |
| `DecoObject` | enum | map.rs:25 | Torch/Pillar/Altar/Chest |
| `Class` | enum | player.rs:252 | Warrior/Rogue/Mage |
| `render_tile` | fn | main.rs:75 | Render tiles with room type theming |
| `render_deco_objects` | fn | main.rs:283 | Render decorative objects |
| `render_merchant_npc` | fn | main.rs:239 | Render merchant NPC in shop rooms |
| `shop_screen` | fn | ui.rs:653 | Shop buy/sell interface |

## KEY FUNCTIONS

| Function | File | Purpose |
|----------|------|---------|
| `Map::new` | map.rs:81 | Generate dungeon with rooms and corridors |
| `Map::assign_room_types` | map.rs:180 | Assign special rooms per floor |
| `Map::generate_decorations` | map.rs:647 | Place torches, pillars, chests, altars |
| `Map::get_room_type_at` | map.rs:638 | Get room type for coordinate |
| `render_deco_objects` | main.rs:283 | Render torches (pulsing), pillars, altars, chests |
| `render_merchant_npc` | main.rs:239 | Render `$` merchant in shop rooms |
| `shop_screen` | ui.rs:653 | Shop UI with buy/sell |
| `generate_shop_inventory` | items.rs:835 | Generate 10 shop items based on level |
| `get_sell_price` | items.rs:882 | Calculate item sell value |
| Player movement | main.rs:2200+ | Trap triggers, shrine interactions |
| Player gold drops | main.rs:2837+ | Gold added on monster death |

## CONVENTIONS

- **Module structure**: One module per major subsystem (.rs file)
- **Visibility arrays**: `visibility[x][y]` = seen ever, `current_visibility[x][y]` = currently visible
- **Map coordinates**: (x, y) where x=col, y=row
- **Color usage**: crossterm `Color` enum for terminal colors

## ANTI-PATTERNS

- `as any`, `@ts-ignore` - NOT applicable (Rust, not TypeScript)
- Empty catch blocks - avoid
- Magic numbers - use named constants where helpful

## UNIQUE STYLES

- Large single-file modules (main.rs 3200+ lines, monster.rs 2400+ lines)
- Procedural room generation with corridors and special rooms
- Turn-based with monster tick timer (500ms)
- Character creation screen before game start
- Special rooms: Treasure, Trap, Shrine, Secret, Shop with unique visuals
- Decorative objects: torches (pulsing), pillars, altars, chests
- Floor-specific tile themes (dungeon/cavern/void)
- Shop system every 3 floors with merchant NPC

## COMMANDS

```bash
cd ironveil
cargo run          # Run the game
cargo build        # Compile
cargo run --release # Optimized build
```

## NOTES

- Fog of war currently DISABLED (all tiles always visible)
- Save/load uses JSON serialization to filesystem
- Terminal size determines map dimensions
- 3 character classes: Warrior, Rogue, Mage
- 5 abilities per class (4 active + 1 ultimate), unlock at levels 1/5/10/15/20
- Gold currency system: monsters drop 5-15g, bosses 50-100g
- Shop appears every 3 floors, press E to interact
