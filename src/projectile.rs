use crate::map::{Map, Tile};

pub struct Projectile {
    pub x: usize,
    pub y: usize,
    pub dx: i32,
    pub dy: i32,
    pub damage: i32,
    pub symbol: char,
    #[allow(dead_code)]
    pub source_name: String,
}

impl Projectile {
    /// Pick the right ASCII symbol based on travel direction.
    pub fn symbol_for_direction(dx: i32, dy: i32) -> char {
        match (dx, dy) {
            (1, 0) | (-1, 0) => '-',
            (0, 1) | (0, -1) => '|',
            (1, 1) | (-1, -1) => '\\',
            (-1, 1) | (1, -1) => '/',
            _ => '*',
        }
    }

    /// Advance this projectile one tile. Returns false if it should be removed
    /// (hit wall or went out of bounds). Does NOT check player/monster hits —
    /// that's done in main.rs after the move.
    pub fn advance(&mut self, map: &Map) -> bool {
        let nx = self.x as i32 + self.dx;
        let ny = self.y as i32 + self.dy;

        if nx < 0 || ny < 0 {
            return false;
        }

        let ux = nx as usize;
        let uy = ny as usize;

        if ux >= map.width || uy >= map.height {
            return false;
        }

        if map.tiles[ux][uy] == Tile::Wall {
            return false;
        }

        self.x = ux;
        self.y = uy;
        true
    }
}
