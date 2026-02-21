use rand::{Rng, RngExt};

#[derive(Clone)]
pub enum MonsterType {
    Goblin,
    Skeleton,
    Troll,
}

pub struct Monster {
    pub x: usize,
    pub y: usize,
    pub symbol: char,
    pub name: String,
    pub hp: i32,
    pub max_hp: i32,
    pub attack: i32,
    pub monster_type: MonsterType,
}

impl Monster {
    pub fn new(x: usize, y: usize, monster_type: MonsterType) -> Self {
        match monster_type {
            MonsterType::Goblin => Self {
                x, y, symbol: 'g', name: "Goblin".to_string(),
                hp: 6, max_hp: 6, attack: 2, monster_type: MonsterType::Goblin,
            },
            MonsterType::Skeleton => Self {
                x, y, symbol: 's', name: "Skeleton".to_string(),
                hp: 10, max_hp: 10, attack: 4, monster_type: MonsterType::Skeleton,
            },
            MonsterType::Troll => Self {
                x, y, symbol: 'T', name: "Troll".to_string(),
                hp: 20, max_hp: 20, attack: 8, monster_type: MonsterType::Troll,
            },
        }
    }

    pub fn random_monster(x: usize, y: usize) -> Self {
        let mut rng = rand::rng();
        let roll = rng.random_range(0..100);
        
        let m_type = if roll < 60 {
            MonsterType::Goblin
        } else if roll < 90 {
            MonsterType::Skeleton
        } else {
            MonsterType::Troll
        };

        Self::new(x, y, m_type)
    }

    pub fn take_damage(&mut self, amount: i32) {
        self.hp = (self.hp - amount).max(0);
    }

    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }
}
