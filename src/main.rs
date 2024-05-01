use std::fs::File;
use std::io::{BufReader, Read};
use std::env;
use std::path::Path;
use std::process;

const BUFFER_SIZE: usize = 16;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = env::args().skip(1).collect::<Vec<_>>();

    let path = args.first();

    if path.is_none() {
        eprintln!("Usage: vx <path>");
        process::exit(1);
    }

    let path = env::current_dir()?.join(path.unwrap());

    let file = File::open(path)?;

    let mut reader = BufReader::new(file);
    let mut address: usize = 0x0;

    loop {
        let mut buffer = vec![0u8; BUFFER_SIZE];
        let read = reader.read(&mut buffer)?;

        if read > 0 {
            // address column
            print!("\x1b[36m{address:#08X?}\x1b[m ");

            // empty space
            print!(" ");

            // hex column
            // TODO color by column % 2
            for (i, byte) in buffer[..read].iter().enumerate() {
                if i % 2 == 0 {
                    print!("\x1b[1m{byte:02X?}\x1b[m");
                } else {
                    print!("{byte:02X?} ");
                }
            }

            // empty hex cell
            for _ in 0..BUFFER_SIZE - read {
                print!("   ");
            }

            // empty space
            print!(" ");

            // char column
            for byte in &buffer[..read] {
                let char = *byte as char;

                if char.is_ascii_whitespace() {
                    print!(" ");
                } else if char.is_ascii_alphabetic() {
                    print!("\x1b[32m{char}\x1b[m");
                } else if char.is_ascii_punctuation() {
                    print!("\x1b[33m{char}\x1b[m");
                } else if char.is_ascii_digit() {
                    print!("\x1b[35m{char}\x1b[m");
                } else {
                    print!("\x1b[38m.\x1b[m")
                }
            }

            println!();
            address += BUFFER_SIZE;
            continue;
        }

        break;
    }

    Ok(())
}
