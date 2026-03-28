# Ironveil Development Changelog

## v0.4.1 — UI/UX Visual Overhaul
*2026-03-28*

### Phase 1: Unicode Box-Drawing
- Replaced ASCII borders (`+---+`) with Unicode box-drawing characters (`╔═══╗`) in character creation and inventory screens
- Updated minimap borders to use `┌───┐` style
- Applied rarity-based colors to inventory items:
  - Common: White, Uncommon: Green, Rare: Cyan, Epic: Magenta, Legendary: Yellow

### Phase 2: Title Screen & Enhanced UI
- Added new title screen with ASCII Ironveil logo
- Menu options: New Game, Load Game, Quit
- Unicode borders on all UI screens

### Phase 3: Combat & Flavor Updates
- Combat messages now use Unicode symbols:
  - `⚔` for player attacks
  - `★ CRITICAL STRIKE! ★` for critical hits
  - `➤` for monster attacks
  - `🔥` for dragon fire attacks
  - `👻` for shadow energy attacks
  - `✝` for monster kills
- Enhanced floor transition messages with themed text:
  - Floor 5: "You descend to the Dungeons... A dark presence lurks below..."
  - Floor 10: "You descend to the Bone Pits... The air grows cold..."
  - Floor 15: "You descend to the Shadow Realm... The final battle awaits!"

---

## v0.4.0 — Artifacts, Minimap & Boss Updates
*2026-03-27*

### Combat System Updates
- Critical hit system: DEX-based crit chance (1% per DEX above 10), 150% crit multiplier
- Rogue starts with 4% base crit chance
- Damage variance: ±10% randomization on all melee damage
- Combat log now shows "CRITICAL!" on crit hits

### Artifact Items
- Added 9 artifact items with unique passive effects (marked with `* Name *`)
- Artifacts are class-specific, drop from bosses and rare floor 6+ kills
- **Warrior**: Ragefang (kill = +1 ATK stacks), Stonehide Plate (+3 DEF at low HP), Warlord Signet (War Cry bonus)
- **Rogue**: Shadowfang (ShadowStep crits), Wraithwalkers (+15% dodge, counterattack), Venomcoil (Poison Blade boost)
- **Mage**: Stormcaller Staff (Chain Lightning boost), Frostweave Robe (Frost Nova boost), Mindfire Crown (ability damage multiplier)
- Artifacts render Yellow on the ground, displayed with asterisks in inventory

### Named Unique Items
- Added 9 named items with better stats than regular drops
- Shadow Slicer (+4 dmg, +1 DEX), Bone Crusher (+6 dmg, +2 STR), Spellbound Staff (+3 dmg, +4 INT)
- Veteran's Plate (+5 def, +2 CON), Swiftboots (+1 def, +4 DEX), Sage's Robe (+2 def, +3 INT)
- Lion's Amulet (+4 STR), Eagle's Eye (+4 DEX), Dragon Heart (+4 CON)
- Floor-gated spawning (floor 4+ and 5+)

### Minimap
- 30x15 character minimap in bottom-right corner
- Shows explored tiles only (fog of war respected)
- Player `@` always visible, stairs `>` shown when explored
- Currently visible areas shown brighter than explored-but-hidden

### Boss Room Generation
- Last room now always a large open arena (18-24 wide × 12-16 tall)
- Boss room connected to previous room via tunnels
- Stairs placed in center of boss room

### Boss ASCII Intro
- Boss floors (5, 10, 15+) show ASCII art intro before battle
- Goblin King: crown art with gold/yellow colors
- Bone Dragon: dragon art with fire breath
- Shadow Lord: shadow entity art with darkness
- Typewriter animation effect, waits for keypress before fight

### Boss Scaling
- Bosses now spawn every 5 floors (5, 10, 15, 20, 25...)
- All floors 15+ use Shadow Lord boss

---

## v0.3.2 — Fog of War & Performance & Death Screen
*2026-03-26*

### Fog of War
- Implemented proper line-of-sight visibility (walls block vision)
- Tiles within radius 8 are visible if no walls blocking
- "Seen" memory: explored areas remain visible but dimmed
- Unexplored areas render as black

### Delta Rendering
- Added delta rendering for fog of war (only redraws changed tiles)
- Full render removed from inner loop to eliminate input lag
- Map tiles only update when player visibility changes
- Smooth performance: no flickering, immediate input response

### Entity Visibility
- Monsters hidden in unexplored areas
- Ground items hidden in unexplored areas
- Webs hidden in unexplored areas

### Save/Load
- Visibility state preserved across save/load

### Death Screen
- Animated skull appearing line by line on death
- "YOU HAVE DIED" text flashes 3 times
- Stats revealed one by one: class, floor reached, monsters slain, damage dealt, damage taken, cause of death
- [R] Play again / [Q] Quit options
- Run stats tracked: monsters_slain, damage_dealt, damage_taken, cause_of_death

---

## v0.3.1 — Player Visibility & Floor Reveal
*2026-03-25*

### Player Visibility
- Player character now bright White (not class-based colors)
- Pulsing effect: toggles White/Yellow every 500ms
- Easier to locate on the map

### Progressive Floor Reveal
- New floors reveal in a wave from player outward
- Ring-by-ring reveal animation (~1-2 seconds)
- Visual feedback when entering new floors

### Coordinate Display
- Player position shown in UI: `Pos:(X,Y)`
- Easy reference for navigation

### Reverted Features
- Camera system (viewport scrolling) - reverted due to usability issues
- Player trail - reverted with camera
- Fog of war - FIXED in v0.3.2 (now working with LOS)
- See `docs/REVERTED.md` for remaining reverted features

---

## v0.3.0 — Monster Expansion & Quality of Life
*2026-03-25*

### Save/Load System
- Added Ctrl+S to save game, Ctrl+L to load game
- Serialize/Deserialize on all game structs (Player, Map, Monster, Items)
- Saves to `save.json` in JSON format
- Loads restore full game state: player, map, monsters, ground items, floor, log

### Fog of War
- Tiles only visible within 8-tile radius of player (line-of-sight)
- Explored tiles dimmed (dark grey), unexplored tiles black
- Visibility updates on every player movement
- `Map.update_visibility()` using Bresenham line-of-sight

### Buff/Debuff Visualization
- Player turns White when damage buff active
- Player turns Green when poisoned
- Visual feedback for status effects

### Floor Theming
- Floors 1-3: Grey walls, dark grey floors (dungeon)
- Floors 4-6: Dark yellow walls (cavern)
- Floors 7+: Dark red walls, dark magenta floors (void)

### Monster Expansion — 18 New Types

#### Undead
- **Zombie** (`z` Grey): HP 12, ATK 3. Regenerates 1 HP per tick. Slow chase.
- **Ghoul** (`G` DarkYellow): HP 8, ATK 5. Poison bite (3 ticks). Fast pursuit.
- **Specter** (`p` DarkGrey): HP 6, ATK 7. Phases through walls. Silent stalker.

#### Demons
- **Imp** (`i` Red): HP 7, ATK 4. Ranged fire bolt (3-tick cooldown). Small and fast.
- **Demon** (`D` Red): HP 18, ATK 6. Tough melee fighter. Aggressive pursuit.
- **Hellfire Elemental** (`f` DarkRed): HP 12, ATK 5. Fire AOE (2-tick poison).

#### Beasts
- **Minotaur** (`M` DarkRed): HP 25, ATK 8. Charging beast. Territorial.
- **Bear** (`B` Yellow): HP 15, ATK 4→8. Berserk at low HP (double damage).
- **Wolf Pack** (`w` White): HP 6, ATK 3. Fast hunter. Roams in packs.

#### Humanoids
- **Orc** (`O` DarkYellow): HP 14, ATK 5. Standard warrior. Balanced stats.
- **Bandit** (`b` DarkYellow): HP 7, ATK 4. Drain attack (steals life). Opportunistic.
- **Assassin** (`a` DarkGrey): HP 6, ATK 9. One-shot specialist. Stealthy approach.

#### Elementals
- **Fire Elemental** (`F` Red): HP 10, ATK 5. Fire AOE on hit. Burns adjacent.
- **Earth Elemental** (`E` DarkCyan): HP 20, ATK 5. Tough and slow. Rocky defense.
- **Ice Elemental** (`I` Cyan): HP 12, ATK 4. Freeze attack. Cold embrace.

#### Constructs
- **Golem** (`G` Green): HP 40, ATK 3. Very slow (every other turn). Stone skin.
- **Sentry** (`s` White): HP 10, ATK 6. Ranged laser (2-tick cooldown). Stationary guardian.
- **Bomber** (`o` DarkMagenta): HP 8, ATK 10. Kamikaze. Explodes on contact.

### Spawn Table Updates
- Floors 1-3: Original roster only
- Floors 4-5: Full early roster + Wraith
- Floors 6-8: Adds Zombie, Ghoul
- Floors 9+: All monsters including Specter

### Documentation
- Created MONSTERS.md — monster catalog with stats
- Created CLASSES.md — character classes and abilities
- Created ITEMS.md — weapons, armor, rings, potions
- Created BIOMES.md — floor themes and biomes
- Created CONTROLS.md — keybindings reference

---

## Phase 5: Isometric Rendering (Attempted & Reverted)
*2026-03-25*

### Attempted
- Migrated from bracket-lib to Fancy console for isometric rendering
- Created `render.rs` with isometric projection math, camera system, z-sorting
- Implemented 2-tile tall walls with face rendering
- Multi-glyph entity rendering (player with head/body)

### Reverted
- Flickering issues with Fancy console
- Reverted to 2D ASCII rendering via `git revert`
- Will revisit with better understanding of bracket-lib rendering

---

## Phase 4: The World Has Rules
*Completed: 2026-02-22*

### 4.9 HUD Overhaul
- Status line 1 now shows Level alongside Floor, HP, Class, and Weapon.
- New XP bar with progress indicator: `XP:42/120 [###-------]`.
- New ability status line: `[1]Power Attack (Ready) [2]Locked (Lv5)` with cooldown display.
- Player poison status shown when active: `POISONED(3)`.
- Message log shifted to accommodate the ability status line.

### 4.8 Boss System — Three Unique Bosses
- **Goblin King** (Floor 5): Symbol `K` gold. 60 HP, 5 ATK. Summons 2 goblin minions every 4 ticks (3 when enraged). Enrages at <50% HP for +2 damage. 150 XP.
- **Bone Dragon** (Floor 10): Symbol `D` dark red. 100 HP, 6 ATK. Breath attack: line AoE dealing 8 damage, range 3 (5 when enraged). Slow movement (every 2 ticks). Enrages at <50%: breath fires every 2 ticks. 300 XP.
- **Shadow Lord** (Floor 15): Symbol `S` magenta. 120 HP, 5 ATK. Teleports every 3 ticks (2 when enraged). Drain attack heals for half damage dealt. Shadow Pulse AoE: 3 damage in radius 2 (radius 3 when enraged). 500 XP.
- Bosses block stairs until defeated: "The stairs are sealed by a dark power!"
- Boss floor announcements on entry with dramatic messages.
- Boss death celebration: "*** BOSS DEFEATED! ***" with guaranteed high-tier loot drop.
- Bosses spawn in the stairs room, offset from stairs tile.

### 4.7 Player Poison System
- Added `poison_ticks` field to Player struct.
- Spider's PoisonAttack now applies poison to the player.
- Poison deals 1 HP damage per monster tick, decrements each tick.
- Poison status shown in HUD ability line.

### 4.6 Web Stuck Mechanic
- Spider webs rendered as `:` in White via `HashSet<(usize,usize)>`.
- Player stepping on a web consumes it and sets 2-tick stuck duration.
- While stuck, player movement is blocked but abilities still work.
- Web stuck counter decrements on monster tick; "You break free!" on expiry.

### 4.5 Bat Swarm Group Spawning
- Bat Swarms now spawn in groups of 2-3 per room instead of solo.
- Each bat offset within the room to avoid stacking.

### 4.4 Four New Monster Types
- **Bat Swarm** (`b` DarkYellow): Fast (every tick), 30% erratic movement, 25% dodge at tier 2+. HP 4, ATK 1. 15 XP.
- **Spider** (`x` DarkMagenta): Ambush predator (waits until player within 4 tiles). Places webs at 3-5 tile distance. Poison bite at tier 2+. HP 8, ATK 3. 15 XP.
- **Wraith** (`W` DarkCyan): Phases through walls, invisible while inside walls, invulnerable while phasing. Drain attack heals for half damage. Retreats into walls. HP 12, ATK 5. 30 XP.
- **Necromancer** (`N` DarkRed): Coward (flees if < 6 tiles). Resurrects dead monsters at 50% HP (75% at tier 3). Max 3 resurrections (5 at tier 3). HP 8, ATK 2. 50 XP.
- Monster spawn table scales with floor depth (Goblins only on floor 1 → full roster by floor 6+).

### 4.3 Monster Status Effects
- `stun_ticks`: Skip turn, rendered DarkGrey (applied by WarCry ability).
- `freeze_ticks`: Skip turn, rendered Cyan (applied by Frost Nova ability).
- `poison_ticks`: 1 damage/tick, rendered Green (applied by Poison Blade ability).
- Status effects decrement each monster tick. Poison kills grant XP.

### 4.2 XP & Leveling System
- XP gained from: monster kills (melee, ability, poison), floor descent (floor * 5 XP).
- 10 levels with escalating thresholds: 50, 120, 220, 360, 540, 780, 1080, 1440, 1900.
- Level-up grants: +3 max HP, class-specific stat boosts (+1 primary stat every 2 levels).
- Cooldown reduction at levels 3 and 7 (-1 tick on ability cooldowns).
- Second ability unlocks at level 5.

### 4.1 Class Ability System
- **Warrior**: [1] Power Attack — 2x melee damage on next hit (6 tick cooldown). [2] War Cry — stuns all monsters within 4 tiles for 2 ticks (8 tick cooldown, unlocks Lv5).
- **Rogue**: [1] Shadow Step — teleport up to 4 tiles in a direction, shadow strike buff if monster adjacent at landing (5 tick cooldown). [2] Poison Blade — next 3 melee hits apply 3 ticks of poison (7 tick cooldown, unlocks Lv5).
- **Mage**: [1] Chain Lightning — fires 6 tiles, hits first monster for 3+INT damage, chains to 2 more nearby for decreasing damage (6 tick cooldown). [2] Frost Nova — freezes all monsters within 3 tiles for 2 ticks + INT-based damage (8 tick cooldown, unlocks Lv5).
- Two-step direction input: Press `1`/`2` to ready, then arrow key for direction (or any other key to cancel).
- Instant-activation abilities (Power Attack, War Cry, Poison Blade, Frost Nova) trigger immediately on `1`/`2`.
- Cooldowns tick on the 500ms monster tick. Status shown in HUD.

## Phase 3: You Have an Identity
*Completed: 2026-02-22*

### 3.8 HUD Updates
- Status bar now shows Floor, HP, Class name, and equipped weapon name.
- Second status line displays effective stats (STR/DEX/INT/CON), armor defense, and Tab:Inventory hint.
- 3-line message log positioned below status lines.

### 3.7 Combat Formula Overhaul
- Player melee damage: `1 + weapon_bonus + (STR-10)/2`, minimum 1.
- Incoming damage reduced by armor defense: `raw - armor_def`, minimum 1.
- Dodge check on all incoming damage (melee + projectile + goblin dash): `(DEX-10)*3`% chance.
- Dodge messages in log: "You dodge the Goblin's attack!", "You dodge the arrow!"

### 3.6 Starting Equipment
- Warrior starts with Iron Shortsword (+2 dmg) and Leather Armor (+1 def).
- Rogue starts with Twin Daggers (+1 dmg).
- Mage starts with Wooden Staff (+1 dmg) and Ring of Intellect (+2 INT).
- Starting gear is pre-equipped via `equip_starting_gear()`, not placed in backpack.

### 3.5 Item Spawning & Drops
- ~25% chance per room to spawn a ground item (skip player spawn room).
- Monster drops on kill: ~30% chance. Drop type weighted: 50% potion, 25% weapon, 15% armor, 10% ring.
- Ground items stored in `HashMap<(usize,usize), Item>`, rendered with type-colored symbols.
- Walking over a ground item auto-picks it up into inventory (or "inventory full" message).
- Items placed at monster death position on drop.

### 3.4 Inventory System
- Tab opens full-screen inventory overlay, pauses monster tick while open.
- Shows equipped weapon/armor/ring and backpack contents (up to 10 items).
- Letter keys (a-j) to use potions or equip weapons/armor/rings.
- Equipping swaps old equipment back into backpack.
- Using a potion heals with INT bonus and removes it from inventory.
- Tab/Esc closes inventory, redraws map, resets monster tick timer.

### 3.3 Item System (`items.rs`)
- New `items.rs` module with `Item` struct and `ItemType` enum (Weapon/Armor/Ring/Potion).
- Items have damage_bonus, defense_bonus, stat_bonus (type + value), heal_amount.
- Display names include stats: "Shortsword (+2 dmg)", "Ring of Strength (+1 STR)".
- Tier-scaled generation: floors 1-3 tier 1, floors 4-6 tier 2, floors 7+ tier 3.
- Weapons: Dagger (+1) → Shortsword (+2) → Longsword (+3) / Greataxe (+5).
- Armor: Leather (+1) → Chainmail (+2) → Plate (+4).
- Rings: Strength/Agility/Intellect/Vitality with scaling bonus (+1/+2/+3).
- Room spawn weighted: 40% potion, 30% weapon, 20% armor, 10% ring.

### 3.2 Character Creation Screen (`ui.rs`)
- New `ui.rs` module with centered box UI for class selection.
- Up/Down arrows or 1/2/3 keys to select, Enter to confirm.
- ASCII art per class, stat bars, starting gear preview, playstyle description.
- Shown on game start and on restart after death.

### 3.1 Class & Stats System (`player.rs`)
- Three classes: Warrior (Red @), Rogue (Green @), Mage (Blue @).
- `Stats` struct: STR, DEX, INT, CON with derived gameplay effects.
- STR → melee damage bonus `(STR-10)/2`. DEX → dodge chance `(DEX-10)*3`%.
- CON → max HP `20+(CON-10)`. INT → potion healing bonus `+1 per 2 INT above 10`.
- `Equipment` struct with weapon/armor/ring slots affecting combat.
- `Player` rewritten with class, base_stats, equipment, inventory (cap 10).
- Ring bonuses feed into effective stats, which drive all combat calculations.
- `Tile::Potion` removed — potions are now inventory items.
- Player persists across floor transitions (HP, inventory, equipment carry over).

## Phase 2.6: Real-Time Monster AI
*Completed: 2026-02-22*

### 2.6.1 Independent Monster Tick System
- Replaced turn-based game loop with real-time hybrid model.
- Monsters now act independently on a 500ms tick timer, regardless of player input.
- Player moves instantly on keypress (uncapped speed) — no waiting for monster turns.
- Game loop uses non-blocking `event::poll()` (50ms timeout) instead of blocking `event::read()`.
- Projectiles advance on the monster tick (every 500ms).
- Refactored monster processing and projectile processing into dedicated functions.
- Extracted UI rendering into `render_ui()` helper.

## Phase 2.5: Monsters That Think
*Completed: 2026-02-21*

### 2.5.9 Floor-Tier Scaling System
- Monster behaviors scale with floor depth: Tier 1 (floors 1-3), Tier 2 (floors 4-6), Tier 3 (floors 7+).
- `Monster::new()` takes a `floor` parameter, `spawn_monsters_for_floor()` passes current floor through.
- Early floors teach basics; deeper floors punish mistakes with tactical AI.

### 2.5.8 Troll AI — Corridor Blocking & Berserk Mode
- Tier 1: Slow chase (acts every other turn). Hits hard (8 damage).
- Tier 2+: Blocks corridors — stops moving and waits when in a narrow passage with LOS on player.
- Tier 3+: Berserk mode at <30% HP — attack jumps to 12, no longer slow. "The Troll flies into a rage!"

### 2.5.7 Goblin AI — Retreat & Lure
- Tier 1: Aggressive chase + melee attack.
- Tier 2+: Retreats when low HP, pathfinds toward nearest ally (lure behavior).
- Tier 3+: Moves 2 tiles per turn. Dash-attack if second step lands on player.

### 2.5.6 Skeleton AI — Ranged Arrows & Repositioning
- Tier 1: Chase + melee with A* pathfinding.
- Tier 2+: Fires arrows at 3-8 tile range with LOS. 2-turn cooldown between shots.
- Tier 3+: Repositions away if player closes within 3 tiles.

### 2.5.5 Projectile System
- Created `projectile.rs` with `Projectile` struct (position, direction, damage, symbol).
- Directional arrow symbols: `-`, `|`, `/`, `\` based on travel direction. Rendered in yellow.
- Projectile lifecycle: spawn → advance 1 tile/turn → collide with player/monster/wall → remove.
- Arrow-player hits deal damage with log message. Arrow-monster hits stop the arrow (friendly fire).

### 2.5.4 Monster Behavior State Machine
- Added `BehaviorState` enum: Idle, Chase, Attack, Ranged, Retreat, Reposition.
- Added `MonsterAction` enum: Nothing, MoveTo, MeleeAttack, FireProjectile.
- `decide_action()` dispatches to monster-specific AI functions per turn.
- Each monster type has its own AI function with tier-aware behavior.

### 2.5.3 Monster Struct Overhaul
- Added fields: `behavior`, `can_see_player`, `last_known_player_pos`, `ranged_cooldown`, `turn_parity`, `floor_tier`, `is_berserk`, `base_attack`.
- Shared helper methods: `flee_from()`, `wander()` (toward last known pos then random), `find_nearest_ally()`.

### 2.5.2 A* Pathfinding
- Implemented A* search in `map.rs` (`astar_next_step`).
- Returns the next step toward a goal, respects walls and occupied monster positions.
- 500-iteration safety limit to prevent lag on large maps.
- Replaces old single-axis movement — monsters navigate around corners.

### 2.5.1 Bresenham Line-of-Sight
- Implemented Bresenham's line algorithm in `map.rs` (`has_line_of_sight`).
- Monsters only detect the player through clear sightlines — walls block vision.
- Also used for Skeleton ranged attack eligibility.
- Added `is_corridor()` helper for Troll corridor-blocking detection.
- Added `distance()` Manhattan distance helper.

## Phase 2: It Wants to Kill You
*Completed: 2026-02-21*

### 2.5 Health Potions (Procedural Items)
- Added `Tile::Potion` variant and `!` symbol rendered in magenta.
- Potions spawn procedurally during map generation (~40% chance per room, avoiding spawn/stairs rooms).
- Walking over a potion auto-picks it up, healing the player for 7 HP (capped at max HP).
- Potions are consumed on pickup — tile reverts to floor.
- Added `Player::heal()` method with max HP clamping.
- Monsters correctly render potion tiles when moving off them.

### 2.4 Scrolling Message Log
- Implemented a persistent UI area at the bottom for game events.
- Added real-time combat feedback: "You hit the Goblin for 5 damage!", "The Troll hits you for 8 damage!".
- Log displays the last 3 messages and handles screen clearing for readability.

### 2.3 Monster AI & Turn-based Movement
- Implemented simple chasing AI: monsters move toward the player if within 10 tiles.
- Turn-based logic: monsters only move or attack after the player takes an action.
- Added collision detection for monsters to prevent them from stacking on the same tile.

### 2.2 Bump-to-Attack Combat
- Implemented standard roguelike "bump" combat (moving into a monster attacks it).
- Added damage calculation and death checks for both player and monsters.
- Monsters now disappear from the screen upon death, restoring the floor tile.

### 2.1 Entity System (Player & Monsters)
- Created `player.rs` and `monster.rs` modules.
- Defined three monster types: Goblin (weak/fast), Skeleton (medium), and Troll (strong/slow).
- Implemented HP tracking and simple stat management for all entities.
- Added random monster spawning (one per room).

## Phase 1: The World Exists
*Completed: 2026-02-21*

### 1.8 Responsive Movement Rate Limiting
- Replaced simple delays with a per-key `Instant`-based cooldown system (100ms).
- Enabled instant direction changes while preventing "runaway" speed when holding a single key.
- Eliminated movement "lag" while maintaining deliberate, roguelike-style control.

### 1.7 Spacious Dungeon Design
- Increased room size constraints (min 6x6, max 15x15) for a more open feel.
- Implemented 2-tile wide corridors (horizontal and vertical) to improve navigation.
- Scaled the number of generated rooms based on terminal area to maintain density.

### 1.6 Dynamic Scaling & Fullscreen Map
- Integrated `crossterm::terminal::size()` to detect terminal dimensions.
- Updated map generation to dynamically fill the entire terminal window.
- Reserved UI space at the bottom for floor tracking and future status bars.

### 1.5 Infinite Descent (Stairs)
- Added `Tile::Stairs` (`>`).
- Implemented floor-transition logic: stepping on stairs triggers a new map generation.
- Added a floor counter UI at the bottom of the screen.

### 1.4 Procedural Dungeon Generation
- Implemented Room-based generation algorithm.
- Added `Rect` struct for room logic and intersection checks.
- Implemented horizontal and vertical tunnel carving.
- Added `get_starting_position` to spawn player on Floor tiles.

### 1.3 Map System & Collision
- Created `map.rs` module.
- Defined `Tile` enum (Wall, Floor) and `Map` struct.
- Implemented collision detection (blocking movement through Walls).

### 1.2 Terminal Rendering & Input
- Enabled terminal raw mode.
- Implemented character rendering (`@`).
- Added keyboard event listener (Arrows to move, `Esc`/`q` to quit).

### 1.1 Project Initialization
- Created Rust project `ironveil`.
- Added `crossterm` for terminal UI and `rand` for procedural generation.
