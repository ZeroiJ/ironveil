mod items;
mod map;
mod monster;
mod player;
mod projectile;
mod render;

use bracket_lib::prelude::*;
use items::{Item, ItemType};
use map::{Map, Tile};
use monster::{Monster, MonsterAction};
use player::{AbilityType, Class, Player};
use projectile::Projectile;
use render::{
    draw_entity, draw_isometric_tile, draw_isometric_wall, get_floor_color, get_wall_colors,
    get_wall_visibility, Camera, RenderLayer,
};
use std::collections::{HashMap, HashSet};

fn to_cp437(c: u8) -> FontCharType {
    c as FontCharType
}

fn cp437_char(c: char) -> FontCharType {
    c as FontCharType
}

fn darken(color: RGB, factor: f32) -> RGB {
    RGB::from_f32(color.r * factor, color.g * factor, color.b * factor)
}

const LAYER_MAP: usize = 0;
const LAYER_HUD: usize = 1;
const LAYER_OVERLAY: usize = 2;

const CONSOLE_WIDTH: u32 = 80;
const CONSOLE_HEIGHT: u32 = 50;

const MONSTER_TICK_MS: f32 = 500.0;

#[derive(Debug, Clone, Copy, PartialEq)]
enum GameMode {
    CharacterCreation,
    Playing,
    Inventory,
    Dead,
}

struct State {
    mode: GameMode,
    monster_tick_accumulator: f32,

    // Game state
    map: Map,
    player: Player,
    monsters: Vec<Monster>,
    projectiles: Vec<Projectile>,
    ground_items: HashMap<(usize, usize), Item>,
    webs: HashSet<(usize, usize)>,

    current_floor: i32,
    log: Vec<String>,
    player_web_stuck: i32,

    // Isometric camera
    camera: Camera,
}

impl State {
    fn new() -> Self {
        let map = Map::new(CONSOLE_WIDTH as usize, (CONSOLE_HEIGHT - 7) as usize);
        let player = Player::new(1, 1, Class::Warrior);

        State {
            mode: GameMode::CharacterCreation,
            monster_tick_accumulator: 0.0,
            map,
            player,
            monsters: Vec::new(),
            projectiles: Vec::new(),
            ground_items: HashMap::new(),
            webs: HashSet::new(),
            current_floor: 1,
            log: vec!["Welcome to Ironveil!".to_string()],
            player_web_stuck: 0,
            camera: Camera::new(CONSOLE_WIDTH as f32, (CONSOLE_HEIGHT - 7) as f32),
        }
    }

    fn log_msg(&mut self, msg: impl Into<String>) {
        self.log.push(msg.into());
        // Keep log from growing infinitely
        if self.log.len() > 50 {
            self.log.remove(0);
        }
    }

    fn generate_floor(&mut self) {
        let (term_width, term_height) = (CONSOLE_WIDTH as usize, (CONSOLE_HEIGHT - 7) as usize);
        self.map = Map::new(term_width, term_height);
        let (sx, sy) = self.map.get_starting_position();
        self.player.x = sx;
        self.player.y = sy;

        // Center camera on player
        self.camera.follow(sx, sy);

        self.monsters = self.map.spawn_monsters_for_floor(self.current_floor);
        self.projectiles.clear();
        self.ground_items = self.map.spawn_ground_items(self.current_floor);
        self.webs.clear();
        self.player_web_stuck = 0;

        self.log_msg(format!("Welcome to floor {}!", self.current_floor));

        match self.current_floor {
            5 => {
                self.log_msg("*** THE GOBLIN KING GUARDS THE WAY! ***");
                self.log_msg("Defeat him to descend deeper...");
            }
            10 => {
                self.log_msg("*** THE BONE DRAGON AWAITS! ***");
                self.log_msg("Its fiery breath fills the chamber...");
            }
            15 => {
                self.log_msg("*** THE SHADOW LORD HAS COME! ***");
                self.log_msg("Darkness pulses from every corner...");
            }
            _ => {}
        }
    }

    fn process_projectiles(&mut self) {
        let mut i = 0;
        while i < self.projectiles.len() {
            let still_alive = self.projectiles[i].advance(&self.map);

            if !still_alive {
                self.log_msg("The arrow thuds into a wall.");
                self.projectiles.remove(i);
                continue;
            }

            let px = self.projectiles[i].x;
            let py = self.projectiles[i].y;

            // Check hit player
            if px == self.player.x && py == self.player.y {
                if self.player.try_dodge() {
                    self.log_msg("You dodge the arrow!");
                } else {
                    let raw_dmg = self.projectiles[i].damage;
                    let dmg = self.player.reduce_damage(raw_dmg);
                    self.player.take_damage(dmg);
                    self.log_msg(format!("An arrow hits you for {} damage!", dmg));
                }
                self.projectiles.remove(i);
                continue;
            }

            // Check hit monster
            let mut hit_monster = false;
            for monster in self.monsters.iter() {
                if monster.is_alive() && monster.x == px && monster.y == py {
                    hit_monster = true;
                    break;
                }
            }

            if hit_monster {
                self.log_msg("The arrow hits a creature.");
                self.projectiles.remove(i);
                continue;
            }

            i += 1;
        }
    }

    fn process_monsters(&mut self) {
        let player_pos = (self.player.x, self.player.y);

        // Extract needed refs so we don't borrow `self` multiple times inside the loop
        let all_positions: Vec<(usize, usize)> = self
            .monsters
            .iter()
            .filter(|m| m.is_alive())
            .map(|m| (m.x, m.y))
            .collect();

        let dead_indices: Vec<usize> = self
            .monsters
            .iter()
            .enumerate()
            .filter(|(_, m)| !m.is_alive() && m.death_pos.is_some())
            .map(|(i, _)| i)
            .collect();

        for i in 0..self.monsters.len() {
            if !self.monsters[i].is_alive() {
                continue;
            }

            // Status effects
            if self.monsters[i].poison_ticks > 0 {
                self.monsters[i].poison_ticks -= 1;
                self.monsters[i].take_damage(1);
                if !self.monsters[i].is_alive() {
                    self.log_msg(format!("The {} dies from poison!", self.monsters[i].name));
                    let xp = self.monsters[i].xp_value();
                    let level_msgs = self.player.gain_xp(xp);
                    self.log_msg(format!("+{} XP", xp));
                    for msg in level_msgs {
                        self.log_msg(msg);
                    }
                    self.monsters[i].death_pos = Some((self.monsters[i].x, self.monsters[i].y));
                    continue;
                }
            }
            if self.monsters[i].stun_ticks > 0 {
                self.monsters[i].stun_ticks -= 1;
                continue;
            }
            if self.monsters[i].freeze_ticks > 0 {
                self.monsters[i].freeze_ticks -= 1;
                continue;
            }

            let occupied: Vec<(usize, usize)> = all_positions
                .iter()
                .enumerate()
                .filter(|(j, _)| *j != i)
                .map(|(_, pos)| *pos)
                .collect();

            let was_berserk = self.monsters[i].is_berserk;
            let action = self.monsters[i].decide_action(
                player_pos,
                &self.map,
                &occupied,
                &all_positions,
                &dead_indices,
            );

            if self.monsters[i].is_berserk && !was_berserk {
                self.log_msg("The Troll flies into a rage!");
            }

            match action {
                MonsterAction::Nothing => {}
                MonsterAction::MeleeAttack { damage, ref name } => {
                    if self.player.try_dodge() {
                        self.log_msg(format!("You dodge the {}'s attack!", name));
                    } else {
                        let dmg = self.player.reduce_damage(damage);
                        self.player.take_damage(dmg);
                        self.log_msg(format!("The {} hits you for {} damage!", name, dmg));
                    }
                }
                MonsterAction::PoisonAttack {
                    damage,
                    ref name,
                    poison_ticks,
                } => {
                    if self.player.try_dodge() {
                        self.log_msg(format!("You dodge the {}'s venomous bite!", name));
                    } else {
                        let dmg = self.player.reduce_damage(damage);
                        self.player.take_damage(dmg);
                        self.player.poison_ticks = poison_ticks;
                        self.log_msg(format!(
                            "The {} bites you for {} damage! Poison courses through your veins!",
                            name, dmg
                        ));
                    }
                }
                MonsterAction::DrainAttack { damage, ref name } => {
                    if self.player.try_dodge() {
                        self.log_msg(format!("You dodge the {}'s spectral grasp!", name));
                    } else {
                        let dmg = self.player.reduce_damage(damage);
                        self.player.take_damage(dmg);
                        let heal = dmg / 2;
                        self.monsters[i].hp =
                            (self.monsters[i].hp + heal).min(self.monsters[i].max_hp);
                        self.log_msg(format!(
                            "The {} drains {} life from you! (heals {})",
                            name, dmg, heal
                        ));
                    }
                }
                MonsterAction::PlaceWeb(wx, wy) => {
                    if self.map.is_walkable(wx, wy) {
                        self.webs.insert((wx, wy));
                    }
                }
                MonsterAction::Resurrect(dead_idx) => {
                    if dead_idx < self.monsters.len() && !self.monsters[dead_idx].is_alive() {
                        if let Some((dx, dy)) = self.monsters[dead_idx].death_pos {
                            let restore_pct = if self.monsters[i].floor_tier >= 3 {
                                75
                            } else {
                                50
                            };
                            let restored_hp =
                                (self.monsters[dead_idx].max_hp * restore_pct / 100).max(1);
                            self.monsters[dead_idx].hp = restored_hp;
                            self.monsters[dead_idx].x = dx;
                            self.monsters[dead_idx].y = dy;
                            self.monsters[dead_idx].death_pos = None;
                            self.monsters[dead_idx].stun_ticks = 0;
                            self.monsters[dead_idx].freeze_ticks = 0;
                            self.monsters[dead_idx].poison_ticks = 0;
                            self.monsters[dead_idx].behavior = monster::BehaviorState::Idle;
                            self.log_msg(format!(
                                "The Necromancer raises the {} from the dead!",
                                self.monsters[dead_idx].name
                            ));
                        }
                    }
                }
                MonsterAction::MoveTo(nx, ny) => {
                    let mut blocked = self
                        .monsters
                        .iter()
                        .enumerate()
                        .any(|(j, m)| i != j && m.is_alive() && m.x == nx && m.y == ny);

                    if nx == self.player.x && ny == self.player.y {
                        blocked = true;
                    }

                    if !blocked && self.map.is_walkable(nx, ny) {
                        self.monsters[i].x = nx;
                        self.monsters[i].y = ny;
                        self.monsters[i].is_phasing = false;

                        // Tier 3 Goblin second move
                        if matches!(self.monsters[i].monster_type, monster::MonsterType::Goblin)
                            && self.monsters[i].floor_tier >= 3
                            && self.monsters[i].can_see_player
                        {
                            let occupied2: Vec<(usize, usize)> = self
                                .monsters
                                .iter()
                                .enumerate()
                                .filter(|(j, m)| *j != i && m.is_alive())
                                .map(|(_, m)| (m.x, m.y))
                                .collect();

                            if let Some(next2) = self.map.astar_next_step(
                                (self.monsters[i].x, self.monsters[i].y),
                                player_pos,
                                &occupied2,
                            ) {
                                let blocked2 = self.monsters.iter().enumerate().any(|(j, m)| {
                                    i != j && m.is_alive() && m.x == next2.0 && m.y == next2.1
                                });

                                if next2.0 == self.player.x && next2.1 == self.player.y {
                                    let damage = self.monsters[i].attack;
                                    if self.player.try_dodge() {
                                        self.log_msg("You dodge the Goblin's dash attack!");
                                    } else {
                                        let dmg = self.player.reduce_damage(damage);
                                        self.player.take_damage(dmg);
                                        self.log_msg(format!(
                                            "The Goblin dashes and strikes for {} damage!",
                                            dmg
                                        ));
                                    }
                                } else if !blocked2 && self.map.is_walkable(next2.0, next2.1) {
                                    self.monsters[i].x = next2.0;
                                    self.monsters[i].y = next2.1;
                                }
                            }
                        }
                    }
                }
                MonsterAction::MoveToPhase(nx, ny) => {
                    if nx < self.map.width
                        && ny < self.map.height
                        && (nx != self.player.x || ny != self.player.y)
                    {
                        self.monsters[i].x = nx;
                        self.monsters[i].y = ny;
                        self.monsters[i].is_phasing = !self.map.is_walkable(nx, ny);
                    }
                }
                MonsterAction::FireProjectile(proj) => {
                    self.log_msg(format!("The {} fires an arrow!", self.monsters[i].name));
                    self.projectiles.push(proj);
                }
                MonsterAction::BossSummon => {
                    let enraged = self.monsters[i].hp * 100 / self.monsters[i].max_hp < 50;
                    let count = if enraged { 3 } else { 2 };
                    let bx = self.monsters[i].x;
                    let by = self.monsters[i].y;
                    let floor = self.monsters[i].floor_tier * 3;
                    let mut spawned = 0;
                    let offsets: [(i32, i32); 8] = [
                        (-1, -1),
                        (0, -1),
                        (1, -1),
                        (-1, 0),
                        (1, 0),
                        (-1, 1),
                        (0, 1),
                        (1, 1),
                    ];

                    for &(dx, dy) in &offsets {
                        if spawned >= count {
                            break;
                        }
                        let nx = bx as i32 + dx;
                        let ny = by as i32 + dy;
                        if nx >= 0 && ny >= 0 {
                            let (ux, uy) = (nx as usize, ny as usize);
                            if ux < self.map.width
                                && uy < self.map.height
                                && self.map.is_walkable(ux, uy)
                                && !(ux == self.player.x && uy == self.player.y)
                            {
                                let blocked = self
                                    .monsters
                                    .iter()
                                    .any(|m| m.is_alive() && m.x == ux && m.y == uy);
                                if !blocked {
                                    let mut goblin = monster::Monster::new(
                                        ux,
                                        uy,
                                        monster::MonsterType::Goblin,
                                        floor,
                                    );
                                    goblin.can_see_player = true;
                                    goblin.behavior = monster::BehaviorState::Chase;
                                    self.monsters.push(goblin);
                                    spawned += 1;
                                }
                            }
                        }
                    }
                    if spawned > 0 {
                        self.log_msg(format!(
                            "The Goblin King bellows! {} goblins rush to his aid!",
                            spawned
                        ));
                    }
                }
                MonsterAction::BreathAttack {
                    dx,
                    dy,
                    damage,
                    range,
                } => {
                    self.log_msg("The Bone Dragon unleashes a torrent of fire!");
                    let mut cx = self.monsters[i].x as i32;
                    let mut cy = self.monsters[i].y as i32;
                    for _ in 0..range {
                        cx += dx;
                        cy += dy;
                        if cx < 0
                            || cy < 0
                            || cx >= self.map.width as i32
                            || cy >= self.map.height as i32
                        {
                            break;
                        }
                        let (ux, uy) = (cx as usize, cy as usize);
                        if !self.map.is_walkable(ux, uy) {
                            break;
                        }
                        if ux == self.player.x && uy == self.player.y {
                            if self.player.try_dodge() {
                                self.log_msg("You duck under the dragon's breath!");
                            } else {
                                let dmg = self.player.reduce_damage(damage);
                                self.player.take_damage(dmg);
                                self.log_msg(format!(
                                    "Dragon fire engulfs you for {} damage!",
                                    dmg
                                ));
                            }
                        }
                    }
                }
                MonsterAction::ShadowPulse { damage, radius } => {
                    let sx = self.monsters[i].x;
                    let sy = self.monsters[i].y;
                    let dist_to_player = Map::distance(sx, sy, self.player.x, self.player.y);
                    self.log_msg("The Shadow Lord releases a wave of dark energy!");
                    if dist_to_player <= radius {
                        if self.player.try_dodge() {
                            self.log_msg("You resist the shadow pulse!");
                        } else {
                            let dmg = self.player.reduce_damage(damage);
                            self.player.take_damage(dmg);
                            self.log_msg(format!("Shadow energy tears at you for {} damage!", dmg));
                        }
                    }
                }
                MonsterAction::BossTeleport => {
                    let px = self.player.x;
                    let py = self.player.y;
                    let mut candidates: Vec<(usize, usize)> = Vec::new();
                    let search = 8;
                    let min_x = (px as i32 - search).max(0) as usize;
                    let max_x = ((px as i32 + search) as usize).min(self.map.width - 1);
                    let min_y = (py as i32 - search).max(0) as usize;
                    let max_y = ((py as i32 + search) as usize).min(self.map.height - 1);

                    for tx in min_x..=max_x {
                        for ty in min_y..=max_y {
                            let d = Map::distance(tx, ty, px, py);
                            if d >= 2
                                && d <= 5
                                && self.map.is_walkable(tx, ty)
                                && !(tx == px && ty == py)
                            {
                                let occ = self
                                    .monsters
                                    .iter()
                                    .any(|m| m.is_alive() && m.x == tx && m.y == ty);
                                if !occ {
                                    candidates.push((tx, ty));
                                }
                            }
                        }
                    }
                    if !candidates.is_empty() {
                        let idx = {
                            use std::time::SystemTime;
                            SystemTime::now()
                                .duration_since(SystemTime::UNIX_EPOCH)
                                .unwrap()
                                .subsec_nanos() as usize
                        } % candidates.len();
                        let (nx, ny) = candidates[idx];
                        self.monsters[i].x = nx;
                        self.monsters[i].y = ny;
                        self.log_msg(
                            "The Shadow Lord vanishes and reappears in a swirl of darkness!",
                        );
                    }
                }
            }
        }
    }

    fn render_game_layer(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(LAYER_MAP);
        ctx.cls();

        let (front_color, top_color) = get_wall_colors(self.current_floor);

        // Render Map - isometric
        for y in 0..self.map.height {
            for x in 0..self.map.width {
                match self.map.tiles[x][y] {
                    Tile::Wall => {
                        let (show_front, show_left, show_right) =
                            get_wall_visibility(&self.map, x, y);
                        draw_isometric_wall(
                            ctx,
                            x,
                            y,
                            &self.camera,
                            front_color,
                            top_color,
                            show_front,
                            show_left,
                            show_right,
                        );
                    }
                    Tile::Floor | Tile::Stairs => {
                        let fg = if matches!(self.map.tiles[x][y], Tile::Stairs) {
                            RGB::named(YELLOW)
                        } else {
                            get_floor_color(x, y)
                        };
                        let glyph = if matches!(self.map.tiles[x][y], Tile::Stairs) {
                            cp437_char('>')
                        } else {
                            cp437_char('·')
                        };
                        draw_isometric_tile(
                            ctx,
                            x,
                            y,
                            &self.camera,
                            glyph,
                            fg,
                            RGB::named(BLACK),
                            RenderLayer::Floor,
                        );
                    }
                }
            }
        }

        // Render Webs
        for &(x, y) in &self.webs {
            draw_isometric_tile(
                ctx,
                x,
                y,
                &self.camera,
                cp437_char(':'),
                RGB::named(WHITE),
                RGB::named(BLACK),
                RenderLayer::Web,
            );
        }

        // Render Ground Items
        for (&(x, y), item) in &self.ground_items {
            let color = match item.item_type {
                ItemType::Weapon => RGB::named(CYAN),
                ItemType::Armor => RGB::named(DARK_GOLDENROD),
                ItemType::Ring => RGB::named(YELLOW),
                ItemType::Potion => RGB::named(MAGENTA),
            };
            draw_isometric_tile(
                ctx,
                x,
                y,
                &self.camera,
                cp437_char(item.symbol),
                color,
                RGB::named(BLACK),
                RenderLayer::Item,
            );
        }

        // Render Monsters
        for monster in &self.monsters {
            if monster.is_alive() && !monster.is_phasing {
                let color = if monster.stun_ticks > 0 {
                    RGB::named(DARK_GRAY)
                } else if monster.freeze_ticks > 0 {
                    RGB::named(CYAN)
                } else if monster.poison_ticks > 0 {
                    RGB::named(GREEN)
                } else {
                    match monster.symbol {
                        'g' => RGB::named(GREEN),
                        's' => RGB::named(WHITE),
                        'T' => RGB::named(RED),
                        'b' => RGB::named(DARK_GOLDENROD),
                        'x' => RGB::named(PURPLE),
                        'W' => RGB::named(DARK_CYAN),
                        'N' => RGB::named(DARK_RED),
                        'K' => RGB::named(YELLOW),
                        'D' => RGB::named(DARK_RED),
                        'S' => RGB::named(MAGENTA),
                        _ => RGB::named(MAGENTA),
                    }
                };
                draw_isometric_tile(
                    ctx,
                    monster.x,
                    monster.y,
                    &self.camera,
                    cp437_char(monster.symbol),
                    color,
                    RGB::named(BLACK),
                    RenderLayer::Monster,
                );
            }
        }

        // Render Projectiles
        for proj in &self.projectiles {
            draw_isometric_tile(
                ctx,
                proj.x,
                proj.y,
                &self.camera,
                cp437_char(proj.symbol),
                RGB::named(YELLOW),
                RGB::named(BLACK),
                RenderLayer::Projectile,
            );
        }

        // Render Player - multi-glyph
        let p_color = match self.player.class {
            Class::Warrior => RGB::named(RED),
            Class::Rogue => RGB::named(GREEN),
            Class::Mage => RGB::named(BLUE),
        };
        let p_color = if self.player.has_damage_buff() {
            RGB::named(WHITE)
        } else {
            p_color
        };

        // Multi-glyph player rendering
        let glyphs = match self.player.class {
            Class::Warrior => vec![
                (to_cp437(1), p_color, 0.0, -0.5),               // Head
                (to_cp437(206), darken(p_color, 0.7), 0.0, 0.3), // Body
            ],
            Class::Rogue => vec![
                (to_cp437(1), p_color, 0.0, -0.5),
                (to_cp437(208), darken(p_color, 0.7), 0.0, 0.3),
            ],
            Class::Mage => vec![
                (to_cp437(1), p_color, 0.0, -0.8),
                (to_cp437(202), darken(p_color, 0.7), 0.0, 0.3),
                (to_cp437(30), RGB::named(CYAN), 0.0, -1.2), // Hat
            ],
        };

        draw_entity(
            ctx,
            self.player.x,
            self.player.y,
            &self.camera,
            &glyphs,
            RenderLayer::Player,
        );
    }

    fn render_hud_layer(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(LAYER_HUD);
        ctx.cls();

        let map_h = (CONSOLE_HEIGHT - 7) as i32;
        let stats = self.player.effective_stats();
        let weapon_name = self
            .player
            .equipment
            .weapon
            .as_ref()
            .map_or("Fists".to_string(), |w| w.display_name());

        // Status line 1
        let status = format!(
            "Floor: {} | HP: {:2}/{:2} | Lv:{} | {} | {}",
            self.current_floor,
            self.player.hp,
            self.player.max_hp,
            self.player.level,
            self.player.class.name(),
            weapon_name
        );
        ctx.print_color(0, map_h + 1, RGB::named(WHITE), RGB::named(BLACK), &status);

        // Status line 2
        let xp_pct = if self.player.xp_to_next_level > 0 {
            (self.player.xp * 100 / self.player.xp_to_next_level).min(100)
        } else {
            100
        };
        let xp_bar_len = 10;
        let filled = (xp_pct * xp_bar_len / 100) as usize;
        let xp_bar = format!(
            "[{}{}]",
            "#".repeat(filled),
            "-".repeat((xp_bar_len as usize).saturating_sub(filled))
        );

        let stat_line = format!(
            "XP:{}/{} {} | STR:{} DEX:{} INT:{} CON:{} | Def:{} | Tab:Inv",
            self.player.xp,
            self.player.xp_to_next_level,
            xp_bar,
            stats.str_,
            stats.dex,
            stats.int,
            stats.con,
            self.player.equipment.armor_defense()
        );
        ctx.print_color(
            0,
            map_h + 2,
            RGB::named(DARK_GRAY),
            RGB::named(BLACK),
            &stat_line,
        );

        // Status line 3: Abilities
        let a1_text = self
            .player
            .ability_1
            .as_ref()
            .map(|a| format!("[1]{}", a.status_text()))
            .unwrap_or_default();
        let a2_text = self
            .player
            .ability_2
            .as_ref()
            .map(|a| format!("[2]{}", a.status_text()))
            .unwrap_or_else(|| {
                if self.player.level < 5 {
                    "[2]Locked (Lv5)".to_string()
                } else {
                    String::new()
                }
            });
        let poison_text = if self.player.poison_ticks > 0 {
            format!(" | POISONED({})", self.player.poison_ticks)
        } else {
            String::new()
        };
        let ability_line = format!("{} {} {}", a1_text, a2_text, poison_text);

        ctx.print_color(
            0,
            map_h + 3,
            RGB::named(CYAN),
            RGB::named(BLACK),
            &ability_line,
        );

        // Message log (bottom 3 lines)
        for (i, msg) in self.log.iter().rev().take(3).enumerate() {
            ctx.print_color(
                0,
                map_h + 4 + i as i32,
                RGB::named(GREY),
                RGB::named(BLACK),
                msg,
            );
        }
    }
}

fn fill_background(ctx: &mut BTerm, bg: RGB) {
    for y in 0..CONSOLE_HEIGHT as i32 {
        for x in 0..CONSOLE_WIDTH as i32 {
            ctx.set(x, y, bg, bg, cp437_char(' '));
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // --- TICK LOGIC ---
        if self.mode == GameMode::Playing {
            self.monster_tick_accumulator += ctx.frame_time_ms;
            if self.monster_tick_accumulator >= MONSTER_TICK_MS {
                self.monster_tick_accumulator -= MONSTER_TICK_MS;

                self.player.tick_abilities();

                if self.player_web_stuck > 0 {
                    self.player_web_stuck -= 1;
                    if self.player_web_stuck == 0 {
                        self.log_msg("You break free from the web!");
                    }
                }

                if self.player.poison_ticks > 0 {
                    self.player.poison_ticks -= 1;
                    self.player.take_damage(1);
                    self.log_msg("Poison burns in your veins! (-1 HP)");
                }

                self.process_projectiles();
                self.process_monsters();

                if !self.player.is_alive() {
                    self.mode = GameMode::Dead;
                }
            }
        }

        // --- RENDER MAPPING ---
        self.render_game_layer(ctx);
        self.render_hud_layer(ctx);

        // --- LAYER 2: OVERLAYS & MODES ---
        ctx.set_active_console(LAYER_OVERLAY);
        ctx.cls();

        match self.mode {
            GameMode::CharacterCreation => {
                let bg = RGB::from_f32(0.1, 0.1, 0.15);
                fill_background(ctx, bg);
                ctx.print_color_centered(
                    (CONSOLE_HEIGHT / 2 - 2) as i32,
                    RGB::named(WHITE),
                    bg,
                    "=== CHOOSE YOUR CLASS ===",
                );
                ctx.print_color_centered(
                    (CONSOLE_HEIGHT / 2) as i32,
                    RGB::named(RED),
                    bg,
                    "[1] Warrior    [2] Rogue    [3] Mage",
                );

                if let Some(key) = ctx.key {
                    let chosen_class = match key {
                        VirtualKeyCode::Key1 => Some(Class::Warrior),
                        VirtualKeyCode::Key2 => Some(Class::Rogue),
                        VirtualKeyCode::Key3 => Some(Class::Mage),
                        _ => None,
                    };

                    if let Some(c) = chosen_class {
                        self.generate_floor(); // Re-roll map
                        let (sx, sy) = self.map.get_starting_position();
                        self.player = Player::new(sx, sy, c);

                        let (wpn, arm, r) = match c {
                            Class::Warrior => (
                                Some(items::warrior_starting_weapon()),
                                Some(items::warrior_starting_armor()),
                                None,
                            ),
                            Class::Rogue => (Some(items::rogue_starting_weapon()), None, None),
                            Class::Mage => (
                                Some(items::mage_starting_weapon()),
                                None,
                                Some(items::mage_starting_ring()),
                            ),
                        };
                        self.player.equip_starting_gear(wpn, arm, r);
                        self.log.clear();
                        self.log_msg(format!("Welcome, {}. The Ironveil awaits.", c.name()));
                        self.mode = GameMode::Playing;
                    }
                }
            }
            GameMode::Playing => {
                // Input handling
                if let Some(key) = ctx.key {
                    let mut dx = 0;
                    let mut dy = 0;
                    match key {
                        VirtualKeyCode::Up => dy = -1,
                        VirtualKeyCode::Down => dy = 1,
                        VirtualKeyCode::Left => dx = -1,
                        VirtualKeyCode::Right => dx = 1,
                        VirtualKeyCode::Key1 => {
                            if let Some(ref mut a) = self.player.ability_1 {
                                if a.is_ready() && !a.is_active {
                                    a.activate();
                                    let name = a.name.clone();
                                    self.log_msg(format!("{} active!", name));
                                } else {
                                    self.log_msg("Ability not ready.");
                                }
                            }
                        }
                        // TODO: Map more input mapping properly for inventory and dying
                        _ => {}
                    }

                    if dx != 0 || dy != 0 {
                        let nx = self.player.x as i32 + dx;
                        let ny = self.player.y as i32 + dy;
                        if self.map.is_walkable(nx as usize, ny as usize) {
                            self.player.x = nx as usize;
                            self.player.y = ny as usize;
                            self.camera.follow(self.player.x, self.player.y);
                        }
                    }
                }
            }
            GameMode::Inventory => {
                let bg = RGB::from_f32(0.1, 0.1, 0.15);
                fill_background(ctx, bg);
                ctx.print_color_centered(
                    (CONSOLE_HEIGHT / 2) as i32,
                    RGB::named(WHITE),
                    bg,
                    "=== INVENTORY (TODO) ===",
                );
                if let Some(VirtualKeyCode::Tab) | Some(VirtualKeyCode::Escape) = ctx.key {
                    self.mode = GameMode::Playing;
                }
            }
            GameMode::Dead => {
                ctx.print_color_centered(
                    (CONSOLE_HEIGHT / 2) as i32,
                    RGB::named(RED),
                    RGB::named(BLACK),
                    "=== YOU HAVE DIED ===",
                );
                ctx.print_color_centered(
                    (CONSOLE_HEIGHT / 2 + 2) as i32,
                    RGB::named(GREY),
                    RGB::named(BLACK),
                    "Press Enter to restart",
                );
                if let Some(VirtualKeyCode::Return) = ctx.key {
                    self.mode = GameMode::CharacterCreation;
                }
            }
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Ironveil")
        .with_fps_cap(60.0)
        .with_fancy_console(CONSOLE_WIDTH, CONSOLE_HEIGHT, "terminal8x8.png")
        .with_simple_console(CONSOLE_WIDTH, CONSOLE_HEIGHT, "terminal8x8.png")
        .with_simple_console(CONSOLE_WIDTH, CONSOLE_HEIGHT, "terminal8x8.png")
        .build()?;

    let gs = State::new();
    main_loop(context, gs)
}
