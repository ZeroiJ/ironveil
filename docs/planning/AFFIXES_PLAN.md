# Advanced Loot: Affixes & Exotics — Implementation Plan

## Research Summary

### Diablo 2 Affix System
- **Magic items**: 1 prefix + 1 suffix from `MagicPrefix.txt` / `MagicSuffix.txt`
- **Rare items**: 3-6 random affixes (at least 1 prefix + 1 suffix)
- **Unique items**: Fixed properties, no random affixes
- **Affix level (alvl)**: Each affix has a minimum item level requirement
- **Affix selection**: Weighted by `frequency` field, filtered by item type compatibility

### Path of Exile System
- **Prefixes/Suffixes**: Up to 3 of each on rare items (6 total mods)
- **Mod tiers**: Each affix has multiple tiers (T1-T8) based on item level
- **Exclusive pools**: Prefixes and suffixes are separate, can't have duplicate mods from same group

### Key Design Decisions for Ironveil

**Simplified approach (fits terminal roguelike):**
- Max 2 affixes per item (1 prefix + 1 suffix) — keeps display clean
- Affixes scale with floor depth (higher floors = better affix ranges)
- Exotic tier: fixed unique effects instead of random affixes
- Affixes modify: damage, defense, stats, HP, cooldowns, special effects

---

## 1. Affix Data Structure

### Prefix Enum (stat boosts)

| Prefix | Effect | Min Floor | Max Floor | Stat Range |
|--------|--------|-----------|-----------|------------|
| **Sharp** | +damage | 1 | 5 | +1 to +3 |
| **Vicious** | +damage | 4 | 10 | +2 to +5 |
| **Brutal** | +damage | 8 | 15 | +3 to +8 |
| **Sturdy** | +defense | 1 | 5 | +1 to +3 |
| **Fortified** | +defense | 4 | 10 | +2 to +5 |
| **Iron** | +defense | 8 | 15 | +3 to +8 |
| **Swift** | +DEX | 1 | 10 | +1 to +3 |
| **Mighty** | +STR | 1 | 10 | +1 to +3 |
| **Arcane** | +INT | 1 | 10 | +1 to +3 |
| **Vital** | +CON | 1 | 10 | +1 to +3 |
| **Vampiric** | Heal 1 HP per kill | 6 | 15 | 1-2 HP |
| **Hasty** | -1 ability cooldown | 8 | 15 | 1 tick |

### Suffix Enum (utility effects)

| Suffix | Effect | Min Floor | Max Floor | Stat Range |
|--------|--------|-----------|-----------|------------|
| **of Health** | +max HP | 1 | 10 | +3 to +10 |
| **of Warding** | +max HP | 6 | 15 | +5 to +15 |
| **of the Bear** | +STR | 4 | 15 | +1 to +4 |
| **of the Fox** | +DEX | 4 | 15 | +1 to +4 |
| **of the Owl** | +INT | 4 | 15 | +1 to +4 |
| **of the Ox** | +CON | 4 | 15 | +1 to +4 |
| **of Haste** | -1 ability cooldown | 10 | 15 | 1 tick |
| **of Frost** | Freeze chance on hit | 8 | 15 | 5-10% |
| **of Flame** | Burn chance on hit | 8 | 15 | 5-10% |
| **of the Leech** | Lifesteal % | 10 | 15 | 5-15% |

---

## 2. Exotic Items (Unique Tier)

Exotics are named items with fixed, game-changing effects. They replace the current artifact system with more variety.

### Weapons

| Name | Type | Effect | Trade-off |
|------|------|--------|-----------|
| **Bloodthirst** | Weapon | +50% damage | -30% max HP |
| **Soul Reaper** | Weapon | Kills heal 5 HP | -3 STR |
| **The World-Eater** | Weapon | Every 5th hit deals 3x damage | -20% attack speed (every other tick) |
| **Whisper of the Void** | Weapon | Attacks ignore armor/defense | -50% base damage |
| **Crimson Dancer** | Weapon | +2 damage per consecutive hit (resets on miss) | Resets to 0 if you take damage |
| **Godsbane** | Weapon | 10% chance to instantly kill non-boss enemies | -50% damage vs bosses |
| **The Peacemaker** | Weapon | Monsters won't aggro you unless attacked first | +100% damage taken when attacked |
| **Last Breath** | Weapon | Damage scales with missing HP (up to 3x at 1 HP) | -50% damage at full HP |

### Armor

| Name | Type | Effect | Trade-off |
|------|------|--------|-----------|
| **Phoenix Down** | Armor | Revive once per floor with 50% HP | -20% all stats |
| **Frostbite Gauntlets** | Armor | Freeze enemies on melee hit (10% chance) | -2 INT |
| **Stormcaller's Mantle** | Armor | Lightning chains to 2 extra targets | -3 CON |
| **Shadow-weave Boots** | Armor | Shadow Step has no cooldown | Costs 2 HP per use |
| **The Iron Maiden** | Armor | Reflect 50% of incoming damage back | -3 DEF, take 10% more damage |
| **Skin of the Hydra** | Armor | Regenerate 1 HP every 3 ticks | -50% potion effectiveness |
| **Aegis of the Fallen** | Armor | First hit each floor is negated | -10 max HP permanently per floor |
| **The Coward's Cloak** | Armor | +30% dodge chance | -50% damage dealt |
| **Berserker's Plate** | Armor | +50% damage when below 30% HP | -30% damage when above 70% HP |
| **Shroud of the Nameless** | Armor | Invisible to monsters for 3 ticks after killing one | -20% movement speed |

### Rings

| Name | Type | Effect | Trade-off |
|------|------|--------|-----------|
| **The Glass Cannon** | Ring | +10 INT | Max HP set to 1 |
| **Time Weaver's Ring** | Ring | All cooldowns -2 | -5 DEX |
| **Ring of the Gambler** | Ring | 25% chance to deal 4x damage | 25% chance to deal 0 damage |
| **The Martyr's Band** | Ring | Allies (if any) take 50% less damage | You take 25% more damage |
| **Ouroboros** | Ring | Abilities cost no cooldown but drain 2 HP per use | -5 CON |
| **The Hoarder's Signet** | Ring | +200% gold drops | -50% XP from kills |
| **Ring of Echoes** | Ring | Each ability hits twice | +50% cooldown on all abilities |
| **The Pacifist's Oath** | Ring | +50% XP from kills | Cannot deal critical hits |
| **Fate's Thread** | Ring | Reroll one death per run (revive at 25% HP) | -30% gold, -30% XP, -20% all stats |

---

### Exotic Drop Rates

| Source | Drop Chance |
|--------|-------------|
| Normal monster | 0.1% |
| Elite/champion monster | 0.5% |
| Boss (Goblin King, etc.) | 5% |
| Floor 10+ boss | 10% |
| Floor 15+ boss | 15% |
| Treasure room chest | 2% |

### Exotic Item Rules
- Only 1 exotic can be equipped at a time (prevents broken combos)
- Exotics cannot be sold (too valuable, shop won't buy them)
- Exotics display in **Bright Red** on the ground
- Exotic name format: `* Item Name *` (surrounded by asterisks)
- Trade-offs are always active while equipped

---

## 3. Item Generation Flow

```
Monster dies → roll rarity → select base item → apply affixes
                                    ↓
                    Common: 0 affixes (flat stats only)
                    Uncommon: 0-1 affix (50% chance)
                    Rare: 1-2 affixes
                    Epic: 2 affixes (1 prefix + 1 suffix)
                    Legendary: 2 affixes + bonus stat range (+50%)
                    Exotic: Fixed unique properties
```

### Affix Selection Algorithm
1. Roll number of affixes based on rarity
2. Roll prefix (if slots available) — weighted random from valid prefixes
3. Roll suffix (if slots available) — weighted random from valid suffixes
4. Apply affix stat ranges based on floor depth
5. Generate display name: `"[Prefix] BaseName of [Suffix]"`

---

## 4. File Changes

### `src/items.rs`
- Add `Affix` enum with all prefixes/suffixes
- Add `ExoticType` enum for unique items
- Add `prefix: Option<Affix>`, `suffix: Option<Affix>` to Item struct
- Add `exotic_type: Option<ExoticType>` to Item struct
- Add `apply_affixes(&mut self, floor: i32)` method
- Add `generate_affixed_item(base: Item, rarity: Rarity, floor: i32) -> Item`
- Update `display_name()` to include affix names
- Update `random_weapon/armor/ring` to generate affixes

### `src/player.rs`
- Update `effective_stats()` to include affix bonuses
- Add `lifesteal: i32` field (from Vampiric/Leech affixes)
- Add `cooldown_reduction: i32` field (from Hasty/Haste affixes)
- Add `freeze_chance: i32` field (from Frost affix)
- Add `burn_chance: i32` field (from Flame affix)

### `src/main.rs`
- Apply lifesteal on monster kill
- Apply freeze/burn chance on melee hit
- Apply cooldown reduction to ability cooldowns
- Handle exotic item effects (Glass Cannon HP, Shadow-weave Boots, etc.)
- Update HUD to show affix bonuses

### `src/ui.rs`
- Update shop screen to display affixed item names
- Update inventory to show affix effects in item details

---

## 5. Display Format

### Inventory
```
[a] Sharp Iron Shortsword of Health  RARE
    (+3 dmg, +8 HP)

[b] Vampiric Longsword of the Leech  EPIC
    (+4 dmg, Heal 1 HP/kill, 10% lifesteal)

[c] * The Glass Cannon *             EXOTIC
    (+10 INT, Max HP: 1)
```

### Shop
```
Sharp Iron Shortsword of Health    45g  [a] Buy
Vampiric Longsword of the Leech    80g  [b] Buy
* The Glass Cannon *              120g  [c] Buy
```

### Ground Items
- Common/Uncommon: White/Green (current)
- Rare: Cyan
- Epic: Magenta
- Legendary: Yellow
- Exotic: Bright Red (new)

---

## 6. Implementation Steps

| Step | What | Files | Effort |
|------|------|-------|--------|
| 1 | Add Affix enum with all prefixes/suffixes | `items.rs` | Low |
| 2 | Add ExoticType enum with unique effects | `items.rs` | Low |
| 3 | Add affix/exotic fields to Item struct | `items.rs` | Low |
| 4 | Implement affix application logic | `items.rs` | Medium |
| 5 | Update display_name() for affixed items | `items.rs` | Low |
| 6 | Add affix fields to Player (lifesteal, etc.) | `player.rs` | Low |
| 7 | Implement affix effects in combat | `main.rs` | Medium |
| 8 | Handle exotic item special effects | `main.rs` | Medium |
| 9 | Update shop UI for affixed items | `ui.rs` | Low |
| 10 | Update ground item rendering for exotics | `main.rs` | Low |
| 11 | Build and test | — | — |

---

## 7. Success Criteria

- [ ] Common items: no affixes, flat stats only
- [ ] Uncommon items: 50% chance of 1 affix
- [ ] Rare items: 1-2 affixes (prefix + optional suffix)
- [ ] Epic items: always 2 affixes (1 prefix + 1 suffix)
- [ ] Legendary items: 2 affixes with +50% stat ranges
- [ ] Exotic items: fixed unique effects, no random affixes
- [ ] Display names include affix text
- [ ] Affix stats apply to player effective stats
- [ ] Lifesteal heals on kill
- [ ] Freeze/burn chance triggers on melee
- [ ] Cooldown reduction reduces ability cooldowns
- [ ] Exotic trade-offs apply correctly
- [ ] Shop shows affixed items with correct prices
- [ ] Build passes with no errors

---

## 8. Design Decisions

### Why 2 affixes max?
- Terminal display is limited — longer names wrap awkwardly
- Keeps the system readable and understandable
- Diablo 2 magic items also cap at 2 affixes
- Can expand to 3+ later if needed

### Why separate prefix/suffix pools?
- Prevents duplicate effects (can't get "Sharp" twice)
- Matches Diablo/PoE conventions
- Makes item names predictable: `[Prefix] Name of [Suffix]`

### Why Exotics instead of Uniques?
- "Exotic" is more thematic for a dark dungeon crawler
- Fixed effects make them memorable and build-defining
- Trade-offs create interesting decisions (power vs risk)

### Affix scaling with floor
- Floor 1-3: basic affixes (+1 stats)
- Floor 4-6: mid-tier affixes (+2-3 stats, lifesteal)
- Floor 7+: high-tier affixes (+4-8 stats, cooldown reduction)
- Ensures affixes feel meaningful at every stage
