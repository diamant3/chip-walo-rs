mod chip8;
mod constants;

use chip8::Chip8;

// audio implementation from https://gitlab.com/langurmonkey/rchip8/-/commit/bdb44b0c6af845b7edc37f19ca26d004215fe8cc
struct SquareWave {
    phase_inc: f32,
    phase: f32,
    vol: f32,
}

impl sdl2::audio::AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for x in out.iter_mut() {
            *x = if self.phase <= 0.2 {
                self.vol
            } else {
                -self.vol
            };

            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

pub struct Beep {
    dev: sdl2::audio::AudioDevice<SquareWave>,
}

impl Beep {
    pub fn new(ctx: &sdl2::Sdl) -> Self {
        let audio_subsystem = ctx.audio().unwrap();

        let desired_spec = sdl2::audio::AudioSpecDesired {
            freq: Some(44100),
            channels: Some(1),
            samples: None,
        };

        let dev = audio_subsystem
            .open_playback(None, &desired_spec, |callback| {
                SquareWave {
                    phase_inc: 440.0 / callback.freq as f32,
                    phase: 0.0,
                    vol: 0.2
                }
            })
            .unwrap();

        Beep { dev }
    }

    pub fn play(&self) {
        if self.dev.status() == sdl2::audio::AudioStatus::Stopped ||
            self.dev.status() == sdl2::audio::AudioStatus::Paused {
                self.dev.resume();
        }
    }

    pub fn pause(&self) {
        self.dev.pause();
    }
}

fn main() {
    let mut chip_walo = Chip8::init();
    let args = std::env::args().collect::<Vec<String>>();

    let sdl2_ctx = sdl2::init().unwrap();
    let beep = Beep::new(&sdl2_ctx);
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

        if chip_walo.dt > 0 {
            chip_walo.dt -= 1;
        }

        if chip_walo.st > 0 {
            chip_walo.st -= 1;
            beep.play();
        } else {
            beep.pause();
        }

        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. }
                | sdl2::event::Event::KeyDown {
                    scancode: Some(sdl2::keyboard::Scancode::Escape),
                    ..
                } => break 'running,
                // key down
                sdl2::event::Event::KeyDown {
                    scancode: Some(sdl2::keyboard::Scancode::Num1),
                    repeat: false,
                    ..
                } => {
                    chip_walo.key[0x1] = true;
                }
                sdl2::event::Event::KeyDown {
                    scancode: Some(sdl2::keyboard::Scancode::Num2),
                    repeat: false,
                    ..
                } => {
                    chip_walo.key[0x2] = true;
                }
                sdl2::event::Event::KeyDown {
                    scancode: Some(sdl2::keyboard::Scancode::Num3),
                    repeat: false,
                    ..
                } => {
                    chip_walo.key[0x3] = true;
                }
                sdl2::event::Event::KeyDown {
                    scancode: Some(sdl2::keyboard::Scancode::Num4),
                    repeat: false,
                    ..
                } => {
                    chip_walo.key[0xc] = true;
                }
                sdl2::event::Event::KeyDown {
                    scancode: Some(sdl2::keyboard::Scancode::Q),
                    repeat: false,
                    ..
                } => {
                    chip_walo.key[0x4] = true;
                }
                sdl2::event::Event::KeyDown {
                    scancode: Some(sdl2::keyboard::Scancode::W),
                    repeat: false,
                    ..
                } => {
                    chip_walo.key[0x5] = true;
                }
                sdl2::event::Event::KeyDown {
                    scancode: Some(sdl2::keyboard::Scancode::E),
                    repeat: false,
                    ..
                } => {
                    chip_walo.key[0x6] = true;
                }
                sdl2::event::Event::KeyDown {
                    scancode: Some(sdl2::keyboard::Scancode::R),
                    repeat: false,
                    ..
                } => {
                    chip_walo.key[0xd] = true;
                }
                sdl2::event::Event::KeyDown {
                    scancode: Some(sdl2::keyboard::Scancode::A),
                    repeat: false,
                    ..
                } => {
                    chip_walo.key[0x7] = true;
                }
                sdl2::event::Event::KeyDown {
                    scancode: Some(sdl2::keyboard::Scancode::S),
                    repeat: false,
                    ..
                } => {
                    chip_walo.key[0x8] = true;
                }
                sdl2::event::Event::KeyDown {
                    scancode: Some(sdl2::keyboard::Scancode::D),
                    repeat: false,
                    ..
                } => {
                    chip_walo.key[0x9] = true;
                }
                sdl2::event::Event::KeyDown {
                    scancode: Some(sdl2::keyboard::Scancode::F),
                    repeat: false,
                    ..
                } => {
                    chip_walo.key[0xe] = true;
                }
                sdl2::event::Event::KeyDown {
                    scancode: Some(sdl2::keyboard::Scancode::Z),
                    repeat: false,
                    ..
                } => {
                    chip_walo.key[0xa] = true;
                }
                sdl2::event::Event::KeyDown {
                    scancode: Some(sdl2::keyboard::Scancode::X),
                    repeat: false,
                    ..
                } => {
                    chip_walo.key[0x0] = true;
                }
                sdl2::event::Event::KeyDown {
                    scancode: Some(sdl2::keyboard::Scancode::C),
                    repeat: false,
                    ..
                } => {
                    chip_walo.key[0xb] = true;
                }
                sdl2::event::Event::KeyDown {
                    scancode: Some(sdl2::keyboard::Scancode::V),
                    repeat: false,
                    ..
                } => {
                    chip_walo.key[0xf] = true;
                }
                // keyup
                sdl2::event::Event::KeyUp {
                    scancode: Some(sdl2::keyboard::Scancode::Num1),
                    ..
                } => {
                    chip_walo.key[0x1] = false;
                }
                sdl2::event::Event::KeyUp {
                    scancode: Some(sdl2::keyboard::Scancode::Num2),
                    ..
                } => {
                    chip_walo.key[0x2] = false;
                }
                sdl2::event::Event::KeyUp {
                    scancode: Some(sdl2::keyboard::Scancode::Num3),
                    ..
                } => {
                    chip_walo.key[0x3] = false;
                }
                sdl2::event::Event::KeyUp {
                    scancode: Some(sdl2::keyboard::Scancode::Num4),
                    ..
                } => {
                    chip_walo.key[0xc] = false;
                }
                sdl2::event::Event::KeyUp {
                    scancode: Some(sdl2::keyboard::Scancode::Q),
                    ..
                } => {
                    chip_walo.key[0x4] = false;
                }
                sdl2::event::Event::KeyUp {
                    scancode: Some(sdl2::keyboard::Scancode::W),
                    ..
                } => {
                    chip_walo.key[0x5] = false;
                }
                sdl2::event::Event::KeyUp {
                    scancode: Some(sdl2::keyboard::Scancode::E),
                    ..
                } => {
                    chip_walo.key[0x6] = false;
                }
                sdl2::event::Event::KeyUp {
                    scancode: Some(sdl2::keyboard::Scancode::R),
                    ..
                } => {
                    chip_walo.key[0xd] = false;
                }
                sdl2::event::Event::KeyUp {
                    scancode: Some(sdl2::keyboard::Scancode::A),
                    ..
                } => {
                    chip_walo.key[0x7] = false;
                }
                sdl2::event::Event::KeyUp {
                    scancode: Some(sdl2::keyboard::Scancode::S),
                    ..
                } => {
                    chip_walo.key[0x8] = false;
                }
                sdl2::event::Event::KeyUp {
                    scancode: Some(sdl2::keyboard::Scancode::D),
                    ..
                } => {
                    chip_walo.key[0x9] = false;
                }
                sdl2::event::Event::KeyUp {
                    scancode: Some(sdl2::keyboard::Scancode::F),
                    ..
                } => {
                    chip_walo.key[0xe] = false;
                }
                sdl2::event::Event::KeyUp {
                    scancode: Some(sdl2::keyboard::Scancode::Z),
                    ..
                } => {
                    chip_walo.key[0xa] = false;
                }
                sdl2::event::Event::KeyUp {
                    scancode: Some(sdl2::keyboard::Scancode::X),
                    ..
                } => {
                    chip_walo.key[0x0] = false;
                }
                sdl2::event::Event::KeyUp {
                    scancode: Some(sdl2::keyboard::Scancode::C),
                    ..
                } => {
                    chip_walo.key[0xb] = false;
                }
                sdl2::event::Event::KeyUp {
                    scancode: Some(sdl2::keyboard::Scancode::V),
                    ..
                } => {
                    chip_walo.key[0xf] = false;
                }
                _ => (),
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

        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}
