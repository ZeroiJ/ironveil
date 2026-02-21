# Ironveil — Phase 2.5 Implementation Plan

## "Monsters That Think"

*Planned: 2026-02-21*

---

## Overview

Phase 2.5 sits between the existing combat system (Phase 2) and the identity/inventory overhaul (Phase 3). The goal is to replace the current "chase and bump" monster AI with a behavior state machine that makes each monster type feel distinct and dangerous. Skeletons fire arrows. Goblins retreat to lure you into traps. Trolls block corridors and go berserk. Intelligence scales with floor depth so early floors teach you the basics and deeper floors punish mistakes.

No GPU compute needed — this is pure algorithmic AI. A single CPU core handles all of it in microseconds per turn.

---

## Design Decisions (Locked In)

| Decision | Choice |
|----------|--------|
| Turn model | Smart turn-based (monsters act after player turn) |
| Ranged attacks | Visible projectiles for physical (arrows), instant-hit for future magic |
| AI complexity | Medium — chase, retreat, ranged, reposition |
| Intelligence scaling | Behavior tiers that unlock with floor depth |
| Pathfinding | A* algorithm (replaces current single-axis movement) |
| Line of sight | Bresenham's line algorithm (replaces Manhattan distance check) |

---

## Current Monster AI (What We're Replacing)

```
if manhattan_distance(monster, player) <= 1:
    bump attack
elif manhattan_distance(monster, player) < 10:
    move one axis toward player (gets stuck on corners)
else:
    do nothing
```

**Problems**:
- All monsters behave identically
- Monsters get stuck on walls and corners
- No retreating, no ranged attacks, no tactical decisions
- No line-of-sight check — monsters "see" through walls
- Feels robotic, not dangerous

---

## New Monster Behavior System

### State Machine

Each monster has a `BehaviorState` enum that determines what it does on its turn:

```
BehaviorState {
    Idle,        // Wander randomly. Haven't spotted player.
    Chase,       // A* pathfind toward the player.
    Attack,      // Adjacent to player — melee hit.
    Ranged,      // Has line of sight + range — fire projectile.
    Retreat,     // Low HP — pathfind AWAY from the player.
    Reposition,  // Move to a better tactical spot (Skeleton: keep distance).
}
```

**State transitions** (evaluated each turn, top to bottom):

```
1. If HP <= 0                          → Dead (removed)
2. If adjacent to player AND not retreating → Attack
3. If has_ranged AND line_of_sight AND in_range → Ranged
4. If HP < 30% of max AND can_retreat  → Retreat
5. If can_see_player                   → Chase
6. If lost_sight_of_player             → Idle (wander toward last known position)
7. Otherwise                           → Idle (random wander)
```

---

## Monster-Specific Behaviors

### Goblin (`g`, Green)

| Stat | Value |
|------|-------|
| HP | 6 |
| Attack | 2 |
| Speed | 1 tile/turn (2 tiles/turn on floors 7+) |
| Range | Melee only |

**Behavior by floor tier**:

| Floors | Behavior |
|--------|----------|
| 1-3 | **Basic**: Chase aggressively. Attack on contact. No retreat. |
| 4-6 | **Intermediate**: Chase, but **retreats when below 30% HP**. Retreats toward other monsters if possible (lure behavior). |
| 7+ | **Tactical**: All of above. Moves **2 tiles per turn** (fast). Tries to lure player into rooms with other monsters. |

**Lure logic** (floors 4+): When retreating, the Goblin doesn't just run away randomly — it pathfinds toward the **nearest other living monster**. If it reaches another monster, it stops retreating and starts chasing again (now you're fighting two).

---

### Skeleton (`s`, White)

| Stat | Value |
|------|-------|
| HP | 10 |
| Melee Attack | 4 |
| Ranged Attack | 3 (arrow) |
| Preferred Range | 4-8 tiles |
| Range | Melee + Ranged (bow) |

**Behavior by floor tier**:

| Floors | Behavior |
|--------|----------|
| 1-3 | **Basic**: Chase and melee only. No ranged attacks. Behaves like current AI but with A* pathfinding. |
| 4-6 | **Intermediate**: Gains **ranged attack**. If player is 3-8 tiles away with line of sight, fires an arrow. If player closes to melee range, fights in melee. |
| 7+ | **Tactical**: All of above. **Repositions to maintain distance** — if player is within 3 tiles, backs away to preferred range before firing. Retreats through doorways to force you into arrow corridors. |

**Arrow behavior**: See Projectile System below.

---

### Troll (`T`, Red)

| Stat | Value |
|------|-------|
| HP | 20 |
| Attack | 8 (normal), 12 (berserk) |
| Speed | 1 tile/turn (moves every OTHER turn — slow) |
| Range | Melee only |

**Behavior by floor tier**:

| Floors | Behavior |
|--------|----------|
| 1-3 | **Basic**: Slow chase — only moves every other turn. Hits hard. Simple and predictable. |
| 4-6 | **Intermediate**: **Corridor blocker** — if the Troll is in a corridor (width 1-2 tiles), it stops moving and waits for you to come to it. Forces you to engage on its terms. |
| 7+ | **Tactical**: All of above. **Berserk mode** — when HP drops below 30%, attack increases from 8 to 12 and speed becomes every turn (no longer slow). Message: "The Troll flies into a rage!" |

**Corridor detection**: Check if the Troll's current position has walls on 2+ opposite sides (left+right or top+bottom). If so, it's in a corridor.

---

## Ranged Attack & Projectile System

### Projectile Struct

```
Projectile {
    x: usize,
    y: usize,
    dx: i32,            // direction X (-1, 0, or 1)
    dy: i32,            // direction Y (-1, 0, or 1)
    damage: i32,
    symbol: char,        // visual representation
    source_name: String, // "Skeleton" (for log messages)
}
```

### Projectile Symbols (based on direction)

| Direction | dx, dy | Symbol |
|-----------|--------|--------|
| Right | (1, 0) | `-` |
| Left | (-1, 0) | `-` |
| Down | (0, 1) | `|` |
| Up | (0, -1) | `|` |
| Down-Right | (1, 1) | `\` |
| Up-Left | (-1, -1) | `\` |
| Down-Left | (-1, 1) | `/` |
| Up-Right | (1, -1) | `/` |

### Projectile Lifecycle

1. **Spawn**: Skeleton fires arrow → projectile created at Skeleton's position, aimed toward player's current position
2. **Move**: Each game turn, all projectiles advance 1 tile in their direction
3. **Render**: Drawn on the map in **Yellow** color, on top of floor tiles
4. **Collision checks** (each step):
   - Hits **player** → deal damage, log message, remove projectile
   - Hits **wall** → remove projectile (arrow stops)
   - Hits **monster** → remove projectile (friendly fire — arrows don't damage other monsters, just stop)
   - Exits map bounds → remove projectile
5. **Skeleton cooldown**: Can only fire every 2 turns (prevent arrow spam)

### Message Log Examples

```
"The Skeleton fires an arrow!"
"An arrow hits you for 3 damage!"
"The arrow thuds into a wall."
"You dodge out of the arrow's path."     // (future: DEX dodge check from Phase 3)
```

---

## Line of Sight — Bresenham's Line Algorithm

### What It Replaces

Current check: `if manhattan_distance < 10 → monster can see player`

This is wrong — monsters currently "see" through walls.

### How Bresenham Works

Draw a line from monster `(x1, y1)` to player `(x2, y2)`. Step through each tile along the line. If ANY tile is a `Wall`, the line of sight is blocked → monster cannot see the player.

### Function Signature

```rust
fn has_line_of_sight(map: &Map, x1: usize, y1: usize, x2: usize, y2: usize) -> bool
```

### Gameplay Impact

- Monsters only spot you when they have a clear line of sight
- Corners and corridors become meaningful — duck around a corner to break line of sight
- Skeleton arrows also use line of sight — they can't shoot through walls
- Makes the dungeon layout tactically important (not just cosmetic)

---

## A* Pathfinding

### What It Replaces

Current movement: `if monster.x < player.x { monster.x += 1 }`

This makes monsters walk into walls and get stuck on corners.

### How A* Works

Find the shortest walkable path from monster to target (player, retreat position, etc.) using A* search on the tile grid. Returns a list of tiles to walk. Monster takes the first step each turn.

### Function Signature

```rust
fn astar_pathfind(map: &Map, start: (usize, usize), goal: (usize, usize), monsters: &[Monster]) -> Option<Vec<(usize, usize)>>
```

- Considers walls as impassable
- Considers other monsters as obstacles (won't path through them)
- Returns `None` if no path exists (monster is trapped)
- Used for both chasing AND retreating (retreat = pathfind away from player)

### Retreat Pathfinding

For retreat behavior, instead of pathfinding toward the player, pathfind toward the tile that:
1. Is walkable
2. Is farther from the player than the monster's current position
3. Is reachable

Simple approach: pick the walkable neighbor tile that maximizes distance from the player. No full A* needed for retreat — just pick the best adjacent tile each turn.

---

## Floor-Tier Scaling Summary

| Floors | AI Tier | New Behaviors Unlocked |
|--------|---------|----------------------|
| 1-3 | **Basic** | A* chase (replaces dumb chase). Melee attack. Proper line of sight. Idle wandering. |
| 4-6 | **Intermediate** | Skeleton gains ranged arrows. Goblin retreats when low HP. Troll blocks corridors. |
| 7+ | **Tactical** | Skeleton repositions to maintain range. Goblin lures toward allies. Troll goes berserk at low HP. Goblin moves 2x speed. |

---

## Monster Struct Changes

Current:
```rust
pub struct Monster {
    pub x, y, symbol, name, hp, max_hp, attack, monster_type
}
```

New fields needed:
```rust
pub struct Monster {
    // existing fields...
    pub behavior: BehaviorState,       // current AI state
    pub can_see_player: bool,          // updated each turn via LOS check
    pub last_known_player_pos: Option<(usize, usize)>,  // for "search" behavior
    pub ranged_cooldown: i32,          // turns until can fire again
    pub turns_since_move: i32,         // for Troll slow movement
    pub floor_tier: i32,              // determines which behaviors are active
}
```

---

## Implementation Order

| Step | What | Files Touched | Depends On | Status |
|------|------|---------------|------------|--------|
| 1 | ✅ Bresenham line-of-sight function | `map.rs` | — | Done |
| 2 | ✅ A* pathfinding function | `map.rs` | — | Done |
| 3 | ✅ BehaviorState enum + new Monster fields | `monster.rs` | — | Done |
| 4 | ✅ Monster state machine (evaluate + transition each turn) | `monster.rs`, `main.rs` | Steps 1-3 | Done |
| 5 | ✅ Projectile struct + spawn/move/render/collide | `projectile.rs`, `main.rs` | Step 1 | Done |
| 6 | ✅ Skeleton ranged behavior (fire arrows, keep distance, reposition) | `monster.rs`, `main.rs` | Steps 4-5 | Done |
| 7 | ✅ Goblin retreat + lure behavior | `monster.rs` | Step 4 | Done |
| 8 | ✅ Troll corridor-blocking + berserk mode | `monster.rs` | Step 4 | Done |
| 9 | ✅ Floor-tier system (enable behaviors per depth range) | `monster.rs`, `map.rs` | Steps 6-8 | Done |
| 10 | ✅ Update CHANGELOG.md | `CHANGELOG.md` | Step 9 | Done |

---

## New/Modified File Structure After Phase 2.5

```
ironveil/
├── Cargo.toml
└── src/
    ├── main.rs          # Game loop — now processes projectiles + smarter monster turns
    ├── map.rs           # + Bresenham LOS, A* pathfinding
    ├── player.rs        # Unchanged
    ├── monster.rs       # + BehaviorState, state machine, per-monster tactics
    └── projectile.rs    # NEW — projectile struct, movement, collision, rendering
```

---

## What This Does NOT Include

- Real-time movement (staying turn-based)
- GPU compute (unnecessary for this scale)
- Neural network AI (algorithmic state machines are more predictable and designable)
- New monster types (save for later phases — the 3 existing types get much deeper)
- Stat-based combat changes (that's Phase 3)

---

## Expected Player Experience After Phase 2.5

**Floor 1-3**: Feels similar to current game but smoother — monsters path around corners instead of getting stuck. Line of sight means you can break chase by ducking around a wall. Good training ground.

**Floor 4-6**: Tension ramps up. First time a Skeleton fires an arrow down a corridor at you is a memorable moment. You learn to use corners for cover. Goblins that run away make you nervous — where are they going? Trolls sitting in corridors force you to commit or find another route.

**Floor 7+**: Genuinely challenging. Goblins are fast and pulling you into multi-monster fights. Skeletons kite you and reposition when you close distance. Trolls go berserk and suddenly hit like trucks. Every room requires a plan.
