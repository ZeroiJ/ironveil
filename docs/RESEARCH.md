# Game Design Research: Ironveil Analysis

**Date:** 2026-03-25
**Game State:** v0.3.1 - Player Visibility & Floor Reveal

---

## 1. Real-Time Hybrid Model - Complementary Mechanics

### Current State
- Real-time monster AI on 500ms tick
- Player moves instantly on keypress (uncapped)
- Projectiles advance on monster tick
- 18+ monster types with unique behaviors

### What Top Roguelikes Do

**Cogmind (Real-time with automation):**
- Auto-labeling: Objects automatically labeled as they enter view
- Smart inventory: Auto-equips optimal items where obvious
- Drag-drop inventory + mouse support
- Nearly 1000 sound effects for feedback

**Roguefort (2026 Modern Roguelike):**
- Click-to-move with pathfinding
- Auto-explore (BFS to nearest unexplored tile)
- Hover tooltips for inspection
- Right-click context menus
- Multi-level undo

**Recommendation for Ironveil:**
1. **Auto-label items on pickup** - Show item names when you pick them up (not just symbols)
2. **Hover inspection** - Mouse over tile shows what's there
3. **Auto-explore** - Simple BFS to nearest item/unexplored room (nice-to-have later)
4. **Sound** - Hard in terminal, but could use terminal bell for alerts

---

## 2. What Top Roguelikes Do Differently

### Brogue - Environmental Depth
- **Room Accretion**: Organic room shapes (circles, blobs, overlapping rectangles)
- **Environmental Interactions**: 
  - Fire spreads through flammable terrain
  - Water/lava create new paths
  - Gas clouds (paralysis, confusion, smoke)
  - Chasms, pits, shallow water
- **Item Identification**: Use items to learn what they do
- **Monster Relationships**: Monsters fight each other, form alliances

### DCSS - Species + Background + Skills
- **27 Species** with unique aptitudes
- **26 Backgrounds** (starting packages)
- **Skill System**: Free-form XP distribution, aptitudes determine speed
- **Runes**: 15 optional objectives per run
- **Multiple Win Conditions**: Grab Orb quickly, or collect all runes

### Caves of Qud - Mutations + Factions
- **70+ Mutations**: Physical, Mental, Defects, Morphotypes
- **Reputation System**: Factions (animals, robots, villages) with standing
- **Cooking**: Combine ingredients for effects
- **True Kin vs Mutants**: Two parallel progression paths

### Hades - Narrative Integration
- **Meta-progression**: Mirror of Darkness unlocks permanent buffs
- **Story Beats**: Each run reveals more narrative
- **Boon System**: God powers that combo together

### Ironveil's Position
- Strong monster variety and AI behaviors
- Good class/ability system
- Boss system is solid
- Missing: Environmental depth, build diversity, long-term goals

---

## 3. Underutilized Systems

### Currently Implemented (Working Well)
| System | Status |
|--------|--------|
| Monster AI behaviors | Good - 18+ types with unique patterns |
| Status effects | Good - stun, freeze, poison visual |
| Boss system | Good - 3 unique bosses with phases |
| XP/Leveling | Good - 10 levels, stat progression |
| Class abilities | Good - 2 abilities per class |
| Floor theming | Visual only |

### Underutilized
| System | Issue | Potential |
|--------|-------|--------|
| **Floor theming** | Only visual colors | Regional monster spawns, special items |
| **Monster drops** | Basic random % | Weighted by monster type, contextual drops |
| **Equipment generation** | Basic random | Set bonuses, named items, unique artifacts |
| **Environmental hazards** | None yet | Traps, lava, water, gas |
| **Room variety** | Basic rectangles | Special rooms, vaults, boss chambers |

### Opportunities
1. **Regional Biomes** - Each floor theme could have exclusive monsters/items
2. **Artifact Items** - Named weapons with special effects
3. **Traps** - Spikes, fire, teleport on floor tiles

---

## 4. Replayability - Making Runs Feel Different

### Current State
- 3 classes (Warrior, Rogue, Mage)
- Procedural dungeon
- Monster variety scales with depth

### What's Missing

**Build Diversity:**
- All Warriors play similar (same abilities)
- Equipment is stat modifiers only
- No build-defining choices

**Roguelike Standards:**
| Feature | Example | Impact |
|---------|---------|--------|
| **Random Starting Loadout** | Brogue: random items at start | Different early-game strategy |
| **Artifacts/Legendaries** | Cogmind: named items with traits | Chase items, build around drops |
| **Mutations** | Caves of Qud: 70+ mutations | Complete build transformation |
| **Factions/Reputation** | Caves of Qud: 20+ factions | Different NPC interactions |
| **Side Objectives** | DCSS: 15 runes | Multiple paths to win |
| **Challenge Modes** | DCSS: sprint, chaos | Replay with constraints |

**Quick Wins for Ironveil:**
1. **Random starting items** - Each run starts with different gear
2. **Artifact system** - Rare named items with unique effects
3. **Floor Objectives** - Optional side goals per floor (e.g., "Defeat 5 enemies with fire damage")

---

## 5. QoL Features Players Expect

### Essential (Modern Expectations)
| Feature | Priority | Implementation |
|---------|----------|-----------------|
| **Minimap** | High | Corner map showing explored area + player + stairs |
| **Item Identification** | Medium | When used once, learn what it does |
| **Mouse Support** | Low | Move/attack with click |
| **Look Mode** | Medium | Press key to inspect tile |
| **Auto-pickup preferences** | Low | Don't pick up certain item types |

### Ironveil-Specific
| Feature | Priority | Implementation |
|---------|----------|-----------------|
| **Death screen with stats** | Medium | Show floors cleared, kills, damage dealt |
| **More detailed tooltips** | Medium | Show full item stats in inventory |
| **Floor progress indicator** | Low | Show how far in the dungeon |

---

## Summary: Recommended Priority List

### Phase 1: Quick QoL (Low Effort, High Impact) — DONE
- [x] **Item auto-identification** - Learn item names on use
- [x] **Minimap** - (Not yet implemented)
- [x] **Death summary screen** - Stats on death ✓ DONE

### Phase 2: Build Diversity (Medium Effort)
- [ ] **Artifact items** - Named weapons/armor with special effects
- [ ] **Random starting loadout** - Variation each run

### Phase 3: Depth (Higher Effort)
- [ ] **Environmental hazards** - Traps, fire spread
- [ ] **Regional biomes** - Exclusive content per floor type
- [ ] **Faction/reputation** - Complex NPC relationships

---

## Implemented from Research

| Feature | Status | Version |
|---------|--------|---------|
| Fog of war (LOS-based) | ✓ Done | v0.3.2 |
| Delta rendering (performance) | ✓ Done | v0.3.2 |
| Death screen with stats | ✓ Done | v0.3.2 |

---

## References

- **Brogue**: Room accretion algorithm, item identification, environmental depth
- **DCSS**: Species+Background system, skill aptitudes, 27 species
- **Caves of Qud**: Mutations (70+), reputation, cooking, True Kin
- **Cogmind**: Automation features, sound design, living world
- **Roguefort (2026)**: Modern UI innovations, click-to-move, auto-explore
- **Hades**: Meta-progression, narrative integration

---

*This analysis prepared for Ironveil development planning.*
