use crate::enums::TokType;

///
/// Token to string
///
pub fn token2str(tokens: &Vec<TokType>) -> String {
    // String to store the output
    let mut output = String::new();
    // Loop through the tokens
    for token in tokens {
        // Check the type of the token
        match token {
            TokType::Fn(name) => {
                output.push_str(&format!("\n${} ", name));
            }

            TokType::InternalFn(name) => {
                output.push_str(&format!("\n_{} ", name));
            }

            TokType::String(value) => {
                output.push_str(&format!("\"{}\" ", value));
            }

            TokType::Integer(value) => {
                output.push_str(&format!("{} ", value));
            }

            TokType::MemAddress(value) => {
                output.push_str(&format!(".{} ", value));
            }

            TokType::Comment(value) => {
                output.push_str(&format!("#{} ", value));
            }
        }
    }

    output
}
