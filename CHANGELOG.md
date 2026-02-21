# Ironveil Development Changelog

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
