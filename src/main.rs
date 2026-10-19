mod chip8;
mod constants;

use chip8::Chip8;

fn main() {
    let mut chip_walo = Chip8::init();
    let args = std::env::args().collect::<Vec<String>>();

    chip_walo.load_rom(args[1].as_ref());

    loop {
        chip_walo.cycle();
    }
}
