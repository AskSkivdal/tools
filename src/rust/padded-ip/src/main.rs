use local_ip_address::local_ip;

// Center the text.
fn center(text: &str, length: usize) -> String {
    // Get the length of the input text
    let text_length = text.len();
    // Get how many spaces should be in front
    let start_padding = (length - text_length) / 2;
    // Get how many spaces that should be behind.
    let end_padding = length - start_padding - text_length;


    let mut padded_text = text.to_string();
    // Add the proper amount of spaces in front of the text in a loop.
    for _i in 0..start_padding {
        padded_text = " ".to_owned() + &padded_text
    }

    // Add the proper amount of spaces behind the text in a loop.
    for _i in 0..end_padding {
        padded_text = padded_text + &" ".to_owned()
    }

    // Return the padded text.
    return padded_text
}

// Test the center function
#[test]
fn test_center() {
    assert_eq!(center("o", 5), "  o  ");
    assert_eq!(center("0.0.0.0", 15), "    0.0.0.0    ")
}

fn main() {
    // Try to get the local ip as a string. If it fails get 0.0.0.0 as a string
    let ip = match local_ip() {
        Ok(v) => format!("{}", v),
        Err(_e) => "0.0.0.0".to_string()
    };

    // Print the ip centered and wrapped in pipe characters. Wrapping because starship strips perceding and trailing spaces.
    println!("│{}│", center(&ip, 16));
}
