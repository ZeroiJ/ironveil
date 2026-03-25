use rand::RngExt;
use serde::{Deserialize, Serialize};

// --- Item Types ---

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ItemType {
    Weapon,
    Armor,
    Ring,
    Potion,
}

// --- Item ---

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub item_type: ItemType,
    pub symbol: char,
    pub damage_bonus: i32,
    pub defense_bonus: i32,
    pub stat_bonus_type: String, // "STR", "DEX", "INT", "CON", or ""
    pub stat_bonus_value: i32,
    pub heal_amount: i32,
    #[allow(dead_code)]
    pub floor_level: i32,
}

impl Item {
    pub fn display_name(&self) -> String {
        match self.item_type {
            ItemType::Weapon => format!("{} (+{} dmg)", self.name, self.damage_bonus),
            ItemType::Armor => format!("{} (+{} def)", self.name, self.defense_bonus),
            ItemType::Ring => {
                if !self.stat_bonus_type.is_empty() {
                    format!(
                        "{} (+{} {})",
                        self.name, self.stat_bonus_value, self.stat_bonus_type
                    )
                } else {
                    self.name.clone()
                }
            }
            ItemType::Potion => format!("{} (heals {})", self.name, self.heal_amount),
        }
    }
}

// --- Item Generation ---

/// Generate a random item appropriate for the given floor.
pub fn random_item(floor: i32) -> Item {
    let mut rng = rand::rng();
    let roll = rng.random_range(0..100);

    if roll < 40 {
        random_potion()
    } else if roll < 70 {
        random_weapon(floor)
    } else if roll < 90 {
        random_armor(floor)
    } else {
        random_ring(floor)
    }
}

/// Generate a random item weighted for monster drops.
pub fn random_drop(floor: i32) -> Item {
    let mut rng = rand::rng();
    let roll = rng.random_range(0..100);

    if roll < 50 {
        random_potion()
    } else if roll < 75 {
        random_weapon(floor)
    } else if roll < 90 {
        random_armor(floor)
    } else {
        random_ring(floor)
    }
}

pub fn random_potion() -> Item {
    Item {
        name: "Health Potion".to_string(),
        item_type: ItemType::Potion,
        symbol: '!',
        damage_bonus: 0,
        defense_bonus: 0,
        stat_bonus_type: String::new(),
        stat_bonus_value: 0,
        heal_amount: 7,
        floor_level: 1,
    }
}

pub fn random_weapon(floor: i32) -> Item {
    let tier = floor_tier(floor);
    match tier {
        1 => Item {
            name: "Dagger".to_string(),
            item_type: ItemType::Weapon,
            symbol: '/',
            damage_bonus: 1,
            defense_bonus: 0,
            stat_bonus_type: String::new(),
            stat_bonus_value: 0,
            heal_amount: 0,
            floor_level: floor,
        },
        2 => Item {
            name: "Shortsword".to_string(),
            item_type: ItemType::Weapon,
            symbol: '/',
            damage_bonus: 2,
            defense_bonus: 0,
            stat_bonus_type: String::new(),
            stat_bonus_value: 0,
            heal_amount: 0,
            floor_level: floor,
        },
        _ => {
            let mut rng = rand::rng();
            if rng.random_bool(0.5) {
                Item {
                    name: "Longsword".to_string(),
                    item_type: ItemType::Weapon,
                    symbol: '/',
                    damage_bonus: 3,
                    defense_bonus: 0,
                    stat_bonus_type: String::new(),
                    stat_bonus_value: 0,
                    heal_amount: 0,
                    floor_level: floor,
                }
            } else {
                Item {
                    name: "Greataxe".to_string(),
                    item_type: ItemType::Weapon,
                    symbol: '/',
                    damage_bonus: 5,
                    defense_bonus: 0,
                    stat_bonus_type: String::new(),
                    stat_bonus_value: 0,
                    heal_amount: 0,
                    floor_level: floor,
                }
            }
        }
    }
}

pub fn random_armor(floor: i32) -> Item {
    let tier = floor_tier(floor);
    match tier {
        1 => Item {
            name: "Leather Armor".to_string(),
            item_type: ItemType::Armor,
            symbol: '[',
            damage_bonus: 0,
            defense_bonus: 1,
            stat_bonus_type: String::new(),
            stat_bonus_value: 0,
            heal_amount: 0,
            floor_level: floor,
        },
        2 => Item {
            name: "Chainmail".to_string(),
            item_type: ItemType::Armor,
            symbol: '[',
            damage_bonus: 0,
            defense_bonus: 2,
            stat_bonus_type: String::new(),
            stat_bonus_value: 0,
            heal_amount: 0,
            floor_level: floor,
        },
        _ => Item {
            name: "Plate Armor".to_string(),
            item_type: ItemType::Armor,
            symbol: '[',
            damage_bonus: 0,
            defense_bonus: 4,
            stat_bonus_type: String::new(),
            stat_bonus_value: 0,
            heal_amount: 0,
            floor_level: floor,
        },
    }
}

pub fn random_ring(floor: i32) -> Item {
    let mut rng = rand::rng();
    let tier = floor_tier(floor);
    let bonus = match tier {
        1 => 1,
        2 => 2,
        _ => 3,
    };

    let stat_roll = rng.random_range(0..4);
    let (stat_name, ring_name) = match stat_roll {
        0 => ("STR", "Ring of Strength"),
        1 => ("DEX", "Ring of Agility"),
        2 => ("INT", "Ring of Intellect"),
        _ => ("CON", "Ring of Vitality"),
    };

    Item {
        name: ring_name.to_string(),
        item_type: ItemType::Ring,
        symbol: '=',
        damage_bonus: 0,
        defense_bonus: 0,
        stat_bonus_type: stat_name.to_string(),
        stat_bonus_value: bonus,
        heal_amount: 0,
        floor_level: floor,
    }
}

fn floor_tier(floor: i32) -> i32 {
    if floor <= 3 {
        1
    } else if floor <= 6 {
        2
    } else {
        3
    }
}

// --- Starting Equipment ---

pub fn warrior_starting_weapon() -> Item {
    Item {
        name: "Iron Shortsword".to_string(),
        item_type: ItemType::Weapon,
        symbol: '/',
        damage_bonus: 2,
        defense_bonus: 0,
        stat_bonus_type: String::new(),
        stat_bonus_value: 0,
        heal_amount: 0,
        floor_level: 1,
    }
}

pub fn warrior_starting_armor() -> Item {
    Item {
        name: "Leather Armor".to_string(),
        item_type: ItemType::Armor,
        symbol: '[',
        damage_bonus: 0,
        defense_bonus: 1,
        stat_bonus_type: String::new(),
        stat_bonus_value: 0,
        heal_amount: 0,
        floor_level: 1,
    }
}

pub fn rogue_starting_weapon() -> Item {
    Item {
        name: "Twin Daggers".to_string(),
        item_type: ItemType::Weapon,
        symbol: '/',
        damage_bonus: 1,
        defense_bonus: 0,
        stat_bonus_type: String::new(),
        stat_bonus_value: 0,
        heal_amount: 0,
        floor_level: 1,
    }
}

pub fn mage_starting_weapon() -> Item {
    Item {
        name: "Wooden Staff".to_string(),
        item_type: ItemType::Weapon,
        symbol: '/',
        damage_bonus: 1,
        defense_bonus: 0,
        stat_bonus_type: String::new(),
        stat_bonus_value: 0,
        heal_amount: 0,
        floor_level: 1,
    }
}

pub fn mage_starting_ring() -> Item {
    Item {
        name: "Ring of Intellect".to_string(),
        item_type: ItemType::Ring,
        symbol: '=',
        damage_bonus: 0,
        defense_bonus: 0,
        stat_bonus_type: "INT".to_string(),
        stat_bonus_value: 2,
        heal_amount: 0,
        floor_level: 1,
    }
}
