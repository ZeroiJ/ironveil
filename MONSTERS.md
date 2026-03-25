# Monster Catalog

## Basic Monsters

| Symbol | Name | HP | ATK | XP | Spawn Floors | Behavior |
|--------|------|-----|-----|-----|--------------|----------|
| `g` | Goblin | 6 | 2 | 10 | 1-6 | Chase + melee |
| `s` | Skeleton | 10 | 4 | 20 | 3-8 | Chase + ranged arrows |
| `T` | Troll | 20 | 8 | 40 | 4-10 | Slow chase + corridor block |
| `b` | Bat Swarm | 4 | 1 | 15 | 2-6 | Fast + erratic movement |
| `x` | Spider | 8 | 3 | 15 | 3-8 | Ambush + webs + poison |

## Undead

| Symbol | Name | HP | ATK | XP | Spawn Floors | Behavior |
|--------|------|-----|-----|-----|--------------|----------|
| `z` | Zombie | 12 | 3 | 18 | 6-10 | Regen 1HP/tick, slow chase |
| `G` | Ghoul | 8 | 5 | 22 | 6-10 | Poison bite (3 ticks) |
| `p` | Specter | 6 | 7 | 35 | 9-15 | Phases through walls |

## Demons

| Symbol | Name | HP | ATK | XP | Spawn Floors | Behavior |
|--------|------|-----|-----|-----|--------------|----------|
| `i` | Imp | 7 | 4 | 25 | 7-12 | Ranged fire bolt |
| `D` | Demon | 18 | 6 | 45 | 7-15 | Tough melee fighter |
| `f` | Hellfire Elemental | 12 | 5 | 38 | 7-12 | Fire AOE (poison) |

## Beasts

| Symbol | Name | HP | ATK | XP | Spawn Floors | Behavior |
|--------|------|-----|-----|-----|--------------|----------|
| `M` | Minotaur | 25 | 8 | 60 | 8-15 | Charging beast |
| `B` | Bear | 15 | 4→8 | 45 | 8-15 | Berserk at low HP |
| `w` | Wolf | 6 | 3 | 20 | 8-15 | Fast hunter, pack spawns |

## Humanoids

| Symbol | Name | HP | ATK | XP | Spawn Floors | Behavior |
|--------|------|-----|-----|-----|--------------|----------|
| `O` | Orc | 14 | 5 | 30 | 7-12 | Balanced warrior |
| `b` | Bandit | 7 | 4 | 25 | 7-12 | Drain attack (life steal) |
| `a` | Assassin | 6 | 9 | 55 | 7-12 | High damage, stealthy |

## Elementals

| Symbol | Name | HP | ATK | XP | Spawn Floors | Behavior |
|--------|------|-----|-----|-----|--------------|----------|
| `F` | Fire Elemental | 10 | 5 | 40 | 8-15 | Fire AOE |
| `E` | Earth Elemental | 20 | 5 | 50 | 8-15 | Tough, rocky defense |
| `I` | Ice Elemental | 12 | 4 | 40 | 8-15 | Freeze attack |

## Constructs

| Symbol | Name | HP | ATK | XP | Spawn Floors | Behavior |
|--------|------|-----|-----|-----|--------------|----------|
| `G` | Golem | 40 | 3 | 70 | 9-15 | Very slow, very tough |
| `s` | Sentry | 10 | 6 | 35 | 9-15 | Ranged laser |
| `o` | Bomber | 8 | 10 | 30 | 9-15 | Kamikaze attacker |

## Bosses

| Symbol | Name | HP | ATK | XP | Floor | Special Abilities |
|--------|------|-----|-----|-----|-------|-------------------|
| `K` | Goblin King | 60 | 5 | 150 | 5 | Summons goblins, enrages at 50% |
| `D` | Bone Dragon | 100 | 6 | 300 | 10 | Breath attack (line AOE), slow |
| `S` | Shadow Lord | 120 | 5 | 500 | 15 | Teleport, drain, shadow pulse |

---

## Status Effects

| Effect | Symbol | Color | Description |
|--------|--------|-------|-------------|
| Stunned | — | DarkGrey | Skip turn |
| Frozen | — | Cyan | Skip turn |
| Poisoned | — | Green | 1 damage/tick |

## Tier Scaling

Monsters scale with floor depth:

| Tier | Floors | Behavior Changes |
|------|--------|------------------|
| 1 | 1-3 | Basic chase + melee |
| 2 | 4-6 | Ranged attacks, retreat, webs |
| 3 | 7+ | Berserk, phase, resurrect, boss mechanics |
