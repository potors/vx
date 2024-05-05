use std::fs::File;
use std::io::{BufReader, Read};
use std::env;
use std::process;

enum AsciiType {
    Null,
    Escape,
    Whitespace,
    Character,
    Number,
    Punctuation,
    Exotic
}

const ASCII_TYPES: [AsciiType; 128] = {
    use AsciiType::*;

    [
        Null, // null
        Exotic, Exotic, // start of heading/text
        Exotic, Exotic, // end of text/transmission
        Exotic, Exotic, Exotic, // enquiry/acknowledge/bell
        Escape, Whitespace, // backspace/TAB
        Escape, Exotic, // LF/vertical tab
        Exotic, Escape, // FF/CR
        Exotic, Exotic, // shift out/in
        Exotic, // data link escape
        Exotic, Exotic, Exotic, Exotic, // device control 1/2/3/4
        Exotic, Exotic, Exotic, // NAK/SYN/ETB
        Escape, // cancel
        Exotic, Exotic, // end of medium/substitute
        Escape,
        Exotic, Exotic, Exotic, Exotic, // file/group/record/unit separator
        Whitespace,
        Punctuation, Punctuation, Punctuation, // ! " #
        Punctuation, Punctuation, Punctuation, // $ % &
        Punctuation, Punctuation, Punctuation, // ' ( )
        Punctuation, Punctuation, Punctuation, // * + ,
        Punctuation, Punctuation, Punctuation, // - . /
        Number, Number, Number, // 0 1 2
        Number, Number, Number, // 3 4 5
        Number, Number, Number, // 6 7 8
        Number, // 9
        Punctuation, Punctuation, Punctuation, // : ; <
        Punctuation, Punctuation, Punctuation, // = > ?
        Punctuation, // @
        Character, Character, Character, Character, Character, Character, // A B C D E F
        Character, Character, Character, Character, Character, Character, // G H I J K L
        Character, Character, Character, Character, Character, Character, // M N O P Q R
        Character, Character, Character, Character, Character, Character, // S T U V W X
        Character, Character, // Y Z
        Punctuation, Punctuation, Punctuation, // [ \ ]
        Punctuation, Punctuation, Punctuation, // ^ _ `
        Character, Character, Character, Character, Character, Character, // a b c d e f
        Character, Character, Character, Character, Character, Character, // g h i j k l
        Character, Character, Character, Character, Character, Character, // m n o p q r
        Character, Character, Character, Character, Character, Character, // s t u v w x
        Character, Character, // y z
        Punctuation, Punctuation, Punctuation, // { | }
        Punctuation, // ~
        Escape, // delete
    ]
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1).collect::<Vec<_>>();

    let path = args.pop();
    let groups = if !args.is_empty() { args.remove(0).parse::<usize>().ok() } else { None };
    let group_size = if !args.is_empty() { args.remove(0).parse::<usize>().ok() } else { None };

    if path.is_none() {
        eprintln!("Usage: vx [groups [group_size]] <path>");
        process::exit(1);
    }

    let path = env::current_dir()?.join(path.unwrap());
    let groups = groups.unwrap_or(8);
    let group_size = group_size.unwrap_or(2);
    let buffer_size = groups * group_size;

    let file = File::open(path)?;

    let mut reader = BufReader::new(file);
    let mut address: usize = 0x0;

    loop {
        let mut buffer = vec![0u8; buffer_size];
        let read = reader.read(&mut buffer)?;

        if read > 0 {
            let bytes = paint_bytes(&buffer[..read]);

            print!("{address:#010X?}  ");

            for (byte, (_, color)) in &bytes {
                print!("\x1b[{color}m{byte:02X?}");
                if address % group_size == group_size - 1 {
                    print!(" ");
                }

                address += 1;
            }

            print!(" ");

            for (_, (char, color)) in &bytes {
                print!("\x1b[{color};1m{char}");
            }

            println!("\x1b[m");
            continue;
        }

        break;
    }

    Ok(())
}

fn paint_bytes(bytes: &[u8]) -> Vec<(u8, (char, &'static str))> {
    bytes.iter().map(|&byte| {
        use AsciiType::*;
        let pretty = 'pretty: {
            if byte >= 128 {
                if byte == !0 {
                    break 'pretty ('.', "34");
                }

                break 'pretty ('.', "0");
            }

            match ASCII_TYPES[byte as usize] {
                Null =>        ('.', "30"),
                Whitespace =>  (' ', "0"),
                Character =>   (byte as char, "32"),
                Number =>      (byte as char, "33"),
                Punctuation => (byte as char, "36"),
                Exotic =>      ('.', "35"),
                Escape =>      ('.', "31")
            }
        };

        (byte, pretty)
    }).collect()
}
