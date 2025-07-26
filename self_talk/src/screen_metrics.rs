

pub struct ScreenMetrics {
    pub width: i32,
    pub height: i32,
    pub initial_width: i32,
    pub ui_scale: f32,
}

impl ScreenMetrics {
    pub fn new(width: i32, height: i32) -> Self {
        Self { width, height, initial_width: width, ui_scale: 1.0 }
    }

    pub fn was_resized(&self, width: i32, height: i32) -> bool {
        self.width != width || self.height != height
    }

    pub fn update(&mut self, width: i32, height: i32) {
        self.width = width;
        self.height = height;
        self.ui_scale = width as f32 / self.initial_width as f32;
    }
}