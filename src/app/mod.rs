const DEFAULT_WIDTH: u32 = 0;
const DEFAULT_HEIGHT: u32 = 0;

pub struct App {
    width: u32,
    height: u32,
}

impl App {
    pub fn new() -> Self {
        Self {
            width: DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT,
        }
    }
}
