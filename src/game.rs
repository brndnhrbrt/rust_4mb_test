mod sprite;

use sprite::*;

pub struct Game {
    pub sprites: Vec<Sprite>,
}

impl Game {
    pub fn new() -> Game {
        Game { sprites: vec![] }
    }

    pub fn init_sprites(&mut self) {
        let mut test_sprite = Sprite::new();
        test_sprite.position = [10, 10];
        test_sprite.size = [16, 16];
        test_sprite.player_controlled = true;
        self.sprites.push(test_sprite);
    }

    pub fn handle_input(&mut self, key_map: [bool; 4]) {
        for mut sprite in self.sprites.iter_mut() {
            if sprite.player_controlled && key_map[0] {
                sprite.position[1] -= 1;
            }
            if sprite.player_controlled && key_map[1] {
                sprite.position[0] -= 1;
            }
            if sprite.player_controlled && key_map[2] {
                sprite.position[1] += 1;
            }
            if sprite.player_controlled && key_map[3] {
                sprite.position[0] += 1;
            }
        }
    }

    pub fn gen_buffer(&mut self, window_width: usize, window_height: usize) -> Vec<u32> {
        let mut buffer = vec![0; window_width * window_height];
        for sprite in self.sprites.iter() {
            for y_size in 0..sprite.size[1] {
                for x_size in 0..sprite.size[0] {
                    let x_pos = x_size + sprite.position[0];
                    let y_pos = (y_size + sprite.position[1]) * window_width;
                    if x_pos > window_width || y_pos > window_height {
                        let pixel_index = y_pos + x_pos;
                        buffer[pixel_index] = 255;
                    }
                }
            }
        }
        return buffer;
    }
}
