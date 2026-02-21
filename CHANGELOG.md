# Ironveil Development Changelog

## Phase 2: It Wants to Kill You
*Completed: 2026-02-21*

### 2.5 Health Potions (Procedural Items)
- Added `Tile::Potion` variant and `!` symbol rendered in magenta.
- Potions spawn procedurally during map generation (~40% chance per room, avoiding spawn/stairs rooms).
- Walking over a potion auto-picks it up, healing the player for 7 HP (capped at max HP).
- Potions are consumed on pickup — tile reverts to floor.
- Added `Player::heal()` method with max HP clamping.
- Monsters correctly render potion tiles when moving off them.

### 2.4 Scrolling Message Log
- Implemented a persistent UI area at the bottom for game events.
- Added real-time combat feedback: "You hit the Goblin for 5 damage!", "The Troll hits you for 8 damage!".
- Log displays the last 3 messages and handles screen clearing for readability.

### 2.3 Monster AI & Turn-based Movement
- Implemented simple chasing AI: monsters move toward the player if within 10 tiles.
- Turn-based logic: monsters only move or attack after the player takes an action.
- Added collision detection for monsters to prevent them from stacking on the same tile.

### 2.2 Bump-to-Attack Combat
- Implemented standard roguelike "bump" combat (moving into a monster attacks it).
- Added damage calculation and death checks for both player and monsters.
- Monsters now disappear from the screen upon death, restoring the floor tile.

### 2.1 Entity System (Player & Monsters)
- Created `player.rs` and `monster.rs` modules.
- Defined three monster types: Goblin (weak/fast), Skeleton (medium), and Troll (strong/slow).
- Implemented HP tracking and simple stat management for all entities.
- Added random monster spawning (one per room).

## Phase 1: The World Exists
*Completed: 2026-02-21*

### 1.8 Responsive Movement Rate Limiting
- Replaced simple delays with a per-key `Instant`-based cooldown system (100ms).
- Enabled instant direction changes while preventing "runaway" speed when holding a single key.
- Eliminated movement "lag" while maintaining deliberate, roguelike-style control.

### 1.7 Spacious Dungeon Design
- Increased room size constraints (min 6x6, max 15x15) for a more open feel.
- Implemented 2-tile wide corridors (horizontal and vertical) to improve navigation.
- Scaled the number of generated rooms based on terminal area to maintain density.

### 1.6 Dynamic Scaling & Fullscreen Map
- Integrated `crossterm::terminal::size()` to detect terminal dimensions.
- Updated map generation to dynamically fill the entire terminal window.
- Reserved UI space at the bottom for floor tracking and future status bars.

### 1.5 Infinite Descent (Stairs)
- Added `Tile::Stairs` (`>`).
- Implemented floor-transition logic: stepping on stairs triggers a new map generation.
- Added a floor counter UI at the bottom of the screen.

### 1.4 Procedural Dungeon Generation
- Implemented Room-based generation algorithm.
- Added `Rect` struct for room logic and intersection checks.
- Implemented horizontal and vertical tunnel carving.
- Added `get_starting_position` to spawn player on Floor tiles.

### 1.3 Map System & Collision
- Created `map.rs` module.
- Defined `Tile` enum (Wall, Floor) and `Map` struct.
- Implemented collision detection (blocking movement through Walls).

### 1.2 Terminal Rendering & Input
- Enabled terminal raw mode.
- Implemented character rendering (`@`).
- Added keyboard event listener (Arrows to move, `Esc`/`q` to quit).

### 1.1 Project Initialization
- Created Rust project `ironveil`.
- Added `crossterm` for terminal UI and `rand` for procedural generation.
