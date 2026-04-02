# Ironveil — Map Tile & Environment Design

**Purpose:** Visual upgrade for all map tiles across floor themes.
**Rule:** Every character here is single-width monospace safe.
**Current:** Walls = `#`, Floors = `.`, Stairs = `>`

---

## 1. Core Design Philosophy

The map needs three visual layers that are instantly readable:

```
1. WALLS    — should feel solid and impassable
2. FLOORS   — should feel walkable and open
3. FEATURES — stairs, items, entities stand out
```

The key constraint: walls and floors must be visually distinct
at a glance even in peripheral vision. Color alone is not enough
— the character choice matters too.

---

## 2. Floor Themes — Tile Sets

### Theme 1: Stone Dungeon (Floors 1-3)
Color: Wall = Grey, Floor = DarkGrey

```
WALLS:    #   (solid, classic, familiar)
FLOORS:   .   (dot, open feel)
STAIRS:   >   (White, bright, easy to spot)
CORRIDOR: ·   (middle dot, slightly lighter than room floor)

Example room:
  # # # # # # # # # # #
  #  . . . . . . . . . #
  #  . . . . . . . . . #
  #  . . . . . . . . . #
  #  . . . . . · · · · ·· (corridor exits)
  #  . . . . . . . . . #
  # # # # # # # # # # #
```

Fog states:
```
Currently visible wall:    #   Grey
Explored wall:             #   DarkGrey
Currently visible floor:   .   DarkGrey
Explored floor:            .   Black (barely visible)
Unexplored:                     (space, pure black)
```

---

### Theme 2: Cavern (Floors 4-6)
Color: Wall = DarkYellow, Floor = DarkGrey

The cavern should feel rough and organic compared to the
clean dungeon. Achieve this by mixing wall characters.

```
WALLS:       %   (rough rock feel, different from dungeon)
FLOORS:      .   (same floor dot, consistent)
STAIRS:      >   (White)
CORRIDOR:    ,   (comma, feels like gravel/dirt path)

Example room:
  % % % % % % % % % % %
  %  . . . . . . . . . %
  %  . . . . . . . . . %
  %  . . . . . . . . . %
  %  . . . . . , , , , ,,  (corridor exits feel gravelly)
  %  . . . . . . . . . %
  % % % % % % % % % % %
```

Fog states:
```
Currently visible wall:    %   DarkYellow
Explored wall:             %   DarkGrey
Currently visible floor:   .   DarkGrey
Explored floor:            .   Black
Corridor visible:          ,   DarkGrey
```

---

### Theme 3: Void / Hell (Floors 7+)
Color: Wall = DarkRed, Floor = DarkMagenta

The void should feel wrong and alien. Use characters that
look unstable and jagged.

```
WALLS:       +   (crosshatch feel, chaotic energy)
FLOORS:      ~   (tilde, feels like corrupted ground)
STAIRS:      >   (Yellow, warm against the cold void)
CORRIDOR:    ~   (same as floor, void has no clean paths)

Example room:
  + + + + + + + + + + +
  +  ~ ~ ~ ~ ~ ~ ~ ~ ~ +
  +  ~ ~ ~ ~ ~ ~ ~ ~ ~ +
  +  ~ ~ ~ ~ ~ ~ ~ ~ ~ +
  +  ~ ~ ~ ~ ~ ~ ~ ~ ~ +
  +  ~ ~ ~ ~ ~ ~ ~ ~ ~ +
  + + + + + + + + + + +
```

Fog states:
```
Currently visible wall:    +   DarkRed
Explored wall:             +   DarkGrey
Currently visible floor:   ~   DarkMagenta
Explored floor:            ~   Black
```

---

## 3. Special Tiles

### Stairs
Always White/Yellow regardless of floor theme.
Bright enough to spot across the room.

```
Unexplored:          (never shown — intentional)
Explored + visible:  >   Yellow
Explored + dim:      >   DarkGrey
```

### Spider Webs
Currently rendered as `:` in White.
No change needed — works well.

```
Web tile:   :   White (visible), DarkGrey (explored)
```

### Corridor vs Room Floor
Subtle distinction helps players understand dungeon layout.

```
Theme 1 (dungeon):
  Room floor:     .   DarkGrey
  Corridor floor: ·   DarkGrey (middle dot U+00B7)

Theme 2 (cavern):
  Room floor:     .   DarkGrey
  Corridor floor: ,   DarkGrey

Theme 3 (void):
  Room floor:     ~   DarkMagenta
  Corridor floor: ~   DarkMagenta (same, void is formless)
```

---

## 4. Full Room Examples — All Themes

### Stone Dungeon Room with Entities
```
  # # # # # # # # # # # # #
  # . . . . . . . . . . . #
  # . g . . . . . . . . . #     g = Goblin (Green)
  # . . . . . . . . . . . #
  # . . . . @ . . . . . . #     @ = Player (White pulse)
  # . . . . . . . . . . . #
  # . . . . . . / . . . . #     / = Weapon on ground (Cyan)
  # . . . . . . . . . . . #
  # # # # # # · · · # # # #     · = corridor exit
```

### Cavern Room with Fog
```
  % % % % % % % % % % % % %
  % . . . . . . . . . . . %     (dim, explored)
  % . . . . . . . . . . . %
            @ . . . . . . %     (bright, currently visible)
            . . . . . . . %
            . . . . . . . %
            . . . , , , , ,,
                            (black, unexplored)
```

### Void Room — Boss Arena
```
  + + + + + + + + + + + + + + + + + + + +
  +  ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ +
  +  ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ +
  +  ~ ~ ~ ~ S ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ +     S = Shadow Lord (Magenta)
  +  ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ +
  +  ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ +
  +  ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ @ ~ ~ ~ ~ ~ ~ +     @ = Player
  +  ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ +
  +  ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ +
  +  ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ +
  +  ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ > ~ ~ +     > = Stairs (Yellow)
  +  ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ +
  + + + + + + + + + + + + + + + + + + + +
```

---

## 5. Minimap Tile Representation

Each minimap cell = scaled region of real map.

```
Full map tile   →   Minimap symbol
─────────────────────────────────
Unexplored      →     (space, black)
Explored wall   →   # DarkGrey
Visible wall    →   # Grey
Explored floor  →   . Black (barely visible)
Visible floor   →   . DarkGrey
Player          →   @ White
Stairs          →   > Yellow (only when explored)
```

---

## 6. Implementation in render_tile()

Replace the current render_tile() function with this
theme-aware version:

```rust
fn render_tile(
    stdout: &mut std::io::Stdout,
    tile: Tile,
    floor: i32,
    visible: bool,
    seen: bool,
) -> std::io::Result<()> {

    // Never seen — pure black
    if !seen {
        execute!(stdout,
            SetForegroundColor(Color::Black),
            Print(" ")
        )?;
        return Ok(());
    }

    // Get theme-specific characters and colors
    let (wall_char, floor_char, wall_color, floor_color) =
        match floor {
            1..=3 => ('#', '.', Color::Grey,      Color::DarkGrey),
            4..=6 => ('%', '.', Color::DarkYellow, Color::DarkGrey),
            _     => ('+', '~', Color::DarkRed,    Color::DarkMagenta),
        };

    // Dim colors for explored-but-not-visible
    let (wall_color, floor_color) = if visible {
        (wall_color, floor_color)
    } else {
        (Color::DarkGrey, Color::Black)
    };

    match tile {
        Tile::Wall => execute!(stdout,
            SetForegroundColor(wall_color),
            Print(wall_char)
        )?,
        Tile::Floor => execute!(stdout,
            SetForegroundColor(floor_color),
            Print(floor_char)
        )?,
        Tile::Stairs => {
            let color = if visible {
                Color::Yellow
            } else {
                Color::DarkGrey
            };
            execute!(stdout,
                SetForegroundColor(color),
                Print('>')
            )?;
        }
    }
    Ok(())
}
```

---

## 7. Color Reference Per Theme

### Theme 1 — Stone Dungeon (Floors 1-3)
```
Wall visible:      #   Grey
Wall explored:     #   DarkGrey
Floor visible:     .   DarkGrey
Floor explored:    .   Black
Stairs visible:    >   Yellow
Stairs explored:   >   DarkGrey
```

### Theme 2 — Cavern (Floors 4-6)
```
Wall visible:      %   DarkYellow
Wall explored:     %   DarkGrey
Floor visible:     .   DarkGrey
Floor explored:    .   Black
Stairs visible:    >   Yellow
Stairs explored:   >   DarkGrey
```

### Theme 3 — Void/Hell (Floors 7+)
```
Wall visible:      +   DarkRed
Wall explored:     +   DarkGrey
Floor visible:     ~   DarkMagenta
Floor explored:    ~   Black
Stairs visible:    >   Yellow
Stairs explored:   >   DarkGrey
```

---

## 8. Regional Biome Monster Spawns

Now that floor themes have visual identity, make
monster spawns match the biome. Currently all
floors can spawn any monster. Restrict by theme:

### Dungeon (Floors 1-3) — Classic monsters
```
Goblin, Skeleton, Troll
Occasional: BatSwarm
```

### Cavern (Floors 4-6) — Cave creatures
```
Spider, BatSwarm, Orc, Bandit
Occasional: Wraith, Zombie
Boss: Goblin King (floor 5)
```

### Deep Cavern (Floors 7-9) — Undead and beasts
```
Zombie, Ghoul, Minotaur, Bear, Wolf Pack
Occasional: Necromancer, Specter
```

### Void/Hell (Floors 10+) — Demons and elementals
```
Imp, Demon, Fire Elemental, Ice Elemental
Occasional: Hellfire Elemental, Assassin, Golem
Boss: Bone Dragon (floor 10), Shadow Lord (floor 15+)
```

This makes descending feel like entering a new world
not just a harder version of the same dungeon.

---

## 9. Notes for Agent

### Changing wall/floor chars by theme
Only touch render_tile() in main.rs. The tile data
(Tile::Wall, Tile::Floor) does not change — only
how they are displayed changes per floor theme.

### Do not change
- Tile enum variants — Wall, Floor, Stairs stay the same
- Map generation logic — rooms and corridors unchanged
- Fog of war logic — visibility grids unchanged
- Entity rendering — monsters/items/player unchanged

### Do change
- render_tile() — add wall_char and floor_char per theme
- The `%` and `+` and `~` characters need no special
  handling — they are standard ASCII, fully safe

### Corridor floor distinction (optional enhancement)
To tell corridors from rooms visually, add a
Tile::Corridor variant OR detect corridor in render
using the existing map.is_corridor(x, y) method
and render with a different character. The
is_corridor() method already exists in map.rs.
