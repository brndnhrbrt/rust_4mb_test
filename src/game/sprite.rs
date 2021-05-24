#[derive(Debug, Copy, Clone)]
pub struct Sprite {
    pub solid: bool,
    pub z_level: u8,
    pub player_controlled: bool,
    pub position: [usize; 2],
    pub size: [usize; 2],
}

impl Sprite {
    pub fn new() -> Sprite {
        Sprite {
            solid: false,
            z_level: 0,
            player_controlled: false,
            position: [0, 0],
            size: [0, 0],
        }
    }
}
