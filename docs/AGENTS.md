# PROJECT KNOWLEDGE BASE

**Generated:** 2026-03-25
**Commit:** 7dc9c9b
**Branch:** master

## OVERVIEW

Terminal-based roguelike game "Ironveil" written in Rust. Uses crossterm for terminal rendering. Turn-based dungeon crawler with procedural generation, combat, inventory, and multiple character classes.

## STRUCTURE

```
ironveil/
├── src/
│   ├── main.rs       # Game loop, rendering, input handling
│   ├── map.rs        # Procedural map generation, visibility
│   ├── monster.rs    # Monster AI, behaviors, types
│   ├── player.rs     # Player state, stats, abilities
│   ├── items.rs      # Item definitions, loot generation
│   ├── ui.rs         # Character creation, inventory screens
│   ├── projectile.rs # Arrow/projectile handling
│   └── save_load.rs  # Game save/load
├── docs/             # Documentation
│   ├── AGENTS.md     # This file
│   ├── BIOMES.md     # Biome definitions
│   ├── CLASSES.md    # Character classes
│   ├── CONTROLS.md   # Input controls
│   ├── ITEMS.md      # Item database
│   ├── MONSTERS.md   # Monster encyclopedia
│   └── *.md          # Plans and improvements
├── CHANGELOG.md      # Version history
├── Cargo.toml        # crossterm, rand, serde
└── target/          # Build output
```

## WHERE TO LOOK

| Task | Location | Notes |
|------|----------|-------|
| Game loop | `main.rs` | Entry point, main loop, input handling |
| Map gen | `map.rs` | Room placement, tunnels, stairs |
| Monster AI | `monster.rs` | Pathfinding, behaviors, combat |
| Player logic | `player.rs` | Stats, abilities, inventory |
| Rendering | `main.rs` | `render_map`, `render_tile`, `render_ui` functions |
| Fog of war | `map.rs` | `reveal_area`, `visibility` arrays |

## CODE MAP

| Symbol | Type | Location | Role |
|--------|------|----------|------|
| `Map` | struct | map.rs:13 | Dungeon map, tiles, visibility |
| `Player` | struct | player.rs:17 | Player state, HP, stats |
| `Monster` | struct | monster.rs:50 | Enemies with AI |
| `Tile` | enum | map.rs:6 | Wall/Floor/Stairs |
| `Class` | enum | player.rs:5 | Warrior/Rogue/Mage |
| `render_map` | fn | main.rs:30 | Render entire map |
| `reveal_area` | fn | map.rs:115 | Fog of war (disabled) |

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

- Large single-file modules (main.rs 1575 lines, monster.rs 2400+ lines)
- Procedural room generation with corridors
- Turn-based with monster tick timer (500ms)
- Character creation screen before game start

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
