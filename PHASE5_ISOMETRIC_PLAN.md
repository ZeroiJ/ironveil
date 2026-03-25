# Phase 5: Isometric Rendering Migration

## Overview
Migrate Ironveil from crossterm terminal rendering to bracket-lib with isometric projection. All game logic (AI, combat, abilities, items, leveling) stays intact. Only the rendering and input layers change.

**Philosophy:** No artwork files. Every visual is code-generated from CP437 glyphs, Unicode block characters, and color. Pure 80s/90s engineer aesthetic — the code IS the art.

---

## Architecture: What Changes vs What Stays

### STAYS INTACT (no changes)
- `player.rs` — Class, Stats, Equipment, Inventory, Abilities, XP, leveling
- `monster.rs` — All 10 monster types, behavior state machines, tier AI
- `items.rs` — Item definitions, generation, starting gear
- `projectile.rs` — Arrow projectile system
- `map.rs` — Dungeon generation algorithm, tile types, room placement (minor adapter needed for rendering)

### GETS REWRITTEN
- `main.rs` — Game loop, rendering, input handling, combat wiring
- `ui.rs` — Character creation screen, inventory screen, HUD
- `Cargo.toml` — Replace crossterm with bracket-lib

### NEW FILES
- `src/render.rs` — Isometric projection math, tile rendering, entity rendering, camera system
- `src/input.rs` — Input handling adapter (bracket-lib VirtualKeyCode → game actions)
- `src/gamestate.rs` — bracket-lib GameState trait implementation, frame timing

---

## The Isometric Projection System

### Core Math
Isometric projection converts 2D grid coordinates to screen pixel coordinates:

```
screen_x = origin_x + (grid_x - grid_y) * TILE_HALF_W
screen_y = origin_y + (grid_x + grid_y) * TILE_HALF_H
```

Reverse (screen → grid, for mouse input later):
```
grid_x = (screen_x / TILE_HALF_W + screen_y / TILE_HALF_H) / 2
grid_y = (screen_y / TILE_HALF_H - screen_x / TILE_HALF_W) / 2
```

### Tile Dimensions
Using bracket-lib's Fancy console with CP437 8x8 font:
- Each logical tile occupies a diamond shape on screen
- `TILE_HALF_W = 2.0` (2 character widths for half-diamond)
- `TILE_HALF_H = 1.0` (1 character height for half-diamond)
- Full tile: 4 chars wide, 2 chars tall in screen space
- These values are tunable — we'll adjust during implementation

### Camera System
- Camera centered on player at all times
- Viewport: ~80x50 character grid (standard bracket-lib window)
- Only tiles within viewport bounds get rendered (frustum culling)
- Camera follows player with optional smooth scrolling (lerp between positions)

### Render Order (Z-Sorting)
Isometric requires back-to-front rendering (painter's algorithm):
1. Sort all renderable objects by `(grid_y + grid_x)` — furthest tiles first
2. Within same depth: floor → wall base → wall top → items → entities → projectiles → effects
3. bracket-lib's Fancy console z_order parameter handles overlap

---

## Visual Design (All Code-Generated, No Art Files)

### Floor Tiles
- **Stone floor**: `·` or `∙` (CP437 250) in dark grey, slight color variation per tile for texture
- **Corridor**: Same glyph, slightly different shade to distinguish from rooms
- **Stairs down**: `>` in bright yellow, pulsing brightness (cycle fg alpha in tick)
- **Web tiles**: `#` in white with low alpha overlay on floor

### Wall System (2-Tile Tall)
Walls render as two vertically stacked glyphs to create 3D block height:

```
  Top face:   ▄ (CP437 220) or ▀ (CP437 223) — lighter shade (lit from above)
  Front face: █ (CP437 219) — darker shade (shadow side)
```

Wall color varies by floor depth:
- Floors 1-5: Dark grey stone (`RGB(60,60,70)` top / `RGB(35,35,45)` front)
- Floors 6-10: Brown-grey cavern (`RGB(70,55,40)` / `RGB(45,35,25)`)
- Floors 11-15: Dark purple-black void (`RGB(50,30,60)` / `RGB(30,15,35)`)

Wall edge detection: Check adjacent tiles to determine which faces are visible. Only render the front face if the tile in front (screen-relative) is not a wall. This prevents z-fighting and looks cleaner.

### Player Characters (Multi-Glyph, 3 Layers)

Each class rendered as 2-3 stacked/overlaid glyphs using Fancy console z-ordering:

**Warrior (Red)**
```
  Head:   ☺ (CP437 1) or Ω (CP437 234) — bright red
  Body:   ╬ (CP437 206) — dark red (armor bulk)
  Shadow: ░ (CP437 176) — black, offset down-right, low alpha
```

**Rogue (Green)**
```
  Head:   ☺ — bright green  
  Body:   ╨ (CP437 208) — dark green (slim profile)
  Shadow: ░ — black, offset down-right, low alpha
```

**Mage (Blue)**
```
  Head:   ☺ — bright blue
  Body:   ╩ (CP437 202) — dark blue (robes)  
  Hat:    ▲ (CP437 30) — cyan, offset up (pointy hat above head)
  Shadow: ░ — black, offset down-right, low alpha
```

All player glyphs positioned at the tile's isometric center, with slight upward offset to "stand on" the floor tile.

### Monsters (Enhanced from Current Single-Glyph)

Each monster gets a body glyph + shadow. Bosses get additional glyphs:

| Monster | Main Glyph | Color | Extra |
|---------|-----------|-------|-------|
| Goblin | g | Green | shadow only |
| Bat Swarm | b | DarkYellow | wing glyphs `v` flanking on fast tick |
| Spider | x | DarkMagenta | `.` leg dots at 4 corners |
| Skeleton | s | White | shadow only |
| Wraith | W | DarkCyan | transparency pulse (alpha cycles 0.3-0.8) |
| Troll | T | Red | 2-glyph tall (head T + body █) |
| Necromancer | N | DarkRed | `*` spark glyph when resurrecting |
| Goblin King | K | Yellow | crown `♦` above, larger shadow |
| Bone Dragon | D | DarkRed | 3 glyphs wide (`<D>` body segments) |
| Shadow Lord | S | Magenta | afterimage trail (previous position ghost) |

### Items on Ground
- Rendered at floor level with slight z-offset above floor
- Gentle color pulse (brightness oscillation) to make them noticeable
- Same glyphs as current: `/` weapon, `[` armor, `=` ring, `!` potion

### Projectiles
- Arrows: `→↓←↑` (directional) or `/\-|` depending on direction
- Chain Lightning: `~` in bright yellow, chain of 3-6 glyphs along path
- Frost Nova: `*` expanding ring in cyan
- War Cry: `○` expanding ring in yellow

### Status Effect Visuals
- Stun: `?` glyph floating above monster, cycling brightness
- Freeze: Monster tinted cyan, `*` ice glyph overlay
- Poison: Monster tinted green, small `·` bubbles rising above

### HUD (Text Layer)
Separate Simple console layer on top, for crisp text:
- Top bar: Floor, HP bar, Level, XP bar (same info as current, horizontal layout)
- Bottom bar: Ability cooldowns, status effects, keybindings
- Message log: 3 lines at bottom, scrolling
- Semi-transparent black background behind HUD text for readability

---

## Implementation Phases

### Step 1: Scaffold — bracket-lib Hello World (Est: 30 min)
- [ ] Update `Cargo.toml`: replace `crossterm` with `bracket-lib = "~0.8"`
- [ ] Create minimal `main.rs` with `BTermBuilder`, `GameState` trait, empty `tick()`
- [ ] Verify it compiles and opens a window
- [ ] Set up console layers: Fancy console (map/entities) + Simple console (HUD text)
- [ ] **Checkpoint: Window opens, shows "Ironveil" text**

### Step 2: Isometric Renderer — Floors Only (Est: 1-2 hours)
- [ ] Create `src/render.rs` with isometric projection functions
- [ ] Implement `world_to_screen(grid_x, grid_y) -> (f32, f32)` 
- [ ] Implement camera struct: `center_x, center_y, viewport_w, viewport_h`
- [ ] Render existing `map.rs` floor tiles as isometric diamonds on Fancy console
- [ ] Implement viewport culling (only render tiles visible on screen)
- [ ] Tune TILE_HALF_W / TILE_HALF_H until spacing looks right
- [ ] **Checkpoint: Generated dungeon visible as isometric floor grid**

### Step 3: Walls with Height (Est: 1-2 hours)
- [ ] Implement wall rendering: 2-glyph-tall blocks (top face + front face)
- [ ] Wall face visibility: only render front/side faces that aren't occluded by other walls
- [ ] Implement back-to-front render ordering (painter's algorithm sort)
- [ ] Floor-depth based wall coloring
- [ ] **Checkpoint: Dungeon rooms and corridors visible with 3D-looking walls**

### Step 4: Input System (Est: 30 min)
- [ ] Create `src/input.rs` — translate bracket-lib `VirtualKeyCode` to game actions
- [ ] Map arrow keys, WASD, Tab, Esc, number keys, letter keys
- [ ] Wire into existing movement/action system
- [ ] **Checkpoint: Player can move with arrow keys, game responds to all keys**

### Step 5: Game State & Timing (Est: 1 hour)
- [ ] Create `src/gamestate.rs` — implement `GameState` trait
- [ ] Move game state into the State struct (player, monsters, map, items, projectiles, etc.)
- [ ] Implement frame-based timing: accumulate `ctx.frame_time_ms`, trigger monster tick at 500ms
- [ ] Implement player input debounce (50ms same-key repeat)
- [ ] Wire floor transitions, death/restart flow
- [ ] **Checkpoint: Game loop runs, monsters tick independently, player moves freely**

### Step 6: Entity Rendering (Est: 1-2 hours)
- [ ] Render player as multi-glyph character at isometric position
- [ ] Render all monsters with body + shadow at isometric positions
- [ ] Implement z-sorting: entities render on top of floors, behind walls that are in front
- [ ] Render ground items with color pulse
- [ ] Render projectiles (arrows) moving through isometric space
- [ ] Render web tiles as overlay on floor
- [ ] **Checkpoint: Full game visible — player, monsters, items all in isometric view**

### Step 7: FOV & Lighting (Est: 1 hour)
- [ ] Existing Bresenham LOS from `map.rs` still works (grid-based, unaffected by rendering)
- [ ] Tiles outside FOV: render with very low brightness or not at all
- [ ] Previously seen tiles: render at ~30% brightness (fog of war)
- [ ] Player "light radius" effect: tiles near player slightly brighter
- [ ] **Checkpoint: Exploration feels mysterious, unseen areas are dark**

### Step 8: UI Overlays (Est: 1-2 hours)
- [ ] HUD: Top bar + bottom bar on Simple console layer (text overlay)
- [ ] HP bar, XP bar, level, floor number, ability cooldowns
- [ ] Message log (3 lines, bottom of screen)
- [ ] Character creation screen: Full-screen overlay using Simple console (reuse existing layout logic)
- [ ] Inventory screen: Full-screen overlay, pause monster tick, same keybindings
- [ ] Death screen with restart prompt
- [ ] **Checkpoint: All UI functional, game fully playable**

### Step 9: Visual Polish (Est: 1-2 hours)
- [ ] Status effect overlays (stun `?`, freeze `*`, poison bubbles)
- [ ] Ability visual effects (lightning chain, frost nova ring, war cry ring)
- [ ] Boss entrance announcements (screen flash, text)
- [ ] Wraith transparency pulsing
- [ ] Bat wing animation on fast tick
- [ ] Shadow Lord afterimage trail
- [ ] Stair pulse animation
- [ ] Smooth camera (lerp to player position instead of snap)
- [ ] **Checkpoint: Game looks polished and alive**

### Step 10: Testing & Tuning (Est: 1 hour)
- [ ] Play-test all 15 floors
- [ ] Verify all 3 classes work correctly
- [ ] Verify all boss fights render properly in isometric
- [ ] Verify projectiles path correctly in isometric
- [ ] Tune tile sizes, colors, z-ordering for best visual result
- [ ] Performance check: ensure 60fps with full dungeon rendered
- [ ] **Checkpoint: Game is complete and polished**

### Step 11: Commit & Push
- [ ] Clean up dead crossterm code
- [ ] Update CHANGELOG.md
- [ ] `cargo build --release` 
- [ ] Commit: "phase 5: isometric rendering migration — bracket-lib, fancy console projection, multi-glyph entities, 2-tile walls, camera system, visual effects"
- [ ] Push to remote

---

## Technical Decisions

### Why Fancy Console (not Sprite Console)?
- Fancy console uses CP437 font glyphs with fractional positioning — perfect for isometric placement of characters without any artwork
- z_order parameter handles depth sorting natively
- Rotation/scaling available if we want spinning death effects etc.
- Sprite console needs actual image files — violates our "no art" rule

### Why Hybrid Game Loop?
- bracket-lib's `tick()` is called every frame (~60fps)
- We accumulate `ctx.frame_time_ms` inside tick to trigger monster actions at 500ms intervals
- Player input processed immediately on the frame it occurs
- This preserves our real-time hybrid model without fighting bracket-lib's architecture

### Console Layer Stack
```
Layer 0: Fancy Console (80x50 logical, 8x8 font) — Map tiles, entities, effects
Layer 1: Simple Console No-BG (80x50, 8x8 font) — HUD text overlay
Layer 2: Simple Console (80x50, 8x8 font) — Full-screen overlays (inventory, char creation, death)
```

Layer 2 is only drawn when an overlay is active. Layer 1 is always drawn. Layer 0 is the isometric world.

### Camera Viewport
- Window: 80x50 characters (640x400 pixels at 8x8 font)
- Map rendering clips to viewport bounds
- Camera offset recalculated each frame based on player position
- Smooth follow: `camera_pos += (target_pos - camera_pos) * 0.15` per frame

### Render Pipeline Per Frame
```
1. Clear Fancy console
2. Calculate camera viewport bounds in grid coords
3. Build render list: all visible tiles + entities + effects within viewport
4. Sort render list by isometric depth (grid_y + grid_x), then by layer type
5. For each item in render list:
   a. Project grid coords to screen coords via world_to_screen()
   b. Apply FOV visibility (bright/dim/hidden)
   c. Draw to Fancy console with appropriate z_order
6. Draw HUD on Simple console layer
7. If overlay active, draw overlay on Layer 2
```

---

## Risk Assessment

| Risk | Impact | Mitigation |
|------|--------|------------|
| Tile spacing looks wrong | Medium | TILE_HALF_W/H are constants, easy to tune |
| Z-sorting visual glitches | Medium | Strict sort by (y+x) + layer type. Debug mode to show z-values |
| Performance with full map | Low | Viewport culling. Only ~40x25 tiles visible at once |
| bracket-lib API surprises | Medium | Well-documented library, large community, good examples |
| Wall occlusion edge cases | Medium | Start simple (always render both faces), optimize later |
| Input feel different | Low | Same keybindings, same debounce logic, just different API |

---

## File Structure After Migration
```
ironveil/
├── Cargo.toml          # bracket-lib ~0.8, rand 0.10.0
├── CHANGELOG.md
├── PHASE5_ISOMETRIC_PLAN.md
├── src/
│   ├── main.rs         # Entry point: BTermBuilder setup, main_loop launch
│   ├── gamestate.rs    # GameState trait impl, frame timing, state machine (play/inventory/death/creation)
│   ├── render.rs       # Isometric projection, camera, tile/entity/effect rendering
│   ├── input.rs        # VirtualKeyCode → game action mapping
│   ├── map.rs          # Dungeon generation (unchanged)
│   ├── player.rs       # Player struct (unchanged)
│   ├── monster.rs      # Monster types & AI (unchanged)
│   ├── items.rs        # Item definitions (unchanged)
│   └── projectile.rs   # Projectile system (unchanged)
```

Note: `ui.rs` gets absorbed into `gamestate.rs` (overlay rendering) and `render.rs` (HUD drawing). The character creation and inventory screens become state-machine modes rather than blocking function calls.

---

## Estimated Total Effort
- Steps 1-3 (rendering foundation): 3-5 hours
- Steps 4-6 (gameplay wiring): 2-4 hours  
- Steps 7-9 (polish): 3-5 hours
- Step 10 (testing): 1 hour
- **Total: ~10-15 hours of implementation**

We will do this ONE STEP AT A TIME, building and testing at each checkpoint before moving on.
