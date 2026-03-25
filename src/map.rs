use rand::RngExt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Tile {
    Wall,
    Floor,
    Stairs,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<Tile>>,
    pub rooms: Vec<Rect>,
    pub visibility: Vec<Vec<bool>>, // true if tile has been seen
    pub current_visibility: Vec<Vec<bool>>, // true if tile is currently visible
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

        Map {
            width,
            height,
            tiles,
            rooms,
            visibility: vec![vec![false; height]; width],
            current_visibility: vec![vec![false; height]; width],
        }
    }

    /// Reveal circular area around player position.
    /// Clears current visibility, then reveals tiles within radius.
    /// Both current_visibility (what's seen now) and visibility (memory) are updated.
    pub fn reveal_area(&mut self, px: usize, py: usize, radius: i32) {
        // Clear current visibility (what's seen this frame)
        for x in 0..self.width {
            for y in 0..self.height {
                self.current_visibility[x][y] = false;
            }
        }

        let r2 = radius * radius;

        // Check all tiles in bounding box around player
        for dy in -radius..=radius {
            for dx in -radius..=radius {
                let nx = px as i32 + dx;
                let ny = py as i32 + dy;

                // Bounds check
                if nx < 0 || ny < 0 || nx >= self.width as i32 || ny >= self.height as i32 {
                    continue;
                }

                // Circular Euclidean distance check
                if (dx * dx + dy * dy) <= r2 {
                    let (ux, uy) = (nx as usize, ny as usize);

                    // Simple illumination - no line of sight check
                    // This reveals walls AND floors within radius
                    self.current_visibility[ux][uy] = true;
                    self.visibility[ux][uy] = true;
                }
            }
        }

        // Always reveal player position
        self.current_visibility[px][py] = true;
        self.visibility[px][py] = true;
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
            self.tiles[x][y] == Tile::Floor || self.tiles[x][y] == Tile::Stairs
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
            15 => Some(crate::monster::MonsterType::ShadowLord),
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
}
