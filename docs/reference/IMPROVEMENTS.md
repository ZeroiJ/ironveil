# Ironveil Improvement Analysis

Based on research of modern roguelike best practices (2024-2025), here are the recommended improvements for Ironveil.

---

## Phase 6 Features (Your Roadmap)

| Priority | Feature | Description | Effort |
|----------|---------|-------------|--------|
| **1** | **Save/Load System** | Serialize game state to JSON, persist across sessions | Medium |
| **2** | **Fog of War** | Only reveal tiles player has seen - adds exploration tension | Medium |
| **3** | **Auto-Explore** | Press key to automatically explore map (QoL essential) | High |
| **4** | **Shop/Merchant NPC** | Buy/sell items between floors | Medium |

---

## Gameplay Improvements

### Essential Modern Roguelike Features (2025)
1. **Auto-explore** - Players hate holding arrow keys in empty corridors
2. **Diversified combat** - Abilities beyond bump-attack (you already have this!)
3. **Strong early game** - First few floors need variety and excitement
4. **Achievements system** - Tracks progress across runs

### Content Expansions
- **More item types**: Scrolls, wands, rings with varied effects
- **Ally/companion system**: Pet that follows you
- **Factions**: Some monsters don't attack each other
- **Secret rooms**: Hidden areas with special loot

### Procedural Generation Upgrades
- **BSP Trees**: More structured room layouts (current random placement is basic)
- **Multiple biomes**: Floor 1-5 = dungeon, 6-10 = caves, 11-15 = void
- **Room variety**: L-shaped rooms, rooms with pillars, water features

---

## Visual/UI Improvements

| Feature | Benefit |
|---------|---------|
| **Color-coded damage numbers** | Visual feedback on hits |
| **Animation frames** | Smooth attack/movement |
| **Particle effects** | Blood splatter, sparkles on pickup |
| **Better HUD** | Enemy health bars, buff indicators |
| **Mouse support** | Click to move, click to target abilities |

---

## Technical Improvements

### bracket-lib Built-in Features (Use These!)
- **A* Pathfinding** - `bracket-pathfinding` crate
- **Noise generation** - `bracket-noise` crate (for caves, terrain)
- **Better color system** - `bracket-color` crate
- **Virtual consoles** - For large maps/scrolling

### Other
- **Save/Load** with serde_json (Phase 6 planned)
- **Configuration file** for keybindings
- **Sound effects** (many roguelikes have 1000+)

---

## Recommended Priority Order

### Phase 6 (Next)
- [x] Save/Load System - essential for playable game
- [x] Fog of War - exploration core mechanic
- [x] Fix broken inventory - Tab key not working

### Phase 6.5 (Enhancements)
- [ ] Better dungeon generation (BSP algorithm)
- [ ] Multiple floor themes (colors, monster types)
- [ ] Auto-explore - major QoL improvement

### Phase 7 (Content)
- [ ] Shops/Merchants
- [ ] More item types (scrolls, wands)
- [ ] Achievements
- [ ] Lore/Story elements

---

## Quick Wins

What has biggest impact with least effort:
1. **Fix inventory** - Currently shows "TODO"
2. **Add more monster variety** - You have 10, good foundation
3. **Better floor theming** - Colors already change with depth
4. **Buff/debuff visualization** - Show poison/stun on character with symbols

---

## References

- Reddit: "must-have features for roguelikes in 2025"
- Cogmind dev blog (Grid Sage Games)
- Bracket-Lib documentation
- 7DRL 2025 devlogs
- Shattered Pixel Dungeon 2025 roadmap
