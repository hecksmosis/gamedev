use raylib::prelude::*;

const WINDOW_FACTOR: f32 = 100.0;
const TILE_SIZE: f32 = 50.0;
const FRAME_WIDTH: f32 = 16.0 * WINDOW_FACTOR / 50.0;
const FRAME_HEIGHT: f32 = 9.0 * WINDOW_FACTOR / 50.0;
const MAP_WIDTH: f32 = 3.0 * FRAME_WIDTH;
const MAP_HEIGHT: f32 = 3.0 * FRAME_HEIGHT;
const MAP_SIZE: usize = MAP_WIDTH as usize * MAP_HEIGHT as usize;

trait Drawable {
    fn draw(&self, d: &mut RaylibDrawHandle);
}

trait Movable {
    fn make_move(&mut self, input: KeyboardKey);
}

struct Map {
    tiles: Vec<Tile>,
    player_pos: Vector2,
}

impl Map {
    fn new(player: &Player) -> Self {
        Map {
            tiles: vec![Tile::Empty; MAP_SIZE],
            player_pos: player.pos,
        }
    }
}

impl Map {
    fn get_map_position(pos: usize) -> Vector2 {
        let pos = pos as f32;
        Vector2::new(
            (pos % MAP_WIDTH) * TILE_SIZE,
            (pos / MAP_WIDTH).floor() * TILE_SIZE,
        )
    }

    fn get_frame(&self) -> Vec<&Tile> {
        // Get the map tiles that leave the player in the middle
        self.tiles
            .iter()
            .enumerate()
            .filter(|&(pos, _)| {
                let map_pos = Self::get_map_position(pos);
                (self.player_pos.x - (map_pos.x / TILE_SIZE)).abs() < (MAP_WIDTH / 2.0)         // TODO: fix this
                    && (self.player_pos.y - (map_pos.y / TILE_SIZE)).abs() < (MAP_HEIGHT / 2.0)
            })
            .map(|(_, tile)| tile)
            .collect::<Vec<_>>()
    }
}

impl Drawable for Map {
    fn draw(&self, d: &mut RaylibDrawHandle) {
        self.get_frame().iter().enumerate().for_each(|(i, tile)| {
            d.draw_rectangle_v(
                Self::get_map_position(i),
                rvec2(TILE_SIZE, TILE_SIZE),
                match tile {
                    Tile::Empty => Color::BLACK,
                    Tile::Wall => Color::RED,
                },
            )
        });
    }
}

#[derive(Default)]
struct Player {
    pos: Vector2,
}

impl Movable for Player {
    fn make_move(&mut self, input: KeyboardKey) {
        match input {
            KeyboardKey::KEY_DOWN if self.pos.y < MAP_HEIGHT - 1.0 => {
                self.pos.y += 1.0;
            }
            KeyboardKey::KEY_UP if self.pos.y > 0.0 => {
                self.pos.y -= 1.0;
            }
            KeyboardKey::KEY_RIGHT if self.pos.x < MAP_WIDTH - 1.0 => {
                self.pos.x += 1.0;
            }
            KeyboardKey::KEY_LEFT if self.pos.x > 0.0 => {
                self.pos.x -= 1.0;
            }
            _ => {}
        }
    }
}

impl Drawable for Player {
    fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle_v(
            self.pos * TILE_SIZE,
            rvec2(TILE_SIZE, TILE_SIZE),
            Color::GREEN,
        );
    }
}

#[derive(Clone, Default)]
enum Tile {
    #[default]
    Empty,
    Wall,
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(16 * WINDOW_FACTOR as i32, 9 * WINDOW_FACTOR as i32)
        .title("hello from raylib")
        .build();

    rl.set_target_fps(60);

    let mut player = Player::default();
    let map = Map::new(&player);
    while !rl.window_should_close() {
        if let Some(key) = rl.get_key_pressed() {
            player.make_move(key);
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        map.draw(&mut d);
        player.draw(&mut d);
    }
}
