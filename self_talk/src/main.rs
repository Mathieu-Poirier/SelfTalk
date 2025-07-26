mod text_box;

use text_box::TextBox;


use raylib::prelude::*;

const WINDOW_WIDTH: i32 = 1980;
const WINDOW_HEIGHT: i32 = 1080;

fn main() {
    // Ensures that raylib uses the same thread it was initialized from
    let (mut ray_handle, init_thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .resizable()
        .title("Hello, World")
        .vsync()
        .build();

    ray_handle.set_target_fps(60);
    let text_box = TextBox::new("Hello, world!".to_string(), Rectangle::new(100.0, 100.0, 500.0, 100.0), 2.0, 10, 1.0, Color::BLACK);
     
    while !ray_handle.window_should_close() {
        let mut draw_handle = ray_handle.begin_drawing(&init_thread);
         
        draw_handle.clear_background(Color::WHITE);
        text_box.draw(&mut draw_handle);
        text_box.draw_text(&mut draw_handle);
    }
}