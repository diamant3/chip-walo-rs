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
            (constants::DISPLAY_WIDTH * constants::DISPLAY_SCALE) as u32,
            (constants::DISPLAY_HEIGHT * constants::DISPLAY_SCALE) as u32,
        )
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture(
            sdl2::pixels::PixelFormatEnum::RGBA32,
            sdl2::render::TextureAccess::Streaming,
            (constants::DISPLAY_WIDTH) as u32,
            (constants::DISPLAY_HEIGHT) as u32,
        )
        .unwrap();

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

        texture
            .with_lock(None, |buf: &mut [u8], pitch: usize| {
                for y in 0..constants::DISPLAY_HEIGHT {
                    for x in 0..constants::DISPLAY_WIDTH {
                        let offset = (y * pitch) + (x * 4);
                        let chip8_pixel = chip_walo.display_buf[(y * constants::DISPLAY_WIDTH) + x];

                        let color_pixel: [u8; 4] = if chip8_pixel != 0 {
                            [255, 0, 0, 255]
                        } else {
                            [20, 20, 20, 255]
                        };

                        buf[offset..offset + 4].copy_from_slice(&color_pixel);
                    }
                }
            })
            .unwrap();

        canvas.clear();
        canvas.copy(&texture, None, None).unwrap();
        canvas.present();
    }
}
