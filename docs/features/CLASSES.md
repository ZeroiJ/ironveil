# Character Classes

## Warrior

**Color:** Red `@`

**Base Stats:**
- STR: 14
- DEX: 10
- INT: 8
- CON: 14
- HP: 28

**Starting Equipment:**
- Iron Shortsword (+2 dmg)
- Leather Armor (+1 def)

**Abilities:**

### Power Attack (Unlock: Lv1)
- **Key:** `1`
- **Effect:** Next melee attack deals 2x damage
- **Cooldown:** 6 ticks (reduced at Lv3, Lv5, Lv7, Lv10, Lv15)
- **Duration:** Until next attack

### War Cry (Unlock: Lv5)
- **Key:** `2`
- **Effect:** Stuns all monsters within 4 tiles for 2 ticks
- **Cooldown:** 8 ticks
- **Range:** 4 tiles (AoE)

### Shield Bash (Unlock: Lv10)
- **Key:** `3`
- **Effect:** Stuns nearest monster within 2 tiles for 2 turns
- **Cooldown:** 5 ticks
- **Range:** 2 tiles

### Battle Cry (Unlock: Lv15)
- **Key:** `4`
- **Effect:** Reduces attack of all enemies within 4 tiles by 2
- **Cooldown:** 12 ticks
- **Range:** 4 tiles (AoE)

### Earthquake (Ultimate, Unlock: Lv20)
- **Key:** `5`
- **Effect:** Massive AoE damage (8 + INT) + stun for 2 turns to all enemies within 5 tiles
- **Cooldown:** 20 ticks
- **Range:** 5 tiles (AoE)

**Playstyle:** Tank. High HP and armor. Close-range combat specialist. Power Attack for burst damage, War Cry for crowd control.

---

## Rogue

**Color:** Green `@`

**Base Stats:**
- STR: 10
- DEX: 16
- INT: 10
- CON: 10
- HP: 20

**Starting Equipment:**
- Twin Daggers (+1 dmg)

**Abilities:**

### Shadow Step (Unlock: Lv1)
- **Key:** `1` → then arrow key for direction
- **Effect:** Teleport up to 4 tiles in chosen direction. If monster adjacent at landing, gain shadow strike buff
- **Cooldown:** 5 ticks (reduced at Lv3, Lv5, Lv7, Lv10, Lv15)
- **Range:** 4 tiles (directional)

### Poison Blade (Unlock: Lv5)
- **Key:** `2`
- **Effect:** Next 3 melee hits apply poison (3 ticks of damage)
- **Cooldown:** 7 ticks
- **Charges:** 3 hits

### Backstab (Unlock: Lv10)
- **Key:** `3` → then arrow key for direction
- **Effect:** Attack in direction dealing 4 + INT damage
- **Cooldown:** 6 ticks
- **Range:** Directional

### Fan of Knives (Unlock: Lv15)
- **Key:** `4`
- **Effect:** AoE damage to all enemies within 4 tiles
- **Cooldown:** 8 ticks
- **Range:** 4 tiles (AoE)

### Assassinate (Ultimate, Unlock: Lv20)
- **Key:** `5` → then arrow key for direction
- **Effect:** High damage (8 + INT), 2x if target below 30% HP
- **Cooldown:** 18 ticks
- **Range:** Directional

**Playstyle:** Mobile. High dodge chance. Hit-and-run tactics. Shadow Step for positioning, Poison Blade for sustained damage.

---

## Mage

**Color:** Blue `@`

**Base Stats:**
- STR: 8
- DEX: 10
- INT: 14
- CON: 10
- HP: 20

**Starting Equipment:**
- Wooden Staff (+1 dmg)
- Ring of Intellect (+2 INT)

**Abilities:**

### Chain Lightning (Unlock: Lv1)
- **Key:** `1` → then arrow key for direction
- **Effect:** Fires 6 tiles in direction. First monster hit takes 3+INT damage. Chains to 2 nearby monsters for decreasing damage
- **Cooldown:** 6 ticks (reduced at Lv3, Lv5, Lv7, Lv10, Lv15)
- **Range:** 6 tiles (directional + chain)

### Frost Nova (Unlock: Lv5)
- **Key:** `2`
- **Effect:** Freezes all monsters within 3 tiles for 2 ticks. Deals INT-based damage
- **Cooldown:** 8 ticks
- **Range:** 3 tiles (AoE)

### Arcane Missiles (Unlock: Lv10)
- **Key:** `3`
- **Effect:** Fires 3 homing missiles at up to 3 nearest enemies (range 6 tiles)
- **Cooldown:** 5 ticks
- **Range:** 6 tiles (homing)

### Mana Shield (Unlock: Lv15)
- **Key:** `4`
- **Effect:** Absorbs next incoming damage hit. Lasts 5 turns or until triggered
- **Cooldown:** 12 ticks

### Meteor (Ultimate, Unlock: Lv20)
- **Key:** `5`
- **Effect:** Massive damage (10 + INT) to target + splash damage (radius 2)
- **Cooldown:** 20 ticks
- **Range:** 8 tiles

**Playstyle:** Ranged caster. High damage, low HP. Use abilities to control crowds. Chain Lightning for groups, Frost Nova for emergencies.

---

## Stat Effects

| Stat | Effect |
|------|--------|
| STR | Melee damage: `(STR-10)/2` bonus |
| DEX | Dodge chance: `(DEX-10)*3`% |
| INT | Potion healing: `+1 per 2 INT above 10` |
| CON | Max HP: `20 + (CON-10)` |

## Level Progression

| Level | XP Required | Rewards |
|-------|-------------|---------|
| 1→2 | 50 | +3 HP, +1 primary stat |
| 2→3 | 120 | +3 HP, cooldown reduction |
| 3→4 | 220 | +3 HP, +1 primary stat |
| 4→5 | 360 | +3 HP, **Ability 2 unlocks** |
| 5→6 | 540 | +3 HP, +1 primary stat |
| 6→7 | 780 | +3 HP, cooldown reduction |
| 7→8 | 1080 | +3 HP, +1 primary stat |
| 8→9 | 1440 | +3 HP |
| 9→10 | 1900 | +3 HP, +1 primary stat |
| 10→11 | 2460 | +3 HP, **Ability 3 unlocks** |
| 11→12 | 3120 | +3 HP, +1 primary stat |
| 12→13 | 3900 | +3 HP |
| 13→14 | 4800 | +3 HP, +1 primary stat |
| 14→15 | 5820 | +3 HP, **Ability 4 unlocks** |
| 15→16 | 6960 | +3 HP, cooldown reduction |
| 16→17 | 8220 | +3 HP, +1 primary stat |
| 17→18 | 9600 | +3 HP |
| 18→19 | 11100 | +3 HP, +1 primary stat |
| 19→20 | 12720 | +3 HP, **Ability 5 (Ultimate) unlocks** |
