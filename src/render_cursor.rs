#[derive(Debug)]
pub struct RenderCursor {
    pub x: f64,
    pub y: f64,
}

impl RenderCursor {
    pub fn new() -> Self {
        Self { x: 0., y: 0. }
    }

    /// Move the cursor to specified (x, y)
    pub fn move_to(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }

    /// Adjust the cursor by (x, y).
    /// I.e., cursor += (x, y)
    pub fn adjust(&mut self, x: f64, y: f64) {
        self.x = self.x + x;
        self.y = self.y + y;
    }
}
