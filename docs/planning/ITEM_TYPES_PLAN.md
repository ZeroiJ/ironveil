# More Item Types - Implementation Plan

## Overview
Expand Ironveil's item system with new consumable categories: Scrolls, Wands, and Potion varieties.

---

## Current System

| Category | Types | Notes |
|----------|-------|-------|
| Weapons | ~8 types | Dagger → Greatsword tier progression |
| Armor | ~6 types | Leather → Plate tier progression |
| Rings | ~4 types | Various stat bonuses |
| Potions | 1 type | Health Potion only |

**Item struct fields:**
- `damage_bonus`, `defense_bonus` - combat stats
- `stat_bonus_type/value` - STR/DEX/INT/CON
- `heal_amount` - for potions
- `rarity` - Common/Rare/Epic/Legendary
- `artifact_effect` - special effects

---

## Proposed Additions

### 1. Scrolls (One-time use)
**Purpose:** Give players situational powerful effects without permanent inventory investment.

**Types:**
| Name | Effect | Cost | Floor |
|------|--------|------|-------|
| Scroll of Fireball | 8 + INT AoE damage (radius 2) | 50g | 3+ |
| Scroll of Teleport | Teleport to stairs or random safe spot | 30g | 2+ |
| Scroll of Identify | Reveal item stats/rarity | 20g | 1+ |
| Scroll of Phase Door | Teleport 3-5 tiles in direction | 40g | 4+ |
| Scroll of Swiftness | Double speed for 5 moves | 60g | 6+ |
| Scroll of Remove Curse | Remove cursed equipment | 45g | 5+ |
| Scroll of Blessing | +2 to all stats for 10 ticks | 75g | 8+ |

**Implementation:**
- New `ItemType::Scroll` 
- New field: `scroll_effect: ScrollEffect` enum
- One-time use: consumed on use
- Found in treasure rooms, shops, rare monster drops

### 2. Wands (Charged, reusable)
**Purpose:** Repeatable utility with limited charges, bridges gap between potions and artifacts.

**Types:**
| Name | Charges | Effect | Cost | Floor |
|------|---------|--------|------|-------|
| Wand of Magic Missiles | 10 | 3 + INT damage, ranged | 80g | 4+ |
| Wand of Healing | 5 | Heal 5 HP to self | 100g | 3+ |
| Wand of Slow | 8 | Slow monster (50% speed) for 3 ticks | 90g | 5+ |
| Wand of Fear | 6 | Monsters flee for 3 tiles for 2 ticks | 110g | 7+ |
| Wand of Light | 15 | Reveal area for 10 ticks | 40g | 2+ |
| Wand of Blink | 8 | Teleport 3 tiles (like ShadowStep) | 120g | 6+ |

**Implementation:**
- New `ItemType::Wand`
- New field: `charges: i32`, `wand_effect: WandEffect` enum
- Recharges: 1 charge per floor, or at shrine
- Wands don't drop from monsters (shop exclusive or high floor treasure)

### 3. Extended Potions
**Purpose:** More tactical variety beyond just healing.

**Types:**
| Name | Effect | Cost | Floor |
|------|--------|------|-------|
| Potion of Greater Healing | Heal 15 HP | 30g | 3+ |
| Potion of Strength | +2 STR for 10 ticks | 25g | 4+ |
| Potion of Dexterity | +2 DEX for 10 ticks | 25g | 4+ |
| Potion of Intellect | +2 INT for 10 ticks | 25g | 4+ |
| Potion of Haste | Double speed for 3 moves | 50g | 6+ |
| Potion of Invisibility | Monsters ignore you for 5 ticks | 75g | 8+ |
| Potion of Poison Resistance | Immunity to poison for 10 ticks | 35g | 5+ |
| Potion of Fire Resistance | Immunity to fire for 10 ticks | 35g | 5+ |

**Implementation:**
- Extend existing `heal_amount` or add new fields
- Add `buff_duration`, `buff_type` fields
- Can stack in inventory (like current potions)

---

## Implementation Steps

### Phase 1: Infrastructure
1. Add `Scroll`, `Wand` to `ItemType` enum
2. Add `ScrollEffect` and `WandEffect` enums
3. Add `charges` and `scroll_effect` fields to `Item` struct
4. Add helper functions to create scroll/wand items

### Phase 2: Loot Generation
1. Update `random_scroll(floor)` function
2. Update `random_wand(floor)` function  
3. Add scrolls/wands to treasure room generation
4. Add to shop inventory (see Shop/Merchant plan)

### Phase 3: UI/Display
1. Update inventory rendering for new types
2. Add new symbols: `?` for scroll, `~` for wand
3. Display charges remaining in inventory

### Phase 4: Usage Mechanics
1. Add scroll/wand to inventory key bindings (a-j)
2. Implement scroll effect when used
3. Implement wand effect when used
4. Track charges, remove when depleted

---

## File Changes

| File | Changes |
|------|---------|
| `items.rs` | Add ItemType variants, new functions, Item struct fields |
| `ui.rs` | Update inventory display for new types |
| `main.rs` | Add scroll/wand to inventory key handlers |

---

## Compatibility Notes

- Existing saves will need migration OR new fields default to empty/0
- Rings already exist - just need more variety (proposal: separate ring types)
- Artifacts already have special effects - scrolls/wands are consumable versions

---

## Priority

| Feature | Effort | Impact |
|---------|--------|--------|
| Extended Potions | Low | Medium |
| Scrolls | Medium | High |
| Wands | Medium | High |

Start with **Extended Potions** (easiest, builds on existing system), then **Scrolls**, then **Wands**.