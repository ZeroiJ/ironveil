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
