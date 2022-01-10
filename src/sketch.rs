use crate::*;

const NUMBER_OF_BARS: usize = 200;
static mut BARS: [f32; NUMBER_OF_BARS] = [20.0; NUMBER_OF_BARS];
static mut COLORS: [(u8, u8, u8, u8); NUMBER_OF_BARS] =
    [(255_u8, 255_u8, 0_u8, 255_u8); NUMBER_OF_BARS];

pub fn setup() {
    unsafe {
        for color in COLORS.iter_mut() {
            *color = (
                rand::gen_range(0_u8, 100_u8),
                rand::gen_range(100_u8, 200_u8),
                rand::gen_range(100_u8, 200_u8),
                255_u8,
            );
        }
    }
    info!("Setup!");
}

pub fn draw(_delta: f64) {
    unsafe {
        for (i, &bar) in BARS.iter().enumerate() {
            let width = screen_width() * 2.0 / NUMBER_OF_BARS as f32;
            draw_rectangle(
                ((width + 2.0) * i as f32) - screen_width(),
                -screen_height(),
                width,
                bar,
                color_u8!(COLORS[i].0, COLORS[i].1, COLORS[i].2, COLORS[i].3),
            );
        }
        draw_circle(0.0, 0.0, 2.0, colors::BEIGE);
        BARS[rand::gen_range(0, 200)] += 5.0;
    }
    draw_ui();
}

fn draw_ui() {
    // Screen space, render fixed ui
    set_default_camera();
    draw_text(
        &format!("mouse: {:?}, fps: {}", mouse_position(), get_fps()),
        10.0,
        20.0,
        30.0,
        colors::BLACK,
    );
}
