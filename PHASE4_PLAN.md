# Phase 4: The World Has Rules

## Overview
Class abilities, monster status effects, 4 new monster types, XP & leveling, boss monsters, and HUD updates.

---

## 4.1 — Class Abilities System

Each class gets 1 ability at start, 2nd ability unlocks at level 5. Activated with number keys `1` and `2`. Cooldowns measured in monster ticks. No mana system.

### Warrior
- **Ability 1 — Power Attack** (from start): Press `1`, next melee bump does 2x damage. Buff lasts until hit lands or 5 ticks pass. Cooldown: 8 ticks after use. Player `@` flashes bright red when active.
- **Ability 2 — War Cry** (level 5): Press `2`, all monsters within 4 tiles stunned for 2 ticks (skip actions). Cooldown: 12 ticks. Stunned monsters render dark grey.

### Rogue
- **Ability 1 — Shadow Step** (from start): Press `1`, then arrow key for direction. Teleport up to 4 tiles in that direction, passing through monsters, stopping at walls. If any monster is adjacent to landing tile, next melee within 2 ticks does 2x damage ("shadow strike" buff). Cooldown: 7 ticks.
- **Ability 2 — Poison Blade** (level 5): Press `2`, next 3 melee hits apply poison (1 damage/tick for 3 ticks). Cooldown: 10 ticks after charges used or 15 ticks expire. Poisoned monsters get green tint.

### Mage
- **Ability 1 — Chain Lightning** (from start): Press `1`, then arrow key for direction. Lightning fires up to 6 tiles in that direction. Hits first monster for `3 + INT_modifier` damage, chains to nearest monster within 3 tiles for `2 + INT_modifier`, chains once more for `1 + INT_modifier`. Cooldown: 6 ticks.
- **Ability 2 — Frost Nova** (level 5): Press `2`, all monsters within 3 tiles frozen 2 ticks AND take `2 + INT_modifier` damage. Cooldown: 10 ticks. Frozen monsters render cyan.

### Ability Data Structure
```rust
struct Ability {
    name: String,
    cooldown_max: i32,
    cooldown_remaining: i32,
    is_active: bool,        // buff currently active
    charges: i32,           // for Poison Blade
    buff_ticks_remaining: i32,
}
```
Added to Player: `ability_1: Option<Ability>`, `ability_2: Option<Ability>`.

Cooldowns tick down in monster tick section. Cooldowns do NOT tick while inventory is open.

### Directional Input
Shadow Step and Chain Lightning need 2-step input (press `1`, then arrow). Add `pending_ability_direction: Option<u8>` to track when waiting for a direction key. Must not block monster ticks while waiting.

---

## 4.2 — Monster Status Effects

| Effect | Duration | Behavior | Visual |
|--------|----------|----------|--------|
| Stunned | 2 ticks | Skip action | Dark grey |
| Frozen | 2 ticks | Skip action | Cyan |
| Poisoned | 3 ticks | 1 dmg/tick | Green tint / `*` next to monster |

Added to Monster: `stun_ticks: i32`, `poison_ticks: i32`, `status_effect: Option<StatusEffect>`.

In `process_monsters()`: if `stun_ticks > 0` -> decrement, skip. If `poison_ticks > 0` -> decrement, take 1 damage.

---

## 4.3 — Four New Monster Types

### Bat Swarm `B` (Dark Red) — Floor 3+
- Charges in straight line (cardinal direction closest to player) at 2 tiles/tick
- Charge damage: 6. After hitting wall: stunned 1 tick
- HP: 8, melee attack: 3
- Tier 2 (floor 7+): speed 3 tiles/tick, HP 12

### Spider `S` (Yellow) — Floor 4+
- Lays web tiles every 4 ticks on tiles it moves off
- Retreats when seeing player, laying webs in path
- Webs rendered as `:` white. Player walks on web -> loses next move input ("stuck in web!"), web consumed
- Monsters unaffected by webs. Web storage: `HashMap<(usize,usize), bool>`
- HP: 8, attack: 3
- Tier 2 (floor 7+): webs every 2 ticks, spits webs at player from 4 tiles

### Wraith `W` (Dark Grey) — Floor 5+
- Moves through walls. Invisible when inside wall tile, visible on floor tiles
- Attacks from wall (adjacent through wall), then phases back into nearest wall
- Can only be damaged when on Floor tile. HP: 6, attack: 5
- Tier 1 (floor 5-6): attacks then retreats, slow (every other tick)
- Tier 2 (floor 7+): stays on floor 1 tick before retreat, acts every tick

### Necromancer `N` (Magenta) — Floor 6+
- Keeps distance (5-8 tiles from player, like Skeleton reposition)
- Every 6 ticks: resurrects random dead monster at 50% HP at its death position
- Max 3 resurrections per Necromancer. Weak melee (2 damage)
- HP: 12
- Tier 2 (floor 10+): resurrect every 4 ticks, max 5, 75% HP
- Dead monsters need `death_position` field. Resurrected monsters re-enter monster vec as alive.

### Monster XP Values
| Monster | XP |
|---------|----|
| Goblin | 10 |
| Bat Swarm | 15 |
| Spider | 15 |
| Skeleton | 20 |
| Wraith | 30 |
| Troll | 40 |
| Necromancer | 50 |
| Boss | 100 |

---

## 4.4 — XP & Auto-Leveling

### XP Sources
1. Monster kills (XP per type, see above). Resurrected monsters give XP again.
2. Floor descent: `current_floor * 20` XP when stepping on stairs.

### Level Thresholds
Formula: `threshold(n) = threshold(n-1) + 20 + (n-1) * 40`

| Level | XP Required |
|-------|-------------|
| 2 | 50 |
| 3 | 120 |
| 4 | 220 |
| 5 | 360 |
| 6 | 540 |
| 7 | 780 |
| 8 | 1080 |
| 9 | 1440 |
| 10 | 1900 |

### Level Up Rewards
- +3 max HP (heal 3 immediately)
- Class stat boosts: Warrior +1 STR/+1 CON, Rogue +1 DEX/+1 STR, Mage +1 INT/+1 DEX
- Levels 3 and 7: all ability cooldowns reduced by 1 tick (permanent)
- Level 5: unlock 2nd ability
- Log message: "You reach level 3! STR +1, CON +1, Max HP +3"

### Player Fields
```rust
xp: i32,
level: i32,
xp_to_next_level: i32,
```

---

## 4.5 — Boss Monsters

Every 5th floor (5, 10, 15, 20...), boss spawns in the stairs room. Stairs blocked while boss alive ("The way is blocked by the Goblin King!").

### Floor 5 — Goblin King `G` (Bright Green)
- HP: 60, Attack: 6
- Every 5 ticks: summons a Goblin in stairs room (max 3 summoned alive)
- Below 30% HP: attack 10, summon every 3 ticks
- Drop: guaranteed tier 2 weapon or armor

### Floor 10 — Bone Dragon `D` (Bright White)
- HP: 120, Attack: 10
- Every 4 ticks: breath cone (3 deep, widening) in player direction, 6 damage
- Tail swipe: 8 damage if player behind and adjacent
- Drop: guaranteed tier 3 weapon or armor

### Floor 15 — Shadow Lord `L` (Bright Magenta)
- HP: 160, Attack: 12
- Every 3 ticks: teleports to random tile within 6 of player, leaves shadow pool (3 dmg, lasts 5 ticks)
- Every 6 ticks: dark pulse AoE 5 damage within 3 tiles
- Drop: guaranteed tier 3 weapon + ring

### Floor 20+
Repeat cycle with +50% HP, +25% attack from base.

### Boss Fields
```rust
is_boss: bool,
boss_type: Option<BossType>,
boss_tick_counter: i32,
boss_summon_count: i32,
```

---

## 4.6 — HUD Updates

```
Floor: 3 | HP: 27/30 | Lv.4 Warrior | Iron Shortsword (+2 dmg)
STR:16 DEX:10 INT:8 CON:16 | Def:1 | XP: 180/220 | Tab:Inventory
[1] Power Attack: READY                [message log line 1]
                                        [message log line 2]
                                        [message log line 3]
```

---

## Implementation Order

| Step | What | Files |
|------|------|-------|
| 1 | ~~Write PHASE4_PLAN.md~~ | `PHASE4_PLAN.md` |
| 2 | Ability data structures + cooldown tick system | `player.rs`, `main.rs` |
| 3 | Warrior Power Attack (simplest ability, tests system) | `player.rs`, `main.rs` |
| 4 | Monster status effects (stun/poison) in process_monsters | `monster.rs`, `main.rs` |
| 5 | Rogue Shadow Step (directional input + teleport) | `player.rs`, `main.rs` |
| 6 | Mage Chain Lightning (directional + chain logic) | `player.rs`, `main.rs` |
| 7 | XP system + auto-leveling + stat growth | `player.rs`, `main.rs` |
| 8 | Level 5 second abilities (War Cry, Poison Blade, Frost Nova) | `player.rs`, `main.rs` |
| 9 | Bat Swarm monster (line charge AI) | `monster.rs` |
| 10 | Spider monster (web traps, web tile system) | `monster.rs`, `map.rs`, `main.rs` |
| 11 | Wraith monster (wall-phase AI) | `monster.rs`, `main.rs` |
| 12 | Necromancer monster (resurrect dead) | `monster.rs`, `main.rs` |
| 13 | Boss system framework (is_boss, stairs blocking) | `monster.rs`, `map.rs`, `main.rs` |
| 14 | Goblin King boss (floor 5) | `monster.rs` |
| 15 | Bone Dragon boss (floor 10) | `monster.rs` |
| 16 | Shadow Lord boss (floor 15) | `monster.rs` |
| 17 | Boss loot drops | `items.rs`, `main.rs` |
| 18 | HUD updates (level, XP, ability cooldowns) | `main.rs` |
| 19 | Update CHANGELOG.md | `CHANGELOG.md` |
| 20 | Commit and push | — |
