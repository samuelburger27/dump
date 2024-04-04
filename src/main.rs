use std::{
    env::args,
    fs::File,
    io::{self, Read, Write},
    path::PathBuf,
};
enum Output {
    Hex,
    Oct,
    Bin,
    Dec,
    Str,
    HexPlusStr
}
struct Mode {
    out: Output,
    line_l: u32,
    raw: bool
}

fn main() {
    let args = args().skip(1);
    for a in args {
        hex_dump(a);
    }
}

fn hex_dump(str: String) {
    let path = PathBuf::from(str);
    let file = match File::open(path) {
        Ok(file) => file,
        Err(why) => panic!("Couldnt open file, {}", why),
    };
    for (i, byte) in file.bytes().enumerate() {
        let b = byte.unwrap();
        if i % 16 == 0 {
            print!("\n{:06x} ", i)
        }
        print!("{:02x} ", b);
        io::stdout().flush().unwrap();
    }
    print!("\n");
}
