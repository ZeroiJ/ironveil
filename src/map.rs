use rand::RngExt;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Tile {
    Wall,
    Floor,
    Stairs,
    SecretDoor,
    ShallowWater,
    DeepWater,
    Lava,
    Chasm,
    ChasmEdge,
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum RoomType {
    Normal,
    Treasure,
    Trap,
    Shrine,
    Secret,
    Boss,
    Spawn,
    Shop,
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum DecoObject {
    Torch,
    Pillar,
    Altar,
    Chest,
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TrapType {
    Spike,
    Fire,
    Teleport,
    Alarm,
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum DungeonFeature {
    WallCrack,
    FloorDebris,
    Bloodstain,
    MossPatch,
    ScorchMark,
    WaterPuddle,
}

#[derive(Clone, Copy, PartialEq)]
pub enum VaultType {
    ThroneRoom,
    Armory,
    Prison,
    RitualChamber,
    TreasureVault,
    Library,
}

pub struct VaultTemplate {
    pub vault_type: VaultType,
    pub pattern: &'static [&'static str],
    pub min_width: usize,
    pub min_height: usize,
}

impl VaultTemplate {
    const fn new(vault_type: VaultType, pattern: &'static [&'static str]) -> Self {
        let min_height = pattern.len();
        let min_width = if min_height > 0 { pattern[0].len() } else { 0 };
        Self {
            vault_type,
            pattern,
            min_width,
            min_height,
        }
    }
}

pub const VAULT_TEMPLATES: &[VaultTemplate] = &[
    VaultTemplate::new(
        VaultType::ThroneRoom,
        &[
            "###.###", "#.....#", "#.P.P.#", "...*...", "#.P.P.#", "#.....#", "###.###",
        ],
    ),
    VaultTemplate::new(
        VaultType::Armory,
        &["#####", "#C.C#", "#...#", "#C.C#", "##.##"],
    ),
    VaultTemplate::new(
        VaultType::Prison,
        &[
            "###.###", "#.#.#.#", "#.#.#.#", ".......", "#.#.#.#", "#.#.#.#", "###.###",
        ],
    ),
    VaultTemplate::new(
        VaultType::RitualChamber,
        &[
            "#######", "#T...T#", "#.....#", "#..A..#", "#.....#", "#T...T#", "###.###",
        ],
    ),
    VaultTemplate::new(
        VaultType::TreasureVault,
        &["#####", "#CCC#", "#C*C#", "#CCC#", "##.##"],
    ),
    VaultTemplate::new(
        VaultType::Library,
        &[
            "#######", "#P...P#", "#.....#", "...A...", "#.....#", "#P...P#", "#######",
        ],
    ),
];

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum MachineType {
    ShrineTrap,
    TreasureKey,
    BossHazard,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Machine {
    pub machine_type: MachineType,
    pub trigger_pos: (usize, usize),
    pub effect_pos: (usize, usize),
    pub activated: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<Tile>>,
    pub rooms: Vec<Rect>,
    pub room_types: Vec<RoomType>,
    pub deco_objects: HashMap<(usize, usize), DecoObject>,
    pub trap_tiles: HashMap<(usize, usize), TrapType>,
    pub features: HashMap<(usize, usize), DungeonFeature>,
    pub shrine_used: HashSet<(usize, usize)>,
    pub machines: Vec<Machine>,
    pub visibility: Vec<Vec<bool>>,
    pub current_visibility: Vec<Vec<bool>>,
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Rect {
    pub x1: usize,
    pub y1: usize,
    pub x2: usize,
    pub y2: usize,
}

impl Rect {
    fn new(x: usize, y: usize, w: usize, h: usize) -> Self {
        Self {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }

    fn center(&self) -> (usize, usize) {
        ((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
    }

    fn intersects(&self, other: &Rect) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }
}

impl Map {
    pub fn new(width: usize, height: usize, floor: i32) -> Self {
        let mut rng = rand::rng();

        let mut tiles = vec![vec![Tile::Wall; height]; width];
        let mut rooms: Vec<Rect> = Vec::new();

        let gen_type = if floor <= 3 {
            "bsp"
        } else if floor <= 6 {
            "cave"
        } else {
            "bsp_deep"
        };

        match gen_type {
            "cave" => {
                rooms = Map::generate_cave(&mut tiles, width, height, &mut rng);
            }
            "bsp_deep" => {
                rooms = Map::generate_bsp(&mut tiles, width, height, 6, &mut rng, floor);
            }
            _ => {
                rooms = Map::generate_bsp(&mut tiles, width, height, 4, &mut rng, floor);
            }
        }

        let target_loops = rng.random_range(3..=5);
        Map::create_loops(&mut tiles, width, height, target_loops);
        Map::cleanup_dungeon(&mut tiles, width, height);

        let room_types = vec![RoomType::Normal; rooms.len()];

        Map {
            width,
            height,
            tiles,
            rooms,
            room_types,
            deco_objects: HashMap::new(),
            trap_tiles: HashMap::new(),
            features: HashMap::new(),
            shrine_used: HashSet::new(),
            machines: Vec::new(),
            visibility: vec![vec![false; height]; width],
            current_visibility: vec![vec![false; height]; width],
        }
    }

    fn generate_bsp(
        tiles: &mut Vec<Vec<Tile>>,
        width: usize,
        height: usize,
        split_depth: i32,
        rng: &mut impl RngExt,
        floor: i32,
    ) -> Vec<Rect> {
        let mut rooms = Vec::new();
        let padding = 2;
        let mut rects = vec![Rect::new(
            padding,
            padding,
            width - padding * 2,
            height - padding * 2,
        )];

        for _ in 0..split_depth {
            let new_rects: Vec<Rect> = rects
                .iter()
                .flat_map(|r| {
                    let rw = r.x2.saturating_sub(r.x1);
                    let rh = r.y2.saturating_sub(r.y1);
                    if rw < 10 || rh < 10 {
                        return vec![*r];
                    }
                    let split_horizontal = if rw > rh * 2 {
                        false
                    } else if rh > rw * 2 {
                        true
                    } else {
                        rng.random_bool(0.5)
                    };

                    if split_horizontal {
                        let split = rng.random_range(r.y1 + 5..r.y2.saturating_sub(5));
                        if split <= r.y1 + 3 || split >= r.y2 - 3 {
                            return vec![*r];
                        }
                        vec![
                            Rect {
                                x1: r.x1,
                                y1: r.y1,
                                x2: r.x2,
                                y2: split,
                            },
                            Rect {
                                x1: r.x1,
                                y1: split,
                                x2: r.x2,
                                y2: r.y2,
                            },
                        ]
                    } else {
                        let split = rng.random_range(r.x1 + 5..r.x2.saturating_sub(5));
                        if split <= r.x1 + 3 || split >= r.x2 - 3 {
                            return vec![*r];
                        }
                        vec![
                            Rect {
                                x1: r.x1,
                                y1: r.y1,
                                x2: split,
                                y2: r.y2,
                            },
                            Rect {
                                x1: split,
                                y1: r.y1,
                                x2: r.x2,
                                y2: r.y2,
                            },
                        ]
                    }
                })
                .collect();
            rects = new_rects;
        }

        let term_ratio = width as f32 / 120.0;
        let min_room_w = (6.0 * term_ratio).max(6.0) as usize;
        let min_room_h = (5.0 * term_ratio).max(5.0) as usize;
        let max_room_w = (18.0 * term_ratio).max(10.0) as usize;
        let max_room_h = (14.0 * term_ratio).max(8.0) as usize;

        for rect in &rects {
            let rw = rect.x2.saturating_sub(rect.x1);
            let rh = rect.y2.saturating_sub(rect.y1);
            if rw < min_room_w + 2 || rh < min_room_h + 2 {
                continue;
            }

            let room_w = rng.random_range(min_room_w..=(max_room_w.min(rw - 2)));
            let room_h = rng.random_range(min_room_h..=(max_room_h.min(rh - 2)));
            let room_x = rng.random_range(rect.x1 + 1..=(rect.x2.saturating_sub(room_w + 1)));
            let room_y = rng.random_range(rect.y1 + 1..=(rect.y2.saturating_sub(room_h + 1)));

            let room = Rect::new(room_x, room_y, room_w, room_h);
            Map::apply_room_to_map(tiles, &room);
            rooms.push(room);
        }

        if rooms.len() > 1 {
            for i in 0..rooms.len() - 1 {
                let (x1, y1) = rooms[i].center();
                let (x2, y2) = rooms[i + 1].center();
                Map::connect_rooms(tiles, x1, y1, x2, y2, rng);
            }
        }

        if rooms.len() > 3 {
            for i in 0..rooms.len() {
                for j in (i + 2)..rooms.len() {
                    if j == i + 1 {
                        continue;
                    }
                    let d = Map::distance(rooms[i].x1, rooms[i].y1, rooms[j].x1, rooms[j].y1);
                    if d < 30 && rng.random_bool(0.2) {
                        let (x1, y1) = rooms[i].center();
                        let (x2, y2) = rooms[j].center();
                        Map::connect_rooms(tiles, x1, y1, x2, y2, rng);
                    }
                }
            }
        }

        let is_boss_floor = floor == 5 || floor == 10 || (floor >= 15 && floor % 5 == 0);

        if is_boss_floor && rooms.len() > 2 {
            let arena_w = (20.0 * term_ratio).clamp(18.0, 24.0) as usize;
            let arena_h = (14.0 * term_ratio).clamp(12.0, 16.0) as usize;
            let prev_room = rooms.last().unwrap();
            let (px, py) = prev_room.center();

            let arena_x = (px as i32
                + rng.random_range(8..15) as i32 * if rng.random_bool(0.5) { 1 } else { -1 })
            .clamp(2, (width - arena_w - 2) as i32) as usize;
            let arena_y = (py as i32
                + rng.random_range(6..12) as i32 * if rng.random_bool(0.5) { 1 } else { -1 })
            .clamp(2, (height - arena_h - 2) as i32) as usize;

            let boss_room = Rect::new(arena_x, arena_y, arena_w, arena_h);
            Map::apply_room_to_map(tiles, &boss_room);

            let (px, py) = prev_room.center();
            let (bx, by) = boss_room.center();
            Map::connect_rooms(tiles, px, py, bx, by, rng);

            let (sx, sy) = boss_room.center();
            if sx < width && sy < height {
                tiles[sx][sy] = Tile::Stairs;
            }

            rooms.push(boss_room);
        } else if let Some(last_room) = rooms.last() {
            let (sx, sy) = last_room.center();
            if sx < width && sy < height {
                tiles[sx][sy] = Tile::Stairs;
            }
        }

        rooms
    }

    fn generate_cave(
        tiles: &mut Vec<Vec<Tile>>,
        width: usize,
        height: usize,
        rng: &mut impl RngExt,
    ) -> Vec<Rect> {
        let mut rooms = Vec::new();

        for x in 0..width {
            for y in 0..height {
                if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
                    tiles[x][y] = Tile::Wall;
                } else if rng.random_bool(0.45) {
                    tiles[x][y] = Tile::Wall;
                } else {
                    tiles[x][y] = Tile::Floor;
                }
            }
        }

        for _ in 0..5 {
            let mut new_tiles = tiles.clone();
            for x in 1..width - 1 {
                for y in 1..height - 1 {
                    let mut wall_count = 0;
                    for dx in -1..=1 {
                        for dy in -1..=1 {
                            if tiles[(x as i32 + dx) as usize][(y as i32 + dy) as usize]
                                == Tile::Wall
                            {
                                wall_count += 1;
                            }
                        }
                    }
                    if wall_count >= 5 {
                        new_tiles[x][y] = Tile::Wall;
                    } else {
                        new_tiles[x][y] = Tile::Floor;
                    }
                }
            }
            *tiles = new_tiles;
        }

        let mut visited = vec![vec![false; height]; width];
        let min_cave_size = 20;

        for x in 1..width - 1 {
            for y in 1..height - 1 {
                if tiles[x][y] == Tile::Floor && !visited[x][y] {
                    let mut cave_tiles = Vec::new();
                    let mut stack = vec![(x, y)];
                    visited[x][y] = true;

                    while let Some((cx, cy)) = stack.pop() {
                        cave_tiles.push((cx, cy));
                        for &(dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                            let nx = cx as i32 + dx;
                            let ny = cy as i32 + dy;
                            if nx > 0
                                && ny > 0
                                && (nx as usize) < width - 1
                                && (ny as usize) < height - 1
                                && !visited[nx as usize][ny as usize]
                                && tiles[nx as usize][ny as usize] == Tile::Floor
                            {
                                visited[nx as usize][ny as usize] = true;
                                stack.push((nx as usize, ny as usize));
                            }
                        }
                    }

                    if cave_tiles.len() >= min_cave_size {
                        let mut min_x = width;
                        let mut min_y = height;
                        let mut max_x = 0;
                        let mut max_y = 0;
                        for &(cx, cy) in &cave_tiles {
                            min_x = min_x.min(cx);
                            min_y = min_y.min(cy);
                            max_x = max_x.max(cx);
                            max_y = max_y.max(cy);
                        }
                        rooms.push(Rect::new(min_x, min_y, max_x - min_x, max_y - min_y));
                    } else {
                        for &(cx, cy) in &cave_tiles {
                            tiles[cx][cy] = Tile::Wall;
                        }
                    }
                }
            }
        }

        if rooms.len() > 1 {
            for i in 0..rooms.len() - 1 {
                let (x1, y1) = rooms[i].center();
                let (x2, y2) = rooms[i + 1].center();
                Map::connect_rooms(tiles, x1, y1, x2, y2, rng);
            }
        }

        if let Some(last_room) = rooms.last() {
            let (sx, sy) = last_room.center();
            if sx < width && sy < height {
                tiles[sx][sy] = Tile::Stairs;
            }
        }

        rooms
    }

    fn connect_rooms(
        tiles: &mut Vec<Vec<Tile>>,
        x1: usize,
        y1: usize,
        x2: usize,
        y2: usize,
        rng: &mut impl RngExt,
    ) {
        let corridor_width = if rng.random_bool(0.3) { 2 } else { 1 };

        let mut cx = x1 as i32;
        let mut cy = y1 as i32;
        let ex = x2 as i32;
        let ey = y2 as i32;

        let horizontal_first = rng.random_bool(0.5);

        if horizontal_first {
            while cx != ex {
                for dy in 0..corridor_width as i32 {
                    let tx = cx;
                    let ty = cy + dy;
                    if tx > 0
                        && ty > 0
                        && (tx as usize) < tiles.len()
                        && (ty as usize) < tiles[0].len()
                    {
                        tiles[tx as usize][ty as usize] = Tile::Floor;
                    }
                }
                cx += if cx < ex { 1 } else { -1 };
            }
            while cy != ey {
                for dx in 0..corridor_width as i32 {
                    let tx = cx + dx;
                    let ty = cy;
                    if tx > 0
                        && ty > 0
                        && (tx as usize) < tiles.len()
                        && (ty as usize) < tiles[0].len()
                    {
                        tiles[tx as usize][ty as usize] = Tile::Floor;
                    }
                }
                cy += if cy < ey { 1 } else { -1 };
            }
        } else {
            while cy != ey {
                for dx in 0..corridor_width as i32 {
                    let tx = cx + dx;
                    let ty = cy;
                    if tx > 0
                        && ty > 0
                        && (tx as usize) < tiles.len()
                        && (ty as usize) < tiles[0].len()
                    {
                        tiles[tx as usize][ty as usize] = Tile::Floor;
                    }
                }
                cy += if cy < ey { 1 } else { -1 };
            }
            while cx != ex {
                for dy in 0..corridor_width as i32 {
                    let tx = cx;
                    let ty = cy + dy;
                    if tx > 0
                        && ty > 0
                        && (tx as usize) < tiles.len()
                        && (ty as usize) < tiles[0].len()
                    {
                        tiles[tx as usize][ty as usize] = Tile::Floor;
                    }
                }
                cx += if cx < ex { 1 } else { -1 };
            }
        }
    }

    pub fn assign_room_types(&mut self, floor: i32) {
        if self.rooms.is_empty() {
            return;
        }

        self.room_types[0] = RoomType::Spawn;
        if self.rooms.len() > 1 {
            self.room_types[self.rooms.len() - 1] = RoomType::Boss;
        }

        let mut rng = rand::rng();
        let special_chance = rng.random_range(0..100);

        if special_chance < 33 && self.rooms.len() > 2 {
            let candidates: Vec<usize> = (1..self.rooms.len() - 1)
                .filter(|&i| self.room_types[i] == RoomType::Normal)
                .collect();

            if !candidates.is_empty() {
                let room_idx = candidates[rng.random_range(0..candidates.len())];
                let room_type = match floor {
                    1..=3 => {
                        let r = rng.random_range(0..100);
                        if r < 50 {
                            RoomType::Treasure
                        } else if r < 80 {
                            RoomType::Trap
                        } else {
                            RoomType::Shrine
                        }
                    }
                    4..=9 => {
                        let r = rng.random_range(0..100);
                        if r < 30 {
                            RoomType::Treasure
                        } else if r < 65 {
                            RoomType::Trap
                        } else if r < 85 {
                            RoomType::Shrine
                        } else {
                            RoomType::Secret
                        }
                    }
                    _ => {
                        let r = rng.random_range(0..100);
                        if r < 20 {
                            RoomType::Treasure
                        } else if r < 50 {
                            RoomType::Trap
                        } else if r < 70 {
                            RoomType::Shrine
                        } else {
                            RoomType::Secret
                        }
                    }
                };
                self.room_types[room_idx] = room_type;
            }
        }

        if floor > 0 && floor % 3 == 0 && self.rooms.len() > 3 {
            let candidates: Vec<usize> = (1..self.rooms.len() - 1)
                .filter(|&i| self.room_types[i] == RoomType::Normal)
                .collect();

            if !candidates.is_empty() {
                let mut rng = rand::rng();
                let room_idx = candidates[rng.random_range(0..candidates.len())];
                self.room_types[room_idx] = RoomType::Shop;
            }
        }
    }

    /// Reveal circular area around player position with line-of-sight.
    pub fn reveal_area(&mut self, px: usize, py: usize, radius: i32) {
        // Reset current visibility
        for x in 0..self.width {
            for y in 0..self.height {
                self.current_visibility[x][y] = false;
            }
        }

        let r = radius;
        let px_i = px as i32;
        let py_i = py as i32;

        for dx in -r..=r {
            for dy in -r..=r {
                if dx * dx + dy * dy > r * r {
                    continue;
                }
                let tx = px_i + dx;
                let ty = py_i + dy;
                if tx < 0 || ty < 0 {
                    continue;
                }
                let tx = tx as usize;
                let ty = ty as usize;
                if tx >= self.width || ty >= self.height {
                    continue;
                }
                if self.has_line_of_sight(px, py, tx, ty) {
                    self.current_visibility[tx][ty] = true;
                    self.visibility[tx][ty] = true;
                }
            }
        }

        // Player tile always visible
        if px < self.width && py < self.height {
            self.current_visibility[px][py] = true;
            self.visibility[px][py] = true;
        }
    }

    /// Reveal tiles at exactly the given radius (ring reveal for animation)
    /// Returns true if any new tiles were revealed
    pub fn reveal_ring(&mut self, px: usize, py: usize, radius: i32) -> bool {
        let px = px as i32;
        let py = py as i32;
        let r = radius;
        let r2 = r * r;
        let prev_r2 = (r - 1) * (r - 1);
        let mut any_revealed = false;

        for y in 0..self.height {
            for x in 0..self.width {
                let dx = x as i32 - px;
                let dy = y as i32 - py;
                let dist2 = dx * dx + dy * dy;

                // Only reveal tiles in the ring (between prev radius and this radius)
                if dist2 <= r2 && dist2 > prev_r2 {
                    self.current_visibility[x][y] = true;
                    self.visibility[x][y] = true;
                    any_revealed = true;
                }
            }
        }

        // Always reveal player position
        if px >= 0 && py >= 0 && (px as usize) < self.width && (py as usize) < self.height {
            self.current_visibility[px as usize][py as usize] = true;
            self.visibility[px as usize][py as usize] = true;
        }

        any_revealed
    }

    /// Fully reveal entire map (for debug or when disabling fog)
    pub fn reveal_all(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.current_visibility[x][y] = true;
                self.visibility[x][y] = true;
            }
        }
    }

    /// Legacy alias for backwards compatibility
    pub fn update_visibility(&mut self, px: usize, py: usize, radius: i32) {
        self.reveal_area(px, py, radius);
    }

    /// Spawn ground items across rooms. Returns a HashMap of position -> Item.
    /// ~25% chance per room (skip room 0 = player spawn).
    pub fn spawn_ground_items(&self, floor: i32) -> HashMap<(usize, usize), crate::items::Item> {
        let mut items = HashMap::new();
        let mut rng = rand::rng();

        for i in 1..self.rooms.len() {
            if rng.random_bool(0.25) {
                let px = rng.random_range(self.rooms[i].x1 + 1..self.rooms[i].x2 - 1);
                let py = rng.random_range(self.rooms[i].y1 + 1..self.rooms[i].y2 - 1);
                let (cx, cy) = self.rooms[i].center();
                // Don't place on monster spawn (center) or stairs
                if self.tiles[px][py] == Tile::Floor && (px != cx || py != cy) {
                    items.insert((px, py), crate::items::random_item(floor));
                }
            }
        }
        items
    }

    fn apply_room_to_map(tiles: &mut Vec<Vec<Tile>>, room: &Rect) {
        for x in room.x1..room.x2 {
            for y in room.y1..room.y2 {
                tiles[x][y] = Tile::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(tiles: &mut Vec<Vec<Tile>>, x1: usize, x2: usize, y: usize) {
        use std::cmp::{max, min};
        let h = tiles[0].len();
        for x in min(x1, x2)..=max(x1, x2) {
            tiles[x][y] = Tile::Floor;
            // Make tunnel 2 tiles wide
            if y + 1 < h - 1 {
                tiles[x][y + 1] = Tile::Floor;
            }
        }
    }

    fn apply_vertical_tunnel(tiles: &mut Vec<Vec<Tile>>, y1: usize, y2: usize, x: usize) {
        use std::cmp::{max, min};
        let w = tiles.len();
        for y in min(y1, y2)..=max(y1, y2) {
            tiles[x][y] = Tile::Floor;
            // Make tunnel 2 tiles wide
            if x + 1 < w - 1 {
                tiles[x + 1][y] = Tile::Floor;
            }
        }
    }

    pub fn is_walkable(&self, x: usize, y: usize) -> bool {
        if x < self.width && y < self.height {
            matches!(
                self.tiles[x][y],
                Tile::Floor | Tile::Stairs | Tile::SecretDoor
            )
        } else {
            false
        }
    }

    pub fn spawn_monsters_for_floor(&self, floor: i32) -> Vec<crate::monster::Monster> {
        let mut monsters = Vec::new();
        let mut rng = rand::rng();
        // Skip rooms[0] (player spawn) and rooms.last() (stairs spawn)
        for i in 1..self.rooms.len().saturating_sub(1) {
            let (cx, cy) = self.rooms[i].center();
            let m = crate::monster::Monster::random_monster(cx, cy, floor);
            if m.monster_type == crate::monster::MonsterType::BatSwarm {
                // Bat Swarms spawn in groups of 2-3
                let count = rng.random_range(2..=3);
                let room = &self.rooms[i];
                for j in 0..count {
                    // Offset each bat within the room
                    let ox = match j {
                        0 => 0,
                        1 => {
                            if cx + 1 < room.x2 {
                                1
                            } else {
                                0
                            }
                        }
                        _ => {
                            if cy + 1 < room.y2 {
                                0
                            } else {
                                0
                            }
                        }
                    };
                    let oy = match j {
                        0 => 0,
                        1 => 0,
                        _ => {
                            if cy + 1 < room.y2 {
                                1
                            } else {
                                0
                            }
                        }
                    };
                    let bx = (cx + ox).min(room.x2 - 1);
                    let by = (cy + oy).min(room.y2 - 1);
                    monsters.push(crate::monster::Monster::new(
                        bx,
                        by,
                        crate::monster::MonsterType::BatSwarm,
                        floor,
                    ));
                }
            } else {
                monsters.push(m);
            }
        }

        // Spawn boss on boss floors (near the stairs in the last room)
        let boss_type = match floor {
            5 => Some(crate::monster::MonsterType::GoblinKing),
            10 => Some(crate::monster::MonsterType::BoneDragon),
            f if f >= 15 && f % 5 == 0 => Some(crate::monster::MonsterType::ShadowLord),
            _ => None,
        };
        if let Some(bt) = boss_type {
            if let Some(last_room) = self.rooms.last() {
                let (cx, cy) = last_room.center();
                // Offset boss 1 tile from stairs so they don't overlap
                let bx = if cx + 1 < last_room.x2 {
                    cx + 1
                } else {
                    cx.saturating_sub(1)
                };
                monsters.push(crate::monster::Monster::new(bx, cy, bt, floor));
            }
        }

        monsters
    }

    /// Bresenham's line algorithm for line-of-sight checks.
    /// Returns true if there is a clear line from (x1,y1) to (x2,y2) with no Wall tiles blocking.
    pub fn has_line_of_sight(&self, x1: usize, y1: usize, x2: usize, y2: usize) -> bool {
        let mut cx = x1 as i32;
        let mut cy = y1 as i32;
        let tx = x2 as i32;
        let ty = y2 as i32;

        let dx = (tx - cx).abs();
        let dy = -(ty - cy).abs();
        let sx = if cx < tx { 1 } else { -1 };
        let sy = if cy < ty { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            // Don't check the start or end tile themselves — only tiles in between
            if (cx != x1 as i32 || cy != y1 as i32) && (cx != tx || cy != ty) {
                let ux = cx as usize;
                let uy = cy as usize;
                if ux >= self.width || uy >= self.height || self.tiles[ux][uy] == Tile::Wall {
                    return false;
                }
            }
            if cx == tx && cy == ty {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                cx += sx;
            }
            if e2 <= dx {
                err += dx;
                cy += sy;
            }
        }
        true
    }

    /// A* pathfinding. Returns the next step toward `goal` from `start`, or None if no path.
    /// `occupied` is a list of positions blocked by other monsters.
    pub fn astar_next_step(
        &self,
        start: (usize, usize),
        goal: (usize, usize),
        occupied: &[(usize, usize)],
    ) -> Option<(usize, usize)> {
        use std::cmp::Ordering;
        use std::collections::{BinaryHeap, HashMap};

        #[derive(Eq, PartialEq)]
        struct Node {
            cost: i32,
            pos: (usize, usize),
        }
        impl Ord for Node {
            fn cmp(&self, other: &Self) -> Ordering {
                other.cost.cmp(&self.cost) // min-heap
            }
        }
        impl PartialOrd for Node {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        let mut open = BinaryHeap::new();
        let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
        let mut g_score: HashMap<(usize, usize), i32> = HashMap::new();

        g_score.insert(start, 0);
        let h = |p: (usize, usize)| -> i32 {
            (p.0 as i32 - goal.0 as i32).abs() + (p.1 as i32 - goal.1 as i32).abs()
        };
        open.push(Node {
            cost: h(start),
            pos: start,
        });

        // Limit search to prevent lag on huge maps
        let mut iterations = 0;
        let max_iterations = 500;

        while let Some(Node { pos, .. }) = open.pop() {
            iterations += 1;
            if iterations > max_iterations {
                return None;
            }

            if pos == goal {
                // Reconstruct path, return the first step
                let mut current = goal;
                while let Some(&prev) = came_from.get(&current) {
                    if prev == start {
                        return Some(current);
                    }
                    current = prev;
                }
                return Some(goal); // start == goal neighbor
            }

            let neighbors: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            for (ndx, ndy) in &neighbors {
                let nx = pos.0 as i32 + ndx;
                let ny = pos.1 as i32 + ndy;
                if nx < 0 || ny < 0 {
                    continue;
                }
                let np = (nx as usize, ny as usize);

                if !self.is_walkable(np.0, np.1) {
                    continue;
                }
                // Don't path through other monsters (unless it's the goal itself)
                if np != goal && occupied.contains(&np) {
                    continue;
                }

                let tentative_g = g_score
                    .get(&pos)
                    .copied()
                    .unwrap_or(i32::MAX)
                    .saturating_add(1);
                if tentative_g < g_score.get(&np).copied().unwrap_or(i32::MAX) {
                    came_from.insert(np, pos);
                    g_score.insert(np, tentative_g);
                    open.push(Node {
                        cost: tentative_g + h(np),
                        pos: np,
                    });
                }
            }
        }
        None
    }

    /// Returns the distance between two points (Manhattan).
    pub fn distance(x1: usize, y1: usize, x2: usize, y2: usize) -> i32 {
        (x1 as i32 - x2 as i32).abs() + (y1 as i32 - y2 as i32).abs()
    }

    /// A* pathfinding that returns the full path distance between two points.
    /// Returns None if no path exists. Used for loop creation to find walls
    /// that separate distant floor tiles.
    fn astar_distance(&self, start: (usize, usize), goal: (usize, usize)) -> Option<i32> {
        use std::cmp::Ordering;
        use std::collections::{BinaryHeap, HashMap};

        if start == goal {
            return Some(0);
        }

        #[derive(Eq, PartialEq)]
        struct Node {
            cost: i32,
            pos: (usize, usize),
        }
        impl Ord for Node {
            fn cmp(&self, other: &Self) -> Ordering {
                other.cost.cmp(&self.cost) // min-heap
            }
        }
        impl PartialOrd for Node {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        let mut open = BinaryHeap::new();
        let mut g_score: HashMap<(usize, usize), i32> = HashMap::new();

        g_score.insert(start, 0);
        let h = |p: (usize, usize)| -> i32 {
            (p.0 as i32 - goal.0 as i32).abs() + (p.1 as i32 - goal.1 as i32).abs()
        };
        open.push(Node {
            cost: h(start),
            pos: start,
        });

        // Limit search to prevent lag — loop creation doesn't need to find very long paths
        let mut iterations = 0;
        let max_iterations = 800;

        while let Some(Node { pos, .. }) = open.pop() {
            iterations += 1;
            if iterations > max_iterations {
                return None;
            }

            if pos == goal {
                return g_score.get(&goal).copied();
            }

            let neighbors: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            for (ndx, ndy) in &neighbors {
                let nx = pos.0 as i32 + ndx;
                let ny = pos.1 as i32 + ndy;
                if nx < 0 || ny < 0 {
                    continue;
                }
                let np = (nx as usize, ny as usize);

                if !self.is_walkable(np.0, np.1) {
                    continue;
                }

                let tentative_g = g_score
                    .get(&pos)
                    .copied()
                    .unwrap_or(i32::MAX)
                    .saturating_add(1);
                if tentative_g < g_score.get(&np).copied().unwrap_or(i32::MAX) {
                    g_score.insert(np, tentative_g);
                    open.push(Node {
                        cost: tentative_g + h(np),
                        pos: np,
                    });
                }
            }
        }
        None
    }

    /// Brogue-style loop creation: scan walls with floor on both sides,
    /// punch through if the floor tiles are distant by A* pathfinding.
    /// Target: 3-5 loops per floor.
    fn create_loops(tiles: &mut Vec<Vec<Tile>>, width: usize, height: usize, target_loops: usize) {
        let min_astar_distance = 15;
        let mut loops_created = 0;
        let mut candidates: Vec<(usize, usize, (usize, usize), (usize, usize))> = Vec::new();

        let temp_map = Map {
            width,
            height,
            tiles: tiles.clone(),
            rooms: Vec::new(),
            room_types: Vec::new(),
            deco_objects: HashMap::new(),
            trap_tiles: HashMap::new(),
            features: HashMap::new(),
            shrine_used: HashSet::new(),
            machines: Vec::new(),
            visibility: vec![vec![false; height]; width],
            current_visibility: vec![vec![false; height]; width],
        };

        for x in 1..width - 1 {
            for y in 1..height - 1 {
                if tiles[x][y] != Tile::Wall {
                    continue;
                }

                let floor_left = x > 0 && tiles[x - 1][y] == Tile::Floor;
                let floor_right = x < width - 1 && tiles[x + 1][y] == Tile::Floor;

                if floor_left && floor_right {
                    let left_pos = (x - 1, y);
                    let right_pos = (x + 1, y);
                    if let Some(dist) = temp_map.astar_distance(left_pos, right_pos) {
                        if dist >= min_astar_distance {
                            candidates.push((x, y, left_pos, right_pos));
                        }
                    }
                }

                let floor_up = y > 0 && tiles[x][y - 1] == Tile::Floor;
                let floor_down = y < height - 1 && tiles[x][y + 1] == Tile::Floor;

                if floor_up && floor_down {
                    let up_pos = (x, y - 1);
                    let down_pos = (x, y + 1);
                    if let Some(dist) = temp_map.astar_distance(up_pos, down_pos) {
                        if dist >= min_astar_distance {
                            candidates.push((x, y, up_pos, down_pos));
                        }
                    }
                }
            }
        }

        candidates.sort_by(|a, b| {
            let dist_a = temp_map.astar_distance(a.2, a.3).unwrap_or(0);
            let dist_b = temp_map.astar_distance(b.2, b.3).unwrap_or(0);
            dist_b.cmp(&dist_a)
        });

        for (wx, wy, _, _) in candidates {
            if loops_created >= target_loops {
                break;
            }

            if tiles[wx][wy] != Tile::Wall {
                continue;
            }

            tiles[wx][wy] = Tile::Floor;
            loops_created += 1;
        }
    }

    fn cleanup_dungeon(tiles: &mut Vec<Vec<Tile>>, width: usize, height: usize) {
        let mut changed = true;
        let max_passes = 3;
        let mut pass = 0;

        while changed && pass < max_passes {
            changed = false;
            pass += 1;

            for x in 1..width - 1 {
                for y in 1..height - 1 {
                    if tiles[x][y] == Tile::Wall {
                        let floor_nw = tiles[x - 1][y - 1] == Tile::Floor;
                        let floor_ne = tiles[x + 1][y - 1] == Tile::Floor;
                        let floor_sw = tiles[x - 1][y + 1] == Tile::Floor;
                        let floor_se = tiles[x + 1][y + 1] == Tile::Floor;
                        let floor_n = tiles[x][y - 1] == Tile::Floor;
                        let floor_s = tiles[x][y + 1] == Tile::Floor;
                        let floor_w = tiles[x - 1][y] == Tile::Floor;
                        let floor_e = tiles[x + 1][y] == Tile::Floor;

                        let diag_gap =
                            (floor_nw && floor_se && !floor_n && !floor_s && !floor_w && !floor_e)
                                || (floor_ne
                                    && floor_sw
                                    && !floor_n
                                    && !floor_s
                                    && !floor_w
                                    && !floor_e);

                        if diag_gap {
                            tiles[x][y] = Tile::Floor;
                            changed = true;
                            continue;
                        }

                        let wall_n = tiles[x][y - 1] == Tile::Wall;
                        let wall_s = tiles[x][y + 1] == Tile::Wall;
                        let wall_w = tiles[x - 1][y] == Tile::Wall;
                        let wall_e = tiles[x + 1][y] == Tile::Wall;
                        let cardinal_walls = [wall_n, wall_s, wall_w, wall_e]
                            .iter()
                            .filter(|&&w| w)
                            .count();

                        if cardinal_walls == 0 {
                            tiles[x][y] = Tile::Floor;
                            changed = true;
                            continue;
                        }

                        if cardinal_walls == 1 {
                            let floor_count = [floor_n, floor_s, floor_w, floor_e]
                                .iter()
                                .filter(|&&f| f)
                                .count();
                            if floor_count >= 2 {
                                tiles[x][y] = Tile::Floor;
                                changed = true;
                            }
                        }
                    }
                }
            }
        }
    }

    /// Check if a position is in a corridor (walls on 2+ opposite sides).
    pub fn is_corridor(&self, x: usize, y: usize) -> bool {
        if x == 0 || y == 0 || x >= self.width - 1 || y >= self.height - 1 {
            return false;
        }
        let wall_left = self.tiles[x - 1][y] == Tile::Wall;
        let wall_right = self.tiles[x + 1][y] == Tile::Wall;
        let wall_up = self.tiles[x][y - 1] == Tile::Wall;
        let wall_down = self.tiles[x][y + 1] == Tile::Wall;
        (wall_left && wall_right) || (wall_up && wall_down)
    }

    pub fn get_room_type_at(&self, x: usize, y: usize) -> RoomType {
        for (i, room) in self.rooms.iter().enumerate() {
            if x >= room.x1 && x < room.x2 && y >= room.y1 && y < room.y2 {
                return self.room_types[i];
            }
        }
        RoomType::Normal
    }

    pub fn get_starting_position(&self) -> (usize, usize) {
        for x in 0..self.width {
            for y in 0..self.height {
                if self.tiles[x][y] == Tile::Floor {
                    return (x, y);
                }
            }
        }
        (1, 1)
    }

    pub fn generate_decorations(&mut self) {
        let mut rng = rand::rng();

        for (i, room_type) in self.room_types.iter().enumerate() {
            let room = &self.rooms[i];

            match room_type {
                RoomType::Treasure => {
                    let (cx, cy) = room.center();
                    self.deco_objects.insert((cx, cy), DecoObject::Chest);
                }
                RoomType::Shrine => {
                    let (cx, cy) = room.center();
                    self.deco_objects.insert((cx, cy), DecoObject::Altar);
                    if room.x2 - room.x1 > 10 || room.y2 - room.y1 > 8 {
                        let pillar_positions = [
                            (room.x1 + 2, room.y1 + 2),
                            (room.x2 - 3, room.y1 + 2),
                            (room.x1 + 2, room.y2 - 3),
                            (room.x2 - 3, room.y2 - 3),
                        ];
                        for &(px, py) in &pillar_positions {
                            if px < self.width && py < self.height {
                                self.deco_objects.insert((px, py), DecoObject::Pillar);
                            }
                        }
                    }
                }
                RoomType::Normal | RoomType::Trap => {
                    let torch_count = rng.random_range(1..=2);
                    for _ in 0..torch_count {
                        for y in (room.y1 + 1)..(room.y2 - 1) {
                            for x in (room.x1 + 1)..(room.x2 - 1) {
                                if self.tiles[x][y] == Tile::Floor {
                                    let has_wall_neighbor = (x > 0
                                        && self.tiles[x - 1][y] == Tile::Wall)
                                        || (x < self.width - 1
                                            && self.tiles[x + 1][y] == Tile::Wall)
                                        || (y > 0 && self.tiles[x][y - 1] == Tile::Wall)
                                        || (y < self.height - 1
                                            && self.tiles[x][y + 1] == Tile::Wall);
                                    if has_wall_neighbor && rng.random_bool(0.3) {
                                        self.deco_objects.insert((x, y), DecoObject::Torch);
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    if room.x2 - room.x1 > 10 || room.y2 - room.y1 > 8 {
                        let pillar_positions = [
                            (room.x1 + 2, room.y1 + 2),
                            (room.x2 - 3, room.y1 + 2),
                            (room.x1 + 2, room.y2 - 3),
                            (room.x2 - 3, room.y2 - 3),
                        ];
                        for &(px, py) in &pillar_positions {
                            if px < self.width && py < self.height {
                                self.deco_objects.insert((px, py), DecoObject::Pillar);
                            }
                        }
                    }
                }
                RoomType::Secret => {
                    let (cx, cy) = room.center();
                    self.deco_objects.insert((cx, cy), DecoObject::Chest);
                    if room.x2 - room.x1 > 6 && room.y2 - room.y1 > 6 {
                        for dy in 2..(room.y2 - room.y1 - 2) {
                            for dx in 2..(room.x2 - room.x1 - 2) {
                                let rx = room.x1 + dx;
                                let ry = room.y1 + dy;
                                if self.tiles[rx][ry] == Tile::Floor && rng.random_bool(0.1) {
                                    self.deco_objects.insert((rx, ry), DecoObject::Chest);
                                }
                            }
                        }
                    }
                }
                RoomType::Trap => {
                    for y in (room.y1 + 1)..(room.y2 - 1) {
                        for x in (room.x1 + 1)..(room.x2 - 1) {
                            if self.tiles[x][y] == Tile::Floor && rng.random_bool(0.2) {
                                let trap_type = match rng.random_range(0..4) {
                                    0 => TrapType::Spike,
                                    1 => TrapType::Fire,
                                    2 => TrapType::Teleport,
                                    _ => TrapType::Alarm,
                                };
                                self.trap_tiles.insert((x, y), trap_type);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    pub fn scatter_features(&mut self, floor: i32) {
        let mut rng = rand::rng();

        for x in 1..self.width - 1 {
            for y in 1..self.height - 1 {
                if self.tiles[x][y] == Tile::Wall {
                    let has_floor_neighbor = (x > 0 && self.tiles[x - 1][y] == Tile::Floor)
                        || (x < self.width - 1 && self.tiles[x + 1][y] == Tile::Floor)
                        || (y > 0 && self.tiles[x][y - 1] == Tile::Floor)
                        || (y < self.height - 1 && self.tiles[x][y + 1] == Tile::Floor);

                    if has_floor_neighbor {
                        let crack_chance = 0.02 + (floor as f64 * 0.005);
                        if rng.random_bool(crack_chance.min(0.08)) {
                            self.features.insert((x, y), DungeonFeature::WallCrack);
                        }
                    }
                }

                if self.tiles[x][y] == Tile::Floor
                    && !self.deco_objects.contains_key(&(x, y))
                    && !self.trap_tiles.contains_key(&(x, y))
                {
                    let room_type = self.get_room_type_at(x, y);

                    match room_type {
                        RoomType::Trap => {
                            if rng.random_bool(0.15) {
                                self.features.insert((x, y), DungeonFeature::ScorchMark);
                            }
                        }
                        RoomType::Shrine | RoomType::Treasure => {
                            if rng.random_bool(0.12) {
                                self.features.insert((x, y), DungeonFeature::MossPatch);
                            }
                        }
                        RoomType::Boss => {
                            if rng.random_bool(0.20) {
                                self.features.insert((x, y), DungeonFeature::Bloodstain);
                            }
                        }
                        _ => {
                            if rng.random_bool(0.03) {
                                self.features.insert((x, y), DungeonFeature::FloorDebris);
                            }
                            if floor >= 4 && floor <= 6 && rng.random_bool(0.05) {
                                self.features.insert((x, y), DungeonFeature::WaterPuddle);
                            }
                        }
                    }
                }
            }
        }

        for (i, room_type) in self.room_types.iter().enumerate() {
            if *room_type == RoomType::Normal || *room_type == RoomType::Boss {
                let room = &self.rooms[i];
                let spawn_x = (room.x1 + room.x2) / 2;
                let spawn_y = (room.y1 + room.y2) / 2;

                for dx in -1i32..=1 {
                    for dy in -1i32..=1 {
                        let bx = (spawn_x as i32 + dx) as usize;
                        let by = (spawn_y as i32 + dy) as usize;
                        if bx < self.width
                            && by < self.height
                            && self.tiles[bx][by] == Tile::Floor
                            && !self.deco_objects.contains_key(&(bx, by))
                            && rng.random_bool(0.25)
                        {
                            self.features.insert((bx, by), DungeonFeature::Bloodstain);
                        }
                    }
                }
            }
        }
    }

    /// Generate a cellular automata blob for lake/chasm placement
    /// Returns a 2D grid of booleans where true = liquid/chasm
    fn generate_ca_blob(
        &self,
        width: usize,
        height: usize,
        initial_density: f64,
        iterations: usize,
    ) -> Vec<Vec<bool>> {
        let mut rng = rand::rng();
        let mut grid = vec![vec![false; height]; width];

        // Initialize with random fill
        for x in 0..width {
            for y in 0..height {
                grid[x][y] = rng.random_bool(initial_density);
            }
        }

        // Run CA iterations
        for _ in 0..iterations {
            let mut new_grid = vec![vec![false; height]; width];
            for x in 0..width {
                for y in 0..height {
                    let mut neighbors = 0;
                    for dx in -1i32..=1 {
                        for dy in -1i32..=1 {
                            if dx == 0 && dy == 0 {
                                continue;
                            }
                            let nx = x as i32 + dx;
                            let ny = y as i32 + dy;
                            if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                                if grid[nx as usize][ny as usize] {
                                    neighbors += 1;
                                }
                            } else {
                                // Treat out of bounds as filled (helps create rounded edges)
                                neighbors += 1;
                            }
                        }
                    }
                    // 5-neighbor rule: cell is alive if it has 5+ neighbors or was alive with 4+
                    new_grid[x][y] = neighbors >= 5 || (grid[x][y] && neighbors >= 4);
                }
            }
            grid = new_grid;
        }

        grid
    }

    /// Flood fill to count connected floor tiles from a starting position
    fn flood_fill_count(
        &self,
        start_x: usize,
        start_y: usize,
        excluded: &HashSet<(usize, usize)>,
    ) -> usize {
        let mut visited = HashSet::new();
        let mut stack = vec![(start_x, start_y)];
        let mut count = 0;

        while let Some((x, y)) = stack.pop() {
            if visited.contains(&(x, y)) {
                continue;
            }
            if excluded.contains(&(x, y)) {
                continue;
            }
            if x >= self.width || y >= self.height {
                continue;
            }
            if self.tiles[x][y] != Tile::Floor {
                continue;
            }

            visited.insert((x, y));
            count += 1;

            if x > 0 {
                stack.push((x - 1, y));
            }
            if x < self.width - 1 {
                stack.push((x + 1, y));
            }
            if y > 0 {
                stack.push((x, y - 1));
            }
            if y < self.height - 1 {
                stack.push((x, y + 1));
            }
        }

        count
    }

    /// Check if placing a lake at the given position would disconnect the dungeon
    fn would_disconnect(&self, lake_tiles: &HashSet<(usize, usize)>) -> bool {
        // Find a floor tile not in the lake to start flood fill
        let mut start = None;
        for x in 1..self.width - 1 {
            for y in 1..self.height - 1 {
                if self.tiles[x][y] == Tile::Floor && !lake_tiles.contains(&(x, y)) {
                    start = Some((x, y));
                    break;
                }
            }
            if start.is_some() {
                break;
            }
        }

        let Some((sx, sy)) = start else {
            return true;
        };

        // Count all floor tiles
        let total_floor: usize = (1..self.width - 1)
            .flat_map(|x| (1..self.height - 1).map(move |y| (x, y)))
            .filter(|&(x, y)| self.tiles[x][y] == Tile::Floor && !lake_tiles.contains(&(x, y)))
            .count();

        // Count reachable floor tiles
        let reachable = self.flood_fill_count(sx, sy, lake_tiles);

        // If not all floor tiles are reachable, the lake would disconnect
        reachable < total_floor
    }

    /// Place a lake (water, lava, or chasm) using cellular automata
    /// floor 4-6: water lakes
    /// floor 7+: lava lakes or chasms
    pub fn place_lakes(&mut self, floor: i32) {
        let mut rng = rand::rng();

        // Determine lake type based on floor
        let lake_type = if floor >= 7 {
            if rng.random_bool(0.5) {
                Tile::Lava
            } else {
                Tile::Chasm
            }
        } else if floor >= 4 {
            Tile::DeepWater
        } else {
            return; // No lakes on floors 1-3
        };

        // Number of lakes to attempt (1-2)
        let num_lakes = if rng.random_bool(0.3) { 2 } else { 1 };

        for _ in 0..num_lakes {
            // Generate a CA blob (size varies)
            let blob_width = rng.random_range(8..16);
            let blob_height = rng.random_range(8..16);
            let blob = self.generate_ca_blob(blob_width, blob_height, 0.42, 5);

            // Try multiple positions to place the lake
            let mut best_placement: Option<(usize, usize, HashSet<(usize, usize)>)> = None;

            for _ in 0..30 {
                // Pick a random room to place the lake in
                if self.rooms.is_empty() {
                    break;
                }
                let room_idx = rng.random_range(0..self.rooms.len());
                let room = &self.rooms[room_idx];

                // Skip small rooms
                let room_width = room.x2 - room.x1;
                let room_height = room.y2 - room.y1;
                if room_width < 6 || room_height < 6 {
                    continue;
                }

                // Calculate offset to center blob in room
                let offset_x = room.x1 + (room_width.saturating_sub(blob_width)) / 2;
                let offset_y = room.y1 + (room_height.saturating_sub(blob_height)) / 2;

                // Convert blob to map coordinates
                let mut lake_tiles = HashSet::new();
                for bx in 0..blob_width {
                    for by in 0..blob_height {
                        if !blob[bx][by] {
                            continue;
                        }
                        let mx = offset_x + bx;
                        let my = offset_y + by;

                        // Only place on floor tiles, not near edges
                        if mx < 2 || mx >= self.width - 2 {
                            continue;
                        }
                        if my < 2 || my >= self.height - 2 {
                            continue;
                        }
                        if self.tiles[mx][my] != Tile::Floor {
                            continue;
                        }

                        // Don't place on doors, stairs, or special tiles
                        if self.deco_objects.contains_key(&(mx, my)) {
                            continue;
                        }
                        if self.trap_tiles.contains_key(&(mx, my)) {
                            continue;
                        }

                        lake_tiles.insert((mx, my));
                    }
                }

                // Need at least 6 tiles for a meaningful lake
                if lake_tiles.len() < 6 {
                    continue;
                }

                // Check connectivity
                if !self.would_disconnect(&lake_tiles) {
                    best_placement = Some((offset_x, offset_y, lake_tiles));
                    break;
                }
            }

            // Apply the lake
            if let Some((_, _, lake_tiles)) = best_placement {
                // First pass: place the deep tiles
                for &(x, y) in &lake_tiles {
                    self.tiles[x][y] = lake_type;
                }

                // Second pass: add wreaths (shallow water or chasm edges)
                let wreath_type = match lake_type {
                    Tile::DeepWater => Tile::ShallowWater,
                    Tile::Lava => Tile::ShallowWater, // Cooling lava at edges
                    Tile::Chasm => Tile::ChasmEdge,
                    _ => continue,
                };

                for &(x, y) in &lake_tiles {
                    for dx in -1i32..=1 {
                        for dy in -1i32..=1 {
                            if dx == 0 && dy == 0 {
                                continue;
                            }
                            let nx = (x as i32 + dx) as usize;
                            let ny = (y as i32 + dy) as usize;
                            if nx < self.width && ny < self.height {
                                if self.tiles[nx][ny] == Tile::Floor {
                                    self.tiles[nx][ny] = wreath_type;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn stamp_vaults(&mut self, floor: i32) {
        if floor < 2 {
            return;
        }

        let mut rng = rand::rng();
        let num_vaults = if floor >= 5 { 2 } else { 1 };
        let mut stamped_rooms: HashSet<usize> = HashSet::new();

        for _ in 0..num_vaults {
            let suitable_templates: Vec<&VaultTemplate> = VAULT_TEMPLATES
                .iter()
                .filter(|v| match v.vault_type {
                    VaultType::ThroneRoom => floor >= 5,
                    VaultType::Armory => true,
                    VaultType::Prison => floor >= 3,
                    VaultType::RitualChamber => floor >= 4,
                    VaultType::TreasureVault => floor >= 3,
                    VaultType::Library => floor >= 2,
                })
                .collect();

            if suitable_templates.is_empty() {
                continue;
            }

            for room_idx in 0..self.rooms.len() {
                if stamped_rooms.contains(&room_idx) {
                    continue;
                }
                if self.room_types.get(room_idx) == Some(&RoomType::Spawn) {
                    continue;
                }
                if self.room_types.get(room_idx) == Some(&RoomType::Boss) {
                    continue;
                }

                let room = &self.rooms[room_idx];
                let room_width = room.x2 - room.x1;
                let room_height = room.y2 - room.y1;

                let fitting_templates: Vec<&&VaultTemplate> = suitable_templates
                    .iter()
                    .filter(|v| v.min_width <= room_width && v.min_height <= room_height)
                    .collect();

                if fitting_templates.is_empty() {
                    continue;
                }
                if !rng.random_bool(0.4) {
                    continue;
                }

                let template = fitting_templates[rng.random_range(0..fitting_templates.len())];

                let offset_x = room.x1 + (room_width - template.min_width) / 2;
                let offset_y = room.y1 + (room_height - template.min_height) / 2;

                for (py, row) in template.pattern.iter().enumerate() {
                    for (px, ch) in row.chars().enumerate() {
                        let mx = offset_x + px;
                        let my = offset_y + py;

                        if mx >= self.width || my >= self.height {
                            continue;
                        }

                        match ch {
                            '#' => {
                                if self.tiles[mx][my] == Tile::Floor {
                                    self.tiles[mx][my] = Tile::Wall;
                                }
                            }
                            '.' => {
                                if self.tiles[mx][my] == Tile::Wall {
                                    self.tiles[mx][my] = Tile::Floor;
                                }
                            }
                            'P' => {
                                self.tiles[mx][my] = Tile::Floor;
                                self.deco_objects.insert((mx, my), DecoObject::Pillar);
                            }
                            'T' => {
                                self.tiles[mx][my] = Tile::Floor;
                                self.deco_objects.insert((mx, my), DecoObject::Torch);
                            }
                            'A' => {
                                self.tiles[mx][my] = Tile::Floor;
                                self.deco_objects.insert((mx, my), DecoObject::Altar);
                            }
                            'C' => {
                                self.tiles[mx][my] = Tile::Floor;
                                self.deco_objects.insert((mx, my), DecoObject::Chest);
                            }
                            '*' => {
                                self.tiles[mx][my] = Tile::Floor;
                            }
                            _ => {}
                        }
                    }
                }

                stamped_rooms.insert(room_idx);
                break;
            }
        }
    }

    pub fn place_machines(&mut self, floor: i32) {
        if floor < 3 {
            return;
        }

        let mut rng = rand::rng();

        let shrine_rooms: Vec<usize> = self
            .room_types
            .iter()
            .enumerate()
            .filter(|(_, rt)| **rt == RoomType::Shrine)
            .map(|(i, _)| i)
            .collect();

        for shrine_room_idx in shrine_rooms {
            if !rng.random_bool(0.5) {
                continue;
            }

            let trigger_room = &self.rooms[shrine_room_idx];
            let trigger_pos = trigger_room.center();

            let trap_room_idx = (0..self.rooms.len())
                .filter(|&i| i != shrine_room_idx)
                .filter(|&i| self.room_types.get(i) != Some(&RoomType::Spawn))
                .filter(|&i| self.room_types.get(i) != Some(&RoomType::Boss))
                .max_by_key(|&i| {
                    let r = &self.rooms[i];
                    let c = r.center();
                    let dx = c.0 as i32 - trigger_pos.0 as i32;
                    let dy = c.1 as i32 - trigger_pos.1 as i32;
                    dx * dx + dy * dy
                });

            if let Some(trap_idx) = trap_room_idx {
                let trap_room = &self.rooms[trap_idx];
                let effect_pos = trap_room.center();

                self.machines.push(Machine {
                    machine_type: MachineType::ShrineTrap,
                    trigger_pos,
                    effect_pos,
                    activated: false,
                });
            }
        }

        if floor >= 5 {
            let boss_rooms: Vec<usize> = self
                .room_types
                .iter()
                .enumerate()
                .filter(|(_, rt)| **rt == RoomType::Boss)
                .map(|(i, _)| i)
                .collect();

            for boss_room_idx in boss_rooms {
                let room = &self.rooms[boss_room_idx];
                let room_width = room.x2 - room.x1;
                let room_height = room.y2 - room.y1;

                if room_width >= 8 && room_height >= 8 {
                    let hazard_positions = [
                        (room.x1 + 2, room.y1 + 2),
                        (room.x2 - 3, room.y1 + 2),
                        (room.x1 + 2, room.y2 - 3),
                        (room.x2 - 3, room.y2 - 3),
                    ];

                    for &(hx, hy) in &hazard_positions {
                        if hx < self.width
                            && hy < self.height
                            && self.tiles[hx][hy] == Tile::Floor
                            && rng.random_bool(0.4)
                        {
                            self.tiles[hx][hy] = if floor >= 7 {
                                Tile::Lava
                            } else {
                                Tile::DeepWater
                            };

                            for dx in -1i32..=1 {
                                for dy in -1i32..=1 {
                                    if dx == 0 && dy == 0 {
                                        continue;
                                    }
                                    let nx = (hx as i32 + dx) as usize;
                                    let ny = (hy as i32 + dy) as usize;
                                    if nx < self.width
                                        && ny < self.height
                                        && self.tiles[nx][ny] == Tile::Floor
                                    {
                                        self.tiles[nx][ny] = if floor >= 7 {
                                            Tile::ShallowWater
                                        } else {
                                            Tile::ShallowWater
                                        };
                                    }
                                }
                            }
                        }
                    }

                    self.machines.push(Machine {
                        machine_type: MachineType::BossHazard,
                        trigger_pos: room.center(),
                        effect_pos: room.center(),
                        activated: true,
                    });
                }
            }
        }
    }

    pub fn activate_machine(
        &mut self,
        trigger_pos: (usize, usize),
    ) -> Option<(MachineType, (usize, usize))> {
        for machine in &mut self.machines {
            if machine.trigger_pos == trigger_pos && !machine.activated {
                machine.activated = true;

                if machine.machine_type == MachineType::ShrineTrap {
                    let (ex, ey) = machine.effect_pos;
                    if ex < self.width && ey < self.height {
                        self.trap_tiles.insert((ex, ey), TrapType::Fire);
                        for dx in -1i32..=1 {
                            for dy in -1i32..=1 {
                                let nx = (ex as i32 + dx) as usize;
                                let ny = (ey as i32 + dy) as usize;
                                if nx < self.width
                                    && ny < self.height
                                    && self.tiles[nx][ny] == Tile::Floor
                                {
                                    if rand::rng().random_bool(0.3) {
                                        self.trap_tiles.insert((nx, ny), TrapType::Fire);
                                    }
                                }
                            }
                        }
                    }
                }

                return Some((machine.machine_type, machine.effect_pos));
            }
        }
        None
    }
}
