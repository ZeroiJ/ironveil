# Reverted Features

This file tracks features that were implemented but reverted due to issues.

---

## Camera System + Player Trail

**Commit:** `410f11a` (reverted in `cdb72b1`)
**Date:** 2026-03-25

### What was implemented:
- Fixed map size (120x80) larger than viewport
- Camera viewport (60x20) centered on player
- Player trail (8 dots behind player)
- Coordinate display in UI

### Why reverted:
- Camera scrolling caused rendering issues
- Player had difficulty tracking their character with scrolling view
- The viewport was too small for the game feel
- Complexity increased without proportional benefit

### Lessons learned:
- Camera system needs more work before re-implementation
- Should test camera with smaller viewport first
- Consider alternative approaches (e.g., minimap instead of full camera)

### Possible future approach:
- Keep map at terminal size (no camera)
- Add minimap in corner showing full dungeon
- Use other visibility aids (brighter player, pulsing, coordinates)

---

## Fog of War

**Implemented then disabled**
**Date:** 2026-03-25

### What was attempted:
- Circular visibility radius around player (8 tiles)
- Line-of-sight checking (walls block vision)
- "Seen" memory for previously visited areas

### Why disabled:
- Visibility algorithm (Bresenham line-of-sight) was blocking too many tiles
- Irregular reveal pattern with gaps
- Only tiles player stepped on were visible, not surrounding area

### Possible future approach:
- Simple circular reveal without line-of-sight
- Or use established FOV algorithms (shadowcasting, recursive shadowcast)
- Test thoroughly before enabling

---

## Notes

When re-implementing any of these features:
1. Test incrementally at each step
2. Get user feedback early
3. Keep the core game fun factor priority
4. Don't overcomplicate - simple solutions often work better
