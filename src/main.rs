use std::fs::File;
use std::io::{BufReader, Read};
use std::env;
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
            let bytes = paint_bytes(&buffer[..read]);

            // address column
            print!("\x1b[1m{address:#08X?}\x1b[m");

            // empty space
            print!("  ");

            // hex column
            for (i, (byte, (_, color))) in bytes.iter().enumerate() {
                let space = if i % 2 == 0 { "" } else { " " };
                print!("\x1b[{color}m{byte:02X?}\x1b[m{space}");
            }

            // empty hex cell
            for i in 0..BUFFER_SIZE - read {
                let space = if i % 2 == 0 { " " } else { "" };
                print!("  {space}");
            }

            // empty space
            print!("  ");

            // char column
            for (_, (char, color)) in bytes {
                print!("\x1b[{color}m{char}\x1b[m");
            }

            println!();
            address += BUFFER_SIZE;
            continue;
        }

        break;
    }

    Ok(())
}

fn paint_bytes(bytes: &[u8]) -> Vec<(u8, (char, &'static str))> {
    bytes.iter().map(|&byte| {
        let pretty = if byte as char == ' ' {
            (byte as char, "0")
        } else if byte.is_ascii_digit() {
            (byte as char, "33")
        } else if byte.is_ascii_alphabetic() {
            (byte as char, "32")
        } else if byte.is_ascii_punctuation() {
            (byte as char, "36")
        } else if byte.is_ascii_control() {
            ('.', "31")
        } else {
            ('.', "0")
        };

        (byte, pretty)
    }).collect()
}
