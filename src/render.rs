use bracket_lib::prelude::{BTerm, Degrees, FontCharType, PointF, RGB, RGBA};

fn to_cp437(c: char) -> FontCharType {
    match c {
        '!' => 33,
        '#' => 35,
        '$' => 36,
        '%' => 37,
        '&' => 38,
        '*' => 42,
        '+' => 43,
        '-' => 45,
        '.' => 46,
        '/' => 47,
        '<' => 60,
        '=' => 61,
        '>' => 62,
        '?' => 63,
        '@' => 64,
        '[' => 91,
        '\\' => 92,
        ']' => 93,
        '^' => 94,
        '_' => 95,
        '`' => 96,
        '{' => 123,
        '|' => 124,
        '}' => 125,
        '~' => 126,
        '·' => 250,
        '▀' => 220,
        '▄' => 223,
        '▌' => 221,
        '▐' => 222,
        '☺' => 1,
        '▲' => 30,
        '╬' => 206,
        '╨' => 208,
        '╩' => 202,
        'Ω' => 234,
        '░' => 176,
        _ => c as FontCharType,
    }
}

pub const TILE_HALF_W: f32 = 2.0;
pub const TILE_HALF_H: f32 = 1.0;

pub struct Camera {
    pub target_x: f32,
    pub target_y: f32,
    pub viewport_w: f32,
    pub viewport_h: f32,
}

impl Camera {
    pub fn new(viewport_w: f32, viewport_h: f32) -> Self {
        Self {
            target_x: 0.0,
            target_y: 0.0,
            viewport_w,
            viewport_h,
        }
    }

    pub fn follow(&mut self, grid_x: usize, grid_y: usize) {
        let (sx, sy) = world_to_screen(grid_x as f32, grid_y as f32);
        self.target_x = sx;
        self.target_y = sy;
    }

    pub fn get_view_bounds(&self) -> (i32, i32, i32, i32) {
        let left = self.target_x - self.viewport_w / 2.0;
        let top = self.target_y - self.viewport_h / 2.0;
        let grid_top_left = screen_to_world(left, top);
        let margin = 2;
        (
            (grid_top_left.0 - margin as f32).floor() as i32,
            (grid_top_left.1 - margin as f32).floor() as i32,
            (grid_top_left.0 + margin as f32).ceil() as i32 + self.viewport_w as i32 / 2,
            (grid_top_left.1 + margin as f32).ceil() as i32 + self.viewport_h as i32 / 2,
        )
    }
}

pub fn world_to_screen(grid_x: f32, grid_y: f32) -> (f32, f32) {
    let screen_x = (grid_x - grid_y) * TILE_HALF_W;
    let screen_y = (grid_x + grid_y) * TILE_HALF_H;
    (screen_x, screen_y)
}

pub fn screen_to_world(screen_x: f32, screen_y: f32) -> (f32, f32) {
    let grid_x = (screen_x / TILE_HALF_W + screen_y / TILE_HALF_H) / 2.0;
    let grid_y = (screen_y / TILE_HALF_H - screen_x / TILE_HALF_W) / 2.0;
    (grid_x, grid_y)
}

pub fn render_depth(grid_x: usize, grid_y: usize, layer: RenderLayer) -> i32 {
    let base_depth = (grid_x + grid_y) as i32;
    base_depth + layer as i32
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RenderLayer {
    Floor = 0,
    WallBase = 10,
    WallTop = 15,
    Web = 20,
    Item = 25,
    Monster = 30,
    Player = 35,
    Projectile = 40,
    Effect = 50,
}

pub fn draw_isometric_tile(
    ctx: &mut BTerm,
    grid_x: usize,
    grid_y: usize,
    camera: &Camera,
    glyph: FontCharType,
    fg: RGB,
    bg: RGB,
    layer: RenderLayer,
) {
    let (screen_x, screen_y) = world_to_screen(grid_x as f32, grid_y as f32);
    let render_x = screen_x - camera.target_x + camera.viewport_w / 2.0;
    let render_y = screen_y - camera.target_y + camera.viewport_h / 2.0;

    if render_x < -TILE_HALF_W * 2.0
        || render_x > camera.viewport_w + TILE_HALF_W * 2.0
        || render_y < -TILE_HALF_H * 2.0
        || render_y > camera.viewport_h + TILE_HALF_H * 2.0
    {
        return;
    }

    let depth = render_depth(grid_x, grid_y, layer);
    ctx.set_fancy(
        PointF::new(render_x, render_y),
        depth as i32,
        Degrees(0.0),
        PointF::new(1.0, 1.0),
        fg,
        bg,
        glyph,
    );
}

pub fn draw_isometric_wall(
    ctx: &mut BTerm,
    grid_x: usize,
    grid_y: usize,
    camera: &Camera,
    front_color: RGB,
    top_color: RGB,
    show_front: bool,
    show_left: bool,
    show_right: bool,
) {
    let (screen_x, screen_y) = world_to_screen(grid_x as f32, grid_y as f32);
    let render_x = screen_x - camera.target_x + camera.viewport_w / 2.0;
    let render_y = screen_y - camera.target_y + camera.viewport_h / 2.0;

    if render_x < -TILE_HALF_W * 2.0
        || render_x > camera.viewport_w + TILE_HALF_W * 2.0
        || render_y < -TILE_HALF_H * 4.0
        || render_y > camera.viewport_h + TILE_HALF_H * 2.0
    {
        return;
    }

    let wall_height = TILE_HALF_H * 2.0;

    let top_depth = render_depth(grid_x, grid_y, RenderLayer::WallTop);
    ctx.set_fancy(
        PointF::new(render_x, render_y - wall_height),
        top_depth as i32,
        Degrees(0.0),
        PointF::new(1.0, 1.0),
        top_color,
        RGB::from_f32(0.0, 0.0, 0.0),
        to_cp437('▀'),
    );

    if show_front {
        let front_depth = render_depth(grid_x, grid_y, RenderLayer::WallBase);
        ctx.set_fancy(
            PointF::new(render_x, render_y - TILE_HALF_H),
            front_depth as i32,
            Degrees(0.0),
            PointF::new(1.0, 1.0),
            front_color,
            RGB::from_f32(0.0, 0.0, 0.0),
            to_cp437('▄'),
        );
    }

    if show_left {
        let left_depth = render_depth(grid_x, grid_y, RenderLayer::WallBase) + 1;
        ctx.set_fancy(
            PointF::new(render_x - TILE_HALF_W, render_y - TILE_HALF_H * 1.5),
            left_depth as i32,
            Degrees(0.0),
            PointF::new(1.0, 1.0),
            RGB::from_f32(
                front_color.r * 0.7,
                front_color.g * 0.7,
                front_color.b * 0.7,
            ),
            RGB::from_f32(0.0, 0.0, 0.0),
            to_cp437('▌'),
        );
    }

    if show_right {
        let right_depth = render_depth(grid_x, grid_y, RenderLayer::WallBase) + 2;
        ctx.set_fancy(
            PointF::new(render_x + TILE_HALF_W, render_y - TILE_HALF_H * 1.5),
            right_depth as i32,
            Degrees(0.0),
            PointF::new(1.0, 1.0),
            RGB::from_f32(
                front_color.r * 0.8,
                front_color.g * 0.8,
                front_color.b * 0.8,
            ),
            RGB::from_f32(0.0, 0.0, 0.0),
            to_cp437('▐'),
        );
    }
}

pub fn draw_entity(
    ctx: &mut BTerm,
    grid_x: usize,
    grid_y: usize,
    camera: &Camera,
    glyphs: &[(FontCharType, RGB, f32, f32)],
    base_layer: RenderLayer,
) {
    let (screen_x, screen_y) = world_to_screen(grid_x as f32, grid_y as f32);
    let render_x = screen_x - camera.target_x + camera.viewport_w / 2.0;
    let render_y = screen_y - camera.target_y + camera.viewport_h / 2.0;

    if render_x < -2.0
        || render_x > camera.viewport_w + 2.0
        || render_y < -2.0
        || render_y > camera.viewport_h + 2.0
    {
        return;
    }

    let base_depth = render_depth(grid_x, grid_y, base_layer);

    for (i, (glyph, color, ox, oy)) in glyphs.iter().enumerate() {
        let depth = base_depth + i as i32;
        ctx.set_fancy(
            PointF::new(render_x + ox, render_y + oy),
            depth as i32,
            Degrees(0.0),
            PointF::new(1.0, 1.0),
            *color,
            RGB::from_f32(0.0, 0.0, 0.0),
            *glyph,
        );
    }
}

pub fn get_wall_colors(floor: i32) -> (RGB, RGB) {
    match floor {
        1..=5 => (
            RGB::from_f32(0.22, 0.22, 0.28),
            RGB::from_f32(0.37, 0.37, 0.43),
        ),
        6..=10 => (
            RGB::from_f32(0.27, 0.21, 0.16),
            RGB::from_f32(0.43, 0.33, 0.25),
        ),
        _ => (
            RGB::from_f32(0.18, 0.09, 0.22),
            RGB::from_f32(0.31, 0.18, 0.37),
        ),
    }
}

pub fn get_floor_color(x: usize, y: usize) -> RGB {
    let variation = ((x * 7 + y * 13) % 10) as f32 / 100.0;
    RGB::from_f32(0.15 + variation, 0.15 + variation, 0.17 + variation)
}

pub fn get_wall_visibility(map: &crate::map::Map, x: usize, y: usize) -> (bool, bool, bool) {
    let w = map.width;
    let h = map.height;
    let left_wall = x > 0 && matches!(map.tiles[x - 1][y], crate::map::Tile::Wall);
    let right_wall = x + 1 < w && matches!(map.tiles[x + 1][y], crate::map::Tile::Wall);
    let front_wall = y + 1 < h && matches!(map.tiles[x][y + 1], crate::map::Tile::Wall);
    (!front_wall, !left_wall, !right_wall)
}
