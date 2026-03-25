#pragma once
#include <string>
#include <vector>
#include <optional>
#include "items.h"

// --- Ability System ---

enum class AbilityType {
    PowerAttack,    // Warrior 1: next melee 2x damage
    WarCry,         // Warrior 2: AoE stun
    ShadowStep,     // Rogue 1: directional teleport + shadow strike buff
    PoisonBlade,    // Rogue 2: next 3 hits apply poison
    ChainLightning, // Mage 1: directional chain damage
    FrostNova,      // Mage 2: AoE freeze + damage
};

struct Ability {
    std::string name;
    AbilityType ability_type;
    int cooldown_max;
    int cooldown_remaining;
    bool is_active;
    int charges;
    int buff_ticks_remaining;

    Ability() = default;
    static Ability create(AbilityType type);

    bool is_ready() const { return cooldown_remaining <= 0; }
    void activate();
    void tick();
    bool consume_charge();
    std::string status_text() const;
};

// --- Class System ---

enum class PlayerClass { Warrior, Rogue, Mage };

const char* class_name(PlayerClass c);

struct Stats {
    int str_, dex, int_, con;

    int str_modifier() const { return (str_ - 10) / 2; }
    int dodge_chance() const { return std::max(0, (dex - 10) * 3); }
    int max_hp() const { return 20 + (con - 10); }
    int potion_bonus() const { return std::max(0, (int_ - 10) / 2); }
};

Stats base_stats_for_class(PlayerClass c);

// --- Equipment Slots ---

struct Equipment {
    std::optional<Item> weapon;
    std::optional<Item> armor;
    std::optional<Item> ring;

    int weapon_damage() const { return weapon ? weapon->damage_bonus : 0; }
    int armor_defense() const { return armor ? armor->defense_bonus : 0; }
    // Returns (str, dex, int, con) bonuses
    void ring_stat_bonus(int& rs, int& rd, int& ri, int& rc) const;
};

// --- Player ---

constexpr int INVENTORY_CAPACITY = 10;

struct Player {
    int x, y;
    int hp, max_hp;
    PlayerClass player_class;
    Stats base_stats;
    Equipment equipment;
    std::vector<Item> inventory;
    // Abilities
    std::optional<Ability> ability_1;
    std::optional<Ability> ability_2;
    std::optional<int> pending_ability_direction; // which ability (1 or 2) is waiting for direction
    // XP / Leveling
    int xp;
    int level;
    int xp_to_next_level;
    // Status effects
    int poison_ticks;

    Player(int x, int y, PlayerClass c);

    Stats effective_stats() const;
    int melee_damage() const;
    int reduce_damage(int raw_damage) const;
    bool try_dodge() const;
    int potion_heal_amount(int base_heal) const;
    void take_damage(int amount);
    bool is_alive() const;
    void heal(int amount);
    bool add_to_inventory(const Item& item);
    void equip_from_inventory(int index);
    std::optional<int> use_potion(int index);
    std::optional<Item> drop_from_inventory(int index);
    void equip_starting_gear(std::optional<Item> weapon, std::optional<Item> armor, std::optional<Item> ring);
    void tick_abilities();
    bool has_damage_buff() const;
    bool has_poison_buff() const;
    std::optional<int> consume_damage_buff();
    std::optional<int> consume_poison_buff();
    std::vector<std::string> gain_xp(int amount);

private:
    void recalculate_max_hp();
};
