# Dungeon Complexity Enhancement Plan

**Goal**: Transform BSP-generated dungeons from rigid tree structures into complex, hand-crafted-feeling environments using post-processing techniques from top roguelikes (Brogue, DCSS, Caves of Qud, Cogmind).

**Inspiration**: Brian Walker's Brogue interview (RPS 2015), DCSS vault system, Cogmind tunneler algorithm, Caves of Qud WFC.

---

## Phase 1: Loop Creation (1-2 hours) ⭐ Highest ROI ✅ COMPLETE

**Problem**: BSP produces a linear tree — each room has exactly one parent. Players must backtrack through the same rooms to explore.

**Solution**: After BSP generates rooms + corridors, scan every wall tile. If a wall has floor on both sides AND the two floor tiles are 15+ A* pathfinding steps apart, punch a door through that wall.

**Result**: Transforms linear tree into a network with multiple paths. Players can take alternative routes, avoid backtracking, and feel less trapped.

**Implementation**:
- After `generate_bsp()` returns, add a `create_loops()` pass
- Iterate all wall tiles between rooms
- For each candidate wall, run A* distance check between the two adjacent floor tiles
- If distance >= 15, replace wall with floor (or door tile)
- Target: 3-5 loops per floor

**Files**: `src/map.rs` — new `fn create_loops()` method, called after BSP generation in `Map::new()`

---

## Phase 2: Cleanup Pass (30 min) ✅ COMPLETE

**Problem**: BSP + loop punching creates visual artifacts: diagonal gaps, orphan walls, doors without adjacent walls, walls in the middle of open areas.

**Solution**: A single cleanup pass after all structural generation:
- Diagonal wall openings → knock down one wall
- Doors with <2 adjacent walls → remove door
- Walls surrounded by same impassable terrain → remove
- Orphan wall segments → remove or connect

**Result**: Eliminates "procedural jank" — everything looks intentional.

**Implementation**:
- New `fn cleanup_dungeon()` method
- Called after loop creation, before decoration placement
- Single pass over all tiles, applying rules

**Files**: `src/map.rs` — new `fn cleanup_dungeon()` method

---

## Phase 3: Feature Autogenerator System (2-3 hours) ✅ COMPLETE

**Problem**: Current decoration system (torches, pillars, altars, chests) is limited to 4 types placed deterministically. Rooms feel identical.

**Solution**: Expand to a catalog-based autogenerator system like Brogue's:
- **Wall cracks**: Random wall damage, frequency increases with depth
- **Floor debris**: Scattered rubble on floor tiles
- **Bloodstains**: In combat-heavy areas (monster spawn rooms)
- **Moss patches**: In shrine/treasure rooms, propagate outward
- **Scorch marks**: Near trap rooms
- **Water puddles**: In cavern floors (4-6)

Each feature has: spawn terrain, depth range, frequency curve, max count, propagation rules.

**Result**: Textural variety — no two rooms feel identical even if structurally similar.

**Implementation**:
- New `DungeonFeature` enum in `map.rs`
- `AUTO_GENERATOR_CATALOG` with metadata per feature
- `fn scatter_features()` method called after cleanup
- Existing torch/pillar/chest/altar logic integrated into the catalog

**Files**: `src/map.rs` — `DungeonFeature` enum, catalog, `fn scatter_features()`

---

## Phase 4: Lake/Chasm Imposition (2-3 hours) ✅ COMPLETE

**Problem**: Dungeons feel homogenous — same wall/floor pattern everywhere. No large-scale structure or tactical zones.

**Solution**: After dungeon is built, place lakes (water, lava, chasm) using cellular automata:
1. Generate a CA blob on a temporary layer
2. Slide it around until it doesn't disconnect the dungeon (flood-fill connectivity check)
3. Drop it onto the map, overwriting terrain
4. Add "wreaths" (shallow water around deep water, chasm edges around chasms)

**Floor-specific**:
- Floors 4-6 (cavern): Underground pools, water lakes
- Floors 7+ (void): Lava lakes, chasms with bridges

**Result**: Large-scale structure that creates tactical zones (ranged combat over water, chokepoints around lakes).

**Implementation**:
- New `fn place_lake()` method with CA generation
- Flood-fill connectivity check before placement
- `Tile::Water`, `Tile::Lava`, `Tile::Chasm` variants (or reuse existing with terrain overlay)
- Wreath generation around lake edges

**Files**: `src/map.rs` — `fn place_lake()`, new tile variants or terrain overlay system

---

## Phase 5: Vault Stamping (3-4 hours) ✅ COMPLETE

**Problem**: All rooms are procedurally generated boxes. No moments of "designer intent" — obvious hand-crafted spaces.

**Solution**: Define 5-10 vault templates as ASCII patterns, then stamp them onto specific rooms during `assign_room_types`:
- **Throne Room**: Large room with central dais, pillars, throne
- **Armory**: Weapons on walls, armor stands
- **Prison Cell**: Bars, chains, skeleton in corner
- **Ritual Chamber**: Altar, candles, summoning circle
- **Treasure Vault**: Pile of gold, chests, trapped entrance
- **Library**: Bookshelves, reading desk, scrolls

**How stamping works**:
1. During `assign_room_types()`, mark certain rooms as vault candidates
2. Pick a vault template matching the room's size
3. Overlay the vault pattern onto the room's tiles
4. Preserve connectivity (don't block doors/corridors)

**Result**: Players encounter rooms that feel obviously designed, breaking procedural monotony.

**Implementation**:
- `VaultTemplate` struct with ASCII grid pattern
- Vault catalog with 5-10 templates
- `fn stamp_vault()` method called during room type assignment
- Size-matching logic (pick vault that fits the room)

**Files**: `src/map.rs` — `VaultTemplate` struct, vault catalog, `fn stamp_vault()`

---

## Phase 6: Machines / Interconnected Features (4-6 hours) ✅ COMPLETE

**Problem**: Rooms are isolated. No narrative connection between features on a floor.

**Solution**: Implement Brogue's machine system — clusters of terrain features that relate to each other:
- **Shrine Machine**: Using a shrine triggers a trap elsewhere on the floor
- **Treasure Machine**: Key in one room unlocks a secret passage in another
- **Trap Machine**: Disarming one trap reveals a hidden cache
- **Boss Machine**: Boss room has environmental hazards (lava pits, falling rocks)

**How machines work**:
1. Find a room with a single chokepoint (or mark during room assignment)
2. Pick a machine template
3. Place interconnected features across multiple rooms
4. Wire up activations (shrine use → trap trigger → door open)

**Result**: Narrative arcs within a single floor. "I found a lever, it opened a passage, there was treasure, but it triggered a trap."

**Implementation**:
- `MachineTemplate` struct with feature definitions and activation rules
- Machine catalog with 5-8 templates
- `fn place_machine()` method called after vault stamping
- Activation system in main game loop (shrine use triggers trap)

**Files**: `src/map.rs` — `MachineTemplate` struct, machine catalog, `fn place_machine()`
         `src/main.rs` — activation handling in game loop

---

## Execution Order

```
Phase 1 (Loops) ──┐
Phase 2 (Cleanup) ─┤
                   ├── Phase 3 (Features) ──┐
                   │                        │
Phase 4 (Lakes) ───┘                        ├── Phase 5 (Vaults) ── Phase 6 (Machines)
                                            │
Phase 5 depends on: 1, 2, 3, 4 ─────────────┘
Phase 6 depends on: 1, 2, 3, 4, 5
```

**Immediate next step**: Phase 1 — Loop Creation

---

## Key Design Principles (from Brogue's Brian Walker)

1. **The dungeon should feel concrete and exciting** — not just a flat substrate for monsters
2. **Hand-designed quality** — revealed in stages as you discover secrets
3. **Procedurally generated vaults feel more organic** than grid-defined ones
4. **Edge cases are tough** — test extensively, use seeds for reproducibility
5. **Level generation defines the experience** — more than any other system
