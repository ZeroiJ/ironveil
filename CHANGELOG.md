# Ironveil Development Changelog

## Phase 2.5: Monsters That Think
*Completed: 2026-02-21*

### 2.5.9 Floor-Tier Scaling System
- Monster behaviors scale with floor depth: Tier 1 (floors 1-3), Tier 2 (floors 4-6), Tier 3 (floors 7+).
- `Monster::new()` takes a `floor` parameter, `spawn_monsters_for_floor()` passes current floor through.
- Early floors teach basics; deeper floors punish mistakes with tactical AI.

### 2.5.8 Troll AI — Corridor Blocking & Berserk Mode
- Tier 1: Slow chase (acts every other turn). Hits hard (8 damage).
- Tier 2+: Blocks corridors — stops moving and waits when in a narrow passage with LOS on player.
- Tier 3+: Berserk mode at <30% HP — attack jumps to 12, no longer slow. "The Troll flies into a rage!"

### 2.5.7 Goblin AI — Retreat & Lure
- Tier 1: Aggressive chase + melee attack.
- Tier 2+: Retreats when low HP, pathfinds toward nearest ally (lure behavior).
- Tier 3+: Moves 2 tiles per turn. Dash-attack if second step lands on player.

### 2.5.6 Skeleton AI — Ranged Arrows & Repositioning
- Tier 1: Chase + melee with A* pathfinding.
- Tier 2+: Fires arrows at 3-8 tile range with LOS. 2-turn cooldown between shots.
- Tier 3+: Repositions away if player closes within 3 tiles.

### 2.5.5 Projectile System
- Created `projectile.rs` with `Projectile` struct (position, direction, damage, symbol).
- Directional arrow symbols: `-`, `|`, `/`, `\` based on travel direction. Rendered in yellow.
- Projectile lifecycle: spawn → advance 1 tile/turn → collide with player/monster/wall → remove.
- Arrow-player hits deal damage with log message. Arrow-monster hits stop the arrow (friendly fire).

### 2.5.4 Monster Behavior State Machine
- Added `BehaviorState` enum: Idle, Chase, Attack, Ranged, Retreat, Reposition.
- Added `MonsterAction` enum: Nothing, MoveTo, MeleeAttack, FireProjectile.
- `decide_action()` dispatches to monster-specific AI functions per turn.
- Each monster type has its own AI function with tier-aware behavior.

### 2.5.3 Monster Struct Overhaul
- Added fields: `behavior`, `can_see_player`, `last_known_player_pos`, `ranged_cooldown`, `turn_parity`, `floor_tier`, `is_berserk`, `base_attack`.
- Shared helper methods: `flee_from()`, `wander()` (toward last known pos then random), `find_nearest_ally()`.

### 2.5.2 A* Pathfinding
- Implemented A* search in `map.rs` (`astar_next_step`).
- Returns the next step toward a goal, respects walls and occupied monster positions.
- 500-iteration safety limit to prevent lag on large maps.
- Replaces old single-axis movement — monsters navigate around corners.

### 2.5.1 Bresenham Line-of-Sight
- Implemented Bresenham's line algorithm in `map.rs` (`has_line_of_sight`).
- Monsters only detect the player through clear sightlines — walls block vision.
- Also used for Skeleton ranged attack eligibility.
- Added `is_corridor()` helper for Troll corridor-blocking detection.
- Added `distance()` Manhattan distance helper.

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
