# Ironveil — Algorithms Reference

A detailed breakdown of every algorithm used in Ironveil, explained in plain language with real-world equivalents, formulas, examples, and the actual code.

---

## Table of Contents

1. [Pathfinding & Navigation](#1-pathfinding--navigation)
   - [A* (A-Star) Pathfinding](#a-a-star-pathfinding)
   - [Manhattan Distance](#b-manhattan-distance)
2. [Vision & Light](#2-vision--light)
   - [Bresenham's Line Algorithm](#a-bresenhams-line-algorithm)
   - [Circular Radius Check (Pythagorean)](#b-circular-radius-check)
3. [Procedural Generation](#3-procedural-generation)
   - [Room Placement with Intersection Checks](#a-room-placement-with-intersection-checks)
   - [L-Shaped Tunnel Carving](#b-l-shaped-tunnel-carving)
   - [Boss Arena Generation](#c-boss-arena-generation)
4. [AI Behavior](#4-ai-behavior)
   - [Finite State Machine](#a-finite-state-machine)
   - [Behavior Tiers](#b-behavior-tiers)
   - [Wander / Last Known Position](#c-wander--last-known-position)
5. [Rendering](#5-rendering)
   - [Delta Rendering](#a-delta-rendering)
   - [Layered Rendering Pipeline](#b-layered-rendering-pipeline)
6. [Probability & RNG](#6-probability--rng)
   - [Weighted Random Rolls](#a-weighted-random-rolls)
   - [Floor-Scaled Probability](#b-floor-scaled-probability)
   - [Percentage Chance Gates](#c-percentage-chance-gates)
7. [Combat Math](#7-combat-math)
   - [D&D Modifier Formula](#a-dd-modifier-formula)
   - [Damage Reduction](#b-damage-reduction)
   - [Damage Variance Roll](#c-damage-variance-roll)

---

## 1. Pathfinding & Navigation

### A* (A-Star) Pathfinding

**File:** `src/map.rs` — `astar_next_step()`

**What it does:** Finds the shortest path from a monster to the player while avoiding walls and other monsters.

**Layman's term:** Imagine you're in a maze and want to find the fastest way to the exit. A* is like a smart GPS that checks every possible route, scores them by how far they are from the goal, and always picks the most promising path first. It stops after 500 steps to prevent the game from freezing on huge maps.

**Real-world equivalent:** Google Maps finding the fastest driving route through city streets — but on a grid where you can only move up, down, left, or right.

**How it works:**
1. Start at the monster's position
2. Check all 4 neighboring tiles (up, down, left, right)
3. Score each tile: `f = g + h`
   - `g` = cost to reach this tile from start (number of steps taken)
   - `h` = estimated cost from this tile to goal (Manhattan distance)
4. Pick the tile with the lowest `f` score
5. Repeat until you reach the player (or hit 500 iterations)
6. Return only the *first step* of the path (not the whole path)

**Formula:**
```
f(n) = g(n) + h(n)

where:
  g(n) = actual cost from start to node n
  h(n) = heuristic estimate from n to goal
       = |x₁ - x₂| + |y₁ - y₂|  (Manhattan distance)
```

**Example:**
```
Monster at (2, 3), Player at (8, 7)

Step 1: Check neighbors of (2,3)
  (3,3): g=1, h=|3-8|+|3-7|=5+4=9, f=10
  (1,3): g=1, h=|1-8|+|3-7|=7+4=11, f=12  (worse, skip)
  (2,4): g=1, h=|2-8|+|4-7|=6+3=9, f=10
  (2,2): g=1, h=|2-8|+|2-7|=6+5=11, f=12  (worse, skip)

Pick (3,3) or (2,4) — both have f=10
```

**Actual code:**
```rust
// src/map.rs — astar_next_step()
pub fn astar_next_step(
    &self,
    start: (usize, usize),
    goal: (usize, usize),
    occupied: &[(usize, usize)],
) -> Option<(usize, usize)> {
    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut g_score: HashMap<(usize, usize), i32> = HashMap::new();

    g_score.insert(start, 0);
    open_set.push(AStarNode {
        position: start,
        f_score: self.manhattan_distance(start, goal),
    });

    let mut iterations = 0;
    while let Some(current) = open_set.pop() {
        iterations += 1;
        if iterations > 500 {
            break; // Safety limit
        }

        if current.position == goal {
            // Reconstruct path and return first step
            let mut pos = current.position;
            while let Some(&parent) = came_from.get(&pos) {
                if parent == start {
                    return Some(pos);
                }
                pos = parent;
            }
            return None;
        }

        for &(dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let neighbor = (current.position.0 as i32 + dx, current.position.1 as i32 + dy);
            // ... check bounds, walls, occupied ...
            let tentative_g = g_score[&current.position] + 1;
            if tentative_g < *g_score.get(&neighbor).unwrap_or(&i32::MAX) {
                came_from.insert(neighbor, current.position);
                g_score.insert(neighbor, tentative_g);
                let f = tentative_g + self.manhattan_distance(neighbor, goal) as i32;
                open_set.push(AStarNode { position: neighbor, f_score: f });
            }
        }
    }
    None
}
```

---

### B. Manhattan Distance

**File:** `src/map.rs` — `distance()`

**What it does:** Measures the distance between two tiles on a grid, counting only horizontal and vertical steps (no diagonals).

**Layman's term:** If you're in a city with a perfect grid of streets, Manhattan distance is how many blocks you need to walk to get from point A to point B. You can't cut through buildings (no diagonals), so you go east/west first, then north/south.

**Real-world equivalent:** How many blocks to walk in Manhattan, New York — you can only go along streets, not through buildings.

**Formula:**
```
distance = |x₁ - x₂| + |y₁ - y₂|

where:
  |x| = absolute value of x (always positive)
```

**Example:**
```
Point A: (3, 5)
Point B: (8, 2)

distance = |3 - 8| + |5 - 2|
         = |-5| + |3|
         = 5 + 3
         = 8 tiles

You need to walk 8 steps to get from A to B.
```

**Where it's used:**
- Monster detection range (can the monster see the player?)
- AoE radius checks (is the enemy inside War Cry's range?)
- A* heuristic (estimating distance to goal)
- Boss pulse attack range

**Actual code:**
```rust
// src/map.rs
pub fn distance(x1: usize, y1: usize, x2: usize, y2: usize) -> i32 {
    (x1 as i32 - x2 as i32).abs() + (y1 as i32 - y2 as i32).abs()
}
```

---

## 2. Vision & Light

### A. Bresenham's Line Algorithm

**File:** `src/map.rs` — `has_line_of_sight()`

**What it does:** Draws a straight line between two points on a grid and checks if any walls block the path. Used for fog of war and monster vision.

**Layman's term:** Imagine shining a laser pointer from your eyes to a target. Bresenham's algorithm traces the laser beam tile by tile across the grid. If the beam hits a wall before reaching the target, the target is hidden. If the beam reaches the target unblocked, you can see it.

**Real-world equivalent:** Shining a torch in a dark dungeon — does the beam of light hit a wall before reaching the monster, or does it reach the monster directly?

**How it works:**
1. Start at point A (player position)
2. Calculate the slope to point B (target position)
3. Step through the grid one tile at a time, choosing the tile closest to the ideal line
4. At each step, check if the tile is a wall
5. If a wall is found → no line of sight
6. If we reach point B without hitting a wall → line of sight confirmed

**Formula:**
```
error = 2 * dy - dx
for each step:
  if error > 0:
    y += y_step
    error -= 2 * dx
  x += x_step
  error += 2 * dy
```

**Example:**
```
Player at (0, 0), Target at (5, 3)

Step 0: (0, 0) — floor ✓
Step 1: (1, 0) — floor ✓
Step 2: (2, 1) — floor ✓
Step 3: (3, 1) — floor ✓
Step 4: (4, 2) — WALL ✗ → No line of sight!
```

**Actual code:**
```rust
// src/map.rs — has_line_of_sight()
pub fn has_line_of_sight(&self, x1: usize, y1: usize, x2: usize, y2: usize) -> bool {
    let mut x = x1 as i32;
    let mut y = y1 as i32;
    let dx = (x2 as i32 - x).abs();
    let dy = (y2 as i32 - y).abs();
    let sx = if x1 < x2 { 1 } else { -1 };
    let sy = if y1 < y2 { 1 } else { -1 };
    let mut err = dx - dy;

    loop {
        if x == x2 as i32 && y == y2 as i32 {
            return true;
        }
        if x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height {
            if self.tiles[x as usize][y as usize] == Tile::Wall {
                return false;
            }
        }
        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x += sx;
        }
        if e2 < dx {
            err += dx;
            y += sy;
        }
    }
}
```

---

### B. Circular Radius Check

**File:** `src/map.rs` / `src/main.rs`

**What it does:** Checks whether a tile falls within a circular area around a center point, without using expensive square root calculations.

**Layman's term:** Imagine drawing a circle with a compass on graph paper. This algorithm checks if a specific square on the paper is inside or outside the circle — but instead of measuring the actual distance (which requires a calculator), it uses a clever math trick: if the sum of the squared horizontal and vertical distances is less than the squared radius, the point is inside.

**Real-world equivalent:** Drawing a circle with a compass — is this point inside or outside the circle?

**Formula:**
```
Inside circle if: dx² + dy² ≤ r²

where:
  dx = horizontal distance from center
  dy = vertical distance from center
  r  = radius

No square root needed! We compare squared values instead.
```

**Why no square root?** Square root is computationally expensive. By comparing `dx² + dy²` against `r²` instead of `√(dx² + dy²)` against `r`, we get the same answer much faster.

**Example:**
```
Center: (5, 5), Radius: 3, Check point: (7, 6)

dx = 7 - 5 = 2
dy = 6 - 5 = 1
dx² + dy² = 4 + 1 = 5
r² = 9

5 ≤ 9 → YES, point is inside the circle!

Check point: (9, 9)
dx = 4, dy = 4
dx² + dy² = 16 + 16 = 32
32 ≤ 9 → NO, point is outside the circle.
```

**Where it's used:**
- Fog of war reveal area (radius 8 around player)
- Frost Nova freeze radius (3 tiles)
- War Cry stun radius (4 tiles)
- Boss AoE pulse attacks
- Meteor splash damage (radius 2)

**Actual code:**
```rust
// Used throughout main.rs for AoE checks
let dx = (target_x as i32 - center_x as i32).abs();
let dy = (target_y as i32 - center_y as i32).abs();
if dx * dx + dy * dy <= radius * radius {
    // Target is inside the circle — apply effect
}
```

---

## 3. Procedural Generation

### A. Room Placement with Intersection Checks

**File:** `src/map.rs` — `Map::new()`

**What it does:** Randomly places rooms on the map and rejects any room that would overlap with an existing room.

**Layman's term:** Imagine trying to park cars in a parking lot. You pick a random spot, but before parking, you check if another car is already there. If the spot is taken, you try a different random spot. You keep doing this until you've parked enough cars or run out of attempts.

**Real-world equivalent:** Trying to park cars in a lot — don't put two cars in the same spot.

**How it works:**
1. Pick a random position and size for a new room
2. Check if this room overlaps with any existing room
3. If no overlap → add the room
4. If overlap → discard and try again
5. Repeat until enough rooms are placed or max attempts reached

**Rectangle intersection formula:**
```
Two rectangles overlap if:
  NOT (room1.right < room2.left   OR
       room1.left > room2.right   OR
       room1.bottom < room2.top   OR
       room1.top > room2.bottom)

In other words, they overlap when:
  room1.right >= room2.left AND
  room1.left <= room2.right AND
  room1.bottom >= room2.top AND
  room1.top <= room2.bottom
```

**Example:**
```
Existing room: x=5..15, y=3..13 (10x10 room)
New room attempt: x=12..20, y=8..16 (8x8 room)

Check: 15 >= 12? YES → overlap on x-axis
       13 >= 8?  YES → overlap on y-axis
       → REJECT this room, try again

Next attempt: x=20..28, y=3..11 (8x8 room)
Check: 15 >= 20? NO → no overlap
       → ACCEPT this room
```

**Actual code:**
```rust
// src/map.rs — room intersection check
fn intersects(r1: &Rect, r2: &Rect) -> bool {
    r1.x1 <= r2.x2 && r1.x2 >= r2.x1 && r1.y1 <= r2.y2 && r1.y2 >= r2.y1
}

// Room placement loop
let mut attempts = 0;
while rooms.len() < target_rooms && attempts < 100 {
    let w = rng.random_range(min_w..=max_w);
    let h = rng.random_range(min_h..=max_h);
    let x = rng.random_range(1..=(map_w - w - 1));
    let y = rng.random_range(1..=(map_h - h - 1));
    let new_room = Rect { x1: x, y1: y, x2: x + w, y2: y + h };

    let overlaps = rooms.iter().any(|r| intersects(r, &new_room));
    if !overlaps {
        rooms.push(new_room);
    }
    attempts += 1;
}
```

---

### B. L-Shaped Tunnel Carving

**File:** `src/map.rs` — tunnel carving between rooms

**What it does:** Connects two rooms with corridors by going horizontally first, then vertically (or vice versa), creating an L-shaped path.

**Layman's term:** To connect two rooms, you dig a tunnel that goes straight in one direction, then makes a 90-degree turn and goes straight in the other direction. Sometimes you go east-then-north, sometimes north-then-east — chosen randomly for variety.

**Real-world equivalent:** Street layout in a grid city — to get from intersection A to B, you go east on Main Street, then turn north on 5th Avenue.

**How it works:**
1. Get the center points of two adjacent rooms
2. Flip a coin: 50% chance horizontal-first, 50% vertical-first
3. Carve a corridor in the first direction
4. Carve a corridor in the second direction
5. The two corridors meet at a corner, forming an L-shape

**Example:**
```
Room A center: (5, 5)
Room B center: (15, 12)

Horizontal-first path:
  Carve from (5,5) to (15,5)  ← horizontal corridor
  Carve from (15,5) to (15,12) ← vertical corridor
  Result: ─────────────┐
                       │
                       │

Vertical-first path:
  Carve from (5,5) to (5,12)   ← vertical corridor
  Carve from (5,12) to (15,12) ← horizontal corridor
  Result: │
          │
          └─────────────
```

**Actual code:**
```rust
// src/map.rs — tunnel carving
fn create_tunnel(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
    let mut rng = rand::rng();
    if rng.random_bool(0.5) {
        // Horizontal first, then vertical
        for x in min(x1, x2)..=max(x1, x2) {
            self.tiles[x][y1] = Tile::Floor;
        }
        for y in min(y1, y2)..=max(y1, y2) {
            self.tiles[x2][y] = Tile::Floor;
        }
    } else {
        // Vertical first, then horizontal
        for y in min(y1, y2)..=max(y1, y2) {
            self.tiles[x1][y] = Tile::Floor;
        }
        for x in min(x1, x2)..=max(x1, x2) {
            self.tiles[x][y2] = Tile::Floor;
        }
    }
}
```

---

### C. Boss Arena Generation

**File:** `src/map.rs` — boss room placement

**What it does:** On boss floors (every 5th floor), guarantees a large open arena room at the end of the dungeon, regardless of normal room generation.

**Layman's term:** Normally rooms are placed randomly. But on boss floors, the game says "forget random — I need a big fight arena here." It carves out a large rectangular space (18-24 wide × 12-16 tall) and places the boss and stairs inside it.

**How it works:**
1. After normal room generation, check if this is a boss floor (floor % 5 == 0)
2. Place a large room in the center-to-lower area of the map
3. Connect it to the last normal room with a tunnel
4. Place the boss monster and stairs inside

**Actual code:**
```rust
// Boss room generation (simplified)
if current_floor > 0 && current_floor % 5 == 0 {
    let boss_w = rng.random_range(18..=24);
    let boss_h = rng.random_range(12..=16);
    let boss_x = (map_w / 2) - (boss_w / 2);
    let boss_y = (map_h / 2) - (boss_h / 2);

    // Carve boss arena
    for x in boss_x..(boss_x + boss_w) {
        for y in boss_y..(boss_y + boss_h) {
            tiles[x][y] = Tile::Floor;
        }
    }
}
```

---

## 4. AI Behavior

### A. Finite State Machine

**File:** `src/monster.rs` — `BehaviorState` enum + `decide_action()`

**What it does:** Every monster cycles through predefined states (Idle, Chase, Attack, Ranged, Retreat, Reposition) with rules for when to switch between them.

**Layman's term:** Think of a monster as a robot with a flowchart in its head. At any moment, it's in one "mode" — like "patrolling" or "chasing." Each mode has specific rules: "If I see the player, switch to Chase mode. If I'm next to the player, switch to Attack mode. If my health is low, switch to Retreat mode."

**Real-world equivalent:** A traffic light — it cycles through fixed states (Red → Green → Yellow → Red) with rules for when to change.

**State diagram:**
```
                    sees player
    [Idle] ─────────────────────→ [Chase]
      ↑                              │
      │                              │ adjacent to player
      │         loses sight          ▼
      │◄────────────────────── [Attack]
      │                              │
      │         low HP               │
      └────── [Retreat] ◄────────────┘
```

**Where each state is used:**
- **Idle**: Monster hasn't detected the player yet — stands still or wanders
- **Chase**: Monster sees the player — uses A* pathfinding to pursue
- **Attack**: Monster is adjacent to the player — deals damage
- **Ranged**: Skeleton fires arrows from distance
- **Retreat**: Low HP monster flees from the player
- **Reposition**: Skeleton backs away if player gets too close

**Actual code:**
```rust
// src/monster.rs
pub enum BehaviorState {
    Idle,
    Chase,
    Attack,
    Ranged,
    Retreat,
    Reposition,
}

impl Monster {
    pub fn decide_action(&mut self, ...) -> MonsterAction {
        match self.behavior {
            BehaviorState::Idle => self.idle_behavior(...),
            BehaviorState::Chase => self.chase_behavior(...),
            BehaviorState::Attack => self.attack_behavior(...),
            BehaviorState::Ranged => self.ranged_behavior(...),
            BehaviorState::Retreat => self.retreat_behavior(...),
            BehaviorState::Reposition => self.reposition_behavior(...),
        }
    }
}
```

---

### B. Behavior Tiers

**File:** `src/monster.rs` — tier-based conditional logic

**What it does:** The same monster type behaves differently depending on floor depth. Three tiers of difficulty: basic (floors 1-3), intermediate (4-6), advanced (7+).

**Layman's term:** A Goblin on floor 1 is dumb and slow. The same Goblin on floor 7 is smarter, faster, and more aggressive. It's the same monster with a different "difficulty setting" based on how deep you've gone.

**Real-world equivalent:** A video game's difficulty settings — Easy, Normal, Hard — but applied per-floor instead of globally.

**Example — Goblin behavior by tier:**
```
Tier 1 (Floors 1-3):
  - Simple chase when player is visible
  - Melee attack only
  - 1 tile per turn

Tier 2 (Floors 4-6):
  - Retreats when low HP
  - Runs toward nearest ally (lure behavior)
  - 1 tile per turn

Tier 3 (Floors 7+):
  - All Tier 2 behaviors
  - Moves 2 tiles per turn (double speed)
  - Dash-attack if second step lands on player
```

**Actual code:**
```rust
// src/monster.rs — Goblin AI
fn goblin_ai(&mut self, player_pos, map, occupied, dist) -> MonsterAction {
    if self.floor_tier >= 2 && self.hp < self.max_hp / 3 {
        // Tier 2+: Retreat when low HP
        self.behavior = BehaviorState::Retreat;
        if let Some(ally) = self.find_nearest_ally(monsters) {
            return self.move_toward(ally.x, ally.y, map, occupied);
        }
    }

    if dist <= 10 && self.can_see_player {
        self.behavior = BehaviorState::Chase;
        if let Some(next) = map.astar_next_step(pos, player_pos, occupied) {
            if self.floor_tier >= 3 {
                // Tier 3: Move 2 tiles
                let second_step = map.astar_next_step(next, player_pos, occupied);
                // ... dash attack check ...
            }
            return MonsterAction::MoveTo(next.0, next.1);
        }
    }

    self.wander(map, occupied)
}
```

---

### C. Wander / Last Known Position

**File:** `src/monster.rs` — `wander()` method

**What it does:** When a monster loses sight of the player, it moves toward where it last saw the player, then wanders randomly if the player isn't there.

**Layman's term:** A guard hears a noise and investigates the spot. If no one's there, the guard patrols the area for a bit, then eventually gives up and goes back to normal patrol.

**Real-world equivalent:** A security guard checking where they heard a noise, looking around, then giving up and walking away.

**How it works:**
1. Monster sees player → stores player's position as `last_known_player_pos`
2. Monster loses sight of player → moves toward `last_known_player_pos`
3. Reaches last known position → player not there → wander randomly
4. After wandering for a while → return to Idle state

**Actual code:**
```rust
// src/monster.rs — wander behavior
fn wander(&mut self, map: &Map, occupied: &[(usize, usize)]) -> MonsterAction {
    if let Some((lx, ly)) = self.last_known_player_pos {
        let dist = Map::distance(self.x, self.y, lx, ly);
        if dist > 2 {
            // Move toward last known position
            if let Some(next) = map.astar_next_step(
                (self.x, self.y), (lx, ly), occupied
            ) {
                return MonsterAction::MoveTo(next.0, next.1);
            }
        }
    }

    // Random wander
    let mut rng = rand::rng();
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let dir = directions[rng.random_range(0..4)];
    let nx = self.x as i32 + dir.0;
    let ny = self.y as i32 + dir.1;
    // ... check valid move ...
    MonsterAction::MoveTo(nx as usize, ny as usize)
}
```

---

## 5. Rendering

### A. Delta Rendering

**File:** `src/main.rs` — `render_map_delta()`

**What it does:** Only redraws tiles whose visibility state changed since the last frame, instead of redrawing the entire map every frame.

**Layman's term:** Imagine editing a document. Instead of re-printing the entire document every time you make a change, you only re-print the lines that actually changed. This saves a ton of time and eliminates flickering.

**Real-world equivalent:** Editing a Google Doc — only the characters you type change on screen; the rest of the document stays exactly as it was.

**How it works:**
1. Keep a copy of the previous frame's visibility state (`prev_visibility`)
2. Compare current visibility against previous visibility
3. Only redraw tiles where visibility changed:
   - Was hidden → now visible (newly revealed)
   - Was visible → now hidden (left vision range)
4. Update `prev_visibility` to match current state

**Performance impact:**
- Full render: ~2000+ terminal write calls per frame
- Delta render: ~50-200 terminal write calls per frame (only changed tiles)
- Result: 90% fewer terminal operations, zero flickering, instant input response

**Actual code:**
```rust
// src/main.rs — delta rendering
fn render_map_delta(
    stdout: &mut std::io::Stdout,
    map: &Map,
    prev_visibility: &mut Vec<Vec<bool>>,
    floor: i32,
) -> std::io::Result<()> {
    for x in 0..map.width {
        for y in 0..map.height {
            let now_visible = map.current_visibility[x][y];
            let was_visible = prev_visibility[x][y];

            if now_visible != was_visible {
                // Only render tiles that changed visibility
                render_tile(stdout, x, y, map, floor)?;
                prev_visibility[x][y] = now_visible;
            }
        }
    }
    Ok(())
}
```

---

### B. Layered Rendering Pipeline

**File:** `src/main.rs` — main game loop render order

**What it does:** Renders the game in a fixed, layered order so that entities always appear on top of tiles, and the player always appears on top of monsters.

**Layman's term:** Like painting a picture — you paint the background first, then the objects, then the characters on top. Each layer covers what's beneath it. The player is always the "top layer" so they're never hidden behind a monster.

**Real-world equivalent:** Layers in Photoshop — background layer first, then objects, then characters, then UI overlay.

**Render order (bottom to top):**
```
Layer 1: Map tiles (walls, floors, stairs)
Layer 2: Ground items (weapons, potions on floor)
Layer 3: Monsters
Layer 4: Merchant NPC (in shop rooms)
Layer 5: Player (@)
Layer 6: Projectiles (arrows, spells)
Layer 7: UI overlay (HUD, minimap, message log)
```

**Why order matters:** If two things occupy the same tile, the last one rendered "wins" and is visible. The player must always be on top so they can see their character even when standing on a monster.

**Actual code:**
```rust
// src/main.rs — render order in game loop
render_map(&mut stdout, &map, current_floor)?;        // Layer 1: tiles
render_ground_items(&mut stdout, &map, &ground_items)?; // Layer 2: items
render_monsters(&mut stdout, &map, &monsters)?;       // Layer 3: monsters
render_merchant_npc(&mut stdout, &map, current_floor)?; // Layer 4: merchant
render_player(&mut stdout, &player, pulse_phase)?;    // Layer 5: player
render_projectiles(&mut stdout, &projectiles)?;       // Layer 6: projectiles
render_ui(&mut stdout, &player, &map, &log, ...)?;    // Layer 7: UI
```

---

## 6. Probability & RNG

### A. Weighted Random Rolls

**File:** `src/items.rs` — `random_item()`, `random_drop()`

**What it does:** Uses cumulative probability ranges to randomly select item types with different weights.

**Layman's term:** Imagine a bag with 100 colored marbles: 40 blue (potion), 30 red (weapon), 20 green (armor), 10 yellow (ring). You reach in and pull out one marble at random. The color you get determines what type of item spawns. More marbles of a color = higher chance of that item.

**Real-world equivalent:** A gumball machine where different colored gumballs have different quantities — more red gumballs means you're more likely to get red.

**How it works:**
1. Roll a random number from 0 to 99
2. Check which range the number falls into:
   - 0-39 (40 numbers) → Potion
   - 40-69 (30 numbers) → Weapon
   - 70-89 (20 numbers) → Armor
   - 90-99 (10 numbers) → Ring
3. Generate the selected item type

**Visual breakdown:**
```
Roll:  0  1  2  ... 39 | 40 41 ... 69 | 70 71 ... 89 | 90 91 ... 99
Item:  P  P  P  ...  P  |  W  W  ...  W  |  A  A  ...  A  |  R  R  ...  R
       ←── 40% ──→      ←── 30% ──→      ←── 20% ──→      ←─ 10% ─→
```

**Actual code:**
```rust
// src/items.rs — random_item()
pub fn random_item(floor: i32) -> Item {
    let rarity = roll_rarity(floor);
    let mut rng = rand::rng();
    let roll = rng.random_range(0..100);

    if roll < 40 {
        // 40% chance → Potion
        let mut item = random_potion();
        item.rarity = rarity;
        item
    } else if roll < 70 {
        // 30% chance → Weapon (40-69)
        let mut item = random_weapon(floor);
        item.rarity = rarity;
        item
    } else if roll < 90 {
        // 20% chance → Armor (70-89)
        let mut item = random_armor(floor);
        item.rarity = rarity;
        item
    } else {
        // 10% chance → Ring (90-99)
        let mut item = random_ring(floor);
        item.rarity = rarity;
        item
    }
}
```

---

### B. Floor-Scaled Probability

**File:** `src/items.rs` — `roll_rarity()`

**What it does:** Increases the chance of rare items as the player goes deeper into the dungeon.

**Layman's term:** The deeper you go, the better the loot. On floor 1, legendary items are almost impossible to find. On floor 15, they're still rare but noticeably more common. It's like fishing — the deeper the water, the bigger the fish.

**How it works:**
Each rarity has a base chance that increases with floor number:

```
Legendary: 0.1% + (floor × 0.05%), capped at 2%
Epic:      1.0% + (floor × 0.3%),  capped at 8%
Rare:      5.0% + (floor × 1.0%),  capped at 20%
Uncommon:  20.0% + (floor × 2.0%), capped at 40%
```

**Example probabilities by floor:**
```
Floor 1:  Legendary 0.15%, Epic 1.3%, Rare 6.0%, Uncommon 22%
Floor 5:  Legendary 0.35%, Epic 2.5%, Rare 10%,  Uncommon 30%
Floor 10: Legendary 0.6%,  Epic 4.0%, Rare 15%,  Uncommon 40%
Floor 15: Legendary 0.85%, Epic 5.5%, Rare 20%,  Uncommon 40% (capped)
```

**Actual code:**
```rust
// src/items.rs — roll_rarity()
pub fn roll_rarity(floor: i32) -> Rarity {
    let mut rng = rand::rng();

    let legendary_chance = (0.001 + floor as f32 * 0.0005).min(0.02);
    let epic_chance = (0.01 + floor as f32 * 0.003).min(0.08);
    let rare_chance = (0.05 + floor as f32 * 0.01).min(0.20);
    let uncommon_chance = (0.20 + floor as f32 * 0.02).min(0.40);

    let roll: f32 = rng.random_range(0.0..1.0);

    if roll < legendary_chance {
        Rarity::Legendary
    } else if roll < epic_chance {
        Rarity::Epic
    } else if roll < rare_chance {
        Rarity::Rare
    } else if roll < uncommon_chance {
        Rarity::Uncommon
    } else {
        Rarity::Common
    }
}
```

---

### C. Percentage Chance Gates

**File:** `src/main.rs` — various random events

**What it does:** Simple "roll under threshold" pattern used for all random events in the game.

**Layman's term:** Roll a 100-sided die. If the number is below your target, the event happens. 30% chance = roll under 30. 3% chance = roll under 3.

**Formula:**
```
Event happens if: random(0..100) < threshold

where threshold is the percentage chance (0-100)
```

**Examples:**
```
30% monster drop:    rng.random_range(0..100) < 30
3% artifact drop:    rng.random_range(0..100) < 3
25% room item spawn: rng.random_range(0..100) < 25
50% coin flip:       rng.random_bool(0.5)
```

**Actual code:**
```rust
// Monster item drop (30% chance)
if rng.random_range(0..100) < 30 {
    let drop = items::random_drop(current_floor);
    ground_items.insert(dpos, drop);
}

// Artifact drop (3% chance, floor 6+)
if current_floor >= 6 && rng.random_range(0..100) < 3 {
    let artifact = items::random_artifact(player.class.name());
    ground_items.insert(dpos, artifact);
}
```

---

## 7. Combat Math

### A. D&D Modifier Formula

**File:** `src/player.rs` — `str_modifier()`, `dex_modifier()`, etc.

**What it does:** Converts raw stat values into combat modifiers using the Dungeons & Dragons formula.

**Layman's term:** Your stats aren't the number you add to attacks — they're converted first. A Strength of 14 gives you +2 damage, not +14. The formula takes your stat, subtracts 10, and divides by 2 (rounding down).

**Real-world equivalent:** Converting test scores to letter grades — a score of 85 doesn't mean you get 85 points; it gets converted to a "B" grade.

**Formula:**
```
modifier = floor((stat - 10) / 2)

where floor() rounds down to the nearest integer
```

**Examples:**
```
STR 14: (14 - 10) / 2 = 4 / 2 = +2 damage
STR 10: (10 - 10) / 2 = 0 / 2 = +0 (average)
STR 8:  (8 - 10) / 2  = -2 / 2 = -1 (below average)
STR 18: (18 - 10) / 2 = 8 / 2 = +4 (very strong)
STR 6:  (6 - 10) / 2  = -4 / 2 = -2 (very weak)

DEX 16: dodge = (16 - 10) × 3 = 18% dodge chance
DEX 10: dodge = (10 - 10) × 3 = 0% (no dodge)
DEX 14: dodge = (14 - 10) × 3 = 12% dodge chance
```

**Full stat table:**
```
Stat | Modifier | Effect
-----|----------|------------------
  3  |   -4     | Very weak
  6  |   -2     | Below average
  8  |   -1     | Slightly below
 10  |   +0     | Average (baseline)
 12  |   +1     | Slightly above
 14  |   +2     | Above average
 16  |   +3     | Strong
 18  |   +4     | Very strong
 20  |   +5     | Exceptional
```

**Actual code:**
```rust
// src/player.rs — stat modifiers
impl Stats {
    pub fn str_modifier(&self) -> i32 {
        (self.str_ - 10) / 2  // Integer division floors automatically
    }

    pub fn dex_modifier(&self) -> i32 {
        (self.dex - 10) / 2
    }

    pub fn int_modifier(&self) -> i32 {
        (self.int - 10) / 2
    }

    pub fn con_modifier(&self) -> i32 {
        (self.con - 10) / 2
    }

    pub fn dodge_chance(&self) -> i32 {
        ((self.dex - 10) * 3).max(0)  // Minimum 0% dodge
    }

    pub fn max_hp(&self) -> i32 {
        20 + (self.con - 10)  // Base 20 HP + CON modifier
    }
}
```

---

### B. Damage Reduction

**File:** `src/player.rs` / `src/main.rs` — incoming damage calculation

**What it does:** Reduces incoming damage by the player's armor defense, but always deals at least 1 damage.

**Layman's term:** Armor acts like a shield that absorbs damage. If a monster hits you for 8 damage and your armor absorbs 3, you only take 5. But armor can never make you completely invincible — you always take at least 1 damage, even with the best armor.

**Formula:**
```
final_damage = max(raw_damage - armor_defense, 1)

where max(a, b) returns the larger of a or b
```

**Examples:**
```
Monster hits for 8, armor = 3:
  final = max(8 - 3, 1) = max(5, 1) = 5 damage

Monster hits for 3, armor = 5:
  final = max(3 - 5, 1) = max(-2, 1) = 1 damage  (minimum!)

Monster hits for 15, armor = 4:
  final = max(15 - 4, 1) = max(11, 1) = 11 damage
```

**Why the minimum of 1?** Without it, a player with 10+ defense would take zero damage from weak monsters, making the game unbeatable. The minimum ensures every hit matters.

**Actual code:**
```rust
// src/main.rs — incoming damage calculation
let raw_damage = monster.attack;
let armor_def = player.equipment.armor_defense();
let final_damage = (raw_damage - armor_def).max(1);

player.take_damage(final_damage);
```

---

### C. Damage Variance Roll

**File:** `src/player.rs` — `apply_damage_variance()`

**What it does:** Adds ±10% randomness to damage so combat never feels completely predictable.

**Layman's term:** Even with the same weapon and stats, you won't deal the exact same damage every hit. Sometimes you hit a little harder (up to +10%), sometimes a little softer (down to -10%). It keeps combat exciting — you never know exactly how much damage your next hit will do.

**Formula:**
```
final_damage = floor(base_damage × random(0.90 .. 1.10))

where random(0.90 .. 1.10) generates a random float between 0.90 and 1.10
```

**Examples:**
```
Base damage: 10

Roll 0.92: final = floor(10 × 0.92) = floor(9.2) = 9 damage  (-10%)
Roll 1.00: final = floor(10 × 1.00) = floor(10.0) = 10 damage (exact)
Roll 1.08: final = floor(10 × 1.08) = floor(10.8) = 10 damage (+8%)
Roll 1.10: final = floor(10 × 1.10) = floor(11.0) = 11 damage (+10%)

So a "10 damage" attack actually deals 9-11 damage in practice.
```

**Actual code:**
```rust
// src/player.rs — apply_damage_variance()
pub fn apply_damage_variance(damage: i32) -> i32 {
    let mut rng = rand::rng();
    let variance: f32 = rng.random_range(90..111) as f32 / 100.0;
    // Generates 0.90, 0.91, 0.92, ... 1.10
    (damage as f32 * variance) as i32
    // Float-to-int conversion floors automatically
}

// Usage in melee_damage():
let mut damage = 1 + weapon_bonus + str_modifier;
damage = Self::apply_damage_variance(damage);
if self.roll_crit() {
    damage = (damage as f32 * crit_multiplier) as i32;
}
```

**Full damage pipeline:**
```
1. Base: 1 (minimum) + weapon_bonus + str_modifier
2. Variance: × random(0.90..1.10)
3. Crit check: if crit → × 1.5
4. Buffs: if Power Attack active → × 2
5. Final: apply to target
```

---

## Summary

| Algorithm | File | Purpose | Complexity |
|-----------|------|---------|------------|
| A* Pathfinding | `map.rs` | Monster navigation | O(b^d) — bounded to 500 iterations |
| Manhattan Distance | `map.rs` | Grid distance measurement | O(1) |
| Bresenham's Line | `map.rs` | Line-of-sight checks | O(max(dx, dy)) |
| Circular Radius | `map.rs` / `main.rs` | AoE range checks | O(1) per tile |
| Room Intersection | `map.rs` | Procedural room placement | O(n) per room |
| L-Shaped Tunnels | `map.rs` | Room connectivity | O(width + height) |
| Finite State Machine | `monster.rs` | Monster AI behavior | O(1) per tick |
| Delta Rendering | `main.rs` | Optimized screen updates | O(changed tiles) |
| Weighted RNG | `items.rs` | Loot type selection | O(1) |
| Floor-Scaled RNG | `items.rs` | Rarity scaling | O(1) |
| D&D Modifiers | `player.rs` | Stat-to-bonus conversion | O(1) |
| Damage Variance | `player.rs` | Combat randomness | O(1) |
