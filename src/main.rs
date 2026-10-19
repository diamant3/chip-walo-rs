mod chip8;
mod constants;

use chip8::Chip8;

fn main() {
    let mut chip_walo = Chip8::init();
    let args: Vec<String> = std::env::args().collect();

    // println!("{:?}", args);

    chip_walo.load_rom(args[1].trim());

    loop {
        chip_walo.cycle();
    }
}
