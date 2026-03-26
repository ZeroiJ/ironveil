# Ironveil Dual-Class System Design Document

**Date:** 2026-03-26  
**Status:** Discussion / Planning Phase  
**Reference:** Grim Dawn's Dual-Mastery System

---

## 1. Current Ironveil Class System (v0.3.2)

### Structure Overview

```
Class enum: Warrior | Rogue | Mage (3 fixed)
├── base_stats: Stats (str, dex, int, con)
├── starting_ability_1: Unique per class
└── starting_ability_2: Unlocks at level 5

Player struct has:
├── class: Class
├── ability_1: Option<Ability>
├── ability_2: Option<Ability> (level 5+)
└── level: i32
```

### Key Code Locations

| Component | File | Location |
|-----------|------|----------|
| Class enum | `src/player.rs` | Line 161: `pub enum Class` |
| Class names/stats | `src/player.rs` | Line 167-197: `impl Class` |
| Ability enum | `src/player.rs` | Line 6-14: `pub enum AbilityType` |
| Ability struct | `src/player.rs` | Line 16-25: `pub struct Ability` |
| Player struct | `src/player.rs` | Line 275-305: contains `ability_1`, `ability_2` |
| Player::new() | `src/player.rs` | Line 309-343: class/ability initialization |

### Current Mechanics

- **3 Classes:** Warrior, Rogue, Mage
- **Abilities:** 2 per class (hotkeys 1 and 2)
- **Ability unlock:** ability_2 available at level 5
- **Stats:** STR, DEX, INT, CON derived from class

---

## 2. Proposed: Dual-Mastery System

### Core Concept

Following Grim Dawn's model:
1. Players select a **PRIMARY Mastery** at character creation (level 1-2)
2. At **level 10+**, players unlock a **SECOND Mastery** to combine
3. Both masteries contribute: skills, passives, and stat bonuses
4. Result = dual-class with combined abilities (e.g., "Soldier + Occultist = Witchblade")

### Benefits

- **36 possible combinations** from 6 masteries (vs 3 fixed classes)
- **Synergy hunting** — players discover skill interactions
- **Build diversity** — more replayability
- **Forgiving progression** — easy respec, experimentation encouraged

---

## 3. Pre-Requisites: Things to Do First

### Phase 1: Refactor Class → Mastery

| Task | Why | Impact |
|------|-----|--------|
| Rename `Class` enum to `Mastery` | Grim Dawn terminology alignment | Low |
| Expand from 3 to 6+ masteries | Need pool to create meaningful combos | Medium |
| Add mastery bonuses per point | Each mastery gives +HP, +Energy, stats | Medium |

**Implementation Notes:**
- Keep existing Warrior/Rogue/Mage as "archetype flavors" or fully replace
- Each mastery needs: name, stat bonuses, ability pool, progression requirements

---

### Phase 2: Ability System Overhaul

| Task | Why | Impact |
|------|-----|--------|
| Change `ability_1`, `ability_2` → `Vec<Ability>` pool | Support 4-6 abilities from 2 masteries | High |
| Add "unlock level" to abilities | Some abilities unlock at mastery level 10+ | Medium |
| Group abilities by mastery | UI needs to show "Mastery A abilities" vs "Mastery B" | Medium |

**Implementation Notes:**
- Current: `Option<Ability>` for slots 1-2
- New: `Vec<Ability>` or `[Option<Ability>; 6]` for pool
- Need: ability tier/unlock level data per ability type

---

### Phase 3: Level System Update

| Task | Why | Impact |
|------|-----|--------|
| Add `secondary_mastery: Option<Mastery>` to Player | Track dual-class state | Low |
| Level 10+ triggers second mastery selection | The Grim Dawn unlock trigger | Low |
| Add XP requirements per mastery level | Each mastery levels separately (optional) | Medium |

**Implementation Notes:**
- Check `player.level >= 10` to enable second mastery
- Save/load must handle `secondary_mastery` serialization

---

### Phase 4: UI/UX Changes

| Task | Why | Impact |
|------|-----|--------|
| Character creation: select mastery 1 | First choice at level 1 | Low |
| Level 10 screen: select mastery 2 | Second choice at level 10 | Medium |
| Ability bar: show 4-6 slots (1-4 or QWER) | More abilities = more hotkeys | Medium |
| Add mastery info to HUD | Show both masteries in player display | Low |

**Implementation Notes:**
- Current hotkeys: 1, 2 for abilities
- Expanded: need 4-6 keys (1-4 or QWER)
- Display: "Warrior: Power Attack, War Cry | Mage: Chain Lightning, Frost Nova"

---

## 4. Suggested Mastery Pool (6 to Start)

Based on Grim Dawn but simplified for Ironveil's scope:

| Mastery | Role | Primary Damage | Stat Focus | Abilities |
|---------|------|----------------|------------|-----------|
| **Soldier** | Tank/Buffer | Physical | CON, STR | Power Attack, War Cry |
| **Rogue** | Burst DPS | Pierce, Cold | DEX, STR | Shadow Step, Poison Blade |
| **Mage** | Caster | Lightning, Aether | INT, DEX | Chain Lightning, Frost Nova |
| **Berserker** | Melee DPS | Physical, Bleed | STR, CON | (TBD - new abilities) |
| **Elementalist** | AOE Mage | Fire, Cold | INT, DEX | (TBD - new abilities) |
| **Necromancer** | Pet/Curse | Vitality, Chaos | INT, CON | (TBD - new abilities) |

### Stat Bonuses Per Mastery Point

Like Grim Dawn, each mastery gives:

| Mastery | Physique | Spirit | Cunning | Health | Energy |
|---------|----------|--------|---------|--------|--------|
| Soldier | +5.0 | +1.5 | +3.5 | +28 | +10 |
| Rogue | +3.0 | +2.5 | +5.0 | +20 | +16 |
| Mage | +2.0 | +5.5 | +2.5 | +18 | +24 |
| Berserker | +5.0 | +2.0 | +2.5 | +26 | +12 |
| Elementalist | +2.0 | +5.0 | +3.0 | +18 | +22 |
| Necromancer | +3.0 | +4.0 | +2.5 | +22 | +18 |

---

## 5. All 36 Dual-Class Combinations

When 6 masteries combine, 36 unique classes emerge:

|  | Soldier | Rogue | Mage | Berserker | Elementalist | Necromancer |
|--|---------|-------|------|-----------|--------------|-------------|
| **Soldier** | — | Battlerogue | Warmage | Warlord | Battlemage | Death Knight |
| **Rogue** | Battlerogue | — | Trickster | Slayer | Shadowblade | Assassin |
| **Mage** | Warmage | Trickster | — | Spellblade | Archmage | Warlock |
| **Berserker** | Warlord | Slayer | Spellblade | — | Ravager | Destroyer |
| **Elementalist** | Battlemage | Shadowblade | Archmage | Ravager | — | Elementor |
| **Necromancer** | Death Knight | Assassin | Warlock | Destroyer | Elementor | — |

*Names are placeholder suggestions*

---

## 6. Implementation Approaches

### Option A: Incremental Migration (Recommended)

**Steps:**
1. Add `secondary_mastery: Option<Mastery>` to Player struct
2. Add level 10 second mastery selection screen
3. Abilities pool = ability_1 + ability_2 from BOTH masteries
4. Stat calculation = mastery1_bonus + mastery2_bonus
5. Update UI to show both masteries

**Pros:** Lower risk, can test early, preserves existing functionality  
**Cons:** More conditional logic during transition

---

### Option B: Full Refactor

**Steps:**
1. Remove `Class` enum entirely
2. Create new `Mastery` enum with 6+ options
3. Implement full dual-mastery system from scratch
4. Update all references throughout codebase

**Pros:** Cleaner architecture, no legacy carryover  
**Cons:** Larger change scope, higher risk

---

## 7. What Ironveil Already Has (Helpful)

| Existing System | How it helps |
|-----------------|--------------|
| `player.level: i32` | Can use for mastery unlock trigger (level >= 10) |
| Ability system | Just needs pooling to support 6 abilities |
| Stats system | Mastery bonuses can add to existing stat calculation |
| Character creation UI | Needs modification for 2-step selection |
| Save/load system | Need to serialize secondary_mastery |

---

## 8. Open Questions for Discussion

### Q1: How many masteries initially?
- **Option A:** 6 masteries (Soldier, Rogue, Mage, Berserker, Elementalist, Necromancer)
- **Option B:** Start with 4, expand later
- **Option C:** Other suggestion?

### Q2: When should second mastery unlock?
- **Level 10** (Grim Dawn style) — recommended
- **Level 15** (slower progression)
- **Other threshold?**

### Q3: Keep Warrior/Rogue/Mage as "flavor"?
- **Yes:** They become starting archetypes that auto-select first mastery
- **No:** Fully replace with mastery pool system

### Q4: New abilities for new masteries?
- Each mastery needs 4-6 abilities (2 active, 2-4 passive/toggle)
- Need to design: Berserker, Elementalist, Necromancer ability pools

### Q5: Compatibility with existing saves?
- Old saves: keep Warrior/Rogue/Mage as-is, default secondary to None
- New saves: full dual-mastery support

---

## 9. Next Steps

1. **Decide** on questions above
2. **Create** detailed implementation plan
3. **Phase 1:** Refactor Class → Mastery enum
4. **Phase 2:** Expand to 6 masteries with stat bonuses
5. **Phase 3:** Update ability system for pooling
6. **Phase 4:** Add secondary mastery at level 10
7. **Phase 5:** Update UI

---

## 10. Reference: Grim Dawn Systems

### What Makes Grim Dawn's System Work

1. **True Hybrid Builds** — Unlike fixed classes, combinations create new playstyles
2. **Synergy Hunting** — Finding skill interactions between masteries is part of the fun
3. **Forgiving Progression** — Respecs available at Spirit Guides, low friction
4. **Build Diversity** — Many viable builds for each class combination
5. **No "Wrong" Choice** — Most combinations can clear Ultimate with proper gear

### Key Grim Dawn Design Patterns

- Masteries are tools, not templates
- Stats (Physique/Spirit/Cunning) matter for different builds
- Resistances are critical in higher difficulties
- Gear matters more than "perfect" skill allocation

---

*Document created for Ironveil development planning - to be discussed and refined before implementation.*