extern crate minifb;

mod z80;

use std::fs::File;
use std::io::Read;
use std::env;

use z80::Z80;

fn load_file(name: &str) -> Vec<u8> {
    let mut com_file = match File::open(name) {
        Ok(file) => file,
        Err(why) => panic!("failed to open com: {}", why),
    };
    let mut data: Vec<u8> = Vec::new();
    match com_file.read_to_end(&mut data) {
        Ok(size) => println!("read com {:04X} bytes", size),
        Err(why) => panic!("error reading com: {}", why),
    }

    data
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let (com_name, char_name) = if args.len() == 3 {
        (&args[1], &args[2])
    } else {
        panic!("Invalid com and char rom file!");
    };

    let rom = load_file(com_name);
    let char_rom = load_file(char_name);

    let mut z80 = Z80::new();

    if z80.poke_com_file(rom.as_slice()) == false {
        println!("Com File exceeds ram");
        return;
    }
    if z80.poke_char_rom(char_rom.as_slice()) == false {
        println!("char rom too big!");
        return;
    }

    z80.hard_reset();
    z80.run();
}