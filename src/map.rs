use rand::RngExt;

#[derive(Clone, Copy, PartialEq)]
pub enum Tile {
    Wall,
    Floor,
    Stairs,
    Potion,
}

pub struct Map {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<Tile>>,
    pub rooms: Vec<Rect>,
}

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

        // Scatter health potions across rooms procedurally
        // Skip room 0 (player spawn) — potions appear in dungeon rooms
        for i in 1..rooms.len() {
            // ~40% chance a room has a potion
            if rng.random_bool(0.4) {
                // Pick a random floor tile inside the room (not the center, to avoid monster/stairs overlap)
                let px = rng.random_range(rooms[i].x1 + 1..rooms[i].x2 - 1);
                let py = rng.random_range(rooms[i].y1 + 1..rooms[i].y2 - 1);
                let (cx, cy) = rooms[i].center();
                // Don't overwrite stairs or place exactly on monster spawn (room center)
                if tiles[px][py] == Tile::Floor && (px != cx || py != cy) {
                    tiles[px][py] = Tile::Potion;
                }
            }
        }

        Map {
            width,
            height,
            tiles,
            rooms,
        }
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
            self.tiles[x][y] == Tile::Floor
                || self.tiles[x][y] == Tile::Stairs
                || self.tiles[x][y] == Tile::Potion
        } else {
            false
        }
    }

    pub fn spawn_monsters(&self) -> Vec<crate::monster::Monster> {
        let mut monsters = Vec::new();
        // Skip rooms[0] (player spawn) and rooms.last() (stairs spawn)
        for i in 1..self.rooms.len().saturating_sub(1) {
            let (x, y) = self.rooms[i].center();
            monsters.push(crate::monster::Monster::random_monster(x, y));
        }
        monsters
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
