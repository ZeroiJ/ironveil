# Items Catalog

## Weapons

| Tier | Name | Damage Bonus | Spawn Floors |
|------|------|--------------|--------------|
| 1 | Dagger | +1 | 1-3 |
| 1 | Shortsword | +2 | 1-3 |
| 2 | Longsword | +3 | 4-6 |
| 2 | Battle Axe | +4 | 4-6 |
| 3 | Greataxe | +5 | 7+ |
| 3 | War Hammer | +6 | 7+ |

**Damage Formula:** `1 + weapon_bonus + (STR-10)/2`

---

## Armor

| Tier | Name | Defense Bonus | Spawn Floors |
|------|------|---------------|--------------|
| 1 | Leather Armor | +1 | 1-3 |
| 1 | Hide Armor | +1 | 1-3 |
| 2 | Chainmail | +2 | 4-6 |
| 2 | Scale Mail | +3 | 4-6 |
| 3 | Plate Armor | +4 | 7+ |
| 3 | Full Plate | +5 | 7+ |

**Damage Reduction:** `incoming_damage - armor_def` (minimum 1)

---

## Rings

| Tier | Name | Stat Bonus | Spawn Floors |
|------|------|------------|--------------|
| 1 | Ring of Strength | +1 STR | 1-3 |
| 1 | Ring of Agility | +1 DEX | 1-3 |
| 1 | Ring of Intellect | +1 INT | 1-3 |
| 1 | Ring of Vitality | +1 CON | 1-3 |
| 2 | Ring of Strength | +2 STR | 4-6 |
| 2 | Ring of Agility | +2 DEX | 4-6 |
| 2 | Ring of Intellect | +2 INT | 4-6 |
| 2 | Ring of Vitality | +2 CON | 4-6 |
| 3 | Ring of Strength | +3 STR | 7+ |
| 3 | Ring of Agility | +3 DEX | 7+ |
| 3 | Ring of Intellect | +3 INT | 7+ |
| 3 | Ring of Vitality | +3 CON | 7+ |

---

## Potions

| Name | Heal Amount | Spawn Floors | Notes |
|------|-------------|--------------|-------|
| Minor Healing Potion | 5 | 1-3 | Common |
| Healing Potion | 10 | 4-6 | Standard |
| Greater Healing Potion | 15 | 7+ | Rare |

**Heal Formula:** `base_heal + (INT-10)/2` (rounded up)

---

## Item Spawning

| Type | Ground Spawn Chance | Monster Drop Chance |
|------|---------------------|---------------------|
| Potion | 40% | 50% |
| Weapon | 30% | 25% |
| Armor | 20% | 15% |
| Ring | 10% | 10% |

---

## Item Colors

| Type | Color | Symbol |
|------|-------|--------|
| Weapon | Cyan | `/` |
| Armor | Dark Yellow | `[` |
| Ring | Yellow | `=` |
| Potion | Magenta | `!` |

---

## Inventory

- Capacity: 10 items
- Tab to open inventory screen
- `a-j` to use/equip items
- `A-J` to drop items
- Equipping swaps old gear back to inventory

---

## Affix System

Items can roll prefixes and suffixes based on rarity and floor depth.

### Rarity & Affix Count

| Rarity | Affixes | Example |
|--------|---------|---------|
| Common | 0 | `Dagger` |
| Uncommon | 0-1 (50%) | `Sharp Dagger` |
| Rare | 1-2 | `Sharp Dagger of Health` |
| Epic | 2 (prefix + suffix) | `Vampiric Longsword of the Leech` |
| Legendary | 2 (+50% stat ranges) | `Brutal Greataxe of Flame` |
| Exotic | Fixed unique effects | `* The Glass Cannon *` |

### Prefixes

| Prefix | Type | Effect | Min Floor |
|--------|------|--------|-----------|
| Sharp | Weapon | +damage | 1 |
| Vicious | Weapon | +damage | 4 |
| Brutal | Weapon | +damage | 8 |
| Vampiric | Weapon | Lifesteal on kill | 6 |
| Hasty | Weapon | -1 cooldown | 8 |
| Sturdy | Armor | +defense | 1 |
| Fortified | Armor | +defense | 4 |
| Iron | Armor | +defense | 8 |
| Swift | Armor/Ring | +DEX, -1 cooldown | 1 |
| Mighty | Any | +STR | 1 |
| Arcane | Any | +INT | 1 |
| Vital | Any | +CON | 1 |

### Suffixes

| Suffix | Type | Effect | Min Floor |
|--------|------|--------|-----------|
| of Health | Any | +max HP | 1 |
| of Warding | Any | +max HP | 6 |
| of the Bear | Armor/Ring | +STR | 4 |
| of the Fox | Armor/Ring | +DEX | 4 |
| of the Owl | Armor/Ring | +INT | 4 |
| of the Ox | Armor/Ring | +CON | 4 |
| of Haste | Weapon | -1 cooldown | 10 |
| of Frost | Weapon | Freeze chance on hit | 8 |
| of Flame | Weapon | Burn chance on hit | 8 |
| of the Leech | Weapon | Lifesteal % | 10 |

---

## Exotic Items

Exotics are unique named items with game-changing effects and trade-offs. They drop at 0.1% from normal monsters (floor 6+), 5% from bosses. Only 1 exotic can be equipped at a time. Exotics render in **Red** on the ground.

### Exotic Weapons

| Name | Effect | Trade-off |
|------|--------|-----------|
| **Bloodthirst** | +50% damage | -30% max HP |
| **Soul Reaper** | Kills heal 5 HP | -3 STR |
| **The World-Eater** | Every 5th hit 3x damage | -20% attack speed |
| **Whisper of the Void** | Attacks ignore armor | -50% base damage |
| **Crimson Dancer** | +2 dmg per consecutive hit | Resets on damage taken |
| **Godsbane** | 10% instant kill on non-bosses | -50% vs bosses |
| **The Peacemaker** | Monsters don't aggro unless attacked | +100% damage taken |
| **Last Breath** | Up to 3x damage at 1 HP | -50% damage at full HP |

### Exotic Armor

| Name | Effect | Trade-off |
|------|--------|-----------|
| **Phoenix Down** | Revive once/floor at 50% HP | -20% all stats |
| **Frostbite Gauntlets** | 10% freeze on melee hit | -2 INT |
| **Stormcaller's Mantle** | Lightning chains +2 targets | -3 CON |
| **Shadow-weave Boots** | No Shadow Step cooldown | Costs 2 HP per use |
| **The Iron Maiden** | Reflect 50% incoming damage | -3 DEF, +10% damage taken |
| **Skin of the Hydra** | Regen 1 HP every 3 ticks | -50% potion effectiveness |
| **Aegis of the Fallen** | First hit each floor negated | -10 max HP per floor |
| **The Coward's Cloak** | +30% dodge chance | -50% damage dealt |
| **Berserker's Plate** | +50% damage below 30% HP | -30% damage above 70% HP |
| **Shroud of the Nameless** | Invisible 3 ticks after kill | -20% movement speed |

### Exotic Rings

| Name | Effect | Trade-off |
|------|--------|-----------|
| **The Glass Cannon** | +10 INT | Max HP set to 1 |
| **Time Weaver's Ring** | All cooldowns -2 | -5 DEX |
| **Ring of the Gambler** | 25% chance 4x damage | 25% chance 0 damage |
| **The Martyr's Band** | Allies take 50% less damage | You take 25% more damage |
| **Ouroboros** | No ability cooldowns | Drains 2 HP per ability use |
| **The Hoarder's Signet** | +200% gold drops | -50% XP from kills |
| **Ring of Echoes** | Abilities hit twice | +50% cooldowns |
| **The Pacifist's Oath** | +50% XP from kills | Cannot deal critical hits |
| **Fate's Thread** | Reroll one death per run | -30% gold/XP, -20% all stats |
