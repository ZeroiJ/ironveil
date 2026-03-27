# Converting to True Turn-Based Roguelike — Ironveil Research

**Date:** 2026-03-27  
**Topic:** Converting Real-Time Action Roguelike to Turn-Based (Berlin Interpretation)  
**Reference Games:** NetHack, Dungeon Crawl Stone Soup (DCSS), Brogue, Berlin Interpretation

---

## 1. Why Ironveil Is Currently Not a "True" Roguelike

### The Berlin Interpretation

The Berlin Interpretation (2008) defines the core criteria for a "true" roguelike:

| Criteria | Definition | Ironveil Status |
|----------|------------|----------------|
| **Turn-based** | Each command = single action. Game not sensitive to real-time. | ❌ Real-time monster ticks |
| **Permadeath** | Character progression resets on death | ✅ |
| **Grid-based** | Movement on tile grid | ✅ |
| **Random generation** | Procedural level creation | ✅ |
| **Single character** | Control one @ | ✅ |
| **Tactical combat** | Resource management, positioning | ✅ |
| **Monsters exist** | AI-controlled enemies | ✅ |
| **One-way progression** | Descending floors | ✅ |

### The Problem: Real-Time Monster Ticks

Ironveil currently uses a **500ms monster tick system**:
- Monsters move every 0.5 seconds automatically
- Player input is real-time (instant movement)
- This is an **action roguelike** (like Hades, Enter the Gungeon), not a **traditional roguelike**

The Berlin Interpretation explicitly states:
> "Turn-based: Each command corresponds to a single action/movement. The game is not sensitive to time, you can take your time to choose your action."

**Source:** [Berlin Interpretation - RogueBasin](https://roguebasin.com/index.php/Berlin_Interpretation)

---

## 2. How Turn-Based Combat Works

### The Core Concept

In a true turn-based roguelike:
1. Player takes ONE action (move, attack, use item, wait)
2. Game state advances by ONE turn
3. ALL monsters take their turn(s) in response
4. Repeat

**Time only advances when the player acts.** This is fundamentally different from Ironveil's current system where time advances independently.

### Reference: Brogue Input Model

**Brogue** demonstrates the cleanest TB input pattern:

```c
// From brogue/Movement.c
void playerRuns(short direction) {
    while (!rogue.disturbed) {
        if (!playerMoves(direction)) {
            rogue.disturbed = true;
            // ...
        }
    }
}
```

Each input triggers a discrete turn. Monsters only move after the player acts.

**Source:** [Brogue Movement.c](https://github.com/tmewett/BrogueCE/blob/master/src/brogue/Movement.c)

### Reference: NetHack Per-Monster Turns

**NetHack** processes each monster individually after the player acts:

```c
// From NetHack monmove.c
/* returns 1 if monster died moving, 0 otherwise */
int dochug(register struct monst *mtmp) {
    // ... monster AI decision making
    // ... movement or attack
}
```

Each monster gets a turn, processed sequentially after player input.

**Source:** [NetHack monmove.c](https://github.com/NetHack/NetHack/blob/NetHack-3.6.6_Released/src/monmove.c)

### Reference: DCSS Energy-Based System

**Dungeon Crawl Stone Soup** uses an energy system to handle speed differences:

```cpp
// From DCSS mon-place.cc
static energy_use_type _get_swim_or_move(monster& mon) {
    // Determine energy cost based on terrain
}

static void _swim_or_move_energy(monster& mon) {
    mon.lose_energy(_get_swim_or_move(mon));
}
```

Monsters gain energy each turn. When energy crosses a threshold, they act.

**Source:** [DCSS mon-place.cc](https://github.com/crawl/crawl-ref/blob/master/source/mon-place.cc)

---

## 3. Monster Movement Logic in Turn-Based Systems

### When Do Monsters Move?

| Game | Trigger |
|------|---------|
| NetHack | After player's single action completes |
| DCSS | When monster energy >= threshold (100) |
| Brogue | After each player move, in deterministic order |
| Ironveil (current) | Every 500ms, regardless of player |

### The Turn Sequence

```
Player Input → Player Action Processing → Monster Phase (all monsters act) → Render → Wait for Input
```

In Ironveil's current system:
```
Player Input → Player Action → [500ms passes] → Monster Action → [500ms passes] → Monster Action...
```

### Handling Speed Differences

**Option A: Fixed Speed (Brogue-style)**
- All monsters move once per player turn
- Fast/slow monsters handled via separate "double move" or "skip turn" logic

**Option B: Energy System (DCSS-style)**
- Each monster has `speed` attribute
- Each turn: `monster.energy += monster.speed`
- When `energy >= TURN_ENERGY` (100): monster acts, `energy -= TURN_ENERGY`
- A monster with speed 150 gets 1.5 actions per player turn

---

## 4. Required Code Changes

### 4.1 Remove Real-Time Tick System

**Current (problematic):**
```rust
// main.rs - real-time tick
loop {
    std::thread::sleep(Duration::from_millis(500));
    process_monsters();  // Monsters move every 500ms
}
```

**Turn-Based (target):**
```rust
// main.rs - input-driven turn loop
loop {
    render();
    let input = get_input();
    
    if let Some(action) = input {
        process_player_action(action);  // Player turn
        process_monster_turns();         // All monsters move
    }
}
```

### 4.2 Add Monster Energy System

```rust
struct Monster {
    // ... existing fields
    speed: i32,      // Energy per turn (100 = 1 action/turn)
    energy: i32,    // Accumulated energy
}

const TURN_ENERGY: i32 = 100;

fn process_monster_turns(monsters: &mut Vec<Monster>) {
    // Sort for deterministic order (optional)
    for monster in monsters.iter_mut() {
        monster.energy += monster.speed;
        
        // Monster may get multiple actions per turn
        while monster.energy >= TURN_ENERGY {
            if monster.can_act() {
                monster.take_turn();
            }
            monster.energy -= TURN_ENERGY;
        }
    }
}
```

### 4.3 Convert Input Handling

**Current:**
```rust
// Player moves instantly on keypress
if key == ArrowUp {
    player.move(0, -1);
    // Monster tick already running separately
}
```

**Turn-Based:**
```rust
// Player action advances game state
if key == ArrowUp {
    let moved = player.try_move(0, -1);
    if moved {
        // ONE turn passes
        game_state.advance_turn();  // All monsters move
    }
}
```

### 4.4 Update Ability Cooldowns

**Current:**
```rust
// Cooldowns tick in real-time (every 500ms)
if ability.cooldown_remaining > 0 {
    ability.cooldown_remaining -= 1;
}
```

**Turn-Based:**
```rust
// Cooldowns tick per turn (when monsters move)
if ability.cooldown_remaining > 0 {
    ability.cooldown_remaining -= 1;  // Only when turn advances
}
```

### 4.5 Status Effects Duration

**Current:**
```rust
// Ticks in real-time
if monster.poison_ticks > 0 {
    monster.poison_ticks -= 1;
    monster.take_damage(1);
}
```

**Turn-Based:**
```rust
// Ticks per monster turn
fn monster.take_turn() {
    // ... movement/attack
    
    // After actions, apply DoT
    if self.poison_ticks > 0 {
        self.poison_ticks -= 1;
        self.take_damage(1);
    }
}
```

---

## 5. Migration Plan

### Phase 1: Core Turn System
1. Remove `std::thread::sleep` monster tick loop
2. Add `turn: i32` to game state
3. Create `advance_turn()` function that processes all monster actions
4. Bind every player action to call `advance_turn()`

### Phase 2: Monster Energy
1. Add `speed: i32` and `energy: i32` to Monster struct
2. Implement energy accumulation in `advance_turn()`
3. Handle multiple actions per turn for fast monsters

### Phase 3: Edge Cases
1. Handle "wait" action (pass turn without moving)
2. Handle stunned/frozen monsters (skip turn)
3. Handle death during monster phase
4. Ensure deterministic turn order for reproducibility

### Phase 4: Polish
1. Update cooldowns to use turn-based timing
2. Update status effects (poison, etc.) to tick per turn
3. Add visual feedback for turn passing
4. Test all abilities work correctly in TB mode

---

## 6. Ironveil-Specific Changes Summary

| Component | Current (RT) | Required (TB) |
|-----------|--------------|---------------|
| Monster movement | Every 500ms | After each player action |
| Player input | Instant | Triggers turn advance |
| Cooldowns | Real-time ticks | Per-turn ticks |
| Status effects | Real-time | Per-turn |
| "Wait" action | Not applicable | Pass turn, monsters act |
| Projectiles | Move on tick | Move on turn |

### Monster Speed Reference (DCSS)

| Monster Type | Speed | Actions per Turn |
|--------------|-------|------------------|
| Very slow | 50 | 0.5 (every 2 turns) |
| Slow | 70-80 | 0.7-0.8 |
| Normal | 100 | 1 |
| Fast | 120-150 | 1.2-1.5 |

---

## 7. Alternative: Hybrid Approach

If you want to keep some real-time elements while adding turn-based features:

**Option A: Turn-Based with Timed Abilities**
- Core movement/combat is turn-based
- Abilities have real-time cooldowns (like Hades)
- Projectiles move in real-time

**Option B: Pause System**
- Real-time by default
- Press SPACE to enter "tactical pause"
- While paused, game is effectively turn-based

**Option C: Hybrid Tick (Roguelike/Card Game hybrid)**
- Player has energy (like Slay the Spire)
- Each card/action costs energy
- Monsters act on their own energy schedule

---

## 8. References

### Primary Sources

- **Berlin Interpretation**: [RogueBasin](https://roguebasin.com/index.php/Berlin_Interpretation)
- **NetHack monmove.c**: [GitHub](https://github.com/NetHack/NetHack/blob/NetHack-3.6.6_Released/src/monmove.c)
- **DCSS mon-place.cc**: [GitHub](https://github.com/crawl/crawl-ref/blob/master/source/mon-place.cc)
- **DCSS mon-act.cc**: [GitHub](https://github.com/crawl/crawl-ref/blob/master/source/mon-act.cc)
- **Brogue Movement.c**: [GitHub](https://github.com/tmewett/BrogueCE/blob/master/src/brogue/Movement.c)

### Key Quotes

> "Turn-based: Each command corresponds to a single action/movement. The game is not sensitive to time, you can take your time to choose your action."
> — Berlin Interpretation

---

## 9. Recommendation

For Ironveil to become a "true" roguelike (Berlin Interpretation compliant):

1. **Minimum change**: Replace 500ms monster tick with turn-based system (player action → monster phase)
2. **Recommended**: Implement energy-based system for monster speed variety
3. **Preserve**: All existing abilities, items, and combat mechanics — just change when they execute

The conversion is straightforward: instead of monsters moving on a timer, they move after you move. That's the fundamental difference between action roguelike and turn-based roguelike.

---

*Research document for Ironveil turn-based conversion.*