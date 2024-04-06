use std::{
    default,
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
    HexPlusStr,
}
struct Profile {
    out: Output,
    line_l: u32,
    raw: bool,
}

fn main() {
    let mut profile = basic_profile();
    let args = args().skip(1);
    for a in args {
        if a.as_str().starts_with("-") {
            match a.as_str() {
                "-h" | "--help" => show_help(),
                "-H" => profile.out = Output::Hex,
                "-O" => profile.out = Output::Oct,
                "-B" => profile.out = Output::Bin,
                "-D" => profile.out = Output::Dec,
                "-S" => profile.out = Output::Str,
                "-H+" => profile.out = Output::HexPlusStr,
                "-r" => profile.raw = true,
                _ => {
                    println!("Illegal option: {}", a);
                    show_help();
                }
            }
        } else {
            hex_dump(&profile, a);
        }

    }
}

fn hex_dump(profile:&Profile, str: String) {
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

fn basic_profile() -> Profile {
    let output: Output = Output::Hex;
    let profile = Profile {
        out: output,
        line_l: 32,
        raw: false,
    };
    profile
}

fn show_help() {}
