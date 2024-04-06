use std::{
    env::args,
    fs::File,
    i64,
    io::{self, Read, Write},
    path::PathBuf,
};

#[derive(PartialEq)]
enum Output {
    Hex,
    Oct,
    Bin,
    Dec,
    Str,
    HexPlusStr,
}
struct Profile {
    out: Output,
    length: i64,
    raw: bool,
}
fn main() {
    let mut profile = basic_profile();
    let args = args().skip(1);
    let mut is_len = false;
    for arg in args {
        // get length
        if is_len {
            is_len = false;
            match arg.parse::<i64>() {
                Ok(len) => profile.length = len,
                Err(_) => {
                    println!("Incorrect length");
                    show_help()
                }
            }
        } else if arg.as_str().starts_with("-") {
            match arg.as_str() {
                "-h" | "--help" => show_help(),
                "-H" => profile.out = Output::Hex,
                "-O" => profile.out = Output::Oct,
                "-B" => profile.out = Output::Bin,
                "-D" => profile.out = Output::Dec,
                "-S" => profile.out = Output::Str,
                "-H+" => profile.out = Output::HexPlusStr,
                "-r" => profile.raw = true,
                "-l" => is_len = true,
                _ => {
                    println!("Illegal option: {}", arg);
                    show_help();
                    break;
                }
            }
        } else {
            dump(&profile, arg);
        }
    }
}

fn dump(profile: &Profile, str: String) {
    let path = PathBuf::from(str);
    let file = match File::open(path) {
        Ok(file) => file,
        Err(why) => panic!("Couldnt open file, {}", why),
    };
    let mut buffer: [u8; 16] = [0; 16];
    for (i, b) in file.bytes().enumerate() {
        let byte = b.unwrap();
        // offset
        if !profile.raw && i % 16 == 0 {
            // print String buffer on end of file
            if profile.out == Output::HexPlusStr {
                print_str_buffer(&buffer)
            }
            print!("\n{:06x} ", i)
        }
        print_formatted_byte(byte, &profile.out, profile.raw);
        buffer[i % 16] = byte;
        if !profile.raw {
            print!(" ");
        }
        io::stdout().flush().unwrap();
    }
    print!("\n");
}

fn print_formatted_byte(byte: u8, mode: &Output, raw: bool) {
    match mode {
        Output::Hex | Output::HexPlusStr => print!("{:02x}", byte),
        Output::Oct => print!("{:03o}", byte),
        Output::Bin => print!("{:08b}", byte),
        Output::Dec => print!("{:03}", byte),
        Output::Str => {
            // valid ascii character
            if byte > 0x20 && byte < 0x7f {
                if raw {
                    print!("{}", byte as char);
                } else {
                    print!("'{}'", byte as char);
                }
            } else {
                print!("{:02x}", byte);
            }
        }
    }
}

fn print_str_buffer(buffer: &[u8; 16]) {
    print!(" |");
    for b in buffer.bytes() {
        let byte = b.unwrap();
        // valid ascii
        if byte > 0x20 && byte < 0x7f {
            print!("{}", byte as char)
        } else {
            print!(".");
        }
    }
    print!("|");
}

fn basic_profile() -> Profile {
    let output: Output = Output::Hex;
    let profile = Profile {
        out: output,
        length: -1,
        raw: false,
    };
    profile
}

fn show_help() {
    println!("usage: hexdump [options] [-l length] files ...");
    println!("otions:");
    println!("\t-H  (default) Offset followed by bytes of input data in hexadecimal");
    println!("\t-H+ Offset followed by bytes of input data in hexadecimal, followed by valid ascii bytes in |..|");
    println!("\t-O  Offset followed by bytes of input data in octal");
    println!("\t-B  Offset followed by bytes of input data in binary");
    println!("\t-D  Offset followed by bytes of input data in decimal");
    println!("\t-S  Offset followed by valid ascii bytes as character, invalid in hexadecimal");
    println!("\t-l length  Read only length of bytes");
    println!("\t-r | --raw print data without formating");
}
