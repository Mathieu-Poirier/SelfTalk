use raylib::prelude::*;
pub struct TextBox {
    text: String,
    rectangle: Rectangle,
    roudness: f32,
    segments: i32,
    line_thickness: f32,
    color: Color,
}

impl TextBox {
    pub fn new(text: String, rectangle: Rectangle, roudness: f32, segments: i32, line_thickness: f32, color: Color) -> Self {
        Self { text, rectangle, roudness, segments, line_thickness, color }
    }

    pub fn draw(&self, draw_handle: &mut RaylibDrawHandle) {
        draw_handle.draw_rectangle_rounded_lines_ex(self.rectangle, self.roudness, self.segments, self.line_thickness, self.color);
    }

    pub fn update(&mut self, text: String) {}

    pub fn draw_text(&self, draw_handle: &mut RaylibDrawHandle) {
        draw_handle.draw_text(&self.text, self.rectangle.x as i32, self.rectangle.y as i32, 20, self.color);
    }
}