mod text_box;
mod screen_metrics;
mod constants;

use text_box::TextBox;
use screen_metrics::ScreenMetrics;
use constants::{INITIAL_WINDOW_WIDTH, INITIAL_WINDOW_HEIGHT, TEXT_BOX_X_POSITION, TEXT_BOX_Y_POSITION, TEXT_BOX_LINE_THICKNESS};

use raylib::prelude::*;

use crate::constants::{INITIAL_TEXT_BOX_HEIGHT, INITIAL_TEXT_BOX_WIDTH};

fn main() {
    // Ensures that raylib uses the same thread it was initialized from
    let (mut ray_handle, init_thread) = raylib::init()
        .size(INITIAL_WINDOW_WIDTH, INITIAL_WINDOW_HEIGHT)
        .resizable()
        .title("Hello, World")
        .vsync()
        .build();

    ray_handle.set_target_fps(120);
    ray_handle.set_window_min_size(INITIAL_WINDOW_WIDTH, INITIAL_WINDOW_HEIGHT);

    let mut text_box = TextBox::new("Hello, world!".to_string(), Rectangle::new(TEXT_BOX_X_POSITION, TEXT_BOX_Y_POSITION, INITIAL_TEXT_BOX_WIDTH, INITIAL_TEXT_BOX_HEIGHT), 1.25, 10, TEXT_BOX_LINE_THICKNESS, Color::new(255, 255, 255, 255), 0.5, 0.8);
    let mut screen_metrics: ScreenMetrics = ScreenMetrics::new(INITIAL_WINDOW_WIDTH, INITIAL_WINDOW_HEIGHT);
    text_box.update("Hello, world!".to_string(), &screen_metrics);

    while !ray_handle.window_should_close() {
        let mut draw_handle = ray_handle.begin_drawing(&init_thread);
        draw_handle.clear_background(Color::new(30, 30, 30, 255));
        
        // Update sizes when window is resized
        if screen_metrics.was_resized(draw_handle.get_screen_width(), draw_handle.get_screen_height()) {
            screen_metrics.update(draw_handle.get_screen_width(), draw_handle.get_screen_height());
            text_box.update("Hello, world!".to_string(), &screen_metrics);
        }    

        // Draw all elements
        text_box.draw(&mut draw_handle);
    }
}