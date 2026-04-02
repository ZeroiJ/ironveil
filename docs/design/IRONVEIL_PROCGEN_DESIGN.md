# Ironveil — Procedural Map Design

**Purpose:** Special rooms, decorative objects, corridor distinction,
and visual upgrades to procedural dungeon generation.
**Scope:** map.rs (generation), main.rs (rendering), new RoomType enum

---

## 1. Room Types

Every room gets a `RoomType` assigned at generation time.

```rust
pub enum RoomType {
    Normal,    // standard combat room
    Treasure,  // guaranteed items, no monsters
    Trap,      // hazard tiles that damage player
    Shrine,    // altar, interact for buff
    Secret,    // hidden entrance, rare reward
    Boss,      // last room, always large arena
    Spawn,     // first room, player starts here
}
```

Add `room_type: RoomType` to the `Rect` struct, or create a
parallel `Vec<RoomType>` that maps index to room type.

---

## 2. Special Room Visuals

Each room type uses different wall and floor characters.
Color stays the same as the floor theme — only the
characters change so they're readable in fog too.

### Normal Room (existing)
```
Theme 1:  walls = #   floor = .
Theme 2:  walls = %   floor = .
Theme 3:  walls = +   floor = ~
```

### Treasure Room
```
All themes:  walls = $   floor = "

Example (dungeon theme, visible):
  $ $ $ $ $ $ $ $ $
  $  " " " " " " " $
  $  " " $ " $ " " $     $ = chest objects on floor
  $  " " " " " " " $
  $  " " " " " " " $
  $ $ $ $ $ $ $ $ $

Wall color:  Yellow (all themes — gold = treasure)
Floor color: DarkYellow
Chest ($):   Yellow on floor — same as wall but
             rendered as ground item when picked up
```

### Trap Room
```
All themes:  walls = # / % / +  (same as normal)
             floor = ^ (caret = trap tiles)

Example (cavern theme, visible):
  % % % % % % % % %
  %  . ^ . ^ . ^ . %     ^ = active trap (Red)
  %  ^ . . . . . ^ %
  %  . . ^ . ^ . . %
  %  ^ . . . . . ^ %
  % % % % % % % % %

Trap tile colors:
  Active trap:    ^   Red
  Triggered trap: ^   DarkGrey (already sprung)
  In fog:         ^   DarkRed (ominous even when dim)
```

### Shrine Room
```
All themes:  walls = | (pipe = stone pillars feel)
             floor = : (colon = sacred ground)

Example (dungeon theme, visible):
  | | | | | | | | |
  |  : : : : : : : |
  |  : O : : : O : |     O = pillars in corners
  |  : : : : : : : |
  |  : : & : : : : |     & = altar in center
  |  : : : : : : : |
  |  : O : : : O : |
  |  : : : : : : : |
  | | | | | | | | |

Wall color:   Cyan (all themes — mystical)
Floor color:  DarkCyan
Altar (&):    Yellow, center of room
Pillars (O):  Grey, room corners
```

### Secret Room
```
All themes:  walls = ?   floor = *

The entrance is a single ? tile in a normal room wall.
Walking into it reveals the secret room.

Example:
  # # # # # # # # #     normal room wall
  # . . . . . . . #
  # . . . . . . . ?  <-- secret entrance tile
  # . . . . . . . #
  # # # # # # # # #

  ? ? ? ? ? ? ?
  ?  * * * * * ?         secret room inside
  ?  * $ * $ * ?     $ = guaranteed rare items
  ?  * * * * * ?
  ?  * $ * $ * ?
  ?  * * * * * ?
  ? ? ? ? ? ? ?

Wall color:   Magenta
Floor color:  DarkMagenta
Items ($):    Yellow (guaranteed rare/artifact tier)
```

---

## 3. Corridor Distinction

Corridors use a different floor character than rooms.
Detected using the existing `map.is_corridor(x, y)` method.

```
Theme 1 (dungeon):
  Room floor:     .   DarkGrey
  Corridor floor: ·   DarkGrey   (U+00B7 middle dot)

Theme 2 (cavern):
  Room floor:     .   DarkGrey
  Corridor floor: ,   DarkGrey

Theme 3 (void):
  Room floor:     ~   DarkMagenta
  Corridor floor: -   DarkRed    (void corridors feel cracked)
```

No new tile type needed — detect in `render_tile()`:

```rust
Tile::Floor => {
    let (char, color) = if map.is_corridor(x, y) {
        match floor {
            1..=3 => ('·', Color::DarkGrey),
            4..=6 => (',', Color::DarkGrey),
            _     => ('-', Color::DarkRed),
        }
    } else {
        match floor {
            1..=3 => ('.', Color::DarkGrey),
            4..=6 => ('.', Color::DarkGrey),
            _     => ('~', Color::DarkMagenta),
        }
    };
    // Apply dim if explored but not visible
    let color = if !visible { Color::Black } else { color };
    execute!(stdout, SetForegroundColor(color), Print(char))?;
}
```

---

## 4. Decorative Objects

Stored as a `HashMap<(usize, usize), DecoObject>` on the Map,
generated once per floor alongside ground items.

```rust
pub enum DecoObject {
    Torch,    // * near walls, flickers White/Yellow
    Pillar,   // O in room corners
    Altar,    // & in shrine room center
    Chest,    // $ in treasure/secret rooms
}
```

### Torches — `*` symbol
Placed adjacent to wall tiles in normal and treasure rooms.
1-2 per room. Render alternating White/DarkYellow every
500ms (reuse the player pulse timer logic).

```
Placement rule:
  Pick a floor tile with a wall tile as a direct neighbor.
  Place torch there. Never in corridors.

Visual in dungeon:
  # # # # # # # # #
  # * . . . . . . #     * = torch on wall
  # . . . . . . . #
  # . . . . . . * #
  # # # # # # # # #

Colors:
  Pulse on:   *   White
  Pulse off:  *   DarkYellow
  In fog:     *   DarkGrey
```

### Pillars — `O` symbol
Placed in corners of large rooms (width > 10 or height > 8).
Always 4 pillars, one per corner, 2 tiles from the wall.

```
  # # # # # # # # # # # #
  # . . . . . . . . . . #
  # . O . . . . . . O . #     O = pillars
  # . . . . . . . . . . #
  # . . . . . . . . . . #
  # . O . . . . . . O . #
  # . . . . . . . . . . #
  # # # # # # # # # # # #

Colors:
  Visible:    O   Grey
  In fog:     O   DarkGrey
  Blocks movement: yes (impassable like a wall)
```

### Altar — `&` symbol
Only in shrine rooms. Single tile at room center.
Interacting (bumping into it) triggers shrine effect.

```
  | | | | | | | | |
  | : : : : : : : |
  | : O : : : O : |
  | : : : : : : : |
  | : : & : : : : |     & = altar (center)
  | : : : : : : : |
  | : O : : : O : |
  | | | | | | | | |

Colors:
  Visible:    &   Yellow (glowing, sacred)
  Activated:  &   DarkGrey (used up)
  In fog:     &   DarkYellow
```

### Chests — `$` symbol
In treasure and secret rooms. Each chest is a ground item
spawn point. Walking over picks up the item.
Same pickup logic as regular ground items.

```
Colors:
  Unopened:   $   Yellow
  Opened:     .   (reverts to floor tile after pickup)
  In fog:     $   DarkYellow (visible even when explored)
```

---

## 5. Special Room Generation Logic

Add to `Map::new()` after all rooms are generated.

```
SPAWN RULES:

1. rooms[0]     = always Spawn (player start)
2. rooms.last() = always Boss (stairs + boss arena)
3. Remaining rooms = candidates for special rooms

SPECIAL ROOM FREQUENCY:
  Roll once per floor: 33% chance of special room
  If triggered: pick one type randomly from available pool
  Max 1 special room per floor (never 2)

AVAILABLE POOL by floor:
  Floors 1-3:  Treasure (50%), Trap (30%), Shrine (20%)
  Floors 4-9:  Treasure (30%), Trap (35%), Shrine (20%),
               Secret (15%)
  Floors 10+:  Treasure (20%), Trap (30%), Shrine (20%),
               Secret (30%)

PLACEMENT:
  Pick a random room from candidates (not spawn, not boss,
  not already assigned). Assign its RoomType.
  Apply visual overrides at render time.
```

---

## 6. Shrine Buff Effects

When player bumps into `&` altar tile:

```
Roll randomly from this pool:

  "Strength"    — +2 STR for rest of floor
  "Vitality"    — Heal to full HP
  "Swiftness"   — +10% dodge for rest of floor
  "Knowledge"   — +50 XP bonus
  "Darkness"    — Reveal entire floor map (fog cleared)
  "Warding"     — Next hit you take is reduced by 5 dmg
```

Log message: `"The shrine pulses... [effect name]!"`
Altar changes to DarkGrey after use (one use only).
Each floor can only have one shrine so this is a
meaningful decision, not spammable.

---

## 7. Trap Tile Behavior

Trap tiles (`^`) on the floor trigger when walked on.

```
TRAP TYPES (roll randomly when room is generated):

  Spike trap:    1-4 dmg, message: "Spikes erupt from the floor!"
  Fire trap:     2-3 dmg + 2 poison ticks
                 message: "Flames burst from the ground!"
  Teleport trap: Moves player to random floor tile
                 message: "The floor vanishes beneath you!"
  Alarm trap:    No damage, wakes all monsters on floor
                 message: "An alarm sounds!"

All traps trigger once then become DarkGrey (spent).
Monsters do NOT trigger traps — player only.
```

---

## 8. Secret Room Entrance

The `?` tile in a normal room wall is the secret entrance.
It looks like a wall tile but is walkable.

```
Detection: render ? as wall color (Grey/DarkYellow/DarkRed)
           but Tile type is actually Tile::SecretDoor

Walking into it:
  1. Converts to Tile::Floor
  2. Reveals the adjacent secret room tiles
  3. Log: "You find a hidden passage!"

In fog:
  Explored ?:  render as normal wall (player doesn't know
               they've already passed a secret door unless
               they walked through it)
```

---

## 9. Full Visual Examples Per Theme

### Dungeon Floor 2 — Mixed rooms

```
# # # # # # # # # # # # # # # # # # #
# * . . . . . . # # # # # # # # # # #
# . . g . . . . # . . . . . . . . . #   Normal room (left)
# . . . . . . . # . . O . . . O . . #   Shrine room (right)
# . . . . . . . · · . . . . . . . . #
# . . . . . . . # . . . & . . . . . #
# . . . . . . . # . . O . . . O . . #
# # # # # # # # # . . . . . . . . . #
                  # # # # # # # # # #
```

### Cavern Floor 5 — Trap room visible

```
% % % % % % % % % % %
%  . , , , , , , . . %    , = corridor tiles
%  . . . ^ . ^ . . . %    ^ = trap tiles (Red)
%  . ^ . . . . . ^ . %
%  . . . ^ . ^ . . . %
%  . . . . . . . . . %
% % % % % % % % % % %
```

### Void Floor 10 — Secret room

```
+ + + + + + + + + + +
+  ~ ~ ~ ~ ~ ~ ~ ~ ~ +
+  ~ ~ ~ - - - ~ ~ ~ +    - = corridor to secret
+  ~ ~ ~ ~ ~ ~ ~ ~ ~ +
+ + + + ? + + + + + +     ? = secret door (looks like +)

  ? ? ? ? ? ? ? ? ?
  ?  * * * * * * * ?
  ?  * $ * * $ * * ?       $ = rare items
  ?  * * * * * * * ?
  ?  * $ * * $ * * ?
  ?  * * * * * * * ?
  ? ? ? ? ? ? ? ? ?
```

---

## 10. Render Order Addition

Add decorative objects to the render pipeline in main.rs.
They render AFTER the map tiles, BEFORE entities.

```
Current render order:
  1. render_map_delta()
  2. render_ui()
  3. render_minimap()
  4. render_webs()
  5. render_ground_items()
  6. render_monsters()
  7. render_projectiles()
  8. player @

Updated render order:
  1. render_map_delta()
  2. render_ui()
  3. render_minimap()
  4. render_deco_objects()    <-- ADD HERE
  5. render_webs()
  6. render_ground_items()
  7. render_monsters()
  8. render_projectiles()
  9. player @
```

`render_deco_objects()` must check
`map.current_visibility[x][y]` before rendering each object —
same rule as all other entities.

Torches pulse using the same `pulse_start_time` already in
the game loop. No new timer needed.

---

## 11. Data Structures Summary

New fields needed on Map struct:

```rust
pub room_types: Vec<RoomType>,
pub deco_objects: HashMap<(usize, usize), DecoObject>,
pub trap_tiles: HashMap<(usize, usize), TrapType>,
pub shrine_used: HashSet<(usize, usize)>,
```

New enums in map.rs:

```rust
pub enum RoomType {
    Normal, Treasure, Trap, Shrine, Secret, Boss, Spawn
}

pub enum DecoObject {
    Torch, Pillar, Altar, Chest
}

pub enum TrapType {
    Spike, Fire, Teleport, Alarm
}
```

All need `#[derive(Clone, Serialize, Deserialize)]`
for save/load compatibility.

---

## 12. Notes for Agent

### Tile enum change
Add `Tile::SecretDoor` for the hidden entrance tile.
Renders as wall color but is_walkable() returns true.
Walking through converts it to Tile::Floor.

### Pillar blocking
Pillars (O) block movement. Store pillar positions
in deco_objects. In player movement check, treat
pillar tiles as impassable (same check as wall).

### Safe generation order
```
1. Generate normal rooms (existing logic)
2. Assign room types (new logic)
3. Generate decorations per room type (new logic)
4. Generate ground items (existing logic, skip Treasure
   rooms since chests handle item placement)
5. Spawn monsters (existing logic, skip Treasure/Shrine/
   Secret rooms — no monsters in special rooms)
```

### Serialization
deco_objects, trap_tiles, shrine_used, room_types all
go into SaveData in save_load.rs alongside the existing
map field. Since Map already derives Serialize/Deserialize
and these are new fields on Map, they serialize
automatically once added to the struct.
