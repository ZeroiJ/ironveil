use rand::RngExt;
use serde::{Deserialize, Serialize};

// --- Item Rarity ---

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

impl Rarity {
    pub fn color(&self) -> crossterm::style::Color {
        match self {
            Rarity::Common => crossterm::style::Color::White,
            Rarity::Uncommon => crossterm::style::Color::Green,
            Rarity::Rare => crossterm::style::Color::Cyan,
            Rarity::Epic => crossterm::style::Color::Magenta,
            Rarity::Legendary => crossterm::style::Color::Yellow,
        }
    }

    pub fn label(&self) -> &str {
        match self {
            Rarity::Common => "COMMON",
            Rarity::Uncommon => "UNCOMMON",
            Rarity::Rare => "RARE",
            Rarity::Epic => "EPIC",
            Rarity::Legendary => "LEGENDARY",
        }
    }
}

pub fn roll_rarity(floor: i32) -> Rarity {
    let mut rng = rand::rng();

    let legendary_chance = (0.001 + floor as f32 * 0.0005).min(0.02);
    let epic_chance = (0.01 + floor as f32 * 0.003).min(0.08);
    let rare_chance = (0.05 + floor as f32 * 0.01).min(0.20);
    let uncommon_chance = (0.20 + floor as f32 * 0.02).min(0.40);

    let roll: f32 = rng.random_range(0.0..1.0);

    if roll < legendary_chance {
        Rarity::Legendary
    } else if roll < epic_chance {
        Rarity::Epic
    } else if roll < rare_chance {
        Rarity::Rare
    } else if roll < uncommon_chance {
        Rarity::Uncommon
    } else {
        Rarity::Common
    }
}

// --- Artifact Effects ---

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ArtifactEffect {
    None,
    // Warrior artifacts
    Ragefang,       // kill = +1 atk buff for 3 ticks, stacks 5
    StonehidePlate, // below 30% HP = +3 def, triggers once
    WarlordSignet,  // WarCry also grants +3 atk for 2 ticks
    // Rogue artifacts
    Shadowfang, // post-ShadowStep attacks always crit
    // poison deals 2/tick instead of 1
    Wraithwalkers, // +15% dodge, on dodge next hit = 2x dmg
    Venomcoil,     // PoisonBlade = 5 hits, poisoned enemies take +2 dmg
    // Mage artifacts
    StormcallerStaff, // ChainLightning hits 5 targets, +1 dmg/chain
    FrostweavRobe,    // FrostNova radius 5, frozen kills explode (3 dmg)
    MindFireCrown,    // every 10 kills, next ability = 2x dmg
}

impl ArtifactEffect {
    pub fn description(&self) -> &str {
        match self {
            ArtifactEffect::Ragefang => "Kill = +1 ATK for 3 ticks (max 5 stacks)",
            ArtifactEffect::StonehidePlate => "Below 30% HP: gain +3 DEF permanently (once)",
            ArtifactEffect::WarlordSignet => "War Cry also grants +3 ATK for 2 ticks",
            ArtifactEffect::Shadowfang => {
                "Post-ShadowStep attacks always crit. Poison deals +1/tick"
            }
            ArtifactEffect::Wraithwalkers => "+15% dodge. On dodge: next hit = 2x dmg",
            ArtifactEffect::Venomcoil => "Poison Blade: 5 hits. Poisoned = +2 dmg",
            ArtifactEffect::StormcallerStaff => "Chain Lightning hits 5 targets, +1 dmg/chain",
            ArtifactEffect::FrostweavRobe => "Frost Nova radius 5. Frozen kills explode",
            ArtifactEffect::MindFireCrown => "Every 10 kills: next ability = 2x dmg",
            ArtifactEffect::None => "",
        }
    }
}

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
    pub stat_bonus_type: String,
    pub stat_bonus_value: i32,
    pub heal_amount: i32,
    #[allow(dead_code)]
    pub floor_level: i32,
    pub rarity: Rarity,
    pub is_artifact: bool,
    pub artifact_effect: ArtifactEffect,
}

impl Item {
    pub fn display_name(&self) -> String {
        let base = match self.item_type {
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
        };
        let rarity_suffix = if self.rarity == Rarity::Common {
            String::new()
        } else {
            format!(" [{}]", self.rarity.label())
        };
        if self.is_artifact {
            format!("* {} *{}", base, rarity_suffix)
        } else {
            format!("{}{}", base, rarity_suffix)
        }
    }

    pub fn artifact_description(&self) -> &str {
        self.artifact_effect.description()
    }
}

// --- Item Generation ---

/// Generate a random item appropriate for the given floor.
pub fn random_item(floor: i32) -> Item {
    let rarity = roll_rarity(floor);
    let mut rng = rand::rng();
    let roll = rng.random_range(0..100);

    if roll < 40 {
        let mut item = random_potion();
        item.rarity = rarity;
        item
    } else if roll < 70 {
        let mut item = random_weapon(floor);
        item.rarity = rarity;
        item
    } else if roll < 90 {
        let mut item = random_armor(floor);
        item.rarity = rarity;
        item
    } else {
        let mut item = random_ring(floor);
        item.rarity = rarity;
        item
    }
}

/// Generate a random item weighted for monster drops.
pub fn random_drop(floor: i32) -> Item {
    let rarity = roll_rarity(floor);
    let mut rng = rand::rng();
    let roll = rng.random_range(0..100);

    if roll < 50 {
        let mut item = random_potion();
        item.rarity = rarity;
        item
    } else if roll < 75 {
        let mut item = random_weapon(floor);
        item.rarity = rarity;
        item
    } else if roll < 90 {
        let mut item = random_armor(floor);
        item.rarity = rarity;
        item
    } else {
        let mut item = random_ring(floor);
        item.rarity = rarity;
        item
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
        rarity: Rarity::Common,
        is_artifact: false,
        artifact_effect: ArtifactEffect::None,
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
            rarity: Rarity::Common,
            is_artifact: false,
            artifact_effect: ArtifactEffect::None,
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
            rarity: Rarity::Common,
            is_artifact: false,
            artifact_effect: ArtifactEffect::None,
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
                    rarity: Rarity::Common,
                    is_artifact: false,
                    artifact_effect: ArtifactEffect::None,
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
                    rarity: Rarity::Common,
                    is_artifact: false,
                    artifact_effect: ArtifactEffect::None,
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
            rarity: Rarity::Common,
            is_artifact: false,
            artifact_effect: ArtifactEffect::None,
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
            rarity: Rarity::Common,
            is_artifact: false,
            artifact_effect: ArtifactEffect::None,
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
            rarity: Rarity::Common,
            is_artifact: false,
            artifact_effect: ArtifactEffect::None,
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
        rarity: Rarity::Common,
        is_artifact: false,
        artifact_effect: ArtifactEffect::None,
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

// --- Artifact Items ---

pub fn artifact_ragefang() -> Item {
    Item {
        name: "Ragefang".to_string(),
        item_type: ItemType::Weapon,
        symbol: '/',
        damage_bonus: 3,
        defense_bonus: 0,
        stat_bonus_type: String::new(),
        stat_bonus_value: 0,
        heal_amount: 0,
        floor_level: 6,
        rarity: Rarity::Legendary,
        is_artifact: true,
        artifact_effect: ArtifactEffect::Ragefang,
    }
}

pub fn artifact_stonehide() -> Item {
    Item {
        name: "Stonehide Plate".to_string(),
        item_type: ItemType::Armor,
        symbol: '[',
        damage_bonus: 0,
        defense_bonus: 4,
        stat_bonus_type: String::new(),
        stat_bonus_value: 0,
        heal_amount: 0,
        floor_level: 6,
        rarity: Rarity::Legendary,
        is_artifact: true,
        artifact_effect: ArtifactEffect::StonehidePlate,
    }
}

pub fn artifact_warlord_signet() -> Item {
    Item {
        name: "Warlord's Signet".to_string(),
        item_type: ItemType::Ring,
        symbol: '=',
        damage_bonus: 0,
        defense_bonus: 0,
        stat_bonus_type: "STR".to_string(),
        stat_bonus_value: 2,
        heal_amount: 0,
        floor_level: 6,
        rarity: Rarity::Legendary,
        is_artifact: true,
        artifact_effect: ArtifactEffect::WarlordSignet,
    }
}

pub fn artifact_shadowfang() -> Item {
    Item {
        name: "Shadowfang".to_string(),
        item_type: ItemType::Weapon,
        symbol: '/',
        damage_bonus: 2,
        defense_bonus: 0,
        stat_bonus_type: String::new(),
        stat_bonus_value: 0,
        heal_amount: 0,
        floor_level: 6,
        rarity: Rarity::Legendary,
        is_artifact: true,
        artifact_effect: ArtifactEffect::Shadowfang,
    }
}

pub fn artifact_wraithwalkers() -> Item {
    Item {
        name: "Wraithwalkers".to_string(),
        item_type: ItemType::Armor,
        symbol: '[',
        damage_bonus: 0,
        defense_bonus: 1,
        stat_bonus_type: "DEX".to_string(),
        stat_bonus_value: 3,
        heal_amount: 0,
        floor_level: 6,
        rarity: Rarity::Legendary,
        is_artifact: true,
        artifact_effect: ArtifactEffect::Wraithwalkers,
    }
}

pub fn artifact_venomcoil() -> Item {
    Item {
        name: "Venomcoil Ring".to_string(),
        item_type: ItemType::Ring,
        symbol: '=',
        damage_bonus: 0,
        defense_bonus: 0,
        stat_bonus_type: "DEX".to_string(),
        stat_bonus_value: 2,
        heal_amount: 0,
        floor_level: 6,
        rarity: Rarity::Legendary,
        is_artifact: true,
        artifact_effect: ArtifactEffect::Venomcoil,
    }
}

pub fn artifact_stormcaller() -> Item {
    Item {
        name: "Stormcaller Staff".to_string(),
        item_type: ItemType::Weapon,
        symbol: '/',
        damage_bonus: 2,
        defense_bonus: 0,
        stat_bonus_type: "INT".to_string(),
        stat_bonus_value: 3,
        heal_amount: 0,
        floor_level: 6,
        rarity: Rarity::Legendary,
        is_artifact: true,
        artifact_effect: ArtifactEffect::StormcallerStaff,
    }
}

pub fn artifact_frostweavrobe() -> Item {
    Item {
        name: "Frostweave Robe".to_string(),
        item_type: ItemType::Armor,
        symbol: '[',
        damage_bonus: 0,
        defense_bonus: 2,
        stat_bonus_type: "INT".to_string(),
        stat_bonus_value: 2,
        heal_amount: 0,
        floor_level: 6,
        rarity: Rarity::Legendary,
        is_artifact: true,
        artifact_effect: ArtifactEffect::FrostweavRobe,
    }
}

pub fn artifact_mindfire() -> Item {
    Item {
        name: "Mindfire Crown".to_string(),
        item_type: ItemType::Ring,
        symbol: '=',
        damage_bonus: 0,
        defense_bonus: 0,
        stat_bonus_type: "INT".to_string(),
        stat_bonus_value: 4,
        heal_amount: 0,
        floor_level: 6,
        rarity: Rarity::Legendary,
        is_artifact: true,
        artifact_effect: ArtifactEffect::MindFireCrown,
    }
}

pub fn random_artifact(class_name: &str) -> Item {
    let mut rng = rand::rng();
    match class_name {
        "Warrior" => match rng.random_range(0..3) {
            0 => artifact_ragefang(),
            1 => artifact_stonehide(),
            _ => artifact_warlord_signet(),
        },
        "Rogue" => match rng.random_range(0..3) {
            0 => artifact_shadowfang(),
            1 => artifact_wraithwalkers(),
            _ => artifact_venomcoil(),
        },
        "Mage" => match rng.random_range(0..3) {
            0 => artifact_stormcaller(),
            1 => artifact_frostweavrobe(),
            _ => artifact_mindfire(),
        },
        _ => artifact_ragefang(),
    }
}

pub fn named_shadow_slicer() -> Item {
    Item {
        name: "Shadow Slicer".to_string(),
        item_type: ItemType::Weapon,
        symbol: '/',
        damage_bonus: 4,
        defense_bonus: 0,
        stat_bonus_type: "DEX".to_string(),
        stat_bonus_value: 1,
        heal_amount: 0,
        floor_level: 4,
        rarity: Rarity::Rare,
        is_artifact: false,
        artifact_effect: ArtifactEffect::None,
    }
}

pub fn named_bone_crusher() -> Item {
    Item {
        name: "Bone Crusher".to_string(),
        item_type: ItemType::Weapon,
        symbol: '/',
        damage_bonus: 6,
        defense_bonus: 0,
        stat_bonus_type: "STR".to_string(),
        stat_bonus_value: 2,
        heal_amount: 0,
        floor_level: 5,
        rarity: Rarity::Rare,
        is_artifact: false,
        artifact_effect: ArtifactEffect::None,
    }
}

pub fn named_spellbound_staff() -> Item {
    Item {
        name: "Spellbound Staff".to_string(),
        item_type: ItemType::Weapon,
        symbol: '/',
        damage_bonus: 3,
        defense_bonus: 0,
        stat_bonus_type: "INT".to_string(),
        stat_bonus_value: 4,
        heal_amount: 0,
        floor_level: 5,
        rarity: Rarity::Rare,
        is_artifact: false,
        artifact_effect: ArtifactEffect::None,
    }
}

pub fn named_veterans_plate() -> Item {
    Item {
        name: "Veteran's Plate".to_string(),
        item_type: ItemType::Armor,
        symbol: '[',
        damage_bonus: 0,
        defense_bonus: 5,
        stat_bonus_type: "CON".to_string(),
        stat_bonus_value: 2,
        heal_amount: 0,
        floor_level: 5,
        rarity: Rarity::Rare,
        is_artifact: false,
        artifact_effect: ArtifactEffect::None,
    }
}

pub fn named_swiftboots() -> Item {
    Item {
        name: "Swiftboots".to_string(),
        item_type: ItemType::Armor,
        symbol: '[',
        damage_bonus: 0,
        defense_bonus: 1,
        stat_bonus_type: "DEX".to_string(),
        stat_bonus_value: 4,
        heal_amount: 0,
        floor_level: 4,
        rarity: Rarity::Rare,
        is_artifact: false,
        artifact_effect: ArtifactEffect::None,
    }
}

pub fn named_sages_robe() -> Item {
    Item {
        name: "Sage's Robe".to_string(),
        item_type: ItemType::Armor,
        symbol: '[',
        damage_bonus: 0,
        defense_bonus: 2,
        stat_bonus_type: "INT".to_string(),
        stat_bonus_value: 3,
        heal_amount: 0,
        floor_level: 4,
        rarity: Rarity::Rare,
        is_artifact: false,
        artifact_effect: ArtifactEffect::None,
    }
}

pub fn named_lions_amulet() -> Item {
    Item {
        name: "Lion's Amulet".to_string(),
        item_type: ItemType::Ring,
        symbol: '=',
        damage_bonus: 0,
        defense_bonus: 0,
        stat_bonus_type: "STR".to_string(),
        stat_bonus_value: 4,
        heal_amount: 0,
        floor_level: 5,
        rarity: Rarity::Rare,
        is_artifact: false,
        artifact_effect: ArtifactEffect::None,
    }
}

pub fn named_eagles_eye() -> Item {
    Item {
        name: "Eagle's Eye".to_string(),
        item_type: ItemType::Ring,
        symbol: '=',
        damage_bonus: 0,
        defense_bonus: 0,
        stat_bonus_type: "DEX".to_string(),
        stat_bonus_value: 4,
        heal_amount: 0,
        floor_level: 5,
        rarity: Rarity::Rare,
        is_artifact: false,
        artifact_effect: ArtifactEffect::None,
    }
}

pub fn named_dragon_heart() -> Item {
    Item {
        name: "Dragon Heart".to_string(),
        item_type: ItemType::Ring,
        symbol: '=',
        damage_bonus: 0,
        defense_bonus: 0,
        stat_bonus_type: "CON".to_string(),
        stat_bonus_value: 4,
        heal_amount: 0,
        floor_level: 5,
        rarity: Rarity::Rare,
        is_artifact: false,
        artifact_effect: ArtifactEffect::None,
    }
}

pub fn random_named_item(floor: i32) -> Option<Item> {
    let mut rng = rand::rng();
    if floor < 4 {
        None
    } else if floor < 6 {
        let roll = rng.random_range(0..4);
        match roll {
            0 => Some(named_shadow_slicer()),
            1 => Some(named_swiftboots()),
            2 => Some(named_sages_robe()),
            _ => None,
        }
    } else {
        let roll = rng.random_range(0..10);
        match roll {
            0 => Some(named_bone_crusher()),
            1 => Some(named_spellbound_staff()),
            2 => Some(named_veterans_plate()),
            3 => Some(named_swiftboots()),
            4 => Some(named_sages_robe()),
            5 => Some(named_lions_amulet()),
            6 => Some(named_eagles_eye()),
            7 => Some(named_dragon_heart()),
            _ => None,
        }
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
        rarity: Rarity::Uncommon,
        is_artifact: false,
        artifact_effect: ArtifactEffect::None,
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
        rarity: Rarity::Uncommon,
        is_artifact: false,
        artifact_effect: ArtifactEffect::None,
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
        rarity: Rarity::Uncommon,
        is_artifact: false,
        artifact_effect: ArtifactEffect::None,
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
        rarity: Rarity::Uncommon,
        is_artifact: false,
        artifact_effect: ArtifactEffect::None,
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
        rarity: Rarity::Uncommon,
        is_artifact: false,
        artifact_effect: ArtifactEffect::None,
    }
}
