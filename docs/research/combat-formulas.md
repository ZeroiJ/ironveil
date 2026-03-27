# Combat System Design — Ironveil Research

**Date:** 2026-03-26  
**Topic:** Combat Formulas & Mechanics  
**Reference Games:** Diablo 4, Path of Exile, Last Epoch, Hades, Brogue, DCSS

---

## 1. Damage Formulas

### Core Damage Calculation

The foundational formula used across ARPGs follows a layered approach:

```
Final Damage = (Base Damage + Flat Bonuses) × Multipliers
```

**Diablo 4 Formula:**
```
Damage = (Weapon Damage × Skill%) × (1 + IncDamage%) × (1 + More%) × Global Multipliers
```

**Key Components:**
- **Base Damage:** Weapon damage or skill base
- **Flat Bonuses:** Added damage from stats/items
- **Increased (Inc) Multipliers:** Additive, stack linearly
- **More Multipliers:** Multiplicative, very powerful
- **Global Multipliers:** Class-defining bonuses

### Defense Reduction

Most games use one of these approaches:

**Simple Subtraction (Brogue-style):**
```
Damage = max(1, Attack - Defense)
```

**Percentage Reduction (Diablo-style):**
```
Effective Defense = Defense × (Level Difference Factor)
Damage Reduction = Effective Defense / (Effective Defense + 100)
```

**Last Epoch Formula:**
```
Damage = (Base + Add) × (1 + Inc%) × More% × (1 - EnemyReduction%)
```

### Ironveil Current State

Your existing system uses:
```
dmg = player.attack - monster.defense
```

---

## 2. Attack Speed & Tick Systems

### Attack Speed Formula

**Diablo 4 / ARPG Standard:**
```
Attack Speed = Base Speed × (1 + AttackSpeedBonus%)
```

**Key Points:**
- Attack speed has breakpoints in some games (specific frames required)
- Caps typically at 200% (2× base speed)
- Dual wielding: average of both weapons

### Real-Time Combat Ticks

Ironveil uses a **500ms monster tick** system:
- Monsters act every 500ms
- Player moves instantly on keypress
- Projectiles move on monster tick

**Comparison:**
| Game | Tick System |
|------|-------------|
| Ironveil | 500ms monster tick |
| Hades | Real-time (60fps) |
| Brogue | Turn-based |
| Cogmind | Real-time with cooldowns |

---

## 3. Critical Hit Mechanics

### Critical Chance

**Standard Formula:**
```
Crit Chance = Base% + Bonus%
```

**Diablo 4:**
- Base crit chance: 5% (can be modified)
- Crit multiplier: 50% base (×1.5 damage)

**Last Epoch:**
```
Total Crit Chance = (Base + Added) × (1 + Inc%)
```

### Critical Multiplier

**Standard:**
- Base: 50% (1.5× damage)
- Can be increased through gear/skills
- Some games have **crit damage** stat separately

### Critical Resistance

Advanced games implement crit reduction:
```
Effective Crit Chance = AttackerCrit - DefenderCritRes
```

---

## 4. Dodge, Block & Parry

### Dodge Systems

**Percentage-Based (Simple):**
```
Hit Chance = 100% - Dodge%
If random(0,100) < Dodge%, attack misses
```

**Stat-Based (DCSS/Brogue):**
```
Dodge% = (DEX - 10) × 3%
```

### Block Mechanics

**Flat Reduction:**
```
Damage = RawDamage - BlockValue
```

**Percentage Reduction:**
```
Damage = RawDamage × (1 - Block%)
```

### Parry Systems

Parry often includes:
- Counter-attack opportunity
- Stun/chance to interrupt
- Reduced damage + guaranteed hit back

---

## 5. Status Effects

### Damage Over Time (DoT)

**Poison/Bleed Formula:**
```
Tick Damage = BaseDamage / Duration
Damage applied every tick (usually 1 second)
```

**Ironveil Implementation:**
```rust
if player.poison_ticks > 0 {
    player.poison_ticks -= 1;
    player.take_damage(1);
}
```

### Duration Scaling

| Effect Type | Duration Formula |
|------------|------------------|
| Stun | Fixed ticks or seconds |
| Freeze | Scales with power |
| Poison | Base + stat bonuses |
| Blind | Fixed duration |

### Resistance Systems

Most games implement resistance checks:
```
if random(0,100) < Resistance%:
    effect.apply()
else:
    "Resisted!" message
```

---

## 6. Elemental Systems

### Damage Types

| Type | Common Against | Common Weakness |
|------|----------------|-----------------|
| Physical | Armor | — |
| Fire | Ice, Plants | Water |
| Cold | Fire, Poison | Lightning |
| Lightning | Undead | — |
| Poison | Living | Undead |
| Aether | Energy | Physical |
| Chaos | All | — |

### Conversion Systems

Many ARPGs allow damage conversion:
```
Fire Damage → Lightning Damage
(requires conversion stat or skill)
```

---

## 7. Combat Formulas — Recommended for Ironveil

### Suggested Damage Formula

```rust
fn calculate_damage(attacker: &Attacker, defender: &Defender, skill: &Skill) -> i32 {
    // Base damage from skill or weapon
    let mut damage = skill.base_damage + attacker.attack_power;
    
    // Defense reduction (capped at 90%)
    let defense_reduction = (defender.defense / (defender.defense + 50.0)).min(0.9);
    damage = damage * (1.0 - defense_reduction);
    
    // Critical hits
    let is_crit = random_f32() < attacker.crit_chance;
    if is_crit {
        damage = damage * attacker.crit_multiplier;
    }
    
    // Elemental multipliers (if applicable)
    if skill.element == defender.weakness {
        damage = damage * 1.5;
    }
    
    // Variance (±10%)
    damage = damage * random_range(0.9, 1.1);
    
    damage.max(1)
}
```

### Suggested Stat Bonuses per Level

| Stat | Effect |
|------|--------|
| STR | +1 Attack, +5 HP |
| DEX | +1 Attack, +3% Dodge |
| CON | +10 HP, +1 Defense |
| INT | +1 Magic Damage, +5 MP |

---

## 8. Summary: Ironveil Combat Opportunities

| Current | Improvement |
|---------|-------------|
| Simple damage formula | Add defense scaling, damage variance |
| Fixed damage | Add critical hits with multiplier |
| No dodge | Add DEX-based dodge % |
| Basic status effects | Add duration scaling, resistance |
| No elemental system | Add damage types with conversions |

---

## References

- Diablo 4 Damage Formula (Maxroll)
- Last Epoch Damage Calculation (Arreat Summit)
- Game Dev Stack Exchange: RPG Damage Formulas
- Project Diablo 2 Mechanics Wiki
- Brogue/DCSS turn-based combat

---

*Research document for Ironveil combat system design.*