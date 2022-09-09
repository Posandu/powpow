use crate::enums::TokType;

///
/// Tokenizer
///
/// Token types
///
/// Memory address = .(hex+)
/// String = "..."
/// Integer = 0-9+
/// InternalFn = _(alpha+)
/// Fn = $(alpha+)
/// Comment = # ... \n
///
pub fn tokenize(input: &String) -> Vec<TokType> {
    // Vec to store Tokens
    let mut tokens: Vec<TokType> = vec![];

    // Store location
    let mut pos = 1;

    // The chars of the input
    let chars = input.chars().collect::<Vec<char>>();
    let chars_len = chars.len();

    // Loop through the chars
    while pos < chars_len {
        // Get the current char
        let c = chars[pos];

        // Check if the char is a comment
        if c == '#' {
            let mut value = String::new();

            // Loop through the chars until a new line is found
            while chars[pos] != '\n' {
                pos += 1;
                value.push(chars[pos]);
            }

            tokens.push(TokType::Comment(value));
        }
        // String
        else if c == '"' {
            let mut value = String::new();
            // Loop through the chars until a " is found
            pos += 1;

            loop {
                if chars[pos] == '"' || pos >= chars_len {
                    break;
                }

                value.push(chars[pos]);
                pos += 1;
            }

            tokens.push(TokType::String(value));

            pos += 1;
            continue;
        }
        // Integer
        else if c.is_numeric() {
            let mut value = String::new();
            // Loop through the chars until a non-numeric char is found
            loop {
                if !chars[pos].is_numeric() || pos >= chars_len {
                    break;
                }

                value.push(chars[pos]);
                pos += 1;
            }

            tokens.push(TokType::Integer(value.parse::<u64>().unwrap()));

            continue;
        }
        // Internal function
        else if c == '_' {
            let mut value = String::new();
            // Loop through the chars until a non-alpha char is found
            pos += 1;

            loop {
                if !chars[pos].is_alphabetic() || pos >= chars_len {
                    break;
                }

                value.push(chars[pos]);
                pos += 1;
            }

            tokens.push(TokType::InternalFn(value));

            continue;
        }
        // Function
        else if c == '$' {
            let mut value = String::new();
            // Loop through the chars until a non-alpha char is found
            pos += 1;

            loop {
                if !chars[pos].is_alphabetic() || pos >= chars_len {
                    break;
                }

                value.push(chars[pos]);
                pos += 1;
            }

            tokens.push(TokType::Fn(value));

            continue;
        }
        // Memory address
        else if c == '.' {
            let mut value = String::new();
            // Loop through the chars until a non-hex char is found
            pos += 1;

            loop {
                if !chars[pos].is_ascii_hexdigit() || pos >= chars_len {
                    break;
                }

                value.push(chars[pos]);
                pos += 1;
            }

            tokens.push(TokType::MemAddress(value.parse::<u64>().unwrap()));

            continue;
        }
        // Whitespace
        else if c.is_whitespace() {
            pos += 1;
            continue;
        } else {
            panic!("Invalid character: {}", c);
        }
    }

    tokens
}
