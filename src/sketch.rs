use crate::*;

const NUMBER_OF_BARS: usize = 200;
static mut BARS_: Vec<Bar> = vec![];
struct Bar {
    color: Color,
    width: f32,
    height: f32,
    x: f32,
}

impl Bar {
    const fn new(color: Color, width: f32, x: f32) -> Self {
        Self {
            color,
            width,
            height: 1.0,
            x,
        }
    }

    fn draw(&self) {
        draw_rectangle(
            self.x,
            -screen_height(),
            self.width,
            self.height,
            self.color,
        );
    }
}

pub fn setup() {
    unsafe {
        BARS_.reserve(NUMBER_OF_BARS);
        let width = screen_width() * 2.0 / NUMBER_OF_BARS as f32;
        for i in 0..NUMBER_OF_BARS {
            let color = Color::from_rgba(
                rand::gen_range(0_u8, 100_u8),
                rand::gen_range(100_u8, 200_u8),
                rand::gen_range(100_u8, 200_u8),
                255_u8,
            );
            BARS_.push(Bar::new(
                color,
                width - 2.0,
                width.mul_add(i as f32, -screen_width()),
            ));
        }
    }
    info!("Setup!");
}

pub fn draw(_delta: f64) {
    unsafe {
        for bar in &BARS_ {
            bar.draw();
        }

        BARS_[rand::gen_range(0, NUMBER_OF_BARS)].height += 5.0;
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
