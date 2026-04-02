use crate::items::{ArtifactEffect, Item, ItemType};
use rand::RngExt;
use serde::{Deserialize, Serialize};

// --- Ability System ---

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AbilityType {
    PowerAttack,    // Warrior 1: next melee 2x damage
    WarCry,         // Warrior 2: AoE stun
    ShadowStep,     // Rogue 1: directional teleport + shadow strike buff
    PoisonBlade,    // Rogue 2: next 3 hits apply poison
    ChainLightning, // Mage 1: directional chain damage
    FrostNova,      // Mage 2: AoE freeze + damage
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ability {
    pub name: String,
    pub ability_type: AbilityType,
    pub cooldown_max: i32,
    pub cooldown_remaining: i32,
    pub is_active: bool,           // buff currently active
    pub charges: i32,              // for Poison Blade (3 hits)
    pub buff_ticks_remaining: i32, // ticks until buff expires
}

impl Ability {
    pub fn new(ability_type: AbilityType) -> Self {
        match ability_type {
            AbilityType::PowerAttack => Self {
                name: "Power Attack".to_string(),
                ability_type,
                cooldown_max: 8,
                cooldown_remaining: 0,
                is_active: false,
                charges: 0,
                buff_ticks_remaining: 0,
            },
            AbilityType::WarCry => Self {
                name: "War Cry".to_string(),
                ability_type,
                cooldown_max: 12,
                cooldown_remaining: 0,
                is_active: false,
                charges: 0,
                buff_ticks_remaining: 0,
            },
            AbilityType::ShadowStep => Self {
                name: "Shadow Step".to_string(),
                ability_type,
                cooldown_max: 7,
                cooldown_remaining: 0,
                is_active: false,
                charges: 0,
                buff_ticks_remaining: 0,
            },
            AbilityType::PoisonBlade => Self {
                name: "Poison Blade".to_string(),
                ability_type,
                cooldown_max: 10,
                cooldown_remaining: 0,
                is_active: false,
                charges: 0,
                buff_ticks_remaining: 0,
            },
            AbilityType::ChainLightning => Self {
                name: "Chain Lightning".to_string(),
                ability_type,
                cooldown_max: 6,
                cooldown_remaining: 0,
                is_active: false,
                charges: 0,
                buff_ticks_remaining: 0,
            },
            AbilityType::FrostNova => Self {
                name: "Frost Nova".to_string(),
                ability_type,
                cooldown_max: 10,
                cooldown_remaining: 0,
                is_active: false,
                charges: 0,
                buff_ticks_remaining: 0,
            },
        }
    }

    pub fn is_ready(&self) -> bool {
        self.cooldown_remaining <= 0
    }

    pub fn activate(&mut self) {
        self.cooldown_remaining = self.cooldown_max;
        self.is_active = true;
        // Set buff duration based on type
        match self.ability_type {
            AbilityType::PowerAttack => self.buff_ticks_remaining = 5,
            AbilityType::ShadowStep => self.buff_ticks_remaining = 2,
            AbilityType::PoisonBlade => {
                self.charges = 3;
                self.buff_ticks_remaining = 15; // expires after 15 ticks if charges not used
            }
            _ => {} // instant abilities (WarCry, ChainLightning, FrostNova)
        }
    }

    /// Tick cooldown down by 1. Also tick buff expiration.
    pub fn tick(&mut self) {
        if self.cooldown_remaining > 0 {
            self.cooldown_remaining -= 1;
        }
        if self.is_active && self.buff_ticks_remaining > 0 {
            self.buff_ticks_remaining -= 1;
            if self.buff_ticks_remaining <= 0 {
                self.is_active = false;
                self.charges = 0;
            }
        }
    }

    /// Consume one charge of the buff (e.g. Power Attack lands, Poison Blade hit).
    /// Returns true if the buff is now fully consumed.
    pub fn consume_charge(&mut self) -> bool {
        match self.ability_type {
            AbilityType::PowerAttack | AbilityType::ShadowStep => {
                // Single-use buff — consumed on hit
                self.is_active = false;
                self.buff_ticks_remaining = 0;
                true
            }
            AbilityType::PoisonBlade => {
                self.charges -= 1;
                if self.charges <= 0 {
                    self.is_active = false;
                    self.buff_ticks_remaining = 0;
                    return true;
                }
                false
            }
            _ => false,
        }
    }

    pub fn status_text(&self) -> String {
        if self.is_active {
            if self.charges > 0 {
                format!("{}: ACTIVE ({})", self.name, self.charges)
            } else {
                format!("{}: ACTIVE", self.name)
            }
        } else if self.cooldown_remaining > 0 {
            format!("{}: {}s", self.name, self.cooldown_remaining)
        } else {
            format!("{}: READY", self.name)
        }
    }
}

// --- Class System ---

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Class {
    Warrior,
    Rogue,
    Mage,
}

impl Class {
    pub fn name(&self) -> &'static str {
        match self {
            Class::Warrior => "Warrior",
            Class::Rogue => "Rogue",
            Class::Mage => "Mage",
        }
    }

    pub fn base_stats(&self) -> Stats {
        match self {
            Class::Warrior => Stats {
                str_: 14,
                dex: 10,
                int: 8,
                con: 14,
                crit_chance: 0,
                crit_multiplier: 1.5,
            },
            Class::Rogue => Stats {
                str_: 10,
                dex: 14,
                int: 10,
                con: 10,
                crit_chance: 4,
                crit_multiplier: 1.5,
            },
            Class::Mage => Stats {
                str_: 8,
                dex: 10,
                int: 14,
                con: 8,
                crit_chance: 0,
                crit_multiplier: 1.5,
            },
        }
    }
}

// --- Stats ---

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Stats {
    pub str_: i32,
    pub dex: i32,
    pub int: i32,
    pub con: i32,
    pub crit_chance: i32,     // Base crit chance from DEX (0-100)
    pub crit_multiplier: f32, // Crit damage multiplier (1.5 = 150%)
}

impl Stats {
    /// STR modifier: (STR - 10) / 2, added to melee damage.
    pub fn str_modifier(&self) -> i32 {
        (self.str_ - 10) / 2
    }

    /// DEX dodge chance: (DEX - 10) * 3 percent.
    pub fn dodge_chance(&self) -> i32 {
        ((self.dex - 10) * 3).max(0)
    }

    /// CON HP bonus: base 20 + (CON - 10).
    pub fn max_hp(&self) -> i32 {
        20 + (self.con - 10)
    }

    /// INT potion healing bonus: +1 per 2 INT above 10.
    pub fn potion_bonus(&self) -> i32 {
        ((self.int - 10) / 2).max(0)
    }

    /// CRIT chance from DEX: 1% per DEX above 10.
    pub fn crit_chance(&self) -> i32 {
        ((self.dex - 10) * 1).max(0)
    }

    /// CRIT multiplier: base 150% (1.5x damage).
    pub fn crit_multiplier(&self) -> f32 {
        self.crit_multiplier
    }
}

// --- Equipment Slots ---

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Equipment {
    pub weapon: Option<Item>,
    pub armor: Option<Item>,
    pub ring: Option<Item>,
}

impl Equipment {
    pub fn new() -> Self {
        Self {
            weapon: None,
            armor: None,
            ring: None,
        }
    }

    /// Total damage bonus from equipped weapon.
    pub fn weapon_damage(&self) -> i32 {
        self.weapon.as_ref().map_or(0, |w| w.damage_bonus)
    }

    /// Total defense from equipped armor.
    pub fn armor_defense(&self) -> i32 {
        self.armor.as_ref().map_or(0, |a| a.defense_bonus)
    }

    /// Stat bonuses from equipped ring.
    pub fn ring_stat_bonus(&self) -> (i32, i32, i32, i32) {
        // Returns (str, dex, int, con) bonuses
        if let Some(ring) = &self.ring {
            match ring.stat_bonus_type.as_str() {
                "STR" => (ring.stat_bonus_value, 0, 0, 0),
                "DEX" => (0, ring.stat_bonus_value, 0, 0),
                "INT" => (0, 0, ring.stat_bonus_value, 0),
                "CON" => (0, 0, 0, ring.stat_bonus_value),
                _ => (0, 0, 0, 0),
            }
        } else {
            (0, 0, 0, 0)
        }
    }
}

// --- Player ---

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Player {
    pub x: usize,
    pub y: usize,
    pub hp: i32,
    pub max_hp: i32,
    pub class: Class,
    pub base_stats: Stats,
    pub equipment: Equipment,
    pub inventory: Vec<Item>,
    // Abilities
    pub ability_1: Option<Ability>,
    pub ability_2: Option<Ability>,
    pub pending_ability_direction: Option<u8>, // which ability (1 or 2) is waiting for direction
    // XP / Leveling
    pub xp: i32,
    pub level: i32,
    pub xp_to_next_level: i32,
    // Status effects
    pub poison_ticks: i32,
    // Artifact state
    pub ragefang_stacks: i32,
    pub ragefang_ticks: i32,
    pub stonehide_triggered: bool,
    pub stonehide_bonus: i32,
    pub wraithwalkers_buff: bool,
    pub mindfire_kill_count: i32,
    pub mindfire_ready: bool,
    // Death stats
    pub monsters_slain: u32,
    pub damage_dealt: i32,
    pub damage_taken: i32,
    pub cause_of_death: String,
    pub last_damage_source: Option<(String, i32)>,
    // Shrine buffs (temporary for floor)
    pub bonus_str: i32,
    pub bonus_dodge: i32,
    pub warding_buff: bool,
}

pub const INVENTORY_CAPACITY: usize = 10;

impl Player {
    pub fn new(x: usize, y: usize, class: Class) -> Self {
        let base_stats = class.base_stats();
        let max_hp = base_stats.max_hp();

        // Each class starts with ability 1; ability 2 unlocks at level 5
        let ability_1 = match class {
            Class::Warrior => Some(Ability::new(AbilityType::PowerAttack)),
            Class::Rogue => Some(Ability::new(AbilityType::ShadowStep)),
            Class::Mage => Some(Ability::new(AbilityType::ChainLightning)),
        };

        Self {
            x,
            y,
            hp: max_hp,
            max_hp,
            class,
            base_stats,
            equipment: Equipment::new(),
            inventory: Vec::new(),
            ability_1,
            ability_2: None,
            pending_ability_direction: None,
            xp: 0,
            level: 1,
            xp_to_next_level: 50,
            poison_ticks: 0,
            ragefang_stacks: 0,
            ragefang_ticks: 0,
            stonehide_triggered: false,
            stonehide_bonus: 0,
            wraithwalkers_buff: false,
            mindfire_kill_count: 0,
            mindfire_ready: false,
            monsters_slain: 0,
            damage_dealt: 0,
            damage_taken: 0,
            cause_of_death: String::new(),
            last_damage_source: None,
            bonus_str: 0,
            bonus_dodge: 0,
            warding_buff: false,
        }
    }

    /// Effective stats = base stats + ring bonuses.
    pub fn effective_stats(&self) -> Stats {
        let (rs, rd, ri, rc) = self.equipment.ring_stat_bonus();
        Stats {
            str_: self.base_stats.str_ + rs,
            dex: self.base_stats.dex + rd,
            int: self.base_stats.int + ri,
            con: self.base_stats.con + rc,
            crit_chance: self.base_stats.crit_chance,
            crit_multiplier: self.base_stats.crit_multiplier,
        }
    }

    /// Check for critical hit. Returns true if crit lands.
    pub fn roll_crit(&self) -> bool {
        let stats = self.effective_stats();
        let chance = stats.crit_chance();
        if chance <= 0 {
            return false;
        }
        let mut rng = rand::rng();
        rng.random_range(0..100) < chance
    }

    /// Apply damage variance (±10%) to incoming damage value.
    pub fn apply_damage_variance(damage: i32) -> i32 {
        let mut rng = rand::rng();
        let variance: f32 = rng.random_range(90..111) as f32 / 100.0;
        (damage as f32 * variance) as i32
    }

    /// Total melee damage: base 1 + weapon + STR + artifacts + crit + variance.
    pub fn melee_damage(&self) -> i32 {
        let stats = self.effective_stats();
        let mut damage = 1 + self.equipment.weapon_damage() + stats.str_modifier();
        damage += self.ragefang_stacks;
        if self.wraithwalkers_buff {
            damage *= 2;
        }
        if self.roll_crit() {
            damage = (damage as f32 * stats.crit_multiplier()) as i32;
        }
        damage = Self::apply_damage_variance(damage);
        damage.max(1)
    }

    /// Incoming damage reduction from armor. Minimum 1 damage always gets through.
    pub fn reduce_damage(&self, raw_damage: i32) -> i32 {
        (raw_damage - self.equipment.armor_defense() - self.stonehide_bonus).max(1)
    }

    /// Dodge check. Returns true if the attack is dodged.
    pub fn try_dodge(&self) -> bool {
        let chance = self.effective_dodge_chance();
        if chance <= 0 {
            return false;
        }
        let mut rng = rand::rng();
        rng.random_range(0..100) < chance
    }

    /// Heal amount for potions, including INT bonus.
    pub fn potion_heal_amount(&self, base_heal: i32) -> i32 {
        let stats = self.effective_stats();
        base_heal + stats.potion_bonus()
    }

    pub fn take_damage(&mut self, amount: i32) {
        self.hp = (self.hp - amount).max(0);
    }

    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }

    pub fn heal(&mut self, amount: i32) {
        self.hp = (self.hp + amount).min(self.max_hp);
    }

    /// Try to add an item to inventory. Returns false if full.
    pub fn add_to_inventory(&mut self, item: Item) -> bool {
        if self.inventory.len() >= INVENTORY_CAPACITY {
            return false;
        }
        self.inventory.push(item);
        true
    }

    /// Equip an item from inventory at the given index.
    /// Returns the previously equipped item (if any) back into inventory.
    pub fn equip_from_inventory(&mut self, index: usize) {
        if index >= self.inventory.len() {
            return;
        }

        let item = self.inventory.remove(index);
        let old = match item.item_type {
            ItemType::Weapon => self.equipment.weapon.replace(item),
            ItemType::Armor => self.equipment.armor.replace(item),
            ItemType::Ring => self.equipment.ring.replace(item),
            ItemType::Potion => {
                // Can't equip potions — put back
                self.inventory.insert(index, item);
                return;
            }
        };

        // Put old equipment back into inventory
        if let Some(old_item) = old {
            self.inventory.push(old_item);
        }

        // Recalculate max HP when ring changes CON
        self.recalculate_max_hp();
    }

    /// Use a potion from inventory at the given index. Returns the heal amount, or None.
    pub fn use_potion(&mut self, index: usize) -> Option<i32> {
        if index >= self.inventory.len() {
            return None;
        }
        if self.inventory[index].item_type != ItemType::Potion {
            return None;
        }

        let potion = self.inventory.remove(index);
        let heal = self.potion_heal_amount(potion.heal_amount);
        self.heal(heal);
        Some(heal)
    }

    /// Drop an item from inventory at the given index. Returns the item.
    #[allow(dead_code)]
    pub fn drop_from_inventory(&mut self, index: usize) -> Option<Item> {
        if index >= self.inventory.len() {
            return None;
        }
        Some(self.inventory.remove(index))
    }

    /// Recalculate max HP based on current effective CON.
    fn recalculate_max_hp(&mut self) {
        let stats = self.effective_stats();
        let new_max = stats.max_hp();
        self.max_hp = new_max;
        if self.hp > self.max_hp {
            self.hp = self.max_hp;
        }
    }

    /// Pre-equip starting gear (bypasses inventory).
    pub fn equip_starting_gear(
        &mut self,
        weapon: Option<Item>,
        armor: Option<Item>,
        ring: Option<Item>,
    ) {
        self.equipment.weapon = weapon;
        self.equipment.armor = armor;
        self.equipment.ring = ring;
        self.recalculate_max_hp();
        // Start at full HP with new max
        self.hp = self.max_hp;
    }

    /// Tick ability cooldowns and artifact states. Called each monster tick.
    pub fn tick_abilities(&mut self) {
        if let Some(ref mut a) = self.ability_1 {
            a.tick();
        }
        if let Some(ref mut a) = self.ability_2 {
            a.tick();
        }

        if self.ragefang_ticks > 0 {
            self.ragefang_ticks -= 1;
            if self.ragefang_ticks == 0 {
                self.ragefang_stacks = 0;
            }
        }

        if self.wraithwalkers_buff {
            self.wraithwalkers_buff = false;
        }
    }

    pub fn has_ragefang(&self) -> bool {
        self.equipment
            .weapon
            .as_ref()
            .map_or(false, |w| w.artifact_effect == ArtifactEffect::Ragefang)
    }

    pub fn has_stonehide(&self) -> bool {
        self.equipment.armor.as_ref().map_or(false, |a| {
            a.artifact_effect == ArtifactEffect::StonehidePlate
        })
    }

    pub fn has_wraithwalkers(&self) -> bool {
        self.equipment.armor.as_ref().map_or(false, |a| {
            a.artifact_effect == ArtifactEffect::Wraithwalkers
        })
    }

    pub fn has_mindfire(&self) -> bool {
        self.equipment.ring.as_ref().map_or(false, |r| {
            r.artifact_effect == ArtifactEffect::MindFireCrown
        })
    }

    pub fn has_shadowfang(&self) -> bool {
        self.equipment
            .weapon
            .as_ref()
            .map_or(false, |w| w.artifact_effect == ArtifactEffect::Shadowfang)
    }

    pub fn activate_damage_buff(&mut self) {
        self.wraithwalkers_buff = true;
    }

    pub fn effective_dodge_chance(&self) -> i32 {
        let stats = self.effective_stats();
        let base = stats.dodge_chance();
        if self.has_wraithwalkers() {
            base + 15
        } else {
            base
        }
    }

    pub fn has_warlord_signet(&self) -> bool {
        self.equipment.ring.as_ref().map_or(false, |r| {
            r.artifact_effect == ArtifactEffect::WarlordSignet
        })
    }

    pub fn has_venomcoil(&self) -> bool {
        self.equipment
            .ring
            .as_ref()
            .map_or(false, |r| r.artifact_effect == ArtifactEffect::Venomcoil)
    }

    pub fn has_stormcaller(&self) -> bool {
        self.equipment.weapon.as_ref().map_or(false, |w| {
            w.artifact_effect == ArtifactEffect::StormcallerStaff
        })
    }

    pub fn has_frostweave(&self) -> bool {
        self.equipment.armor.as_ref().map_or(false, |a| {
            a.artifact_effect == ArtifactEffect::FrostweavRobe
        })
    }

    pub fn get_damage_multiplier(&self) -> i32 {
        if self.mindfire_ready {
            2
        } else {
            1
        }
    }

    pub fn consume_mindfire(&mut self) {
        self.mindfire_ready = false;
    }

    /// Check if Power Attack or Shadow Strike buff is active (2x melee).
    pub fn has_damage_buff(&self) -> bool {
        if let Some(ref a) = self.ability_1 {
            if a.is_active
                && (a.ability_type == AbilityType::PowerAttack
                    || a.ability_type == AbilityType::ShadowStep)
            {
                return true;
            }
        }
        if let Some(ref a) = self.ability_2 {
            if a.is_active
                && (a.ability_type == AbilityType::PowerAttack
                    || a.ability_type == AbilityType::ShadowStep)
            {
                return true;
            }
        }
        false
    }

    /// Check if Poison Blade buff is active (apply poison on hit).
    pub fn has_poison_buff(&self) -> bool {
        if let Some(ref a) = self.ability_1 {
            if a.is_active && a.ability_type == AbilityType::PoisonBlade {
                return true;
            }
        }
        if let Some(ref a) = self.ability_2 {
            if a.is_active && a.ability_type == AbilityType::PoisonBlade {
                return true;
            }
        }
        false
    }

    /// Consume a damage buff charge after a melee hit. Returns the ability slot that was consumed (1 or 2).
    pub fn consume_damage_buff(&mut self) -> Option<u8> {
        if let Some(ref mut a) = self.ability_1 {
            if a.is_active
                && (a.ability_type == AbilityType::PowerAttack
                    || a.ability_type == AbilityType::ShadowStep)
            {
                a.consume_charge();
                return Some(1);
            }
        }
        if let Some(ref mut a) = self.ability_2 {
            if a.is_active
                && (a.ability_type == AbilityType::PowerAttack
                    || a.ability_type == AbilityType::ShadowStep)
            {
                a.consume_charge();
                return Some(2);
            }
        }
        None
    }

    /// Consume a poison buff charge after a melee hit. Returns the ability slot that was consumed.
    pub fn consume_poison_buff(&mut self) -> Option<u8> {
        if let Some(ref mut a) = self.ability_1 {
            if a.is_active && a.ability_type == AbilityType::PoisonBlade {
                a.consume_charge();
                return Some(1);
            }
        }
        if let Some(ref mut a) = self.ability_2 {
            if a.is_active && a.ability_type == AbilityType::PoisonBlade {
                a.consume_charge();
                return Some(2);
            }
        }
        None
    }

    /// Add XP and check for level up. Returns a vec of log messages for any level ups.
    pub fn gain_xp(&mut self, amount: i32) -> Vec<String> {
        let mut messages = Vec::new();
        self.xp += amount;

        while self.xp >= self.xp_to_next_level && self.level < 10 {
            self.level += 1;
            let leftover = self.xp - self.xp_to_next_level;

            // Calculate next threshold: threshold(n) = threshold(n-1) + 20 + (n-1)*40
            self.xp_to_next_level = self.xp_to_next_level + 20 + (self.level - 1) * 40;
            self.xp = leftover;

            // +3 max HP, heal 3
            self.max_hp += 3;
            self.hp = (self.hp + 3).min(self.max_hp);

            // Class stat boosts
            let stat_msg = match self.class {
                Class::Warrior => {
                    self.base_stats.str_ += 1;
                    self.base_stats.con += 1;
                    self.recalculate_max_hp();
                    "STR +1, CON +1"
                }
                Class::Rogue => {
                    self.base_stats.dex += 1;
                    self.base_stats.str_ += 1;
                    "DEX +1, STR +1"
                }
                Class::Mage => {
                    self.base_stats.int += 1;
                    self.base_stats.dex += 1;
                    "INT +1, DEX +1"
                }
            };

            messages.push(format!(
                "You reach level {}! {}, Max HP +3",
                self.level, stat_msg
            ));

            // Levels 3 and 7: reduce all ability cooldowns by 1 (permanent)
            if self.level == 3 || self.level == 7 {
                if let Some(ref mut a) = self.ability_1 {
                    a.cooldown_max = (a.cooldown_max - 1).max(1);
                }
                if let Some(ref mut a) = self.ability_2 {
                    a.cooldown_max = (a.cooldown_max - 1).max(1);
                }
                messages.push("Your abilities grow stronger! Cooldowns reduced.".to_string());
            }

            // Level 5: unlock 2nd ability
            if self.level == 5 && self.ability_2.is_none() {
                self.ability_2 = match self.class {
                    Class::Warrior => Some(Ability::new(AbilityType::WarCry)),
                    Class::Rogue => Some(Ability::new(AbilityType::PoisonBlade)),
                    Class::Mage => Some(Ability::new(AbilityType::FrostNova)),
                };
                if let Some(ref a) = self.ability_2 {
                    messages.push(format!("New ability unlocked: {}!", a.name));
                }
            }
        }

        messages
    }
}
