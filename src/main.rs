const WIDTH: usize = 800;
const HEIGHT: usize = 600;
fn main() {
    let sdl2_ctx = sdl2::init().unwrap();
    let sdl2_video = sdl2_ctx.video().unwrap();
    let mut sdl2_event_pump = sdl2_ctx.event_pump().unwrap();
    let window = sdl2_video.window(
        "Physics?", WIDTH as u32,
        HEIGHT as u32).position_centered().build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    'main: loop {
        for event in sdl2_event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => {
                    break 'main;
                },
                _ => {}
            }
        }
        canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 0, 0));
        canvas.clear();
        canvas.present();
    }
}

