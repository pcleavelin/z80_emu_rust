mod z80;
use z80::Z80;

fn main() {
    let mut z80 = Z80::new();

    z80.hard_reset();
    z80.run();
}