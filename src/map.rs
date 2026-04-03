use rand::RngExt;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Tile {
    Wall,
    Floor,
    Stairs,
    SecretDoor,
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

#[derive(Clone, Serialize, Deserialize)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<Tile>>,
    pub rooms: Vec<Rect>,
    pub room_types: Vec<RoomType>,
    pub deco_objects: HashMap<(usize, usize), DecoObject>,
    pub trap_tiles: HashMap<(usize, usize), TrapType>,
    pub shrine_used: HashSet<(usize, usize)>,
    pub visibility: Vec<Vec<bool>>,
    pub current_visibility: Vec<Vec<bool>>,
}

#[derive(Clone, Serialize, Deserialize)]
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
    pub fn new(width: usize, height: usize) -> Self {
        let mut tiles = vec![vec![Tile::Wall; height]; width];
        let mut rooms: Vec<Rect> = Vec::new();

        // Scale rooms based on map size
        let max_rooms = (width * height) / 150;
        let min_size = 6;
        let max_size = 15;

        let mut rng = rand::rng();

        for _ in 0..max_rooms {
            let w = rng.random_range(min_size..max_size);
            let h = rng.random_range(min_size..max_size);
            let x = rng.random_range(1..width - w - 1);
            let y = rng.random_range(1..height - h - 1);

            let new_room = Rect::new(x, y, w, h);
            let mut ok = true;
            for other_room in &rooms {
                if new_room.intersects(other_room) {
                    ok = false;
                    break;
                }
            }

            if ok {
                Map::apply_room_to_map(&mut tiles, &new_room);

                if !rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = rooms[rooms.len() - 1].center();

                    if rng.random_bool(0.5) {
                        Map::apply_horizontal_tunnel(&mut tiles, prev_x, new_x, prev_y);
                        Map::apply_vertical_tunnel(&mut tiles, prev_y, new_y, new_x);
                    } else {
                        Map::apply_vertical_tunnel(&mut tiles, prev_y, new_y, prev_x);
                        Map::apply_horizontal_tunnel(&mut tiles, prev_x, new_x, new_y);
                    }
                }

                rooms.push(new_room);
            }
        }

        // Place stairs in the last room
        if let Some(last_room) = rooms.last() {
            let (x, y) = last_room.center();
            tiles[x][y] = Tile::Stairs;
        }

        let arena_w = rng.random_range(18..24);
        let arena_h = rng.random_range(12..16);
        let arena_x = rng.random_range(2..width.saturating_sub(arena_w + 2));
        let arena_y = rng.random_range(2..height.saturating_sub(arena_h + 2));
        let boss_room = Rect::new(arena_x, arena_y, arena_w, arena_h);

        Map::apply_room_to_map(&mut tiles, &boss_room);

        if let Some(prev_room) = rooms.last() {
            let (new_x, new_y) = boss_room.center();
            let (prev_x, prev_y) = prev_room.center();
            if rng.random_bool(0.5) {
                Map::apply_horizontal_tunnel(&mut tiles, prev_x, new_x, prev_y);
                Map::apply_vertical_tunnel(&mut tiles, prev_y, new_y, new_x);
            } else {
                Map::apply_vertical_tunnel(&mut tiles, prev_y, new_y, prev_x);
                Map::apply_horizontal_tunnel(&mut tiles, prev_x, new_x, new_y);
            }
        }

        let (bx, by) = boss_room.center();
        tiles[bx][by] = Tile::Stairs;

        if !rooms.is_empty() {
            *rooms.last_mut().unwrap() = boss_room.clone();
        } else {
            rooms.push(boss_room);
        }

        let room_types = vec![RoomType::Normal; rooms.len()];

        Map {
            width,
            height,
            tiles,
            rooms,
            room_types,
            deco_objects: HashMap::new(),
            trap_tiles: HashMap::new(),
            shrine_used: HashSet::new(),
            visibility: vec![vec![false; height]; width],
            current_visibility: vec![vec![false; height]; width],
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
}
