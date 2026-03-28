# Ironveil Skill System Design Research

**Date:** 2026-03-27  
**Topic:** Skills & Abilities System Design  
**Reference Games:** Hades, Diablo 4, Path of Exile, Grim Dawn, Slay the Spire, Monster Train

---

## 1. Design Goals

Based on dual-class research and user requirements:
- **4 Active Skills** + **1 Ultimate** per build
- Skills should feel unique per mastery archetype
- Synergy potential between masteries (future dual-class)
- Clear distinction between active/passive/ultimate

---

## 2. Skill Categories

### Active Skills (4 slots)
- Standard abilities with cooldowns
- Used in combat with hotkeys (1, 2, 3, 4)
- Cooldown range: 4-10 ticks

### Ultimate Skill (1 slot)
- More powerful, longer cooldown
- Unlocked at mastery level 5+
- Visual distinction in UI
- Cooldown: 15-20 ticks

### Passive Skills
- Always active, no cooldown
- Provide stat bonuses or auto-triggered effects
- 1-2 passives per mastery (future expansion)

---

## 3. Skill Design by Mastery

### SOLDIER — Tank/Buffer Mastery

**Role:** Defensive utility, group buffs, sustained damage

| Skill | Type | Cooldown | Description |
|-------|------|----------|-------------|
| **Power Strike** | Active | 6 | Heavy melee hit, 2x damage, knocks back adjacent enemies |
| **War Cry** | Active | 8 | Stuns all enemies within 4 tiles for 2 ticks |
| **Shield Bash** | Active | 5 | Melee attack that also applies 1-tick stun |
| **Battle Cry** | Active | 10 | Buff: All allies (including player) +3 attack for 4 ticks |
| **EARTHQUAKE** | Ultimate | 20 | Massive AOE: All enemies within 6 tiles take 15 damage, stun for 3 ticks |

**Passive Options (future):**
- Fortify: +10% defense while above 50% HP
- Resolve: +5% damage per enemy within 3 tiles

---

### ROGUE — Burst DPS Mastery

**Role:** Critical hits, mobility, poison

| Skill | Type | Cooldown | Description |
|-------|------|----------|-------------|
| **Shadow Step** | Active | 5 | Teleport up to 4 tiles in direction, next melee is guaranteed crit |
| **Poison Blade** | Active | 7 | Next 3 melee attacks apply poison (3 ticks, 1 damage/tick) |
| **Backstab** | Active | 6 | Attack from behind deals 2x damage |
| **Fan of Knives** | Active | 8 | Fires 5 projectiles in arc, each hits for DEX-based damage |
| **ASSASSINATE** | Ultimate | 18 | If enemy below 25% HP: instant kill. Else: 10x normal damage |

**Passive Options (future):**
- Deadly Precision: +5% base crit chance
- Poison Mastery: Poison deals +50% damage

---

### MAGE — Caster Mastery

**Role:** Ranged damage, crowd control

| Skill | Type | Cooldown | Description |
|-------|------|----------|-------------|
| **Chain Lightning** | Active | 6 | Fires up to 6 tiles, hits first enemy for 3+INT, chains to 2 more (2+INT, 1+INT) |
| **Frost Nova** | Active | 8 | Freezes all enemies within 3 tiles for 2 ticks + INT-based damage |
| **Arcane Missiles** | Active | 5 | Fires 3 homing missiles at nearest enemy |
| **Mana Shield** | Active | 12 | Absorb next 10 damage using mana instead of HP |
| **METEOR** | Ultimate | 20 | Massive damage (20+INT) to target area, AOE radius 4, stuns for 2 ticks |

**Passive Options (future):**
- Arcane Wisdom: +10% spell damage
- Elemental Resistance: +15% resistance to Fire/Cold/Lightning

---

### BERSERKER — Melee DPS Mastery

**Role:** High single-target damage, bleed, rage

| Skill | Type | Cooldown | Description |
|-------|------|----------|-------------|
| **Cleave** | Active | 5 | Melee swing hits all 8 adjacent tiles |
| **Bleed Wounds** | Active | 7 | Attack applies bleed (5 ticks, 2 damage/tick) |
| **Rage Strike** | Active | 6 | Powerful blow, damage increases based on missing HP |
| **Whirlwind** | Active | 8 | Spin attack: damage all adjacent tiles for 3 ticks while stationary |
| **ANNIHILATE** | Ultimate | 18 | Enters berserk mode: +100% damage, +50% attack speed, -50% defense for 5 ticks |

**Passive Options (future):**
- Blood Rage: +5% damage per stack when below 50% HP
- Iron Skin: Cannot be stunned while above 75% HP

---

### ELEMENTALIST — AOE Mage Mastery

**Role:** Elemental damage, area denial, status effects

| Skill | Type | Cooldown | Description |
|-------|------|----------|-------------|
| **Fireball** | Active | 5 | Projectile explodes on impact: 4+INT damage in radius 2 |
| **Ice Lance** | Active | 6 | Fast projectile, freezes enemy for 1 tick on hit |
| **Lightning Storm** | Active | 10 | Summons storm: 3 random lightning strikes over 4 ticks in radius 5 |
| **Elemental Nova** | Active | 12 | Explosion of all elements: Fire/Cold/Lightning damage to adjacent tiles |
| **CATACLYSM** | Ultimate | 25 | Massive AOE: All enemies on screen take 25+INT damage, apply all elemental effects |

**Passive Options (future):**
- Elemental Synergy: +10% damage if enemy affected by 2+ elements
- Mana Regen: +2 mana per tick

---

### NECROMANCER — Pet/Curse Mastery

**Role:** Minions, debuffs, sustain

| Skill | Type | Cooldown | Description |
|-------|------|----------|-------------|
| **Summon Skeleton** | Active | 15 | Summons skeleton ally (20 HP, 3 ATK) that fights for 30 ticks |
| **Life Drain** | Active | 8 | Drain 3 HP from target, heal player for same amount |
| **Corpse Explosion** | Active | 10 | If dead monster nearby: explodes for 5+INT damage |
| **Curse of Weakness** | Active | 12 | All enemies within 4 tiles: -3 attack for 5 ticks |
| **ARMY OF DEAD** | Ultimate | 25 | Summons 3 skeletons + 1 skeleton champion for 60 ticks |

**Passive Options (future):**
- Undead Mastery: Skeletons deal +25% damage
- Vitality Siphon: Life Drain heals +50%

---

## 4. Skill Deck vs Hotbar Design

### Option A: Fixed Hotbar (Recommended for Ironveil)
- 4 active skill slots + 1 ult slot
- Skills assigned at level-up or via UI
- No deck management between floors
- Simpler implementation, matches current system

### Option B: Deck Builder Style (Alternative)
- Player has skill "deck" of 8-10 skills
- Draw 4 per combat, discard rest
- Rest between each encounter
- More complex, adds strategic depth

**Reference: Hades System**
- Max 6 boons carried at once
- Dash is always available (built-in)
- Attack is always available (built-in)
- Similar to fixed hotbar approach

---

## 5. Cooldown System Reference

| Skill Tier | Cooldown Range | Examples |
|------------|-----------------|----------|
| Quick | 4-5 ticks | Basic attacks, weak spells |
| Standard | 6-8 ticks | Main abilities |
| Heavy | 10-12 ticks | Strong AOE, buffs |
| Ultimate | 15-25 ticks | Game-changing abilities |

**Tick Reference:** 500ms per tick in Ironveil's current system
- 6 tick cooldown = 3 seconds
- 10 tick cooldown = 5 seconds

---

## 6. Skill Synergy Examples (for Dual-Class)

When dual-mastery is implemented, skills can combine:

| Mastery Combo | Synergy Effect |
|---------------|----------------|
| Soldier + Rogue | War Cry + Backstab = guaranteed crit during stun |
| Mage + Elementalist | Chain Lightning + Fireball = burning chain lightning |
| Berserker + Necromancer | Bleed + Life Drain = healing from bleed damage |
| Rogue + Elementalist | Poison Blade + Ice Lance = frozen enemies take double poison |

---

## 7. Implementation Summary

### Skill Count per Mastery

| Component | Count |
|-----------|-------|
| Active Skills | 4 per mastery |
| Ultimate | 1 per mastery |
| Total per build | 4 active + 1 ult = 5 skills |

### Skill Assignment
- Level 1-2: Select first mastery, unlock 2 basic skills
- Level 5: Unlock 3rd skill + first mastery ultimate
- Level 10: Select second mastery (dual-class)
- Level 15: Unlock 4th skill from second mastery

### Hotkey Mapping
- Keys 1, 2, 3, 4: Active skills
- Key 5: Ultimate skill

---

## 8. Reference Games Summary

| Game | Skill System | Slots Used |
|------|---------------|-------------|
| Hades | Boons | 4-6 boons + dash + attack |
| Diablo 4 | Skills tree | 6 equipped + passive |
| Path of Exile | Gem links | 6 linked sockets |
| Grim Dawn | Skills | 8 active + passives |
| Slay the Spire | Draw pile | Hand of 3-5 cards |

**Ironveil Approach:** Hades-style fixed slots (4 + 1 ult) with Diablo-style skill trees (level unlocking)

---

## 9. Next Steps for Implementation

1. **Phase 1:** Expand ability system from 2 → 5 slots per player
2. **Phase 2:** Add skill tier/unlock level to Ability struct
3. **Phase 3:** Create 4 new masteries (Berserker, Elementalist, Necromancer, keep Soldier replacing Warrior)
4. **Phase 4:** Add hotkey 3, 4, 5 support in input handler
5. **Phase 5:** Update UI to show 5 skill slots with cooldowns

---

*Research document for Ironveil skill system design.*