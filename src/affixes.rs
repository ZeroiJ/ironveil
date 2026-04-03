use rand::RngExt;
use serde::{Deserialize, Serialize};

use crate::items::{Item, ItemType, Rarity};

// --- Prefixes ---

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Prefix {
    Sharp,
    Vicious,
    Brutal,
    Sturdy,
    Fortified,
    Iron,
    Swift,
    Mighty,
    Arcane,
    Vital,
    Vampiric,
    Hasty,
}

impl Prefix {
    pub fn name(&self) -> &'static str {
        match self {
            Prefix::Sharp => "Sharp",
            Prefix::Vicious => "Vicious",
            Prefix::Brutal => "Brutal",
            Prefix::Sturdy => "Sturdy",
            Prefix::Fortified => "Fortified",
            Prefix::Iron => "Iron",
            Prefix::Swift => "Swift",
            Prefix::Mighty => "Mighty",
            Prefix::Arcane => "Arcane",
            Prefix::Vital => "Vital",
            Prefix::Vampiric => "Vampiric",
            Prefix::Hasty => "Hasty",
        }
    }

    pub fn min_floor(&self) -> i32 {
        match self {
            Prefix::Sharp
            | Prefix::Sturdy
            | Prefix::Swift
            | Prefix::Mighty
            | Prefix::Arcane
            | Prefix::Vital => 1,
            Prefix::Vicious | Prefix::Fortified => 4,
            Prefix::Vampiric => 6,
            Prefix::Brutal | Prefix::Iron | Prefix::Hasty => 8,
        }
    }

    pub fn applies_to(&self, item_type: &ItemType) -> bool {
        match self {
            Prefix::Sharp | Prefix::Vicious | Prefix::Brutal | Prefix::Vampiric | Prefix::Hasty => {
                *item_type == ItemType::Weapon
            }
            Prefix::Sturdy | Prefix::Fortified | Prefix::Iron => *item_type == ItemType::Armor,
            Prefix::Swift => *item_type == ItemType::Armor || *item_type == ItemType::Ring,
            Prefix::Mighty | Prefix::Arcane | Prefix::Vital => true,
        }
    }
}

// --- Suffixes ---

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Suffix {
    Health,
    Warding,
    OfTheBear,
    OfTheFox,
    OfTheOwl,
    OfTheOx,
    OfHaste,
    OfFrost,
    OfFlame,
    OfTheLeech,
}

impl Suffix {
    pub fn name(&self) -> &'static str {
        match self {
            Suffix::Health => "of Health",
            Suffix::Warding => "of Warding",
            Suffix::OfTheBear => "of the Bear",
            Suffix::OfTheFox => "of the Fox",
            Suffix::OfTheOwl => "of the Owl",
            Suffix::OfTheOx => "of the Ox",
            Suffix::OfHaste => "of Haste",
            Suffix::OfFrost => "of Frost",
            Suffix::OfFlame => "of Flame",
            Suffix::OfTheLeech => "of the Leech",
        }
    }

    pub fn min_floor(&self) -> i32 {
        match self {
            Suffix::Health => 1,
            Suffix::OfTheBear | Suffix::OfTheFox | Suffix::OfTheOwl | Suffix::OfTheOx => 4,
            Suffix::Warding => 6,
            Suffix::OfFrost | Suffix::OfFlame => 8,
            Suffix::OfHaste | Suffix::OfTheLeech => 10,
        }
    }

    pub fn applies_to(&self, item_type: &ItemType) -> bool {
        match self {
            Suffix::Health | Suffix::Warding => true,
            Suffix::OfTheBear | Suffix::OfTheFox | Suffix::OfTheOwl | Suffix::OfTheOx => {
                *item_type == ItemType::Ring || *item_type == ItemType::Armor
            }
            Suffix::OfHaste | Suffix::OfFrost | Suffix::OfFlame | Suffix::OfTheLeech => {
                *item_type == ItemType::Weapon
            }
        }
    }
}

// --- Exotic Types ---

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ExoticType {
    Bloodthirst,
    SoulReaper,
    WorldEater,
    WhisperOfTheVoid,
    CrimsonDancer,
    Godsbane,
    ThePeacemaker,
    LastBreath,
    PhoenixDown,
    FrostbiteGauntlets,
    StormcallersMantle,
    ShadowWeaveBoots,
    TheIronMaiden,
    SkinOfTheHydra,
    AegisOfTheFallen,
    CowardsCloak,
    BerserkersPlate,
    ShroudOfTheNameless,
    TheGlassCannon,
    TimeWeaversRing,
    RingOfTheGambler,
    TheMartyrsBand,
    Ouroboros,
    TheHoardersSignet,
    RingOfEchoes,
    ThePacifistsOath,
    FatesThread,
}

impl ExoticType {
    pub fn name(&self) -> &'static str {
        match self {
            ExoticType::Bloodthirst => "Bloodthirst",
            ExoticType::SoulReaper => "Soul Reaper",
            ExoticType::WorldEater => "The World-Eater",
            ExoticType::WhisperOfTheVoid => "Whisper of the Void",
            ExoticType::CrimsonDancer => "Crimson Dancer",
            ExoticType::Godsbane => "Godsbane",
            ExoticType::ThePeacemaker => "The Peacemaker",
            ExoticType::LastBreath => "Last Breath",
            ExoticType::PhoenixDown => "Phoenix Down",
            ExoticType::FrostbiteGauntlets => "Frostbite Gauntlets",
            ExoticType::StormcallersMantle => "Stormcaller's Mantle",
            ExoticType::ShadowWeaveBoots => "Shadow-weave Boots",
            ExoticType::TheIronMaiden => "The Iron Maiden",
            ExoticType::SkinOfTheHydra => "Skin of the Hydra",
            ExoticType::AegisOfTheFallen => "Aegis of the Fallen",
            ExoticType::CowardsCloak => "The Coward's Cloak",
            ExoticType::BerserkersPlate => "Berserker's Plate",
            ExoticType::ShroudOfTheNameless => "Shroud of the Nameless",
            ExoticType::TheGlassCannon => "The Glass Cannon",
            ExoticType::TimeWeaversRing => "Time Weaver's Ring",
            ExoticType::RingOfTheGambler => "Ring of the Gambler",
            ExoticType::TheMartyrsBand => "The Martyr's Band",
            ExoticType::Ouroboros => "Ouroboros",
            ExoticType::TheHoardersSignet => "The Hoarder's Signet",
            ExoticType::RingOfEchoes => "Ring of Echoes",
            ExoticType::ThePacifistsOath => "The Pacifist's Oath",
            ExoticType::FatesThread => "Fate's Thread",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            ExoticType::Bloodthirst => "+50% damage, -30% max HP",
            ExoticType::SoulReaper => "Kills heal 5 HP, -3 STR",
            ExoticType::WorldEater => "Every 5th hit 3x damage, -20% attack speed",
            ExoticType::WhisperOfTheVoid => "Ignores armor, -50% base damage",
            ExoticType::CrimsonDancer => "+2 dmg per consecutive hit, resets on damage taken",
            ExoticType::Godsbane => "10% instant kill on non-bosses, -50% vs bosses",
            ExoticType::ThePeacemaker => "Monsters don't aggro unless attacked, +100% damage taken",
            ExoticType::LastBreath => "Up to 3x damage at 1 HP, -50% at full HP",
            ExoticType::PhoenixDown => "Revive once/floor at 50% HP, -20% all stats",
            ExoticType::FrostbiteGauntlets => "10% freeze on hit, -2 INT",
            ExoticType::StormcallersMantle => "Lightning chains +2, -3 CON",
            ExoticType::ShadowWeaveBoots => "No Shadow Step cooldown, costs 2 HP/use",
            ExoticType::TheIronMaiden => "Reflect 50% damage, -3 DEF, +10% damage taken",
            ExoticType::SkinOfTheHydra => "Regen 1 HP/3 ticks, -50% potion effectiveness",
            ExoticType::AegisOfTheFallen => "First hit each floor negated, -10 max HP/floor",
            ExoticType::CowardsCloak => "+30% dodge, -50% damage dealt",
            ExoticType::BerserkersPlate => "+50% dmg below 30% HP, -30% above 70% HP",
            ExoticType::ShroudOfTheNameless => "Invisible 3 ticks after kill, -20% movement",
            ExoticType::TheGlassCannon => "+10 INT, Max HP = 1",
            ExoticType::TimeWeaversRing => "All cooldowns -2, -5 DEX",
            ExoticType::RingOfTheGambler => "25% chance 4x damage, 25% chance 0 damage",
            ExoticType::TheMartyrsBand => "Allies take 50% less, you take 25% more",
            ExoticType::Ouroboros => "No ability cooldowns, drains 2 HP per use",
            ExoticType::TheHoardersSignet => "+200% gold drops, -50% XP",
            ExoticType::RingOfEchoes => "Abilities hit twice, +50% cooldowns",
            ExoticType::ThePacifistsOath => "+50% XP, no critical hits",
            ExoticType::FatesThread => "Reroll one death/run, -30% gold/XP, -20% stats",
        }
    }

    pub fn item_type(&self) -> ItemType {
        match self {
            ExoticType::Bloodthirst
            | ExoticType::SoulReaper
            | ExoticType::WorldEater
            | ExoticType::WhisperOfTheVoid
            | ExoticType::CrimsonDancer
            | ExoticType::Godsbane
            | ExoticType::ThePeacemaker
            | ExoticType::LastBreath => ItemType::Weapon,
            ExoticType::PhoenixDown
            | ExoticType::FrostbiteGauntlets
            | ExoticType::StormcallersMantle
            | ExoticType::ShadowWeaveBoots
            | ExoticType::TheIronMaiden
            | ExoticType::SkinOfTheHydra
            | ExoticType::AegisOfTheFallen
            | ExoticType::CowardsCloak
            | ExoticType::BerserkersPlate
            | ExoticType::ShroudOfTheNameless => ItemType::Armor,
            ExoticType::TheGlassCannon
            | ExoticType::TimeWeaversRing
            | ExoticType::RingOfTheGambler
            | ExoticType::TheMartyrsBand
            | ExoticType::Ouroboros
            | ExoticType::TheHoardersSignet
            | ExoticType::RingOfEchoes
            | ExoticType::ThePacifistsOath
            | ExoticType::FatesThread => ItemType::Ring,
        }
    }
}

// --- Affix Application ---

pub fn apply_affixes(item: &mut Item, rarity: &Rarity, floor: i32) {
    if item.is_artifact || item.exotic_type.is_some() || item.item_type == ItemType::Potion {
        return;
    }
    let mut rng = rand::rng();
    let (prefix_count, suffix_count) = match rarity {
        Rarity::Common => (0, 0),
        Rarity::Uncommon => {
            if rng.random_bool(0.5) {
                (1, 0)
            } else {
                (0, 0)
            }
        }
        Rarity::Rare => {
            if rng.random_bool(0.5) {
                (1, 1)
            } else {
                (1, 0)
            }
        }
        Rarity::Epic | Rarity::Legendary => (1, 1),
        Rarity::Exotic => (0, 0),
    };

    let stat_mult = if *rarity == Rarity::Legendary { 2 } else { 1 };

    if prefix_count > 0 {
        let valid_prefixes: Vec<Prefix> = [
            Prefix::Sharp,
            Prefix::Vicious,
            Prefix::Brutal,
            Prefix::Sturdy,
            Prefix::Fortified,
            Prefix::Iron,
            Prefix::Swift,
            Prefix::Mighty,
            Prefix::Arcane,
            Prefix::Vital,
            Prefix::Vampiric,
            Prefix::Hasty,
        ]
        .iter()
        .filter(|p| p.applies_to(&item.item_type) && p.min_floor() <= floor)
        .cloned()
        .collect();

        if !valid_prefixes.is_empty() {
            let prefix = valid_prefixes[rng.random_range(0..valid_prefixes.len())].clone();
            match &prefix {
                Prefix::Sharp => {
                    item.damage_bonus += stat_mult
                        * match floor {
                            1..=3 => 1,
                            4..=7 => 2,
                            _ => 3,
                        }
                }
                Prefix::Vicious => {
                    item.damage_bonus += stat_mult
                        * match floor {
                            4..=7 => 2,
                            _ => 4,
                        }
                }
                Prefix::Brutal => {
                    item.damage_bonus += stat_mult
                        * match floor {
                            8..=11 => 3,
                            _ => 5,
                        }
                }
                Prefix::Sturdy => {
                    item.defense_bonus += stat_mult
                        * match floor {
                            1..=3 => 1,
                            4..=7 => 2,
                            _ => 3,
                        }
                }
                Prefix::Fortified => {
                    item.defense_bonus += stat_mult
                        * match floor {
                            4..=7 => 2,
                            _ => 4,
                        }
                }
                Prefix::Iron => {
                    item.defense_bonus += stat_mult
                        * match floor {
                            8..=11 => 3,
                            _ => 5,
                        }
                }
                Prefix::Swift => {
                    if item.item_type == ItemType::Ring {
                        item.stat_bonus_type = "DEX".to_string();
                        item.stat_bonus_value += stat_mult;
                    }
                    item.cooldown_reduction += 1;
                }
                Prefix::Mighty => {
                    item.stat_bonus_type = "STR".to_string();
                    item.stat_bonus_value += stat_mult;
                }
                Prefix::Arcane => {
                    item.stat_bonus_type = "INT".to_string();
                    item.stat_bonus_value += stat_mult;
                }
                Prefix::Vital => {
                    item.stat_bonus_type = "CON".to_string();
                    item.stat_bonus_value += stat_mult;
                }
                Prefix::Vampiric => {
                    item.lifesteal += match floor {
                        6..=10 => 5,
                        _ => 10,
                    }
                }
                Prefix::Hasty => item.cooldown_reduction += 1,
            }
            item.prefix = Some(prefix);
        }
    }

    if suffix_count > 0 {
        let valid_suffixes: Vec<Suffix> = [
            Suffix::Health,
            Suffix::Warding,
            Suffix::OfTheBear,
            Suffix::OfTheFox,
            Suffix::OfTheOwl,
            Suffix::OfTheOx,
            Suffix::OfHaste,
            Suffix::OfFrost,
            Suffix::OfFlame,
            Suffix::OfTheLeech,
        ]
        .iter()
        .filter(|s| s.applies_to(&item.item_type) && s.min_floor() <= floor)
        .cloned()
        .collect();

        if !valid_suffixes.is_empty() {
            let suffix = valid_suffixes[rng.random_range(0..valid_suffixes.len())].clone();
            match &suffix {
                Suffix::Health => {
                    item.hp_bonus += stat_mult
                        * match floor {
                            1..=5 => 3,
                            6..=10 => 6,
                            _ => 10,
                        }
                }
                Suffix::Warding => {
                    item.hp_bonus += stat_mult
                        * match floor {
                            6..=10 => 5,
                            _ => 10,
                        }
                }
                Suffix::OfTheBear => {
                    item.stat_bonus_type = "STR".to_string();
                    item.stat_bonus_value += stat_mult
                        * match floor {
                            4..=7 => 1,
                            _ => 2,
                        };
                }
                Suffix::OfTheFox => {
                    item.stat_bonus_type = "DEX".to_string();
                    item.stat_bonus_value += stat_mult
                        * match floor {
                            4..=7 => 1,
                            _ => 2,
                        };
                }
                Suffix::OfTheOwl => {
                    item.stat_bonus_type = "INT".to_string();
                    item.stat_bonus_value += stat_mult
                        * match floor {
                            4..=7 => 1,
                            _ => 2,
                        };
                }
                Suffix::OfTheOx => {
                    item.stat_bonus_type = "CON".to_string();
                    item.stat_bonus_value += stat_mult
                        * match floor {
                            4..=7 => 1,
                            _ => 2,
                        };
                }
                Suffix::OfHaste => item.cooldown_reduction += 1,
                Suffix::OfFrost => {
                    item.freeze_chance += match floor {
                        8..=11 => 5,
                        _ => 10,
                    }
                }
                Suffix::OfFlame => {
                    item.burn_chance += match floor {
                        8..=11 => 5,
                        _ => 10,
                    }
                }
                Suffix::OfTheLeech => {
                    item.lifesteal += match floor {
                        10..=12 => 5,
                        _ => 10,
                    }
                }
            }
            item.suffix = Some(suffix);
        }
    }
}

// --- Exotic Generation ---

pub fn generate_exotic(floor: i32) -> Option<Item> {
    let mut rng = rand::rng();
    let all_exotics: Vec<ExoticType> = vec![
        ExoticType::Bloodthirst,
        ExoticType::SoulReaper,
        ExoticType::WorldEater,
        ExoticType::WhisperOfTheVoid,
        ExoticType::CrimsonDancer,
        ExoticType::Godsbane,
        ExoticType::ThePeacemaker,
        ExoticType::LastBreath,
        ExoticType::PhoenixDown,
        ExoticType::FrostbiteGauntlets,
        ExoticType::StormcallersMantle,
        ExoticType::ShadowWeaveBoots,
        ExoticType::TheIronMaiden,
        ExoticType::SkinOfTheHydra,
        ExoticType::AegisOfTheFallen,
        ExoticType::CowardsCloak,
        ExoticType::BerserkersPlate,
        ExoticType::ShroudOfTheNameless,
        ExoticType::TheGlassCannon,
        ExoticType::TimeWeaversRing,
        ExoticType::RingOfTheGambler,
        ExoticType::TheMartyrsBand,
        ExoticType::Ouroboros,
        ExoticType::TheHoardersSignet,
        ExoticType::RingOfEchoes,
        ExoticType::ThePacifistsOath,
        ExoticType::FatesThread,
    ];

    let exotic = all_exotics[rng.random_range(0..all_exotics.len())].clone();
    let item = match &exotic {
        ExoticType::Bloodthirst => Item {
            name: "Bloodthirst".to_string(),
            item_type: ItemType::Weapon,
            symbol: '/',
            damage_bonus: 3,
            defense_bonus: 0,
            stat_bonus_type: String::new(),
            stat_bonus_value: 0,
            heal_amount: 0,
            floor_level: floor,
            rarity: Rarity::Exotic,
            is_artifact: false,
            artifact_effect: crate::items::ArtifactEffect::None,
            prefix: None,
            suffix: None,
            exotic_type: Some(ExoticType::Bloodthirst),
            lifesteal: 0,
            cooldown_reduction: 0,
            freeze_chance: 0,
            burn_chance: 0,
            hp_bonus: 0,
        },
        ExoticType::SoulReaper => Item {
            name: "Soul Reaper".to_string(),
            item_type: ItemType::Weapon,
            symbol: '/',
            damage_bonus: 4,
            defense_bonus: 0,
            stat_bonus_type: String::new(),
            stat_bonus_value: 0,
            heal_amount: 0,
            floor_level: floor,
            rarity: Rarity::Exotic,
            is_artifact: false,
            artifact_effect: crate::items::ArtifactEffect::None,
            prefix: None,
            suffix: None,
            exotic_type: Some(ExoticType::SoulReaper),
            lifesteal: 0,
            cooldown_reduction: 0,
            freeze_chance: 0,
            burn_chance: 0,
            hp_bonus: 0,
        },
        ExoticType::WorldEater => Item {
            name: "The World-Eater".to_string(),
            item_type: ItemType::Weapon,
            symbol: '/',
            damage_bonus: 5,
            defense_bonus: 0,
            stat_bonus_type: String::new(),
            stat_bonus_value: 0,
            heal_amount: 0,
            floor_level: floor,
            rarity: Rarity::Exotic,
            is_artifact: false,
            artifact_effect: crate::items::ArtifactEffect::None,
            prefix: None,
            suffix: None,
            exotic_type: Some(ExoticType::WorldEater),
            lifesteal: 0,
            cooldown_reduction: 0,
            freeze_chance: 0,
            burn_chance: 0,
            hp_bonus: 0,
        },
        ExoticType::WhisperOfTheVoid => Item {
            name: "Whisper of the Void".to_string(),
            item_type: ItemType::Weapon,
            symbol: '/',
            damage_bonus: 2,
            defense_bonus: 0,
            stat_bonus_type: String::new(),
            stat_bonus_value: 0,
            heal_amount: 0,
            floor_level: floor,
            rarity: Rarity::Exotic,
            is_artifact: false,
            artifact_effect: crate::items::ArtifactEffect::None,
            prefix: None,
            suffix: None,
            exotic_type: Some(ExoticType::WhisperOfTheVoid),
            lifesteal: 0,
            cooldown_reduction: 0,
            freeze_chance: 0,
            burn_chance: 0,
            hp_bonus: 0,
        },
        ExoticType::CrimsonDancer => Item {
            name: "Crimson Dancer".to_string(),
            item_type: ItemType::Weapon,
            symbol: '/',
            damage_bonus: 3,
            defense_bonus: 0,
            stat_bonus_type: "DEX".to_string(),
            stat_bonus_value: 2,
            heal_amount: 0,
            floor_level: floor,
            rarity: Rarity::Exotic,
            is_artifact: false,
            artifact_effect: crate::items::ArtifactEffect::None,
            prefix: None,
            suffix: None,
            exotic_type: Some(ExoticType::CrimsonDancer),
            lifesteal: 0,
            cooldown_reduction: 0,
            freeze_chance: 0,
            burn_chance: 0,
            hp_bonus: 0,
        },
        ExoticType::Godsbane => Item {
            name: "Godsbane".to_string(),
            item_type: ItemType::Weapon,
            symbol: '/',
            damage_bonus: 4,
            defense_bonus: 0,
            stat_bonus_type: String::new(),
            stat_bonus_value: 0,
            heal_amount: 0,
            floor_level: floor,
            rarity: Rarity::Exotic,
            is_artifact: false,
            artifact_effect: crate::items::ArtifactEffect::None,
            prefix: None,
            suffix: None,
            exotic_type: Some(ExoticType::Godsbane),
            lifesteal: 0,
            cooldown_reduction: 0,
            freeze_chance: 0,
            burn_chance: 0,
            hp_bonus: 0,
        },
        ExoticType::ThePeacemaker => Item {
            name: "The Peacemaker".to_string(),
            item_type: ItemType::Weapon,
            symbol: '/',
            damage_bonus: 1,
            defense_bonus: 0,
            stat_bonus_type: "CON".to_string(),
            stat_bonus_value: 2,
            heal_amount: 0,
            floor_level: floor,
            rarity: Rarity::Exotic,
            is_artifact: false,
            artifact_effect: crate::items::ArtifactEffect::None,
            prefix: None,
            suffix: None,
            exotic_type: Some(ExoticType::ThePeacemaker),
            lifesteal: 0,
            cooldown_reduction: 0,
            freeze_chance: 0,
            burn_chance: 0,
            hp_bonus: 0,
        },
        ExoticType::LastBreath => Item {
            name: "Last Breath".to_string(),
            item_type: ItemType::Weapon,
            symbol: '/',
            damage_bonus: 2,
            defense_bonus: 0,
            stat_bonus_type: String::new(),
            stat_bonus_value: 0,
            heal_amount: 0,
            floor_level: floor,
            rarity: Rarity::Exotic,
            is_artifact: false,
            artifact_effect: crate::items::ArtifactEffect::None,
            prefix: None,
            suffix: None,
            exotic_type: Some(ExoticType::LastBreath),
            lifesteal: 0,
            cooldown_reduction: 0,
            freeze_chance: 0,
            burn_chance: 0,
            hp_bonus: 0,
        },
        ExoticType::PhoenixDown => Item {
            name: "Phoenix Down".to_string(),
            item_type: ItemType::Armor,
            symbol: '[',
            damage_bonus: 0,
            defense_bonus: 3,
            stat_bonus_type: String::new(),
            stat_bonus_value: 0,
            heal_amount: 0,
            floor_level: floor,
            rarity: Rarity::Exotic,
            is_artifact: false,
            artifact_effect: crate::items::ArtifactEffect::None,
            prefix: None,
            suffix: None,
            exotic_type: Some(ExoticType::PhoenixDown),
            lifesteal: 0,
            cooldown_reduction: 0,
            freeze_chance: 0,
            burn_chance: 0,
            hp_bonus: 0,
        },
        ExoticType::FrostbiteGauntlets => Item {
            name: "Frostbite Gauntlets".to_string(),
            item_type: ItemType::Armor,
            symbol: '[',
            damage_bonus: 0,
            defense_bonus: 2,
            stat_bonus_type: String::new(),
            stat_bonus_value: 0,
            heal_amount: 0,
            floor_level: floor,
            rarity: Rarity::Exotic,
            is_artifact: false,
            artifact_effect: crate::items::ArtifactEffect::None,
            prefix: None,
            suffix: None,
            exotic_type: Some(ExoticType::FrostbiteGauntlets),
            lifesteal: 0,
            cooldown_reduction: 0,
            freeze_chance: 0,
            burn_chance: 0,
            hp_bonus: 0,
        },
        ExoticType::StormcallersMantle => Item {
            name: "Stormcaller's Mantle".to_string(),
            item_type: ItemType::Armor,
            symbol: '[',
            damage_bonus: 0,
            defense_bonus: 3,
            stat_bonus_type: "INT".to_string(),
            stat_bonus_value: 2,
            heal_amount: 0,
            floor_level: floor,
            rarity: Rarity::Exotic,
            is_artifact: false,
            artifact_effect: crate::items::ArtifactEffect::None,
            prefix: None,
            suffix: None,
            exotic_type: Some(ExoticType::StormcallersMantle),
            lifesteal: 0,
            cooldown_reduction: 0,
            freeze_chance: 0,
            burn_chance: 0,
            hp_bonus: 0,
        },
        ExoticType::ShadowWeaveBoots => Item {
            name: "Shadow-weave Boots".to_string(),
            item_type: ItemType::Armor,
            symbol: '[',
            damage_bonus: 0,
            defense_bonus: 1,
            stat_bonus_type: "DEX".to_string(),
            stat_bonus_value: 3,
            heal_amount: 0,
            floor_level: floor,
            rarity: Rarity::Exotic,
            is_artifact: false,
            artifact_effect: crate::items::ArtifactEffect::None,
            prefix: None,
            suffix: None,
            exotic_type: Some(ExoticType::ShadowWeaveBoots),
            lifesteal: 0,
            cooldown_reduction: 0,
            freeze_chance: 0,
            burn_chance: 0,
            hp_bonus: 0,
        },
        ExoticType::TheIronMaiden => Item {
            name: "The Iron Maiden".to_string(),
            item_type: ItemType::Armor,
            symbol: '[',
            damage_bonus: 0,
            defense_bonus: 2,
            stat_bonus_type: String::new(),
            stat_bonus_value: 0,
            heal_amount: 0,
            floor_level: floor,
            rarity: Rarity::Exotic,
            is_artifact: false,
            artifact_effect: crate::items::ArtifactEffect::None,
            prefix: None,
            suffix: None,
            exotic_type: Some(ExoticType::TheIronMaiden),
            lifesteal: 0,
            cooldown_reduction: 0,
            freeze_chance: 0,
            burn_chance: 0,
            hp_bonus: 0,
        },
        ExoticType::SkinOfTheHydra => Item {
            name: "Skin of the Hydra".to_string(),
            item_type: ItemType::Armor,
            symbol: '[',
            damage_bonus: 0,
            defense_bonus: 2,
            stat_bonus_type: "CON".to_string(),
            stat_bonus_value: 2,
            heal_amount: 0,
            floor_level: floor,
            rarity: Rarity::Exotic,
            is_artifact: false,
            artifact_effect: crate::items::ArtifactEffect::None,
            prefix: None,
            suffix: None,
            exotic_type: Some(ExoticType::SkinOfTheHydra),
            lifesteal: 0,
            cooldown_reduction: 0,
            freeze_chance: 0,
            burn_chance: 0,
            hp_bonus: 0,
        },
        ExoticType::AegisOfTheFallen => Item {
            name: "Aegis of the Fallen".to_string(),
            item_type: ItemType::Armor,
            symbol: '[',
            damage_bonus: 0,
            defense_bonus: 4,
            stat_bonus_type: String::new(),
            stat_bonus_value: 0,
            heal_amount: 0,
            floor_level: floor,
            rarity: Rarity::Exotic,
            is_artifact: false,
            artifact_effect: crate::items::ArtifactEffect::None,
            prefix: None,
            suffix: None,
            exotic_type: Some(ExoticType::AegisOfTheFallen),
            lifesteal: 0,
            cooldown_reduction: 0,
            freeze_chance: 0,
            burn_chance: 0,
            hp_bonus: 0,
        },
        ExoticType::CowardsCloak => Item {
            name: "The Coward's Cloak".to_string(),
            item_type: ItemType::Armor,
            symbol: '[',
            damage_bonus: 0,
            defense_bonus: 1,
            stat_bonus_type: "DEX".to_string(),
            stat_bonus_value: 4,
            heal_amount: 0,
            floor_level: floor,
            rarity: Rarity::Exotic,
            is_artifact: false,
            artifact_effect: crate::items::ArtifactEffect::None,
            prefix: None,
            suffix: None,
            exotic_type: Some(ExoticType::CowardsCloak),
            lifesteal: 0,
            cooldown_reduction: 0,
            freeze_chance: 0,
            burn_chance: 0,
            hp_bonus: 0,
        },
        ExoticType::BerserkersPlate => Item {
            name: "Berserker's Plate".to_string(),
            item_type: ItemType::Armor,
            symbol: '[',
            damage_bonus: 0,
            defense_bonus: 3,
            stat_bonus_type: "STR".to_string(),
            stat_bonus_value: 2,
            heal_amount: 0,
            floor_level: floor,
            rarity: Rarity::Exotic,
            is_artifact: false,
            artifact_effect: crate::items::ArtifactEffect::None,
            prefix: None,
            suffix: None,
            exotic_type: Some(ExoticType::BerserkersPlate),
            lifesteal: 0,
            cooldown_reduction: 0,
            freeze_chance: 0,
            burn_chance: 0,
            hp_bonus: 0,
        },
        ExoticType::ShroudOfTheNameless => Item {
            name: "Shroud of the Nameless".to_string(),
            item_type: ItemType::Armor,
            symbol: '[',
            damage_bonus: 0,
            defense_bonus: 2,
            stat_bonus_type: "DEX".to_string(),
            stat_bonus_value: 2,
            heal_amount: 0,
            floor_level: floor,
            rarity: Rarity::Exotic,
            is_artifact: false,
            artifact_effect: crate::items::ArtifactEffect::None,
            prefix: None,
            suffix: None,
            exotic_type: Some(ExoticType::ShroudOfTheNameless),
            lifesteal: 0,
            cooldown_reduction: 0,
            freeze_chance: 0,
            burn_chance: 0,
            hp_bonus: 0,
        },
        ExoticType::TheGlassCannon => Item {
            name: "The Glass Cannon".to_string(),
            item_type: ItemType::Ring,
            symbol: '=',
            damage_bonus: 0,
            defense_bonus: 0,
            stat_bonus_type: "INT".to_string(),
            stat_bonus_value: 10,
            heal_amount: 0,
            floor_level: floor,
            rarity: Rarity::Exotic,
            is_artifact: false,
            artifact_effect: crate::items::ArtifactEffect::None,
            prefix: None,
            suffix: None,
            exotic_type: Some(ExoticType::TheGlassCannon),
            lifesteal: 0,
            cooldown_reduction: 0,
            freeze_chance: 0,
            burn_chance: 0,
            hp_bonus: 0,
        },
        ExoticType::TimeWeaversRing => Item {
            name: "Time Weaver's Ring".to_string(),
            item_type: ItemType::Ring,
            symbol: '=',
            damage_bonus: 0,
            defense_bonus: 0,
            stat_bonus_type: String::new(),
            stat_bonus_value: 0,
            heal_amount: 0,
            floor_level: floor,
            rarity: Rarity::Exotic,
            is_artifact: false,
            artifact_effect: crate::items::ArtifactEffect::None,
            prefix: None,
            suffix: None,
            exotic_type: Some(ExoticType::TimeWeaversRing),
            lifesteal: 0,
            cooldown_reduction: 2,
            freeze_chance: 0,
            burn_chance: 0,
            hp_bonus: 0,
        },
        ExoticType::RingOfTheGambler => Item {
            name: "Ring of the Gambler".to_string(),
            item_type: ItemType::Ring,
            symbol: '=',
            damage_bonus: 0,
            defense_bonus: 0,
            stat_bonus_type: "STR".to_string(),
            stat_bonus_value: 2,
            heal_amount: 0,
            floor_level: floor,
            rarity: Rarity::Exotic,
            is_artifact: false,
            artifact_effect: crate::items::ArtifactEffect::None,
            prefix: None,
            suffix: None,
            exotic_type: Some(ExoticType::RingOfTheGambler),
            lifesteal: 0,
            cooldown_reduction: 0,
            freeze_chance: 0,
            burn_chance: 0,
            hp_bonus: 0,
        },
        ExoticType::TheMartyrsBand => Item {
            name: "The Martyr's Band".to_string(),
            item_type: ItemType::Ring,
            symbol: '=',
            damage_bonus: 0,
            defense_bonus: 0,
            stat_bonus_type: "CON".to_string(),
            stat_bonus_value: 3,
            heal_amount: 0,
            floor_level: floor,
            rarity: Rarity::Exotic,
            is_artifact: false,
            artifact_effect: crate::items::ArtifactEffect::None,
            prefix: None,
            suffix: None,
            exotic_type: Some(ExoticType::TheMartyrsBand),
            lifesteal: 0,
            cooldown_reduction: 0,
            freeze_chance: 0,
            burn_chance: 0,
            hp_bonus: 0,
        },
        ExoticType::Ouroboros => Item {
            name: "Ouroboros".to_string(),
            item_type: ItemType::Ring,
            symbol: '=',
            damage_bonus: 0,
            defense_bonus: 0,
            stat_bonus_type: String::new(),
            stat_bonus_value: 0,
            heal_amount: 0,
            floor_level: floor,
            rarity: Rarity::Exotic,
            is_artifact: false,
            artifact_effect: crate::items::ArtifactEffect::None,
            prefix: None,
            suffix: None,
            exotic_type: Some(ExoticType::Ouroboros),
            lifesteal: 0,
            cooldown_reduction: 0,
            freeze_chance: 0,
            burn_chance: 0,
            hp_bonus: 0,
        },
        ExoticType::TheHoardersSignet => Item {
            name: "The Hoarder's Signet".to_string(),
            item_type: ItemType::Ring,
            symbol: '=',
            damage_bonus: 0,
            defense_bonus: 0,
            stat_bonus_type: "DEX".to_string(),
            stat_bonus_value: 2,
            heal_amount: 0,
            floor_level: floor,
            rarity: Rarity::Exotic,
            is_artifact: false,
            artifact_effect: crate::items::ArtifactEffect::None,
            prefix: None,
            suffix: None,
            exotic_type: Some(ExoticType::TheHoardersSignet),
            lifesteal: 0,
            cooldown_reduction: 0,
            freeze_chance: 0,
            burn_chance: 0,
            hp_bonus: 0,
        },
        ExoticType::RingOfEchoes => Item {
            name: "Ring of Echoes".to_string(),
            item_type: ItemType::Ring,
            symbol: '=',
            damage_bonus: 0,
            defense_bonus: 0,
            stat_bonus_type: "INT".to_string(),
            stat_bonus_value: 3,
            heal_amount: 0,
            floor_level: floor,
            rarity: Rarity::Exotic,
            is_artifact: false,
            artifact_effect: crate::items::ArtifactEffect::None,
            prefix: None,
            suffix: None,
            exotic_type: Some(ExoticType::RingOfEchoes),
            lifesteal: 0,
            cooldown_reduction: 0,
            freeze_chance: 0,
            burn_chance: 0,
            hp_bonus: 0,
        },
        ExoticType::ThePacifistsOath => Item {
            name: "The Pacifist's Oath".to_string(),
            item_type: ItemType::Ring,
            symbol: '=',
            damage_bonus: 0,
            defense_bonus: 0,
            stat_bonus_type: "CON".to_string(),
            stat_bonus_value: 2,
            heal_amount: 0,
            floor_level: floor,
            rarity: Rarity::Exotic,
            is_artifact: false,
            artifact_effect: crate::items::ArtifactEffect::None,
            prefix: None,
            suffix: None,
            exotic_type: Some(ExoticType::ThePacifistsOath),
            lifesteal: 0,
            cooldown_reduction: 0,
            freeze_chance: 0,
            burn_chance: 0,
            hp_bonus: 0,
        },
        ExoticType::FatesThread => Item {
            name: "Fate's Thread".to_string(),
            item_type: ItemType::Ring,
            symbol: '=',
            damage_bonus: 0,
            defense_bonus: 0,
            stat_bonus_type: String::new(),
            stat_bonus_value: 0,
            heal_amount: 0,
            floor_level: floor,
            rarity: Rarity::Exotic,
            is_artifact: false,
            artifact_effect: crate::items::ArtifactEffect::None,
            prefix: None,
            suffix: None,
            exotic_type: Some(ExoticType::FatesThread),
            lifesteal: 0,
            cooldown_reduction: 0,
            freeze_chance: 0,
            burn_chance: 0,
            hp_bonus: 0,
        },
    };

    Some(item)
}
