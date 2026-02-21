use rand::RngExt;

use crate::map::Map;
use crate::projectile::Projectile;

// --- Behavior States ---

#[derive(Clone, Debug, PartialEq)]
pub enum BehaviorState {
    Idle,       // Wander randomly. Haven't spotted player.
    Chase,      // A* pathfind toward the player.
    Attack,     // Adjacent to player — melee hit.
    Ranged,     // Has line of sight + range — fire projectile.
    Retreat,    // Low HP — move away from the player.
    Reposition, // Move to a better tactical spot (Skeleton: keep distance).
}

#[derive(Clone, Debug)]
pub enum MonsterType {
    Goblin,
    Skeleton,
    Troll,
}

// --- Monster Action: what the monster decided to do this turn ---

pub enum MonsterAction {
    Nothing,
    MoveTo(usize, usize),                      // Move to (x, y)
    MeleeAttack { damage: i32, name: String }, // Hit the player
    FireProjectile(Projectile),                // Spawn a projectile
}

// --- Monster Struct ---

pub struct Monster {
    pub x: usize,
    pub y: usize,
    pub symbol: char,
    pub name: String,
    pub hp: i32,
    pub max_hp: i32,
    pub attack: i32,
    #[allow(dead_code)]
    pub base_attack: i32,
    pub monster_type: MonsterType,
    pub behavior: BehaviorState,
    pub can_see_player: bool,
    pub last_known_player_pos: Option<(usize, usize)>,
    pub ranged_cooldown: i32,
    pub turn_parity: i32, // for Troll slow movement (acts every other turn)
    pub floor_tier: i32,  // 1 = basic (floor 1-3), 2 = intermediate (4-6), 3 = tactical (7+)
    pub is_berserk: bool, // Troll berserk mode
}

impl Monster {
    pub fn new(x: usize, y: usize, monster_type: MonsterType, floor: i32) -> Self {
        let floor_tier = if floor <= 3 {
            1
        } else if floor <= 6 {
            2
        } else {
            3
        };

        match monster_type {
            MonsterType::Goblin => Self {
                x,
                y,
                symbol: 'g',
                name: "Goblin".to_string(),
                hp: 6,
                max_hp: 6,
                attack: 2,
                base_attack: 2,
                monster_type: MonsterType::Goblin,
                behavior: BehaviorState::Idle,
                can_see_player: false,
                last_known_player_pos: None,
                ranged_cooldown: 0,
                turn_parity: 0,
                floor_tier,
                is_berserk: false,
            },
            MonsterType::Skeleton => Self {
                x,
                y,
                symbol: 's',
                name: "Skeleton".to_string(),
                hp: 10,
                max_hp: 10,
                attack: 4,
                base_attack: 4,
                monster_type: MonsterType::Skeleton,
                behavior: BehaviorState::Idle,
                can_see_player: false,
                last_known_player_pos: None,
                ranged_cooldown: 0,
                turn_parity: 0,
                floor_tier,
                is_berserk: false,
            },
            MonsterType::Troll => Self {
                x,
                y,
                symbol: 'T',
                name: "Troll".to_string(),
                hp: 20,
                max_hp: 20,
                attack: 8,
                base_attack: 8,
                monster_type: MonsterType::Troll,
                behavior: BehaviorState::Idle,
                can_see_player: false,
                last_known_player_pos: None,
                ranged_cooldown: 0,
                turn_parity: 0,
                floor_tier,
                is_berserk: false,
            },
        }
    }

    pub fn random_monster(x: usize, y: usize, floor: i32) -> Self {
        let mut rng = rand::rng();
        let roll = rng.random_range(0..100);
        let m_type = if roll < 60 {
            MonsterType::Goblin
        } else if roll < 90 {
            MonsterType::Skeleton
        } else {
            MonsterType::Troll
        };
        Self::new(x, y, m_type, floor)
    }

    pub fn take_damage(&mut self, amount: i32) {
        self.hp = (self.hp - amount).max(0);
    }

    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }

    /// Main AI decision function. Called each monster turn.
    /// Returns the action the monster wants to take.
    pub fn decide_action(
        &mut self,
        player_pos: (usize, usize),
        map: &Map,
        occupied: &[(usize, usize)],
        all_monster_positions: &[(usize, usize)], // for goblin lure
    ) -> MonsterAction {
        let dist = Map::distance(self.x, self.y, player_pos.0, player_pos.1);

        // Update line of sight
        self.can_see_player =
            dist <= 12 && map.has_line_of_sight(self.x, self.y, player_pos.0, player_pos.1);

        if self.can_see_player {
            self.last_known_player_pos = Some(player_pos);
        }

        // Tick cooldowns
        if self.ranged_cooldown > 0 {
            self.ranged_cooldown -= 1;
        }

        // --- State evaluation (priority order) ---

        let adjacent = dist <= 1
            && (self.x as i32 - player_pos.0 as i32).abs() <= 1
            && (self.y as i32 - player_pos.1 as i32).abs() <= 1
            && !(self.x == player_pos.0 && self.y == player_pos.1);

        let low_hp = self.hp * 100 / self.max_hp < 30;

        // Dispatch to monster-specific AI
        match self.monster_type {
            MonsterType::Goblin => self.goblin_ai(
                player_pos,
                map,
                occupied,
                all_monster_positions,
                adjacent,
                low_hp,
                dist,
            ),
            MonsterType::Skeleton => {
                self.skeleton_ai(player_pos, map, occupied, adjacent, low_hp, dist)
            }
            MonsterType::Troll => self.troll_ai(player_pos, map, occupied, adjacent, low_hp, dist),
        }
    }

    // ==================== GOBLIN AI ====================
    fn goblin_ai(
        &mut self,
        player_pos: (usize, usize),
        map: &Map,
        occupied: &[(usize, usize)],
        all_monster_positions: &[(usize, usize)],
        adjacent: bool,
        low_hp: bool,
        _dist: i32,
    ) -> MonsterAction {
        // Tier 1+: Attack if adjacent
        if adjacent {
            self.behavior = BehaviorState::Attack;
            return MonsterAction::MeleeAttack {
                damage: self.attack,
                name: self.name.clone(),
            };
        }

        // Tier 2+: Retreat when low HP, lure toward other monsters
        if low_hp && self.floor_tier >= 2 && self.can_see_player {
            self.behavior = BehaviorState::Retreat;

            // Lure: find nearest other living monster and pathfind toward it
            let lure_target = self.find_nearest_ally(all_monster_positions);
            if let Some(target) = lure_target {
                if let Some(next) = map.astar_next_step((self.x, self.y), target, occupied) {
                    return MonsterAction::MoveTo(next.0, next.1);
                }
            }

            // No ally found — just flee away from player
            return self.flee_from(player_pos, map, occupied);
        }

        // Tier 1+: Chase if can see player
        if self.can_see_player {
            self.behavior = BehaviorState::Chase;

            // Tier 3: Goblin moves twice (fast)
            if self.floor_tier >= 3 {
                // First step
                if let Some(next) = map.astar_next_step((self.x, self.y), player_pos, occupied) {
                    // We return one move; main.rs will call a second move for tier 3 goblins
                    return MonsterAction::MoveTo(next.0, next.1);
                }
            }

            if let Some(next) = map.astar_next_step((self.x, self.y), player_pos, occupied) {
                return MonsterAction::MoveTo(next.0, next.1);
            }
        }

        // Idle: wander toward last known position or random
        self.behavior = BehaviorState::Idle;
        self.wander(map, occupied)
    }

    // ==================== SKELETON AI ====================
    fn skeleton_ai(
        &mut self,
        player_pos: (usize, usize),
        map: &Map,
        occupied: &[(usize, usize)],
        adjacent: bool,
        _low_hp: bool,
        dist: i32,
    ) -> MonsterAction {
        // Always fight in melee if adjacent
        if adjacent {
            self.behavior = BehaviorState::Attack;
            return MonsterAction::MeleeAttack {
                damage: self.attack,
                name: self.name.clone(),
            };
        }

        // Tier 3: Reposition — if player is too close (< 4 tiles), back away
        if self.floor_tier >= 3 && self.can_see_player && dist < 4 {
            self.behavior = BehaviorState::Reposition;
            return self.flee_from(player_pos, map, occupied);
        }

        // Tier 2+: Ranged attack if in range (3-8) and has LOS and cooldown ready
        if self.floor_tier >= 2
            && self.can_see_player
            && dist >= 3
            && dist <= 8
            && self.ranged_cooldown <= 0
        {
            self.behavior = BehaviorState::Ranged;
            self.ranged_cooldown = 2; // 2 turn cooldown

            // Calculate arrow direction
            let dx = (player_pos.0 as i32 - self.x as i32).signum();
            let dy = (player_pos.1 as i32 - self.y as i32).signum();
            let symbol = Projectile::symbol_for_direction(dx, dy);

            return MonsterAction::FireProjectile(Projectile {
                x: (self.x as i32 + dx) as usize,
                y: (self.y as i32 + dy) as usize,
                dx,
                dy,
                damage: 3,
                symbol,
                source_name: self.name.clone(),
            });
        }

        // Chase if can see player
        if self.can_see_player {
            self.behavior = BehaviorState::Chase;
            if let Some(next) = map.astar_next_step((self.x, self.y), player_pos, occupied) {
                return MonsterAction::MoveTo(next.0, next.1);
            }
        }

        // Idle
        self.behavior = BehaviorState::Idle;
        self.wander(map, occupied)
    }

    // ==================== TROLL AI ====================
    fn troll_ai(
        &mut self,
        player_pos: (usize, usize),
        map: &Map,
        occupied: &[(usize, usize)],
        adjacent: bool,
        low_hp: bool,
        _dist: i32,
    ) -> MonsterAction {
        // Tier 3: Berserk mode when low HP
        if low_hp && self.floor_tier >= 3 && !self.is_berserk {
            self.is_berserk = true;
            self.attack = 12; // Increased from 8
                              // Don't return — continue to act this turn, but signal berserk
                              // We'll handle the message in main.rs by checking is_berserk
        }

        // Troll is slow: only acts every other turn (unless berserk)
        self.turn_parity += 1;
        if !self.is_berserk && self.turn_parity % 2 != 0 {
            return MonsterAction::Nothing;
        }

        // Attack if adjacent
        if adjacent {
            self.behavior = BehaviorState::Attack;
            return MonsterAction::MeleeAttack {
                damage: self.attack,
                name: self.name.clone(),
            };
        }

        // Tier 2+: Corridor blocker — if in a corridor, don't move. Wait for the player.
        if self.floor_tier >= 2 && map.is_corridor(self.x, self.y) && self.can_see_player {
            self.behavior = BehaviorState::Idle; // Intentionally idle — blocking
            return MonsterAction::Nothing;
        }

        // Chase if can see player
        if self.can_see_player {
            self.behavior = BehaviorState::Chase;
            if let Some(next) = map.astar_next_step((self.x, self.y), player_pos, occupied) {
                return MonsterAction::MoveTo(next.0, next.1);
            }
        }

        // Idle
        self.behavior = BehaviorState::Idle;
        self.wander(map, occupied)
    }

    // ==================== SHARED BEHAVIORS ====================

    /// Flee: move to the adjacent tile that maximizes distance from target.
    fn flee_from(
        &self,
        target: (usize, usize),
        map: &Map,
        occupied: &[(usize, usize)],
    ) -> MonsterAction {
        let neighbors = [
            (self.x.wrapping_sub(1), self.y),
            (self.x + 1, self.y),
            (self.x, self.y.wrapping_sub(1)),
            (self.x, self.y + 1),
        ];

        let mut best: Option<(usize, usize)> = None;
        let mut best_dist = Map::distance(self.x, self.y, target.0, target.1);

        for &(nx, ny) in &neighbors {
            if nx < map.width
                && ny < map.height
                && map.is_walkable(nx, ny)
                && !occupied.contains(&(nx, ny))
            {
                let d = Map::distance(nx, ny, target.0, target.1);
                if d > best_dist {
                    best_dist = d;
                    best = Some((nx, ny));
                }
            }
        }

        if let Some((nx, ny)) = best {
            MonsterAction::MoveTo(nx, ny)
        } else {
            MonsterAction::Nothing
        }
    }

    /// Wander: move toward last known player position, or random walkable neighbor.
    fn wander(&mut self, map: &Map, occupied: &[(usize, usize)]) -> MonsterAction {
        // If we have a last known position, walk toward it
        if let Some(last_pos) = self.last_known_player_pos {
            if let Some(next) = map.astar_next_step((self.x, self.y), last_pos, occupied) {
                // If we reached the last known position, clear it
                if next == last_pos {
                    self.last_known_player_pos = None;
                }
                return MonsterAction::MoveTo(next.0, next.1);
            }
            self.last_known_player_pos = None;
        }

        // Random wander
        let mut rng = rand::rng();
        let dirs: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let &(dx, dy) = &dirs[rng.random_range(0..4)];
        let nx = self.x as i32 + dx;
        let ny = self.y as i32 + dy;
        if nx >= 0 && ny >= 0 {
            let (ux, uy) = (nx as usize, ny as usize);
            if map.is_walkable(ux, uy) && !occupied.contains(&(ux, uy)) {
                return MonsterAction::MoveTo(ux, uy);
            }
        }
        MonsterAction::Nothing
    }

    /// Find the nearest other living monster position to flee toward (for Goblin lure).
    fn find_nearest_ally(&self, all_positions: &[(usize, usize)]) -> Option<(usize, usize)> {
        let mut best: Option<(usize, usize)> = None;
        let mut best_dist = i32::MAX;
        for &pos in all_positions {
            if pos == (self.x, self.y) {
                continue;
            }
            let d = Map::distance(self.x, self.y, pos.0, pos.1);
            if d < best_dist {
                best_dist = d;
                best = Some(pos);
            }
        }
        best
    }
}
