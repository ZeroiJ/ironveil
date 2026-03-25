# Controls Reference

## Movement

| Key | Action |
|-----|--------|
| ↑ / `w` | Move up |
| ↓ / `s` | Move down |
| ← / `a` | Move left |
| → / `d` | Move right |

## Actions

| Key | Action |
|-----|--------|
| `q` / `Esc` | Quit game |
| `Tab` | Open inventory |
| `1` | Activate ability 1 |
| `2` | Activate ability 2 (unlock at Lv5) |
| Arrow key (after 1/2) | Set direction for directional abilities |
| Any other key | Cancel ability direction |

## Save/Load

| Key | Action |
|-----|--------|
| `Ctrl+S` | Save game to `save.json` |
| `Ctrl+L` | Load game from `save.json` |

## Inventory

| Key | Action |
|-----|--------|
| `Tab` | Open/close inventory |
| `Esc` | Close inventory |
| `a` - `j` | Use/equip item at position |
| `A` - `J` | Drop item at position |

---

## Combat

**Melee Attack:**
- Move into a monster to attack (bump-to-attack)

**Dodge:**
- Automatic chance based on DEX stat: `(DEX-10) * 3`%

---

## HUD

**Status Line 1:**
```
Floor:1 HP:20/28 Class:Warrior Weapon:Iron Shortsword
```

**Status Line 2:**
```
XP:42/120 [###-------] STR:14 DEX:10 INT:8 CON:14 Def:1 | Tab:Inv
```

**Ability Line:**
```
[1]Power Attack (Ready) [2]War Cry (Lv5)
```

**Message Log:**
```
> You hit the Goblin for 3 damage!
> The Goblin dies!
> +10 XP
```

---

## Status Effects

| Effect | Display | Color |
|--------|---------|-------|
| Poisoned | Player turns green | Green |
| Damage Buff | Player turns white | White |
| Stunned | Monster dark grey | Dark Grey |
| Frozen | Monster cyan | Cyan |
| Poisoned | Monster green | Green |
