use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use skraakast::square::Square;
use std::time::Duration;

static WINDOW_WIDTH: u32 = 1500;
static WINDOW_HEIGHT: u32 = 800;

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

fn draw_ruler(canvas: &mut WindowCanvas) -> Result<(), String> {
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let mut font = ttf_context.load_font("redhatmono.ttf", 128).unwrap();
    font.set_style(sdl2::ttf::FontStyle::BOLD);
    let texture_creator = canvas.texture_creator();

    (0..=WINDOW_WIDTH / 50).for_each(|x| {
        if x % 2 == 0 {
            canvas.set_draw_color(Color::RGB(255, 255, 255));
        } else {
            canvas.set_draw_color(Color::RGB(0, 0, 0));
        }
        canvas
            .fill_rect(rect!(x * 50, WINDOW_HEIGHT - 2, 50, 2))
            .unwrap();
        let surface = font
            .render(&format!("{:^4}", &x))
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
                Some(rect!(x * 50, WINDOW_HEIGHT - 65, 60, 50)),
            )
            .unwrap();
    });

    (0..=WINDOW_HEIGHT / 50).for_each(|y| {
        if y % 2 == 0 {
            canvas.set_draw_color(Color::RGB(255, 255, 255));
        } else {
            canvas.set_draw_color(Color::RGB(0, 0, 0));
        }
        canvas
            .fill_rect(rect!(WINDOW_WIDTH - 2, (y * 50) as i32, 2, 50))
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
                Some(rect!(WINDOW_WIDTH - 65, (y * 50) as i32, 60, 50)),
            )
            .unwrap();
    });

    Ok(())
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

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
    let mut square = Square::new(50.0, (1500.0 / 3.0, 800.0 / 2.0), 50.0);
    let mut delta = 0.0;
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

        square.propel(0.0, 9.82);
        square.evaluate(delta);

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas
            .fill_rect(rect!(500, 800.0 / 2.0 + square.size, 1000, 1))
            .unwrap();
        canvas
            .fill_rect(rect!(
                square.position.0,
                square.position.1,
                square.size,
                square.size
            ))
            .unwrap();

        draw_ruler(&mut canvas);

        canvas.present();
        delta += 1.0 / 60.0;
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
