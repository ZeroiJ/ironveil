# Meta-Progression Systems — Ironveil Research

**Date:** 2026-03-26  
**Topic:** Meta-Progression & Roguelite Elements  
**Reference Games:** Hades, Slay the Spire, Dead Cells, Rogue Legacy, Path of Exile, Enter the Gungeon, Binding of Isaac

---

## 1. Understanding Meta-Progression

Meta-progression refers to permanent upgrades, unlockables, and progression systems that persist across multiple runs in a roguelike/roguelite game. Unlike character progression (which resets each run), meta-progression gives players a sense of long-term advancement and justifies repeated playthroughs.

### Core Definitions

| Term | Definition |
|------|------------|
| **Roguelike** | Permadeath, procedurally generated, each run is self-contained |
| **Roguelite** | Roguelike with meta-progression elements (runs build toward something) |
| **Meta-Currency** | Currency earned per run, spent on permanent upgrades |
| **Unlockable** | Content initially hidden, revealed through gameplay achievements |
| **Permanent Upgrade** | Upgrade that persists across all future runs |

---

## 2. Meta-Currency Systems

### Hades — Darkness Currency

**System Overview:**
- Darkness is earned by dying in the Underworld
- Earned per run: 20-200+ depending on heat and progress
- Spent at the Mirror of Night for permanent upgrades

**Currency Flow:**
```
Run Completion → Darkness Award → Mirror of Night → Permanent Boons
```

**Mirror of Night Upgrades:**

| Upgrade | Cost | Effect |
|---------|------|--------|
| Shadow Presence | 5 | +20% damage when enemies are unaware |
| Chthonic Vitality | 10 | +20 HP |
| Dark Regeneration | 15 | Heal 1 HP every 10 rooms |
| Stygian Soul | 20 | +2 Death Defiances |
| Greater Reflex | 25 | +15% dodge chance |
| Deep Pockets | 30 | +50 gold capacity |
| Olympian Favor | 35 | Start with 1 Divine Dash |
| Family Favorite | 40 | +10% chance for_duplicates at Pom slots |
| Gods' Pride | 45 | +10% rare card chance at Pom |
| Underworld Extract | 50 | +10% offered nectar chance |
| Ironized Blood | 60 | Start with +1 Death Defiance |

**Titan Blood** — Weapon-specific upgrades:
- Earned from optional bosses and mini-bosses
- Each weapon has 12 Daedalus Hammer upgrades
- Total 60 Titan Blood for full single-weapon mastery

**Keys & Gems:**
- Keys: Unlock Olympian Pact (choose 1 of 3 boons at start)
- Gems: Unlocks secondary weapon aspects

### Slay the Spire — Gold & Relics

**System:**
- No traditional meta-currency
- Progression through unlocking cards and relics
- Ascension levels unlock after first victory

**Currency-Free Design:**
- Progression tied to explicit unlocks, not currency accumulation
- More elegant for deck-builders where economy would complicate design

### Dead Cells — Cells Currency

**System Overview:**
- Cells earned from killing enemies and finding secret rooms
- Spent between runs on mutations and upgrades
- Cells lost on death (high stakes)

**Upgrade Categories:**

| Category | Examples | Unlock Cost |
|----------|----------|-------------|
| Mutations | Dead inside, Tranquility, Networking | 20-50 cells |
| Scrolls | Scroll fragments for partial upgrades | 10-30 cells |
| Alchemy | Potion capacity, throw range | 15-40 cells |

### Rogue Legacy — Heir System & Kingdom Currency

**System:**
- Each run uses a new "Heir" (character with random traits)
- Kingdom currency (Gold) spent on permanent upgrades
- Traits carry over as unlockable knowledge

**Progression Track:**

| Upgrade | Effect |
|---------|--------|
| blacksmith | Equipment quality +10% |
| Church | Health +10% |
| Treasury | Starting gold +50 |
| University | Experience +10% |
| Tavern | Crit chance +5% |

---

## 3. Unlockable Content Architecture

### Types of Unlockables

**1. Game Modes**
- New Game Plus (enhanced difficulty)
- Endless/Survival modes
- Daily challenges
- Custom modifiers (like Hades' Pact of Punishment)

**2. Characters/Classes**
- Unlock by beating game with previous character
- Progressive unlocking (first clear unlocks second)
- Secret characters (found through exploration)

**3. Difficulty Levels**
- Standard: Easy → Normal → Hard
- Modifier-based: "Heat" system (Hades), "Ascension" (Slay the Spire)

**4. cosmetics**
- Skins, visual effects, death animations
- Titles, profile pictures
- Behind-the-scenes art/music

### Slay the Spire — Ascension System

**Design Philosophy:**
- 20 Ascension levels per character
- Each level adds one modifier
- Unlocks incrementally after first character victory

**Sample Modifiers (A1-A20):**

| Level | Modifier | Effect |
|-------|----------|--------|
| A1 | Elite stats +50% | Elites have more HP/damage |
| A2 | Start with 1 less card | Deck starts smaller |
| A3 | Elites appear more | Higher elite spawn rate |
| A4 | Start with 25 gold | Different starting resources |
| A5 | Boss HP +50% | Longer first boss fights |
| A6 | Unknown Potion | Lose potion reward |
| A7 | Start with curses | Deck starts with Dazed |
| A8 | PoorHP events | Free events cost HP |
| A9 | Card reward cost +50 | Upgrades more expensive |
| A10 | Max HP -10% | Permanent health reduction |

### Hades — Pact of Punishment (Heat System)

**Heat Levels:**
- 0-50 heat levels available
- Each heat level adds modifiers
- Some modifiers stack, others are exclusive

**Sample Heat Modifiers:**

| Heat | Name | Effect |
|------|------|--------|
| 2 | Tight Dead | +15% enemy damage |
| 4 | Extreme Measures | Elite skills change at 40% HP |
| 6 | Hell Mode | Enemies scale to your level |
| 8 | Prismatic Ice | Enemies have +20% ice resistance |
| 10 | Damage Control | Your damage -15% |
| 15 | Mount Olympus | Bosses use all abilities |
| 20 | Calisthenics Program | Enemies have +40% HP |
| 30 | Extreme Measures 2 | Elites change at 60% HP |
| 50 | Fear Itself | +100% enemy damage, +75% HP |

**Difficulty Scaling:**
- Up to 15 heat: Rewards scale linearly
- 16-49 heat: Rewards scale sub-linearly (diminishing returns)
- 50+ heat: Trophy only, minimal rewards

---

## 4. Permanent Upgrade Systems

### Upgrade Taxonomy

| Type | Example | Persists? |
|------|---------|-----------|
| Stat Boosts | +10% damage | Yes |
| Resource Boosts | +50 starting gold | Yes |
| New Abilities | Extra dash, new skill | Yes |
| Convenience | Auto-loot, map reveal | Yes |
| Unlockables | New class, mode | Yes |

### Path of Exie — Atlas & Passive Mastery

**Atlas Progression:**
- Maps unlock progressively
- Each map completion reveals adjacent maps
- Maven's gift: Special map unlocks
- Conqueror/Guardian achievements

**Passive Tree Mastery:**
- Additional 20 points after completion
- Selective mastery of favorite builds
- Permanent optimization

### Binding of Isaac — Post-It Notes

**Save File Tracking:**
- Each character has a "post-it note" on save file
- Mark completion of each goal:
  - Beat Mom (√)
  - Beat Mom's Heart (√√)
  - Beat Satan (√√√)
  - Beat Isaac (√√√√)
  - Beat ??? (√√√√√)
  - Beat The Lamb (√√√√√√)
  - Beat Mega Satan (★)
  - Beat Delirium (◆)
  - Beat Beast (▼)

**Unlock Trigger:**
- Completing goals unlocks new items, characters, challenges

---

## 5. Roguelite Progression Curves

### Progression Pacing Models

**Hades Model (Fast Early, Slow Late):**
```
Run 1-5:    30 min → Unlock 2-3 upgrades
Run 5-10:   45 min → Unlock 5-8 upgrades  
Run 10-20:  60 min → Full weapon mastery
Run 20+:    Endgame → Heat pushing
```

**Dead Cells Model (Cell-Gated):**
```
Early Game:  Unlock 3-4 mutations
Mid Game:    Unlock 80% mutations + alchemy
Late Game:   Scroll fragment hunting, endless
```

**Rogue Legacy Model (Kingdom-Gated):**
```
Kingdom Lv 1:  Basic classes
Kingdom Lv 5:  Better equipment
Kingdom Lv 10: Advanced classes
Kingdom Lv 20: All content
```

### Ironveil Considerations

**Recommended Approach:**
1. **Meta-Currency:** Earned from boss kills and floor clear bonuses
2. **Upgrade Categories:**
   - Stats (STR/DEX/CON/INT bonus)
   - Starting loadout (health, gold, items)
   - Convenience (auto-pickup, identify scroll)
   - Unlockables (new classes, biomes)

---

## 6. Pity Systems & Progression Safety

### Preventing Frustration

**Guaranteed Unlocks:**
| Trigger | Unlock |
|---------|--------|
| First boss kill | 1 meta-currency type |
| 10 runs | New class option |
| 50 deaths | "Mercy" upgrade (minor) |

### Grind Caps

**Maximum Grind Parameters:**
- Typical full meta-progression: 50-100 runs
- Hardcore completionists: 200+ runs
- Design for 80% completion at ~40 hours
- Design for 100% completion at ~100 hours

---

## 7. Summary: Ironveil Meta-Progression Design

### Recommended System

**Meta-Currency: Aether Shards**
- Earned from: Boss kills, secret rooms, optional floors
- Spent on: Permanent upgrades between runs
- Lost on death? No (forgiving roguelite)

**Upgrade Categories:**

| Category | Examples | Cost Range |
|----------|----------|------------|
| Stats | +5% damage, +10 HP | 20-50 shards |
| Starting | +20 gold, 1 item | 30-80 shards |
| Convenience | Auto-identify, +1 pylon | 40-100 shards |
| Unlockables | New class, biome | 100-200 shards |

**Unlockable Progression:**
- New Game Plus: Unlock after first clear
- Ascension levels: Each adds +1 enemy difficulty
- New classes: Sequential unlocking
- Hardcore mode: Optional toggle

### Comparison: Reference Games

| Game | Meta-Currency | Progression Style | Grind to 100% |
|------|---------------|-------------------|---------------|
| Hades | Darkness | Upgrade nodes | ~50 runs |
| Slay the Spire | None | Direct unlocks | ~30 hours |
| Dead Cells | Cells | Mutations | ~60 runs |
| Rogue Legacy | Gold | Kingdom upgrades | ~40 runs |
| Path of Exile | None | Atlas completion | ~300 hours |

---

## 8. Implementation Recommendations for Ironveil

### Phase 1: Core Meta-Currency
- Add "Aether Shards" as run completion reward
- Create upgrade menu (between runs)
- Implement 5-8 starter upgrades

### Phase 2: Unlockables
- New Game Plus mode
- Secret classes (unlockable)
- Daily challenges

### Phase 3: Depth
- Ascension/heat system
- Endless mode
- Full cosmetic unlockables

---

## References

- Hades: Supergiant Games (Mirror of Night, Pact of Punishment)
- Slay the Spire: MegaCrit (Ascension system)
- Dead Cells: Motion Twin (Mutation system)
- Rogue Legacy: Cellar Door Games (Kingdom upgrades)
- Path of Exile: Grinding Gear Games (Atlas progression)
- Binding of Edmund: Nicalis (Post-it save system)

---

*Research document for Ironveil meta-progression system design.*