# Shop/Merchant NPC System - Implementation Plan

## Overview
Add a gold currency system and merchant NPC that appears every 3 floors, displaying 10 items appropriate to the player's level. Players can buy and sell items.

---

## 1. Gold Currency System

### Player Changes (`src/player.rs`)
- Add `pub gold: i32` field to Player struct
- Initialize to `0` in `Player::new()`

### Gold Acquisition Sources
| Source | Amount | Notes |
|--------|--------|-------|
| Monster drops | 5-20g per kill | Scales with monster tier |
| Treasure rooms | 10-50g per room | Existing treasure rooms |

### Gold Display
- Show in HUD status line 2: `Gold: 25 | XP: 42/120 ...`

---

## 2. Monster Gold Drops (`src/monster.rs`, `src/main.rs`)

### Implementation
- Add `gold_value: i32` field to Monster struct
- Set based on monster type and floor_tier:
  - Tier 1 (floor 1-3): 5-10g
  - Tier 2 (floor 4-6): 10-15g
  - Tier 3 (floor 7+): 15-20g
- Boss monsters: 50-100g

### Code Changes
- In monster death handling (`main.rs`): `player.gold += monster.gold_value`
- Log: `"+{amount} Gold"`

---

## 3. Shop Room Generation (`src/map.rs`)

### Placement
- Every 3 floors: floors 3, 6, 9, 12, 15...
- Replace one normal room with `RoomType::Shop`
- Shop room contains:
  - Merchant NPC symbol: `$` (bright yellow)
  - Decorative shelves/counter using box characters

### RoomType Addition
- Add `Shop` variant to `RoomType` enum

---

## 4. Shop UI (`src/ui.rs` or `src/main.rs`)

### Trigger
- Player walks adjacent to merchant NPC (`$`)
- Press `E` to interact (or auto-prompt)

### Layout
```
╔══════════════════════════════════════════════════╗
║              MERCHANT'S SHOP                     ║
╠══════════════════════════════════════════════════╣
║  Your Gold: 45g                                  ║
╠══════════════════════════════════════════════════╣
║  Items for Sale:                                 ║
║  [1] Health Potion          15g  [a] Buy         ║
║  [2] Iron Longsword         30g  [b] Buy         ║
║  [3] Leather Armor          25g  [c] Buy         ║
║  ... (10 items total)                            ║
╠══════════════════════════════════════════════════╣
║  Sell Items:                                     ║
║  [4] Old Dagger               5g  [d] Sell       ║
║  [5] Tattered Cloak           3g  [e] Sell       ║
╠══════════════════════════════════════════════════╣
║  [Esc] Leave  │  [a-e] Buy/Sell                  ║
╚══════════════════════════════════════════════════╝
```

### Input Handling
- `a-e`: Buy item (if enough gold)
- `A-E`: Sell item from inventory
- `Esc`: Leave shop

---

## 5. Shop Item Generation (`src/items.rs`)

### Buy Items (10 items based on player level)
| Level Range | Item Types | Price Range |
|-------------|------------|-------------|
| 1-3 | Potions, basic weapons | 5-20g |
| 4-6 | Better weapons, armor | 15-40g |
| 7-10 | Rare items, scrolls | 25-60g |
| 10+ | Epic items, artifacts | 40-100g |

### Sell Prices
- 30-50% of item's base value
- Based on rarity and stats

### Generation Function
- `fn generate_shop_inventory(player_level: i32) -> Vec<Item>`
- Returns 10 items, mix of:
  - 3-4 potions
  - 2-3 weapons
  - 2-3 armor pieces
  - 1-2 rings/special items

---

## 6. Implementation Steps

| Step | What | Files |
|------|------|-------|
| 1 | Add `gold` field to Player | `player.rs` |
| 2 | Add gold drops to monsters | `monster.rs`, `main.rs` |
| 3 | Add `Shop` to RoomType | `map.rs` |
| 4 | Generate shop rooms every 3 floors | `map.rs` |
| 5 | Create shop item generation | `items.rs` |
| 6 | Build shop UI | `ui.rs` |
| 7 | Add shop interaction in game loop | `main.rs` |
| 8 | Update HUD to show gold | `main.rs` |
| 9 | Build and test | — |

---

## 7. File Changes Summary

| File | Changes |
|------|---------|
| `src/player.rs` | Add `gold: i32` field |
| `src/monster.rs` | Add `gold_value: i32` field, set per type |
| `src/items.rs` | Add `generate_shop_inventory()`, `get_sell_price()` |
| `src/map.rs` | Add `RoomType::Shop`, generate shop rooms |
| `src/ui.rs` | Add `shop_screen()` function |
| `src/main.rs` | Shop interaction, HUD gold display, gold drops |

---

## 8. Success Criteria

- [ ] Player starts with 0 gold
- [ ] Monsters drop 5-20g on death
- [ ] Gold shown in HUD
- [ ] Shop appears on floors 3, 6, 9, 12...
- [ ] Merchant NPC visible in shop room
- [ ] Shop UI shows 10 items based on level
- [ ] Can buy items (gold deducted, item added)
- [ ] Can sell items (gold added, item removed)
- [ ] Cannot buy if insufficient gold
- [ ] Build passes with no errors
