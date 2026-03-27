# Item Rarity & Economy — Ironveil Research

**Date:** 2026-03-26  
**Topic:** Loot Tables, Rarity Systems, Item Progression  
**Reference Games:** Diablo 4, Path of Exile, Hades, Brogue, DCSS

---

## 1. Rarity Tiers

### Standard Rarity Ladder

| Rarity | Color | Drop Rate | Purpose |
|--------|-------|-----------|---------|
| **Common** | White/Grey | 60-80% | Filler, crafting mats |
| **Uncommon** | Green | 15-30% | Early upgrades |
| **Rare** | Blue | 5-15% | Mid-game progression |
| **Epic** | Purple | 1-5% | Late-game builds |
| **Legendary** | Orange | 0.1-1% | Chase items |
| **Mythical** | Gold | 0.01-0.1% | Ultra-rare, showcase |

### Color Coding in Roguelikes

**Brogue:**
- Cyan: Scrolls
- Purple: Rings
- Gold: Weapons
- Brown: Food

**DCSS:**
- White: Items
- Green: Potions
- Blue: Scrolls
- Red: Weapons
- Yellow: Gold

**Ironveil Current:**
- Simple colored text output (no visual rarity tiers)

---

## 2. Drop Rate Formulas

### Basic Loot Roll

```rust
fn roll_loot(enemy_level: i32) -> Option<Item> {
    let roll = random_f32(); // 0.0 to 1.0
    
    // Rarity thresholds
    if roll < 0.60 { return Some(create_common(enemy_level)); }
    if roll < 0.85 { return Some(create_uncommon(enemy_level)); }
    if roll < 0.95 { return Some(create_rare(enemy_level)); }
    if roll < 0.99 { return Some(create_epic(enemy_level)); }
    if roll < 1.00 { return Some(create_legendary(enemy_level)); }
    
    None // No drop
}
```

### Floor-Based Scaling

```rust
fn get_rarity_floor_mod(floor: i32) -> f32 {
    // Higher floors = higher chance of rare+
    1.0 + (floor as f32 * 0.1)
}
```

### Boss Drop Bonuses

```rust
fn boss_loot(boss: &Boss) -> Vec<Item> {
    let base_drop_count = 3;
    let bonus_drops = boss.rarity_bonus * random_range(1, 4);
    
    // Boss guaranteed at least rare
    let guaranteed_rare = create_rare(boss.level);
    
    // Combine with random drops
    collect_random_drops(boss.level, base_drop_count + bonus_drops)
}
```

---

## 3. Loot Table Architecture

### Tiered Loot Tables

```rust
struct LootTable {
    common: Vec<ItemTemplate>,
    uncommon: Vec<ItemTemplate>,
    rare: Vec<ItemTemplate>,
    epic: Vec<ItemTemplate>,
    legendary: Vec<ItemTemplate>,
}

impl LootTable {
    fn roll(&self, floor: i32) -> Item {
        let rarity = self.roll_rarity();
        let pool = match rarity {
            Rarity::Common => &self.common,
            Rarity::Uncommon => &self.uncommon,
            Rarity::Rare => &self.rare,
            Rarity::Epic => &self.epic,
            Rarity::Legendary => &self.legendary,
        };
        
        // Select random from pool, scale stats to floor
        let template = pool.random();
        template.spawn_for_floor(floor)
    }
}
```

### Monster-Specific Loot

```rust
struct MonsterLootTable {
    // Base loot all monsters can drop
    base: LootTable,
    // Special drops unique to this monster type
    guaranteed: Vec<ItemTemplate>,
    // Rare personal items
    unique: Vec<ItemTemplate>,
}
```

---

## 4. Equipment Progression

### Damage Scaling by Floor

| Floor | Weapon Damage Range | Armor Range |
|-------|-------------------|-------------|
| 1 | 3-6 | 0-2 |
| 5 | 8-15 | 3-6 |
| 10 | 18-30 | 8-12 |
| 15 | 35-55 | 15-20 |

### Formula

```rust
fn scale_item_power(base_power: i32, floor: i32) -> i32 {
    // Exponential-ish scaling
    let growth = 1.15; // 15% per floor
    (base_power * (growth.powi(floor - 1)) as i32
}
```

### Tier Unlock by Floor

| Floor | Max Tier Available |
|-------|-------------------|
| 1 | Common |
| 3 | Uncommon |
| 6 | Rare |
| 10 | Epic |
| 13 | Legendary |

---

## 5. Economy Systems

### Gold/Currency

**Simple (Ironveil currently):**
```rust
// No gold system in current implementation
// Items have value but no currency exchange
```

**Advanced Approach:**
```rust
struct Currency {
    gold: i32,
}

fn buy_item(item: &Item, player: &mut Player) -> bool {
    if player.gold >= item.price {
        player.gold -= item.price;
        player.inventory.push(item.clone());
        return true;
    }
    false
}

fn sell_item(item: &Item) -> i32 {
    item.price / 2 // 50% buyback value
}
```

### Vendor Pricing

```rust
fn calculate_vendor_price(base_value: i32, player_level: i32) -> i32 {
    // Price scales with player level (gear expectation)
    let level_multiplier = 1.0 + (player_level as f32 * 0.1);
    (base_value * level_multiplier) as i32
}
```

---

## 6. Item Affix System (Advanced)

### Affix Tiers

| Affix Tier | Power Range | Rarity Requirement |
|-----------|-------------|-------------------|
| **Minor** | +1 to stat | Common |
| **Major** | +2 to stat | Uncommon |
| **Superior** | +3 to stat | Rare |
| **Epic** | +4 to stat | Epic |
| **Legendary** | +5 to stat | Legendary |

### Affix Generation

```rust
fn generate_affixes(item: &mut Item, rarity: Rarity) {
    let affix_count = match rarity {
        Rarity::Common => 0,
        Rarity::Uncommon => 1,
        Rarity::Rare => 2,
        Rarity::Epic => 3,
        Rarity::Legendary => 4,
    };
    
    for _ in 0..affix_count {
        let affix = roll_random_affix(rarity);
        item.affixes.push(affix);
    }
}
```

---

## 7. Pity Systems

### Why Pity?

Players get frustrated with bad RNG streaks. Pity systems guarantee eventual rewards.

### Soft Pity (Increasing Probability)

```rust
fn soft_pity(base_chance: f32, streak: i32, max_bonus: f32) -> f32 {
    let bonus = (streak * 0.002).min(max_bonus);
    base_chance + bonus
}
```

### Hard Pity (Guaranteed Drop)

```rust
fn roll_with_pity(chances: &Chances, pity_counter: &mut i32) -> Item {
    *pity_counter += 1;
    
    if *pity_counter >= 50 { // Guaranteed after 50 rolls
        *pity_counter = 0;
        return create_legendary_item();
    }
    
    // Normal roll
    normal_roll(chances)
}
```

---

## 8. Ironveil Recommendations

### Phase 1: Basic Rarity (Low Effort)

1. Add rarity enum to items (Common/Uncommon/Rare/Epic/Legendary)
2. Color-code item display
3. Add floor-based drop scaling

### Phase 2: Item Pool (Medium Effort)

1. Expand item templates per rarity tier
2. Add item level scaling with floor
3. Add vendor buying/selling

### Phase 3: Economy (Medium Effort)

1. Gold currency system
2. Item value pricing
3. Gold drops from enemies

### Phase 4: Advanced (High Effort)

1. Affix system with random rolls
2. Pity timers for rare items
3. Crafting/upgrade system

---

## 9. Summary: Loot Economy Design

| Element | Current Ironveil | Recommended |
|---------|-----------------|---------------|
| Rarity Tiers | None | 5 tiers (Common→Legendary) |
| Color Coding | None | Add to display |
| Drop Rates | Single table | Floor-scaled tables |
| Item Scaling | None | Floor-based power |
| Currency | None | Gold system |
| Affixes | None | 3-4 per item |
| Pity | None | Soft pity on legendaries |

---

## References

- Diablo 4 Loot Guide (Maxroll)
- Path of Exile Loot System
- Game Wisdom: Loot Table Design
- PulseGeek: Loot Drop Rates
- ARPG Loot Systems (Game Developer)

---

*Research document for Ironveil item economy.*