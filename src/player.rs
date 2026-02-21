use crate::items::{Item, ItemType};

// --- Class System ---

#[derive(Clone, Copy, Debug, PartialEq)]
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
            },
            Class::Rogue => Stats {
                str_: 10,
                dex: 14,
                int: 10,
                con: 10,
            },
            Class::Mage => Stats {
                str_: 8,
                dex: 10,
                int: 14,
                con: 8,
            },
        }
    }
}

// --- Stats ---

#[derive(Clone, Debug)]
pub struct Stats {
    pub str_: i32,
    pub dex: i32,
    pub int: i32,
    pub con: i32,
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
}

// --- Equipment Slots ---

#[derive(Clone, Debug)]
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

pub struct Player {
    pub x: usize,
    pub y: usize,
    pub hp: i32,
    pub max_hp: i32,
    pub class: Class,
    pub base_stats: Stats,
    pub equipment: Equipment,
    pub inventory: Vec<Item>,
}

pub const INVENTORY_CAPACITY: usize = 10;

impl Player {
    pub fn new(x: usize, y: usize, class: Class) -> Self {
        let base_stats = class.base_stats();
        let max_hp = base_stats.max_hp();

        Self {
            x,
            y,
            hp: max_hp,
            max_hp,
            class,
            base_stats,
            equipment: Equipment::new(),
            inventory: Vec::new(),
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
        }
    }

    /// Total melee damage: base 1 (fist) + weapon bonus + STR modifier.
    pub fn melee_damage(&self) -> i32 {
        let stats = self.effective_stats();
        (1 + self.equipment.weapon_damage() + stats.str_modifier()).max(1)
    }

    /// Incoming damage reduction from armor. Minimum 1 damage always gets through.
    pub fn reduce_damage(&self, raw_damage: i32) -> i32 {
        (raw_damage - self.equipment.armor_defense()).max(1)
    }

    /// Dodge check. Returns true if the attack is dodged.
    pub fn try_dodge(&self) -> bool {
        let stats = self.effective_stats();
        let chance = stats.dodge_chance();
        if chance <= 0 {
            return false;
        }
        let mut rng = rand::rng();
        use rand::RngExt;
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
}
