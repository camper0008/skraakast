use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use skraakast::square::Square;
use std::time::Duration;

static WINDOW_WIDTH: u32 = 1500;
static WINDOW_HEIGHT: u32 = 900;

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    let window = video_subsystem
        .window("rust-sdl2 demo", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut square = Square::new(50.0, (50.0, 500.0), 50.0);
    square.propel(5000.0, 0.0);
    'running: loop {
        canvas.set_draw_color(Color::RGB(55, 55, 55));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        square.evaluate(1.0 / 60.0);

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas
            .fill_rect(rect!(
                square.position.0,
                square.position.1,
                square.size,
                square.size
            ))
            .unwrap();

        let mut font = ttf_context.load_font("redhatmono.ttf", 128).unwrap();
        font.set_style(sdl2::ttf::FontStyle::BOLD);
        let texture_creator = canvas.texture_creator();

        (0..=WINDOW_HEIGHT).step_by(50).for_each(|y| {
            canvas.set_draw_color(Color::RGB(255, 255, 255));
            canvas
                .fill_rect(rect!(WINDOW_WIDTH - 50, y as i32 - 2, 50, 4))
                .unwrap();
            let surface = font
                .render(&format!("{:>4}", &y))
                .blended(Color::RGBA(255, 255, 255, 255))
                .map_err(|e| e.to_string())
                .unwrap();
            let texture = texture_creator
                .create_texture_from_surface(&surface)
                .map_err(|e| e.to_string())
                .unwrap();

            canvas
                .copy(
                    &texture,
                    None,
                    Some(rect!(WINDOW_WIDTH - 65, y as i32 - 2, 60, 50)),
                )
                .unwrap();
        });

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
