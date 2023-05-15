use std::fs;
use regex::Regex;
use clap::{Parser, ValueEnum};


#[derive(Debug, Clone)]
struct Slice {
    text: String,
    offset: usize,
    length: usize
}

#[derive(Debug, Clone, ValueEnum)]
enum Mode {
    BasicCharacters
}


#[derive(Parser, Debug)]
#[command(author, about, long_about = None)]
struct Args {
    /// The characterset that the program scans for
    #[arg(short, long, default_value = "basic-characters")]
    mode: Mode,

    /// The minimum required characters for something to be recognised as a string.
    #[arg(short,long, default_value_t = 5)]
    characters: u8,

    /// This overrides the mode and characters args and uses this regex instead
    #[arg(short = 'r', long, default_value = None)]
    custom_regex: Option<String>,

    /// Shows the offset at the starting position of the current string.
    #[arg(short = 'o', long)]
    include_offsets: bool,

    /// The path of the file that you want to read out all the strings from.
    #[arg()]
    filepath: String
    
}

fn main() {
    let args = Args::parse();

    let mut rx: String = "".to_string();
    match args.mode {
        Mode::BasicCharacters => {
            rx += "[A-z]"
        }
    }
    println!("{}", args.characters);


    rx += format!("{{{},}}", args.characters).as_str();

    match args.custom_regex {
        Some(v) => rx = v,
        None => ()
    }

    rx = format!("^{}", rx);

    let re = Regex::new(&rx).unwrap();

    let bytes = match fs::read(args.filepath) {
        Ok(v) => v,
        Err(_) => panic!()
    };

    let mut printables: Vec<Slice> = Vec::new();
    let mut printable: Slice = Slice { text: "".to_string(), offset: 0, length: 0 };

    for (i, byte) in bytes.iter().enumerate() {
        if is_printable(*byte) {
            printable.text += std::str::from_utf8(&[*byte]).unwrap()
        } else {
            if printable.text != "" {
                printable.length = printable.text.len();
                printables.push(printable);
                printable = Slice {text: "".to_string(), offset:i+1, length: 0}
            }
            printable.offset = i + 1
            
        }
    }

    for i in printables {
        if re.is_match(i.text.as_str()) {
            if args.include_offsets {
                print!("0x{:X}: ", i.offset);
            }
            println!("{}", i.text);
        }
        
    }
}


fn is_printable(byte: u8) -> bool {
    byte.is_ascii() && (byte>0)
}