mod chip8;
mod constants;

use chip8::Chip8;

fn main() {
    let mut chip_walo = Chip8::init();
    let args = std::env::args().collect::<Vec<String>>();

    let sdl2_ctx = sdl2::init().unwrap();
    let video_subsystem = sdl2_ctx.video().unwrap();
    let window = video_subsystem
        .window(
            "chip-walo-rs",
            (constants::DISPLAY_WIDTH * 10) as u32,
            (constants::DISPLAY_HEIGHT * 10) as u32,
        )
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 255));

    chip_walo.load_rom(args[1].as_ref());

    let mut event_pump = sdl2_ctx.event_pump().unwrap();
    'running: loop {
        chip_walo.cycle();

        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. }
                | sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::ESCAPE),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.clear();
        canvas.present();
    }
}
