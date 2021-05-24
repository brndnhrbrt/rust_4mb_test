mod game;
extern crate minifb;

pub use game::Game;
use minifb::{Key, Window, WindowOptions};
use std::thread::sleep;
use std::time::Duration;

const WIDTH: usize = 160;
const HEIGHT: usize = 144;
const ENLARGEMENT_FACTOR: usize = 5;
const WINDOW_DIMENSIONS: [usize; 2] = [(WIDTH * ENLARGEMENT_FACTOR), (HEIGHT * ENLARGEMENT_FACTOR)];

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new(
        "Game - ESC to exit",
        WINDOW_DIMENSIONS[0],
        WINDOW_DIMENSIONS[1],
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    let mut game = Game::new();
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    game.init_sprites();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut key_map = [false; 4];
        key_map[0] = window.is_key_down(Key::W) || window.is_key_down(Key::Up);
        key_map[1] = window.is_key_down(Key::A) || window.is_key_down(Key::Left);
        key_map[2] = window.is_key_down(Key::S) || window.is_key_down(Key::Down);
        key_map[3] = window.is_key_down(Key::D) || window.is_key_down(Key::Right);

        game.handle_input(key_map);
        buffer = game.gen_buffer(WIDTH, HEIGHT);
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        sleep(Duration::from_nanos(2));
    }
}
