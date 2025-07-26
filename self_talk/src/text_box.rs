
use raylib::prelude::*;
use crate::screen_metrics::ScreenMetrics;
use crate::constants::{INITIAL_TEXT_BOX_WIDTH, INITIAL_TEXT_BOX_HEIGHT};

pub struct TextBox {
    text: String,
    rectangle: Rectangle,
    roudness: f32,
    segments: i32,
    line_thickness: f32,
    anchor_x: f32,
    anchor_y: f32,
    color: Color,
}

impl TextBox {

    pub fn new(text: String, rectangle: Rectangle, roudness: f32, segments: i32, line_thickness: f32, color: Color, anchor_x: f32, anchor_y: f32) -> Self {
        Self { text, rectangle, roudness, segments, line_thickness, anchor_x, anchor_y, color }
    }

    pub fn update(&mut self, text: String, screen_metrics: &ScreenMetrics) {
        self.text = text;
        
        // Scale the base dimensions by UI scale
        self.rectangle.width = INITIAL_TEXT_BOX_WIDTH * screen_metrics.ui_scale / 2.5;
        self.rectangle.height = INITIAL_TEXT_BOX_HEIGHT * screen_metrics.ui_scale / 4.0;
        

        self.rectangle.x = self.anchor_x * screen_metrics.width as f32 - self.rectangle.width / 2.0;
        self.rectangle.y = self.anchor_y * screen_metrics.height as f32 - self.rectangle.height / 4.0;
    }

    // Maybe change this to the filled one?
    pub fn draw(&self, draw_handle: &mut RaylibDrawHandle) {
        draw_handle.draw_rectangle_rounded_lines_ex(self.rectangle, self.roudness, self.segments, self.line_thickness, self.color);
    }

}