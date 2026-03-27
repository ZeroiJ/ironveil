# Procedural Dungeon Generation — Ironveil Research

**Date:** 2026-03-26  
**Topic:** Dungeon Generation Algorithms  
**Reference Games:** Brogue, DCSS, Caves of Qud, Cogmind, Binding of Isaac, Enter the Gungeon

---

## 1. Algorithm Overview

### Common Algorithms

| Algorithm | Type | Strengths | Weaknesses | Games Using |
|-----------|------|-----------|------------|--------------|
| **BSP** | Space Partitioning | Predictable room counts, guaranteed connectivity | Rectilinear feel | DCSS, Binding of Isaac |
| **Cellular Automata** | Cellular | Organic cave shapes | May need repairs | Caves of Qud, Brogue |
| **Random Walk** | Agent-based | Fast, natural paths | Unpredictable connectivity | Various prototypes |
| **Room Placement** | Template | Traditional dungeon feel | Overlap handling | Nethack, Brogue |

---

## 2. Binary Space Partitioning (BSP)

### How It Works

```
1. Start with entire map as one region
2. Recursively split region into two smaller regions
3. Continue until regions are small enough (min size threshold)
4. Place one room in each leaf region
5. Connect sibling regions with corridors
```

### Algorithm Steps

```rust
struct BSPNode {
    area: Rect,
    left: Option<Box<BSPNode>>,
    right: Option<Box<BSPNode>>,
    room: Option<Room>,
}

fn split(node: &mut BSPNode, min_size: i32) {
    if node.area.width <= min_size || node.area.height <= min_size {
        // Create room in this leaf
        node.room = Some(create_random_room(node.area));
        return;
    }
    
    // Choose split direction (horizontal or vertical)
    let split_horizontal = random_bool();
    
    // Choose split position
    let split_pos = calculate_split_position(node.area, split_horizontal);
    
    // Create children
    node.left = Some(Box::new(create_child_left(node.area, split_pos, split_horizontal)));
    node.right = Some(Box::new(create_child_right(node.area, split_pos, split_horizontal)));
    
    // Recurse
    split(node.left, min_size);
    split(node.right, min_size);
}
```

### Corridor Generation

**Connect sibling leaves:**
- If rooms have face-to-face walls → straight corridor
- Otherwise → L-shaped (horizontal then vertical, or vice versa)

### Parameters

| Parameter | Effect |
|-----------|--------|
| Split depth | Number of rooms (2^depth) |
| Min leaf size | Smallest possible room area |
| Split ratio | How balanced splits are (0.4-0.6 = balanced, 0.1-0.9 = varied) |
| Room padding | Space between room and leaf edge |

---

## 3. Cellular Automata (Cave Generation)

### Basic Algorithm

```
1. Fill map randomly with ~45-50% walls
2. For 5-8 iterations:
   - For each cell: count wall neighbors (8-cell radius)
   - If wall_neighbors > 4 → become wall
   - If wall_neighbors < 4 → become floor
3. Apply post-processing (flood fill to remove disconnected areas)
```

### Rule Notation

**B5678/S45678** (Common cave rule):
- Survival: 4-8 neighbors
- Birth: 5-8 neighbors

### Ironveil's Current Approach

Your existing `generate_dungeon()` in map.rs likely uses room placement. This can be enhanced with:

1. **Hybrid:** BSP rooms + cellular automata corridors
2. **Template-based:** Pre-designed room shapes placed via BSP

---

## 4. Seed-Based Generation

### Core Concept

```
fn generate(seed: u64) -> Map {
    // Use seed to initialize random number generator
    // All random decisions now deterministic
    // Same seed = same dungeon
}
```

### Seed Storage

```
Player shares seed: "ABC123"
Other player loads seed → identical dungeon
```

### Implementation

```rust
use rand::SeedableRng;
use rand::rngs::StdRng;

fn generate_with_seed(seed: u64) -> Map {
    let mut rng = StdRng::seed_from_u64(seed);
    
    // All rng calls now deterministic
    let room_count = rng.gen_range(5..12);
    // ... rest of generation
}
```

---

## 5. Room Variety & Special Rooms

### Room Types

| Room Type | Description | Examples |
|-----------|-------------|-----------|
| **Standard** | Basic combat rooms | — |
| **Treasure** | Bonus loot | Chest rooms, gold piles |
| **Boss** | Pre-staircase challenge | Unique enemies |
| **Shop** | Buy/sell items | NPC rooms |
| **Rest** | Save point, healing | Campfires |
| **Secret** | Hidden, valuable | Secret rooms |
| **Event** | Random encounters | Traps, puzzles |

### Special Room Placement

**BSP approach:** Reserve certain leaves for special rooms

**Random approach:** Place after main generation, verify connectivity

---

## 6. Progressive Difficulty Scaling

### Monster Scaling

| Floor | HP | Damage | XP |
|-------|-----|--------|-----|
| 1 | 100% | 100% | 100% |
| 5 | 150% | 130% | 150% |
| 10 | 250% | 180% | 300% |
| 15 | 400% | 250% | 500% |

### Formula Template

```
scale_factor = base × (1 + floor × growth_rate)
monster_hp = base_hp × scale_factor
```

### Loot Tier Progression

| Floor Range | Item Tier |
|------------|-----------|
| 1-3 | Common |
| 4-7 | Uncommon |
| 8-10 | Rare |
| 11-13 | Epic |
| 14-15 | Legendary |

---

## 7. Ironveil Implementation Recommendations

### Phase 1: Room Enhancement (Low Effort)

1. Increase room size variety (min/max variance)
2. Add room shape templates (not just rectangles)
3. Add special rooms (treasure, rest)

### Phase 2: BSP Integration (Medium Effort)

1. Replace random placement with BSP
2. Guaranteed connectivity
3. Controlled room counts

### Phase 3: Seed System (Medium Effort)

1. Add seed input on new game
2. Save seed with save file
3. Share seeds between players

### Phase 4: Advanced Features (High Effort)

1. Cellular automata caves on special floors
2. Hybrid layouts (BSP + procedural)
3. Wave Function Collapse for room interiors

---

## 8. Summary: Generation Options for Ironveil

| Approach | Complexity | Rooms | Connectivity | Style |
|----------|-------------|-------|---------------|-------|
| Random Placement | Low | Variable | Not guaranteed | Traditional |
| BSP | Medium | Fixed (2^n) | Guaranteed | Structured |
| Cellular Automata | Medium | Variable | Needs repair | Caves |
| Hybrid | High | Variable | Guaranteed | Mixed |

---

## References

- RogueBasin: Basic BSP Dungeon Generation
- PulseGeek: Dungeon Generation Algorithms
- Godot BSP Tutorial (slashskill.com)
- BSP Paper (diva-portal.org)
- Antonios Liapis: Constructive Generation Methods

---

*Research document for Ironveil procedural generation.*