mod chip8;
mod constants;

use chip8::Chip8;

fn main() {
    let mut chip_walo = Chip8::init();
    let args: Vec<String> = std::env::args().collect();

    chip_walo.load_rom(args[1].as_bytes().to_vec());

    loop {
        chip_walo.cycle();
    }
}
