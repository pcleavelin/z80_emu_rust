mod z80;

use std::fs::File;
use std::io::Read;
use std::env;

use z80::Z80;

fn main() {
    let args: Vec<String> = env::args().collect();

    let com_name = if args.len() > 1 {
        &args[1]
    } else {
        panic!("Invalid com file!");
    };

    let mut com_file = match File::open(com_name) {
        Ok(file) => file,
        Err(why) => panic!("failed to open com: {}", why),
    };
    let mut data: Vec<u8> = Vec::new();
    match com_file.read_to_end(&mut data) {
        Ok(size) => println!("read com {:04X} bytes", size),
        Err(why) => panic!("error reading com: {}", why),
    }


    let mut z80 = Z80::new();

    if z80.poke_com_file(data.as_slice()) == false {
        println!("Com File exceeds ram");
        return;
    }

    z80.hard_reset();
    z80.run();
}