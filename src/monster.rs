use rand::RngExt;
use serde::{Deserialize, Serialize};

use crate::map::Map;
use crate::projectile::Projectile;

// --- Behavior States ---

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum BehaviorState {
    Idle,       // Wander randomly. Haven't spotted player.
    Chase,      // A* pathfind toward the player.
    Attack,     // Adjacent to player — melee hit.
    Ranged,     // Has line of sight + range — fire projectile.
    Retreat,    // Low HP — move away from the player.
    Reposition, // Move to a better tactical spot (Skeleton: keep distance).
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MonsterType {
    Goblin,
    Skeleton,
    Troll,
    BatSwarm,
    Spider,
    Wraith,
    Necromancer,
    GoblinKing,
    BoneDragon,
    ShadowLord,
    // Undead
    Zombie,
    Ghoul,
    Specter,
    // Demons
    Imp,
    Demon,
    Hellfire,
    // Beasts
    Minotaur,
    Bear,
    WolfPack,
    // Humanoids
    Orc,
    Bandit,
    Assassin,
    // Elementals
    FireElemental,
    EarthElemental,
    IceElemental,
    // Constructs
    Golem,
    Sentry,
    Bomber,
}

// --- Monster Action: what the monster decided to do this turn ---

pub enum MonsterAction {
    Nothing,
    MoveTo(usize, usize),      // Move to (x, y)
    MoveToPhase(usize, usize), // Move to (x, y), can pass through walls (Wraith)
    MeleeAttack {
        damage: i32,
        name: String,
    }, // Hit the player
    PoisonAttack {
        damage: i32,
        name: String,
        poison_ticks: i32,
    }, // Hit + poison player
    DrainAttack {
        damage: i32,
        name: String,
    }, // Hit + heal self (Wraith)
    PlaceWeb(usize, usize),    // Spider places web at position
    FireProjectile(Projectile), // Spawn a projectile
    Resurrect(usize),          // Necromancer resurrects monster at index
    BossSummon,                // Goblin King summons goblin minions
    BreathAttack {
        // Bone Dragon breath: line AoE
        dx: i32,
        dy: i32,
        damage: i32,
        range: i32,
    },
    ShadowPulse {
        // Shadow Lord AoE around self
        damage: i32,
        radius: i32,
    },
    BossTeleport, // Shadow Lord teleport to random floor tile
}

// --- Monster Struct ---

#[derive(Clone, Serialize, Deserialize)]
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
    // Status effects
    pub stun_ticks: i32,       // Stunned: skip turn, rendered DarkGrey
    pub freeze_ticks: i32,     // Frozen: skip turn, rendered Cyan
    pub poison_ticks: i32,     // Poisoned: 1 damage/tick, rendered Green
    pub attack_reduction: i32, // Reduced attack from Battle Cry
    // Special fields for new monsters
    pub web_cooldown: i32,                 // Spider: ticks until next web
    pub summon_count: i32,                 // Necromancer: resurrections used
    pub summon_max: i32,                   // Necromancer: max resurrections
    pub is_phasing: bool,                  // Wraith: currently inside a wall
    pub death_pos: Option<(usize, usize)>, // where this monster died (for resurrection)
    pub is_boss: bool,                     // Boss monster flag
    pub boss_tick: i32,                    // Boss ability timer
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
                stun_ticks: 0,
                freeze_ticks: 0,
                poison_ticks: 0,
                attack_reduction: 0,
                web_cooldown: 0,
                summon_count: 0,
                summon_max: 0,
                is_phasing: false,
                death_pos: None,
                is_boss: false,
                boss_tick: 0,
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
                stun_ticks: 0,
                freeze_ticks: 0,
                poison_ticks: 0,
                attack_reduction: 0,
                web_cooldown: 0,
                summon_count: 0,
                summon_max: 0,
                is_phasing: false,
                death_pos: None,
                is_boss: false,
                boss_tick: 0,
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
                stun_ticks: 0,
                freeze_ticks: 0,
                poison_ticks: 0,
                attack_reduction: 0,
                web_cooldown: 0,
                summon_count: 0,
                summon_max: 0,
                is_phasing: false,
                death_pos: None,
                is_boss: false,
                boss_tick: 0,
            },
            MonsterType::BatSwarm => Self {
                x,
                y,
                symbol: 'b',
                name: "Bat Swarm".to_string(),
                hp: 4,
                max_hp: 4,
                attack: 1,
                base_attack: 1,
                monster_type: MonsterType::BatSwarm,
                behavior: BehaviorState::Idle,
                can_see_player: false,
                last_known_player_pos: None,
                ranged_cooldown: 0,
                turn_parity: 0,
                floor_tier,
                is_berserk: false,
                stun_ticks: 0,
                freeze_ticks: 0,
                poison_ticks: 0,
                attack_reduction: 0,
                web_cooldown: 0,
                summon_count: 0,
                summon_max: 0,
                is_phasing: false,
                death_pos: None,
                is_boss: false,
                boss_tick: 0,
            },
            MonsterType::Spider => Self {
                x,
                y,
                symbol: 'x',
                name: "Spider".to_string(),
                hp: 8,
                max_hp: 8,
                attack: 3,
                base_attack: 3,
                monster_type: MonsterType::Spider,
                behavior: BehaviorState::Idle,
                can_see_player: false,
                last_known_player_pos: None,
                ranged_cooldown: 0,
                turn_parity: 0,
                floor_tier,
                is_berserk: false,
                stun_ticks: 0,
                freeze_ticks: 0,
                poison_ticks: 0,
                attack_reduction: 0,
                web_cooldown: 4,
                summon_count: 0,
                summon_max: 0,
                is_phasing: false,
                death_pos: None,
                is_boss: false,
                boss_tick: 0,
            },
            MonsterType::Wraith => Self {
                x,
                y,
                symbol: 'W',
                name: "Wraith".to_string(),
                hp: 12,
                max_hp: 12,
                attack: 5,
                base_attack: 5,
                monster_type: MonsterType::Wraith,
                behavior: BehaviorState::Idle,
                can_see_player: false,
                last_known_player_pos: None,
                ranged_cooldown: 0,
                turn_parity: 0,
                floor_tier,
                is_berserk: false,
                stun_ticks: 0,
                freeze_ticks: 0,
                poison_ticks: 0,
                attack_reduction: 0,
                web_cooldown: 0,
                summon_count: 0,
                summon_max: 0,
                is_phasing: false,
                death_pos: None,
                is_boss: false,
                boss_tick: 0,
            },
            MonsterType::Necromancer => Self {
                x,
                y,
                symbol: 'N',
                name: "Necromancer".to_string(),
                hp: 8,
                max_hp: 8,
                attack: 2,
                base_attack: 2,
                monster_type: MonsterType::Necromancer,
                behavior: BehaviorState::Idle,
                can_see_player: false,
                last_known_player_pos: None,
                ranged_cooldown: 0,
                turn_parity: 0,
                floor_tier,
                is_berserk: false,
                stun_ticks: 0,
                freeze_ticks: 0,
                poison_ticks: 0,
                attack_reduction: 0,
                web_cooldown: 0,
                summon_count: 0,
                summon_max: if floor_tier >= 3 { 5 } else { 3 },
                is_phasing: false,
                death_pos: None,
                is_boss: false,
                boss_tick: 0,
            },
            MonsterType::GoblinKing => Self {
                x,
                y,
                symbol: 'K',
                name: "Goblin King".to_string(),
                hp: 60,
                max_hp: 60,
                attack: 5,
                base_attack: 5,
                monster_type: MonsterType::GoblinKing,
                behavior: BehaviorState::Idle,
                can_see_player: false,
                last_known_player_pos: None,
                ranged_cooldown: 0,
                turn_parity: 0,
                floor_tier,
                is_berserk: false,
                stun_ticks: 0,
                freeze_ticks: 0,
                poison_ticks: 0,
                attack_reduction: 0,
                web_cooldown: 0,
                summon_count: 0,
                summon_max: 4,
                is_phasing: false,
                death_pos: None,
                is_boss: true,
                boss_tick: 0,
            },
            MonsterType::BoneDragon => Self {
                x,
                y,
                symbol: 'D',
                name: "Bone Dragon".to_string(),
                hp: 100,
                max_hp: 100,
                attack: 6,
                base_attack: 6,
                monster_type: MonsterType::BoneDragon,
                behavior: BehaviorState::Idle,
                can_see_player: false,
                last_known_player_pos: None,
                ranged_cooldown: 0,
                turn_parity: 0,
                floor_tier,
                is_berserk: false,
                stun_ticks: 0,
                freeze_ticks: 0,
                poison_ticks: 0,
                attack_reduction: 0,
                web_cooldown: 0,
                summon_count: 0,
                summon_max: 0,
                is_phasing: false,
                death_pos: None,
                is_boss: true,
                boss_tick: 0,
            },
            MonsterType::ShadowLord => Self {
                x,
                y,
                symbol: 'S',
                name: "Shadow Lord".to_string(),
                hp: 120,
                max_hp: 120,
                attack: 5,
                base_attack: 5,
                monster_type: MonsterType::ShadowLord,
                behavior: BehaviorState::Idle,
                can_see_player: false,
                last_known_player_pos: None,
                ranged_cooldown: 0,
                turn_parity: 0,
                floor_tier,
                is_berserk: false,
                stun_ticks: 0,
                freeze_ticks: 0,
                poison_ticks: 0,
                attack_reduction: 0,
                web_cooldown: 0,
                summon_count: 0,
                summon_max: 0,
                is_phasing: false,
                death_pos: None,
                is_boss: true,
                boss_tick: 0,
            },
            // Undead
            MonsterType::Zombie => Self {
                x,
                y,
                symbol: 'z',
                name: "Zombie".to_string(),
                hp: 12,
                max_hp: 12,
                attack: 3,
                base_attack: 3,
                monster_type: MonsterType::Zombie,
                behavior: BehaviorState::Idle,
                can_see_player: false,
                last_known_player_pos: None,
                ranged_cooldown: 0,
                turn_parity: 0,
                floor_tier,
                is_berserk: false,
                stun_ticks: 0,
                freeze_ticks: 0,
                poison_ticks: 0,
                attack_reduction: 0,
                web_cooldown: 0,
                summon_count: 0,
                summon_max: 0,
                is_phasing: false,
                death_pos: None,
                is_boss: false,
                boss_tick: 0,
            },
            MonsterType::Ghoul => Self {
                x,
                y,
                symbol: 'G',
                name: "Ghoul".to_string(),
                hp: 8,
                max_hp: 8,
                attack: 5,
                base_attack: 5,
                monster_type: MonsterType::Ghoul,
                behavior: BehaviorState::Idle,
                can_see_player: false,
                last_known_player_pos: None,
                ranged_cooldown: 0,
                turn_parity: 0,
                floor_tier,
                is_berserk: false,
                stun_ticks: 0,
                freeze_ticks: 0,
                poison_ticks: 0,
                attack_reduction: 0,
                web_cooldown: 0,
                summon_count: 0,
                summon_max: 0,
                is_phasing: false,
                death_pos: None,
                is_boss: false,
                boss_tick: 0,
            },
            MonsterType::Specter => Self {
                x,
                y,
                symbol: 'p',
                name: "Specter".to_string(),
                hp: 6,
                max_hp: 6,
                attack: 7,
                base_attack: 7,
                monster_type: MonsterType::Specter,
                behavior: BehaviorState::Idle,
                can_see_player: false,
                last_known_player_pos: None,
                ranged_cooldown: 0,
                turn_parity: 0,
                floor_tier,
                is_berserk: false,
                stun_ticks: 0,
                freeze_ticks: 0,
                poison_ticks: 0,
                attack_reduction: 0,
                web_cooldown: 0,
                summon_count: 0,
                summon_max: 0,
                is_phasing: false,
                death_pos: None,
                is_boss: false,
                boss_tick: 0,
            },
            // Demons
            MonsterType::Imp => Self {
                x,
                y,
                symbol: 'i',
                name: "Imp".to_string(),
                hp: 7,
                max_hp: 7,
                attack: 4,
                base_attack: 4,
                monster_type: MonsterType::Imp,
                behavior: BehaviorState::Idle,
                can_see_player: false,
                last_known_player_pos: None,
                ranged_cooldown: 0,
                turn_parity: 0,
                floor_tier,
                is_berserk: false,
                stun_ticks: 0,
                freeze_ticks: 0,
                poison_ticks: 0,
                attack_reduction: 0,
                web_cooldown: 0,
                summon_count: 0,
                summon_max: 0,
                is_phasing: false,
                death_pos: None,
                is_boss: false,
                boss_tick: 0,
            },
            MonsterType::Demon => Self {
                x,
                y,
                symbol: 'D',
                name: "Demon".to_string(),
                hp: 18,
                max_hp: 18,
                attack: 6,
                base_attack: 6,
                monster_type: MonsterType::Demon,
                behavior: BehaviorState::Idle,
                can_see_player: false,
                last_known_player_pos: None,
                ranged_cooldown: 0,
                turn_parity: 0,
                floor_tier,
                is_berserk: false,
                stun_ticks: 0,
                freeze_ticks: 0,
                poison_ticks: 0,
                attack_reduction: 0,
                web_cooldown: 0,
                summon_count: 0,
                summon_max: 0,
                is_phasing: false,
                death_pos: None,
                is_boss: false,
                boss_tick: 0,
            },
            MonsterType::Hellfire => Self {
                x,
                y,
                symbol: 'f',
                name: "Hellfire Elemental".to_string(),
                hp: 12,
                max_hp: 12,
                attack: 5,
                base_attack: 5,
                monster_type: MonsterType::Hellfire,
                behavior: BehaviorState::Idle,
                can_see_player: false,
                last_known_player_pos: None,
                ranged_cooldown: 0,
                turn_parity: 0,
                floor_tier,
                is_berserk: false,
                stun_ticks: 0,
                freeze_ticks: 0,
                poison_ticks: 0,
                attack_reduction: 0,
                web_cooldown: 0,
                summon_count: 0,
                summon_max: 0,
                is_phasing: false,
                death_pos: None,
                is_boss: false,
                boss_tick: 0,
            },
            // Beasts
            MonsterType::Minotaur => Self {
                x,
                y,
                symbol: 'M',
                name: "Minotaur".to_string(),
                hp: 25,
                max_hp: 25,
                attack: 8,
                base_attack: 8,
                monster_type: MonsterType::Minotaur,
                behavior: BehaviorState::Idle,
                can_see_player: false,
                last_known_player_pos: None,
                ranged_cooldown: 0,
                turn_parity: 0,
                floor_tier,
                is_berserk: false,
                stun_ticks: 0,
                freeze_ticks: 0,
                poison_ticks: 0,
                attack_reduction: 0,
                web_cooldown: 0,
                summon_count: 0,
                summon_max: 0,
                is_phasing: false,
                death_pos: None,
                is_boss: false,
                boss_tick: 0,
            },
            MonsterType::Bear => Self {
                x,
                y,
                symbol: 'B',
                name: "Bear".to_string(),
                hp: 15,
                max_hp: 15,
                attack: 4,
                base_attack: 4,
                monster_type: MonsterType::Bear,
                behavior: BehaviorState::Idle,
                can_see_player: false,
                last_known_player_pos: None,
                ranged_cooldown: 0,
                turn_parity: 0,
                floor_tier,
                is_berserk: false,
                stun_ticks: 0,
                freeze_ticks: 0,
                poison_ticks: 0,
                attack_reduction: 0,
                web_cooldown: 0,
                summon_count: 0,
                summon_max: 0,
                is_phasing: false,
                death_pos: None,
                is_boss: false,
                boss_tick: 0,
            },
            MonsterType::WolfPack => Self {
                x,
                y,
                symbol: 'w',
                name: "Wolf".to_string(),
                hp: 6,
                max_hp: 6,
                attack: 3,
                base_attack: 3,
                monster_type: MonsterType::WolfPack,
                behavior: BehaviorState::Idle,
                can_see_player: false,
                last_known_player_pos: None,
                ranged_cooldown: 0,
                turn_parity: 0,
                floor_tier,
                is_berserk: false,
                stun_ticks: 0,
                freeze_ticks: 0,
                poison_ticks: 0,
                attack_reduction: 0,
                web_cooldown: 0,
                summon_count: 0,
                summon_max: 0,
                is_phasing: false,
                death_pos: None,
                is_boss: false,
                boss_tick: 0,
            },
            // Humanoids
            MonsterType::Orc => Self {
                x,
                y,
                symbol: 'O',
                name: "Orc".to_string(),
                hp: 14,
                max_hp: 14,
                attack: 5,
                base_attack: 5,
                monster_type: MonsterType::Orc,
                behavior: BehaviorState::Idle,
                can_see_player: false,
                last_known_player_pos: None,
                ranged_cooldown: 0,
                turn_parity: 0,
                floor_tier,
                is_berserk: false,
                stun_ticks: 0,
                freeze_ticks: 0,
                poison_ticks: 0,
                attack_reduction: 0,
                web_cooldown: 0,
                summon_count: 0,
                summon_max: 0,
                is_phasing: false,
                death_pos: None,
                is_boss: false,
                boss_tick: 0,
            },
            MonsterType::Bandit => Self {
                x,
                y,
                symbol: 'b',
                name: "Bandit".to_string(),
                hp: 7,
                max_hp: 7,
                attack: 4,
                base_attack: 4,
                monster_type: MonsterType::Bandit,
                behavior: BehaviorState::Idle,
                can_see_player: false,
                last_known_player_pos: None,
                ranged_cooldown: 0,
                turn_parity: 0,
                floor_tier,
                is_berserk: false,
                stun_ticks: 0,
                freeze_ticks: 0,
                poison_ticks: 0,
                attack_reduction: 0,
                web_cooldown: 0,
                summon_count: 0,
                summon_max: 0,
                is_phasing: false,
                death_pos: None,
                is_boss: false,
                boss_tick: 0,
            },
            MonsterType::Assassin => Self {
                x,
                y,
                symbol: 'a',
                name: "Assassin".to_string(),
                hp: 6,
                max_hp: 6,
                attack: 9,
                base_attack: 9,
                monster_type: MonsterType::Assassin,
                behavior: BehaviorState::Idle,
                can_see_player: false,
                last_known_player_pos: None,
                ranged_cooldown: 0,
                turn_parity: 0,
                floor_tier,
                is_berserk: false,
                stun_ticks: 0,
                freeze_ticks: 0,
                poison_ticks: 0,
                attack_reduction: 0,
                web_cooldown: 0,
                summon_count: 0,
                summon_max: 0,
                is_phasing: false,
                death_pos: None,
                is_boss: false,
                boss_tick: 0,
            },
            // Elementals
            MonsterType::FireElemental => Self {
                x,
                y,
                symbol: 'F',
                name: "Fire Elemental".to_string(),
                hp: 10,
                max_hp: 10,
                attack: 5,
                base_attack: 5,
                monster_type: MonsterType::FireElemental,
                behavior: BehaviorState::Idle,
                can_see_player: false,
                last_known_player_pos: None,
                ranged_cooldown: 0,
                turn_parity: 0,
                floor_tier,
                is_berserk: false,
                stun_ticks: 0,
                freeze_ticks: 0,
                poison_ticks: 0,
                attack_reduction: 0,
                web_cooldown: 0,
                summon_count: 0,
                summon_max: 0,
                is_phasing: false,
                death_pos: None,
                is_boss: false,
                boss_tick: 0,
            },
            MonsterType::EarthElemental => Self {
                x,
                y,
                symbol: 'E',
                name: "Earth Elemental".to_string(),
                hp: 20,
                max_hp: 20,
                attack: 5,
                base_attack: 5,
                monster_type: MonsterType::EarthElemental,
                behavior: BehaviorState::Idle,
                can_see_player: false,
                last_known_player_pos: None,
                ranged_cooldown: 0,
                turn_parity: 0,
                floor_tier,
                is_berserk: false,
                stun_ticks: 0,
                freeze_ticks: 0,
                poison_ticks: 0,
                attack_reduction: 0,
                web_cooldown: 0,
                summon_count: 0,
                summon_max: 0,
                is_phasing: false,
                death_pos: None,
                is_boss: false,
                boss_tick: 0,
            },
            MonsterType::IceElemental => Self {
                x,
                y,
                symbol: 'I',
                name: "Ice Elemental".to_string(),
                hp: 12,
                max_hp: 12,
                attack: 4,
                base_attack: 4,
                monster_type: MonsterType::IceElemental,
                behavior: BehaviorState::Idle,
                can_see_player: false,
                last_known_player_pos: None,
                ranged_cooldown: 0,
                turn_parity: 0,
                floor_tier,
                is_berserk: false,
                stun_ticks: 0,
                freeze_ticks: 0,
                poison_ticks: 0,
                attack_reduction: 0,
                web_cooldown: 0,
                summon_count: 0,
                summon_max: 0,
                is_phasing: false,
                death_pos: None,
                is_boss: false,
                boss_tick: 0,
            },
            // Constructs
            MonsterType::Golem => Self {
                x,
                y,
                symbol: 'G',
                name: "Golem".to_string(),
                hp: 40,
                max_hp: 40,
                attack: 3,
                base_attack: 3,
                monster_type: MonsterType::Golem,
                behavior: BehaviorState::Idle,
                can_see_player: false,
                last_known_player_pos: None,
                ranged_cooldown: 0,
                turn_parity: 1,
                floor_tier,
                is_berserk: false,
                stun_ticks: 0,
                freeze_ticks: 0,
                poison_ticks: 0,
                attack_reduction: 0,
                web_cooldown: 0,
                summon_count: 0,
                summon_max: 0,
                is_phasing: false,
                death_pos: None,
                is_boss: false,
                boss_tick: 0,
            },
            MonsterType::Sentry => Self {
                x,
                y,
                symbol: 's',
                name: "Sentry".to_string(),
                hp: 10,
                max_hp: 10,
                attack: 6,
                base_attack: 6,
                monster_type: MonsterType::Sentry,
                behavior: BehaviorState::Idle,
                can_see_player: false,
                last_known_player_pos: None,
                ranged_cooldown: 0,
                turn_parity: 0,
                floor_tier,
                is_berserk: false,
                stun_ticks: 0,
                freeze_ticks: 0,
                poison_ticks: 0,
                attack_reduction: 0,
                web_cooldown: 0,
                summon_count: 0,
                summon_max: 0,
                is_phasing: false,
                death_pos: None,
                is_boss: false,
                boss_tick: 0,
            },
            MonsterType::Bomber => Self {
                x,
                y,
                symbol: 'o',
                name: "Bomber".to_string(),
                hp: 8,
                max_hp: 8,
                attack: 10,
                base_attack: 10,
                monster_type: MonsterType::Bomber,
                behavior: BehaviorState::Idle,
                can_see_player: false,
                last_known_player_pos: None,
                ranged_cooldown: 0,
                turn_parity: 0,
                floor_tier,
                is_berserk: false,
                stun_ticks: 0,
                freeze_ticks: 0,
                poison_ticks: 0,
                attack_reduction: 0,
                web_cooldown: 0,
                summon_count: 0,
                summon_max: 0,
                is_phasing: false,
                death_pos: None,
                is_boss: false,
                boss_tick: 0,
            },
        }
    }

    pub fn random_monster(x: usize, y: usize, floor: i32) -> Self {
        let mut rng = rand::rng();
        let roll = rng.random_range(0..100);

        // Spawn table scales with floor depth
        let m_type = if floor <= 1 {
            // Floor 1: only goblins
            MonsterType::Goblin
        } else if floor <= 2 {
            // Floor 2: goblins + bat swarms
            if roll < 60 {
                MonsterType::Goblin
            } else {
                MonsterType::BatSwarm
            }
        } else if floor <= 3 {
            // Floor 3: goblins, bat swarms, spiders
            if roll < 40 {
                MonsterType::Goblin
            } else if roll < 65 {
                MonsterType::BatSwarm
            } else if roll < 85 {
                MonsterType::Spider
            } else {
                MonsterType::Skeleton
            }
        } else if floor <= 5 {
            // Floors 4-5: full early roster + wraith
            if roll < 25 {
                MonsterType::Goblin
            } else if roll < 40 {
                MonsterType::BatSwarm
            } else if roll < 55 {
                MonsterType::Spider
            } else if roll < 75 {
                MonsterType::Skeleton
            } else if roll < 90 {
                MonsterType::Wraith
            } else {
                MonsterType::Troll
            }
        } else if floor <= 8 {
            // Floors 6-8: add zombies and ghouls
            if roll < 12 {
                MonsterType::Goblin
            } else if roll < 22 {
                MonsterType::BatSwarm
            } else if roll < 32 {
                MonsterType::Spider
            } else if roll < 42 {
                MonsterType::Skeleton
            } else if roll < 52 {
                MonsterType::Wraith
            } else if roll < 62 {
                MonsterType::Troll
            } else if roll < 75 {
                MonsterType::Necromancer
            } else if roll < 85 {
                MonsterType::Zombie
            } else {
                MonsterType::Ghoul
            }
        } else {
            // Floor 9+: everything including specters
            if roll < 10 {
                MonsterType::Goblin
            } else if roll < 18 {
                MonsterType::BatSwarm
            } else if roll < 26 {
                MonsterType::Spider
            } else if roll < 36 {
                MonsterType::Skeleton
            } else if roll < 46 {
                MonsterType::Wraith
            } else if roll < 56 {
                MonsterType::Troll
            } else if roll < 66 {
                MonsterType::Necromancer
            } else if roll < 76 {
                MonsterType::Zombie
            } else if roll < 86 {
                MonsterType::Ghoul
            } else {
                MonsterType::Specter
            }
        };
        Self::new(x, y, m_type, floor)
    }

    pub fn take_damage(&mut self, amount: i32) {
        self.hp = (self.hp - amount).max(0);
    }

    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }

    /// XP reward for killing this monster.
    pub fn xp_value(&self) -> i32 {
        match self.monster_type {
            MonsterType::Goblin => 10,
            MonsterType::BatSwarm => 15,
            MonsterType::Spider => 15,
            MonsterType::Skeleton => 20,
            MonsterType::Wraith => 30,
            MonsterType::Troll => 40,
            MonsterType::Necromancer => 50,
            MonsterType::GoblinKing => 150,
            MonsterType::BoneDragon => 300,
            MonsterType::ShadowLord => 500,
            MonsterType::Zombie => 18,
            MonsterType::Ghoul => 22,
            MonsterType::Specter => 35,
            // Demons
            MonsterType::Imp => 25,
            MonsterType::Demon => 45,
            MonsterType::Hellfire => 38,
            MonsterType::Minotaur => 60,
            MonsterType::Bear => 45,
            MonsterType::WolfPack => 20,
            // Humanoids
            MonsterType::Orc => 30,
            MonsterType::Bandit => 25,
            MonsterType::Assassin => 55,
            // Elementals
            MonsterType::FireElemental => 40,
            MonsterType::EarthElemental => 50,
            MonsterType::IceElemental => 40,
            // Constructs
            MonsterType::Golem => 70,
            MonsterType::Sentry => 35,
            MonsterType::Bomber => 30,
        }
    }

    /// Main AI decision function. Called each monster turn.
    /// Returns the action the monster wants to take.
    /// `dead_indices` is used by Necromancer to find dead monsters to resurrect.
    pub fn decide_action(
        &mut self,
        player_pos: (usize, usize),
        map: &Map,
        occupied: &[(usize, usize)],
        all_monster_positions: &[(usize, usize)],
        dead_indices: &[usize], // indices of dead monsters (for Necromancer)
    ) -> MonsterAction {
        let dist = Map::distance(self.x, self.y, player_pos.0, player_pos.1);

        // Wraith / Shadow Lord: can see through walls (uses distance only, not LOS)
        if self.monster_type == MonsterType::Wraith || self.monster_type == MonsterType::ShadowLord
        {
            self.can_see_player = dist <= 10;
        } else {
            // Update line of sight
            self.can_see_player =
                dist <= 12 && map.has_line_of_sight(self.x, self.y, player_pos.0, player_pos.1);
        }

        if self.can_see_player {
            self.last_known_player_pos = Some(player_pos);
        }

        // Tick cooldowns
        if self.ranged_cooldown > 0 {
            self.ranged_cooldown -= 1;
        }
        if self.web_cooldown > 0 {
            self.web_cooldown -= 1;
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
            MonsterType::BatSwarm => self.bat_swarm_ai(player_pos, map, occupied, adjacent, dist),
            MonsterType::Spider => self.spider_ai(player_pos, map, occupied, adjacent, dist),
            MonsterType::Wraith => self.wraith_ai(player_pos, map, occupied, adjacent, dist),
            MonsterType::Necromancer => {
                self.necromancer_ai(player_pos, map, occupied, adjacent, dist, dead_indices)
            }
            MonsterType::GoblinKing => {
                self.goblin_king_ai(player_pos, map, occupied, adjacent, dist)
            }
            MonsterType::BoneDragon => {
                self.bone_dragon_ai(player_pos, map, occupied, adjacent, dist)
            }
            MonsterType::ShadowLord => self.shadow_lord_ai(player_pos, map, adjacent, dist),
            // Undead
            MonsterType::Zombie => {
                self.zombie_ai(player_pos, map, occupied, adjacent, low_hp, dist)
            }
            MonsterType::Ghoul => self.ghoul_ai(player_pos, map, occupied, adjacent, low_hp, dist),
            MonsterType::Specter => self.specter_ai(player_pos, map, occupied, adjacent, dist),
            // Demons
            MonsterType::Imp => self.imp_ai(player_pos, map, occupied, adjacent, low_hp, dist),
            MonsterType::Demon => self.demon_ai(player_pos, map, occupied, adjacent, low_hp, dist),
            MonsterType::Hellfire => self.hellfire_ai(player_pos, map, occupied, adjacent, dist),
            // Beasts
            MonsterType::Minotaur => {
                self.minotaur_ai(player_pos, map, occupied, adjacent, low_hp, dist)
            }
            MonsterType::Bear => self.bear_ai(player_pos, map, occupied, adjacent, low_hp, dist),
            MonsterType::WolfPack => self.wolf_ai(player_pos, map, occupied, adjacent, dist),
            // Humanoids
            MonsterType::Orc => self.orc_ai(player_pos, map, occupied, adjacent, low_hp, dist),
            MonsterType::Bandit => self.bandit_ai(player_pos, map, occupied, adjacent, dist),
            MonsterType::Assassin => {
                self.assassin_ai(player_pos, map, occupied, adjacent, low_hp, dist)
            }
            // Elementals
            MonsterType::FireElemental => {
                self.fire_elemental_ai(player_pos, map, occupied, adjacent, dist)
            }
            MonsterType::EarthElemental => {
                self.earth_elemental_ai(player_pos, map, occupied, adjacent, low_hp, dist)
            }
            MonsterType::IceElemental => {
                self.ice_elemental_ai(player_pos, map, occupied, adjacent, dist)
            }
            // Constructs
            MonsterType::Golem => self.golem_ai(player_pos, map, occupied, adjacent, low_hp, dist),
            MonsterType::Sentry => self.sentry_ai(player_pos, map, occupied, adjacent, dist),
            MonsterType::Bomber => self.bomber_ai(player_pos, map, occupied, adjacent, dist),
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

    // ==================== BAT SWARM AI ====================
    // Fast, erratic, swarms. Moves every tick (no parity skip).
    // 30% chance to move randomly. Tier 2+: 25% dodge (handled in main.rs combat).
    fn bat_swarm_ai(
        &mut self,
        player_pos: (usize, usize),
        map: &Map,
        occupied: &[(usize, usize)],
        adjacent: bool,
        _dist: i32,
    ) -> MonsterAction {
        let mut rng = rand::rng();

        // Attack if adjacent
        if adjacent {
            self.behavior = BehaviorState::Attack;
            return MonsterAction::MeleeAttack {
                damage: self.attack,
                name: self.name.clone(),
            };
        }

        if self.can_see_player {
            self.behavior = BehaviorState::Chase;

            // 30% chance to move erratically
            if rng.random_range(0..100) < 30 {
                return self.wander(map, occupied);
            }

            if let Some(next) = map.astar_next_step((self.x, self.y), player_pos, occupied) {
                return MonsterAction::MoveTo(next.0, next.1);
            }
        }

        self.behavior = BehaviorState::Idle;
        self.wander(map, occupied)
    }

    // ==================== SPIDER AI ====================
    // Web trapper. Places webs when LOS to player and 3-5 tiles away.
    // Ambush: hides until player within 4 tiles.
    // Tier 2+: poison bite on melee.
    fn spider_ai(
        &mut self,
        player_pos: (usize, usize),
        map: &Map,
        occupied: &[(usize, usize)],
        adjacent: bool,
        dist: i32,
    ) -> MonsterAction {
        // Attack if adjacent
        if adjacent {
            self.behavior = BehaviorState::Attack;
            if self.floor_tier >= 2 {
                // Poison bite
                return MonsterAction::PoisonAttack {
                    damage: self.attack,
                    name: self.name.clone(),
                    poison_ticks: 2,
                };
            }
            return MonsterAction::MeleeAttack {
                damage: self.attack,
                name: self.name.clone(),
            };
        }

        // Web trap: when we see the player at 3-5 tiles, place web at player position
        if self.can_see_player && dist >= 3 && dist <= 5 && self.web_cooldown <= 0 {
            self.web_cooldown = if self.floor_tier >= 2 { 2 } else { 4 };
            self.behavior = BehaviorState::Ranged;
            return MonsterAction::PlaceWeb(player_pos.0, player_pos.1);
        }

        // Ambush: stay hidden until player within 4 tiles
        if !self.can_see_player || dist > 4 {
            self.behavior = BehaviorState::Idle;
            return MonsterAction::Nothing; // Stay perfectly still
        }

        // Chase (close range)
        self.behavior = BehaviorState::Chase;
        if let Some(next) = map.astar_next_step((self.x, self.y), player_pos, occupied) {
            return MonsterAction::MoveTo(next.0, next.1);
        }

        MonsterAction::Nothing
    }

    // ==================== WRAITH AI ====================
    // Phases through walls. Drains HP on hit. Attacks from walls then retreats.
    // Can only be damaged when on floor tile (handled in main.rs combat).
    fn wraith_ai(
        &mut self,
        player_pos: (usize, usize),
        map: &Map,
        occupied: &[(usize, usize)],
        adjacent: bool,
        dist: i32,
    ) -> MonsterAction {
        // Tier 1: slow (every other tick)
        if self.floor_tier <= 1 {
            self.turn_parity += 1;
            if self.turn_parity % 2 != 0 {
                return MonsterAction::Nothing;
            }
        }

        // If adjacent and on floor: drain attack, then retreat into nearest wall
        if adjacent && !self.is_phasing {
            self.behavior = BehaviorState::Attack;
            return MonsterAction::DrainAttack {
                damage: self.attack,
                name: self.name.clone(),
            };
        }

        // After attacking: retreat into nearest wall
        if self.behavior == BehaviorState::Attack && !self.is_phasing {
            self.behavior = BehaviorState::Retreat;
            // Find nearest wall tile
            if let Some(wall_pos) = self.find_nearest_wall(map) {
                return MonsterAction::MoveToPhase(wall_pos.0, wall_pos.1);
            }
        }

        // If phasing (in wall): move toward player through walls
        if self.is_phasing && self.can_see_player {
            self.behavior = BehaviorState::Chase;
            // Try to emerge adjacent to player
            let neighbors = [
                (player_pos.0.wrapping_sub(1), player_pos.1),
                (player_pos.0 + 1, player_pos.1),
                (player_pos.0, player_pos.1.wrapping_sub(1)),
                (player_pos.0, player_pos.1 + 1),
            ];
            // Prefer floor tiles adjacent to player
            for &(nx, ny) in &neighbors {
                if nx < map.width
                    && ny < map.height
                    && map.is_walkable(nx, ny)
                    && !occupied.contains(&(nx, ny))
                    && !(nx == player_pos.0 && ny == player_pos.1)
                {
                    return MonsterAction::MoveToPhase(nx, ny);
                }
            }
            // Otherwise move through walls toward player
            let dx = (player_pos.0 as i32 - self.x as i32).signum();
            let dy = (player_pos.1 as i32 - self.y as i32).signum();
            let nx = (self.x as i32 + dx) as usize;
            let ny = (self.y as i32 + dy) as usize;
            if nx < map.width && ny < map.height {
                return MonsterAction::MoveToPhase(nx, ny);
            }
        }

        // Not phasing, can see player: move toward them (floor tiles only)
        if self.can_see_player && dist > 1 {
            self.behavior = BehaviorState::Chase;
            if let Some(next) = map.astar_next_step((self.x, self.y), player_pos, occupied) {
                return MonsterAction::MoveTo(next.0, next.1);
            }
            // If no floor path, phase through walls
            let dx = (player_pos.0 as i32 - self.x as i32).signum();
            let dy = (player_pos.1 as i32 - self.y as i32).signum();
            let nx = (self.x as i32 + dx) as usize;
            let ny = (self.y as i32 + dy) as usize;
            if nx < map.width && ny < map.height {
                return MonsterAction::MoveToPhase(nx, ny);
            }
        }

        self.behavior = BehaviorState::Idle;
        self.wander(map, occupied)
    }

    // ==================== NECROMANCER AI ====================
    // Keeps distance (6+ tiles). Every 3 ticks resurrects a dead monster.
    // Coward: runs if player gets close. Tier 3: 75% HP resurrect, more summons.
    fn necromancer_ai(
        &mut self,
        player_pos: (usize, usize),
        map: &Map,
        occupied: &[(usize, usize)],
        adjacent: bool,
        dist: i32,
        dead_indices: &[usize],
    ) -> MonsterAction {
        // Weak melee if cornered
        if adjacent {
            self.behavior = BehaviorState::Attack;
            return MonsterAction::MeleeAttack {
                damage: self.attack,
                name: self.name.clone(),
            };
        }

        // Coward: if player too close (< 6 tiles), flee
        if self.can_see_player && dist < 6 {
            self.behavior = BehaviorState::Retreat;
            return self.flee_from(player_pos, map, occupied);
        }

        // Resurrect: every 3rd tick (use turn_parity counter)
        self.turn_parity += 1;
        let resurrect_interval = if self.floor_tier >= 3 { 4 } else { 6 };
        if self.turn_parity % resurrect_interval == 0
            && self.summon_count < self.summon_max
            && !dead_indices.is_empty()
        {
            // Pick a random dead monster to resurrect
            let mut rng = rand::rng();
            let pick = rng.random_range(0..dead_indices.len());
            self.summon_count += 1;
            return MonsterAction::Resurrect(dead_indices[pick]);
        }

        // Reposition: stay 6-8 tiles from player
        if self.can_see_player {
            if dist > 8 {
                // Move closer
                self.behavior = BehaviorState::Chase;
                if let Some(next) = map.astar_next_step((self.x, self.y), player_pos, occupied) {
                    return MonsterAction::MoveTo(next.0, next.1);
                }
            } else {
                // In good range, hold position
                self.behavior = BehaviorState::Reposition;
                return MonsterAction::Nothing;
            }
        }

        self.behavior = BehaviorState::Idle;
        self.wander(map, occupied)
    }

    // ==================== SHARED BEHAVIOR HELPERS ====================

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

    /// Find the nearest wall tile adjacent to this monster (for Wraith retreat).
    fn find_nearest_wall(&self, map: &Map) -> Option<(usize, usize)> {
        // Search in expanding radius
        for radius in 1..=3 {
            let sx = (self.x as i32 - radius).max(0) as usize;
            let sy = (self.y as i32 - radius).max(0) as usize;
            let ex = ((self.x as i32 + radius) as usize).min(map.width - 1);
            let ey = ((self.y as i32 + radius) as usize).min(map.height - 1);

            let mut best: Option<(usize, usize)> = None;
            let mut best_dist = i32::MAX;

            for tx in sx..=ex {
                for ty in sy..=ey {
                    if map.tiles[tx][ty] == crate::map::Tile::Wall
                        || map.tiles[tx][ty] == crate::map::Tile::SecretDoor
                    {
                        let d = Map::distance(self.x, self.y, tx, ty);
                        if d < best_dist {
                            best_dist = d;
                            best = Some((tx, ty));
                        }
                    }
                }
            }

            if best.is_some() {
                return best;
            }
        }
        None
    }

    /// Check if this monster can dodge (Bat Swarm tier 2+).
    pub fn can_dodge_attack(&self) -> bool {
        if self.monster_type == MonsterType::BatSwarm && self.floor_tier >= 2 {
            let mut rng = rand::rng();
            return rng.random_range(0..100) < 25;
        }
        false
    }

    // ==================== GOBLIN KING AI (Boss - Floor 5) ====================
    // Summons goblin minions every 4 ticks (every 3 when enraged <50% HP).
    // Melee attacks when adjacent. Chases otherwise.
    // Enraged: attacks every tick, summons more frequently.
    fn goblin_king_ai(
        &mut self,
        player_pos: (usize, usize),
        map: &Map,
        occupied: &[(usize, usize)],
        adjacent: bool,
        dist: i32,
    ) -> MonsterAction {
        self.boss_tick += 1;
        let enraged = self.hp * 100 / self.max_hp < 50;
        let summon_interval = if enraged { 3 } else { 4 };

        // Summon minions on interval (if under cap)
        if self.boss_tick % summon_interval == 0 && self.summon_count < self.summon_max {
            self.summon_count += 1;
            return MonsterAction::BossSummon;
        }

        // Melee attack when adjacent
        if adjacent {
            let damage = if enraged {
                self.attack + 2 // Enraged: extra damage
            } else {
                self.attack
            };
            self.behavior = BehaviorState::Attack;
            return MonsterAction::MeleeAttack {
                damage,
                name: self.name.clone(),
            };
        }

        // Chase player
        if self.can_see_player && dist <= 12 {
            self.behavior = BehaviorState::Chase;
            if let Some(next) = map.astar_next_step((self.x, self.y), player_pos, occupied) {
                return MonsterAction::MoveTo(next.0, next.1);
            }
        }

        self.wander(map, occupied)
    }

    // ==================== BONE DRAGON AI (Boss - Floor 10) ====================
    // Slow movement (every 2 ticks). Breath attack in line every 3 ticks (every 2 when enraged).
    // Tail swipe (melee) when adjacent. Moves toward player when not attacking.
    fn bone_dragon_ai(
        &mut self,
        player_pos: (usize, usize),
        map: &Map,
        occupied: &[(usize, usize)],
        adjacent: bool,
        dist: i32,
    ) -> MonsterAction {
        self.boss_tick += 1;
        let enraged = self.hp * 100 / self.max_hp < 50;

        // Breath attack on interval when player is in range + LOS
        let breath_interval = if enraged { 2 } else { 3 };
        let breath_range = if enraged { 5 } else { 3 };
        let breath_damage = 8;

        if self.boss_tick % breath_interval == 0
            && dist <= breath_range
            && dist > 1
            && map.has_line_of_sight(self.x, self.y, player_pos.0, player_pos.1)
        {
            // Calculate direction toward player
            let dx = (player_pos.0 as i32 - self.x as i32).signum();
            let dy = (player_pos.1 as i32 - self.y as i32).signum();
            return MonsterAction::BreathAttack {
                dx,
                dy,
                damage: breath_damage,
                range: breath_range,
            };
        }

        // Tail swipe when adjacent
        if adjacent {
            self.behavior = BehaviorState::Attack;
            return MonsterAction::MeleeAttack {
                damage: self.attack,
                name: self.name.clone(),
            };
        }

        // Slow movement: only move every 2 ticks
        if self.boss_tick % 2 != 0 {
            return MonsterAction::Nothing;
        }

        // Chase player
        if self.can_see_player && dist <= 12 {
            self.behavior = BehaviorState::Chase;
            if let Some(next) = map.astar_next_step((self.x, self.y), player_pos, occupied) {
                return MonsterAction::MoveTo(next.0, next.1);
            }
        }

        self.wander(map, occupied)
    }

    // ==================== SHADOW LORD AI (Boss - Floor 15) ====================
    // Teleports every 3 ticks (every 2 when enraged). Drain attack when adjacent.
    // Shadow pulse AoE every 5 ticks (every 4 when enraged, larger radius).
    fn shadow_lord_ai(
        &mut self,
        player_pos: (usize, usize),
        map: &Map,
        adjacent: bool,
        dist: i32,
    ) -> MonsterAction {
        self.boss_tick += 1;
        let enraged = self.hp * 100 / self.max_hp < 50;

        // Shadow pulse AoE
        let pulse_interval = if enraged { 4 } else { 5 };
        let pulse_radius = if enraged { 3 } else { 2 };
        let pulse_damage = 3;

        if self.boss_tick % pulse_interval == 0 && dist <= pulse_radius + 1 {
            return MonsterAction::ShadowPulse {
                damage: pulse_damage,
                radius: pulse_radius,
            };
        }

        // Teleport periodically
        let tp_interval = if enraged { 2 } else { 3 };
        if self.boss_tick % tp_interval == 0 && dist > 2 {
            return MonsterAction::BossTeleport;
        }

        // Drain attack when adjacent (heal for damage dealt)
        if adjacent {
            self.behavior = BehaviorState::Attack;
            return MonsterAction::DrainAttack {
                damage: self.attack,
                name: self.name.clone(),
            };
        }

        // Move toward player if visible
        if self.can_see_player && dist <= 12 {
            self.behavior = BehaviorState::Chase;
            // Shadow Lord can sense player through walls like Wraith
            let neighbors = [
                (self.x.wrapping_sub(1), self.y),
                (self.x + 1, self.y),
                (self.x, self.y.wrapping_sub(1)),
                (self.x, self.y + 1),
            ];
            let mut best: Option<(usize, usize)> = None;
            let mut best_dist = dist;
            for &(nx, ny) in &neighbors {
                if nx < map.width && ny < map.height && map.is_walkable(nx, ny) {
                    let d = Map::distance(nx, ny, player_pos.0, player_pos.1);
                    if d < best_dist {
                        best_dist = d;
                        best = Some((nx, ny));
                    }
                }
            }
            if let Some((nx, ny)) = best {
                return MonsterAction::MoveTo(nx, ny);
            }
        }

        MonsterAction::Nothing
    }

    // ==================== ZOMBIE AI ====================
    fn zombie_ai(
        &mut self,
        player_pos: (usize, usize),
        map: &Map,
        occupied: &[(usize, usize)],
        adjacent: bool,
        low_hp: bool,
        dist: i32,
    ) -> MonsterAction {
        // Zombie: slow, tough, regenerates HP. Chase player when seen.
        self.hp = (self.hp + 1).min(self.max_hp);

        if adjacent {
            return MonsterAction::MeleeAttack {
                damage: self.attack,
                name: self.name.clone(),
            };
        }

        if self.can_see_player && dist <= 8 {
            if let Some((nx, ny)) = map.astar_next_step((self.x, self.y), player_pos, occupied) {
                return MonsterAction::MoveTo(nx, ny);
            }
        }

        self.behavior = BehaviorState::Idle;
        self.wander(map, occupied)
    }

    fn ghoul_ai(
        &mut self,
        player_pos: (usize, usize),
        map: &Map,
        occupied: &[(usize, usize)],
        adjacent: bool,
        low_hp: bool,
        dist: i32,
    ) -> MonsterAction {
        if adjacent {
            return MonsterAction::PoisonAttack {
                damage: self.attack,
                name: self.name.clone(),
                poison_ticks: 3,
            };
        }

        if self.can_see_player && dist <= 10 {
            if let Some((nx, ny)) = map.astar_next_step((self.x, self.y), player_pos, occupied) {
                return MonsterAction::MoveTo(nx, ny);
            }
        }

        self.behavior = BehaviorState::Idle;
        self.wander(map, occupied)
    }

    // ==================== SPECTER AI ====================
    fn specter_ai(
        &mut self,
        player_pos: (usize, usize),
        map: &Map,
        occupied: &[(usize, usize)],
        adjacent: bool,
        dist: i32,
    ) -> MonsterAction {
        // Specter: silent stalker, high damage, fast. Phases through walls.
        if adjacent {
            return MonsterAction::MeleeAttack {
                damage: self.attack,
                name: self.name.clone(),
            };
        }

        if self.can_see_player && dist <= 6 {
            // Phase through walls
            let dx = player_pos.0 as i32 - self.x as i32;
            let dy = player_pos.1 as i32 - self.y as i32;
            let step_x = if dx != 0 { dx.signum() } else { 0 };
            let step_y = if dy != 0 { dy.signum() } else { 0 };

            let nx = (self.x as i32 + step_x) as usize;
            let ny = (self.y as i32 + step_y) as usize;

            if nx < map.width && ny < map.height {
                return MonsterAction::MoveToPhase(nx, ny);
            }
        }

        MonsterAction::Nothing
    }

    fn imp_ai(
        &mut self,
        player_pos: (usize, usize),
        map: &Map,
        occupied: &[(usize, usize)],
        adjacent: bool,
        _low_hp: bool,
        dist: i32,
    ) -> MonsterAction {
        // Imp: small, fast, ranged fire bolt
        if adjacent {
            return MonsterAction::MeleeAttack {
                damage: self.attack,
                name: self.name.clone(),
            };
        }

        if self.can_see_player && dist <= 5 && self.ranged_cooldown <= 0 {
            self.ranged_cooldown = 3;
            let dx = (player_pos.0 as i32 - self.x as i32).signum();
            let dy = (player_pos.1 as i32 - self.y as i32).signum();
            let symbol = Projectile::symbol_for_direction(dx, dy);
            return MonsterAction::FireProjectile(Projectile {
                x: (self.x as i32 + dx) as usize,
                y: (self.y as i32 + dy) as usize,
                dx,
                dy,
                damage: self.attack,
                symbol,
                source_name: self.name.clone(),
            });
        }

        if self.can_see_player && dist <= 8 {
            if let Some((nx, ny)) = map.astar_next_step((self.x, self.y), player_pos, occupied) {
                return MonsterAction::MoveTo(nx, ny);
            }
        }

        self.behavior = BehaviorState::Idle;
        self.wander(map, occupied)
    }

    fn demon_ai(
        &mut self,
        player_pos: (usize, usize),
        map: &Map,
        occupied: &[(usize, usize)],
        adjacent: bool,
        _low_hp: bool,
        dist: i32,
    ) -> MonsterAction {
        // Demon: tough melee, aggressive
        if adjacent {
            return MonsterAction::MeleeAttack {
                damage: self.attack,
                name: self.name.clone(),
            };
        }

        if self.can_see_player && dist <= 10 {
            if let Some((nx, ny)) = map.astar_next_step((self.x, self.y), player_pos, occupied) {
                return MonsterAction::MoveTo(nx, ny);
            }
        }

        self.behavior = BehaviorState::Idle;
        self.wander(map, occupied)
    }

    fn hellfire_ai(
        &mut self,
        player_pos: (usize, usize),
        map: &Map,
        occupied: &[(usize, usize)],
        adjacent: bool,
        dist: i32,
    ) -> MonsterAction {
        // Hellfire Elemental: AOE fire damage to adjacent tiles
        if adjacent {
            return MonsterAction::PoisonAttack {
                damage: self.attack,
                name: self.name.clone(),
                poison_ticks: 2,
            };
        }

        if self.can_see_player && dist <= 8 {
            if let Some((nx, ny)) = map.astar_next_step((self.x, self.y), player_pos, occupied) {
                return MonsterAction::MoveTo(nx, ny);
            }
        }

        self.behavior = BehaviorState::Idle;
        self.wander(map, occupied)
    }

    fn minotaur_ai(
        &mut self,
        player_pos: (usize, usize),
        map: &Map,
        occupied: &[(usize, usize)],
        adjacent: bool,
        _low_hp: bool,
        dist: i32,
    ) -> MonsterAction {
        if adjacent {
            return MonsterAction::MeleeAttack {
                damage: self.attack,
                name: self.name.clone(),
            };
        }
        if self.can_see_player && dist <= 10 {
            if let Some((nx, ny)) = map.astar_next_step((self.x, self.y), player_pos, occupied) {
                return MonsterAction::MoveTo(nx, ny);
            }
        }
        self.behavior = BehaviorState::Idle;
        self.wander(map, occupied)
    }

    fn bear_ai(
        &mut self,
        player_pos: (usize, usize),
        map: &Map,
        occupied: &[(usize, usize)],
        adjacent: bool,
        low_hp: bool,
        dist: i32,
    ) -> MonsterAction {
        if adjacent {
            return MonsterAction::MeleeAttack {
                damage: if low_hp { self.attack * 2 } else { self.attack },
                name: self.name.clone(),
            };
        }
        if self.can_see_player && dist <= 8 {
            if let Some((nx, ny)) = map.astar_next_step((self.x, self.y), player_pos, occupied) {
                return MonsterAction::MoveTo(nx, ny);
            }
        }
        self.behavior = BehaviorState::Idle;
        self.wander(map, occupied)
    }

    fn wolf_ai(
        &mut self,
        player_pos: (usize, usize),
        map: &Map,
        occupied: &[(usize, usize)],
        adjacent: bool,
        dist: i32,
    ) -> MonsterAction {
        if adjacent {
            return MonsterAction::MeleeAttack {
                damage: self.attack,
                name: self.name.clone(),
            };
        }
        if self.can_see_player && dist <= 12 {
            if let Some((nx, ny)) = map.astar_next_step((self.x, self.y), player_pos, occupied) {
                return MonsterAction::MoveTo(nx, ny);
            }
        }
        self.behavior = BehaviorState::Idle;
        self.wander(map, occupied)
    }

    fn orc_ai(
        &mut self,
        player_pos: (usize, usize),
        map: &Map,
        occupied: &[(usize, usize)],
        adjacent: bool,
        _low_hp: bool,
        dist: i32,
    ) -> MonsterAction {
        if adjacent {
            return MonsterAction::MeleeAttack {
                damage: self.attack,
                name: self.name.clone(),
            };
        }
        if self.can_see_player && dist <= 10 {
            if let Some((nx, ny)) = map.astar_next_step((self.x, self.y), player_pos, occupied) {
                return MonsterAction::MoveTo(nx, ny);
            }
        }
        self.behavior = BehaviorState::Idle;
        self.wander(map, occupied)
    }

    fn bandit_ai(
        &mut self,
        player_pos: (usize, usize),
        map: &Map,
        occupied: &[(usize, usize)],
        adjacent: bool,
        dist: i32,
    ) -> MonsterAction {
        if adjacent {
            return MonsterAction::DrainAttack {
                damage: self.attack,
                name: self.name.clone(),
            };
        }
        if self.can_see_player && dist <= 8 {
            if let Some((nx, ny)) = map.astar_next_step((self.x, self.y), player_pos, occupied) {
                return MonsterAction::MoveTo(nx, ny);
            }
        }
        self.behavior = BehaviorState::Idle;
        self.wander(map, occupied)
    }

    fn assassin_ai(
        &mut self,
        player_pos: (usize, usize),
        map: &Map,
        occupied: &[(usize, usize)],
        adjacent: bool,
        _low_hp: bool,
        dist: i32,
    ) -> MonsterAction {
        if adjacent {
            return MonsterAction::MeleeAttack {
                damage: self.attack,
                name: self.name.clone(),
            };
        }
        if self.can_see_player && dist <= 6 {
            if let Some((nx, ny)) = map.astar_next_step((self.x, self.y), player_pos, occupied) {
                return MonsterAction::MoveTo(nx, ny);
            }
        }
        self.behavior = BehaviorState::Idle;
        self.wander(map, occupied)
    }

    fn fire_elemental_ai(
        &mut self,
        player_pos: (usize, usize),
        map: &Map,
        occupied: &[(usize, usize)],
        adjacent: bool,
        dist: i32,
    ) -> MonsterAction {
        if adjacent {
            return MonsterAction::PoisonAttack {
                damage: self.attack,
                name: self.name.clone(),
                poison_ticks: 2,
            };
        }
        if self.can_see_player && dist <= 8 {
            if let Some((nx, ny)) = map.astar_next_step((self.x, self.y), player_pos, occupied) {
                return MonsterAction::MoveTo(nx, ny);
            }
        }
        self.behavior = BehaviorState::Idle;
        self.wander(map, occupied)
    }

    fn earth_elemental_ai(
        &mut self,
        player_pos: (usize, usize),
        map: &Map,
        occupied: &[(usize, usize)],
        adjacent: bool,
        _low_hp: bool,
        dist: i32,
    ) -> MonsterAction {
        if adjacent {
            return MonsterAction::MeleeAttack {
                damage: self.attack,
                name: self.name.clone(),
            };
        }
        if self.can_see_player && dist <= 8 {
            if let Some((nx, ny)) = map.astar_next_step((self.x, self.y), player_pos, occupied) {
                return MonsterAction::MoveTo(nx, ny);
            }
        }
        self.behavior = BehaviorState::Idle;
        self.wander(map, occupied)
    }

    fn ice_elemental_ai(
        &mut self,
        player_pos: (usize, usize),
        map: &Map,
        occupied: &[(usize, usize)],
        adjacent: bool,
        dist: i32,
    ) -> MonsterAction {
        if adjacent {
            return MonsterAction::PoisonAttack {
                damage: self.attack,
                name: self.name.clone(),
                poison_ticks: 2,
            };
        }
        if self.can_see_player && dist <= 8 {
            if let Some((nx, ny)) = map.astar_next_step((self.x, self.y), player_pos, occupied) {
                return MonsterAction::MoveTo(nx, ny);
            }
        }
        self.behavior = BehaviorState::Idle;
        self.wander(map, occupied)
    }

    fn golem_ai(
        &mut self,
        player_pos: (usize, usize),
        map: &Map,
        occupied: &[(usize, usize)],
        adjacent: bool,
        _low_hp: bool,
        dist: i32,
    ) -> MonsterAction {
        self.turn_parity = (self.turn_parity + 1) % 2;
        if self.turn_parity == 0 {
            return MonsterAction::Nothing;
        }
        if adjacent {
            return MonsterAction::MeleeAttack {
                damage: self.attack,
                name: self.name.clone(),
            };
        }
        if self.can_see_player && dist <= 10 {
            if let Some((nx, ny)) = map.astar_next_step((self.x, self.y), player_pos, occupied) {
                return MonsterAction::MoveTo(nx, ny);
            }
        }
        self.behavior = BehaviorState::Idle;
        self.wander(map, occupied)
    }

    fn sentry_ai(
        &mut self,
        player_pos: (usize, usize),
        map: &Map,
        occupied: &[(usize, usize)],
        adjacent: bool,
        dist: i32,
    ) -> MonsterAction {
        if adjacent {
            return MonsterAction::MeleeAttack {
                damage: self.attack,
                name: self.name.clone(),
            };
        }
        if self.can_see_player && dist <= 6 && self.ranged_cooldown <= 0 {
            self.ranged_cooldown = 2;
            let dx = (player_pos.0 as i32 - self.x as i32).signum();
            let dy = (player_pos.1 as i32 - self.y as i32).signum();
            let symbol = Projectile::symbol_for_direction(dx, dy);
            return MonsterAction::FireProjectile(Projectile {
                x: (self.x as i32 + dx) as usize,
                y: (self.y as i32 + dy) as usize,
                dx,
                dy,
                damage: self.attack,
                symbol,
                source_name: self.name.clone(),
            });
        }
        self.behavior = BehaviorState::Idle;
        self.wander(map, occupied)
    }

    fn bomber_ai(
        &mut self,
        player_pos: (usize, usize),
        map: &Map,
        occupied: &[(usize, usize)],
        adjacent: bool,
        dist: i32,
    ) -> MonsterAction {
        if adjacent {
            return MonsterAction::MeleeAttack {
                damage: self.attack * 2,
                name: self.name.clone(),
            };
        }
        if self.can_see_player && dist <= 6 {
            if let Some((nx, ny)) = map.astar_next_step((self.x, self.y), player_pos, occupied) {
                return MonsterAction::MoveTo(nx, ny);
            }
        }
        self.behavior = BehaviorState::Idle;
        self.wander(map, occupied)
    }
}
