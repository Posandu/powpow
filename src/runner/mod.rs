use crate::enums::{TokType, TokValue};
use std::collections::HashMap;

///
/// Runner
///
/// Runs the code
///
pub fn run(tokens: &Vec<TokType>) {
    // HashMap to store the memory
    let mut memory: HashMap<u64, TokValue> = HashMap::new();
    // Position when running
    let mut pos = 0;

    //
    // Run the code
    //
    while pos < tokens.len() {
        let token = &tokens[pos];

        //
        // Match token and do the job
        //
        match token {
            //
            // Set memory value
            //
            TokType::MemAddress(address) => {
                // Get the next token
                let next_token = &tokens[pos + 1];

                // Match the next token
                match next_token {
                    TokType::Integer(val) => {
                        memory.insert(*address, TokValue::Integer(*val));
                    }
                    TokType::String(val) => {
                        memory.insert(*address, TokValue::String(val));
                    }
                    _ => {
                        pos += 1;
                        continue;
                    }
                }
            }
            //
            // Function call
            //
            TokType::Fn(_fname) => {
                let after_val = &tokens[pos + 1];

                match after_val {
                    TokType::MemAddress(addr) => {
                        let val = memory.get(addr).unwrap();

                        match val {
                            TokValue::String(val) => {
                                println!("{}", val);
                            }

                            TokValue::Integer(val) => {
                                println!("{}", val);
                            }
                        }
                    }

                    _ => panic!("Invalid function call"),
                }
            }
            //
            // Internal function call
            //
            TokType::InternalFn(name) => match name.as_str() {
                //
                // Delete memory
                //
                "d" => {
                    let address = &tokens[pos + 1];

                    match address {
                        TokType::MemAddress(addr) => {
                            memory.remove(addr);
                        }

                        _ => panic!("Invalid address"),
                    }
                }
                //
                // Add two numbers
                //
                "a" => {
                    let first = match &tokens[pos + 1] {
                        TokType::MemAddress(addr) => memory.get(addr).unwrap(),
                        _ => panic!("Invalid address"),
                    };

                    let second = match &tokens[pos + 2] {
                        TokType::MemAddress(addr) => memory.get(addr).unwrap(),
                        _ => panic!("Invalid address"),
                    };

                    let to = match &tokens[pos + 2] {
                        TokType::MemAddress(addr) => memory.get(addr).unwrap(),
                        _ => panic!("Invalid address"),
                    };

                    match (first, second, to) {
                        (
                            TokValue::Integer(first),
                            TokValue::Integer(second),
                            TokValue::Integer(to),
                        ) => {
                            memory.insert(*to, TokValue::Integer(first + second));
                        }
                        _ => panic!("Invalid types"),
                    }
                }
                //
                // Subtract two numbers
                //
                "s" => {
                    let first = match &tokens[pos + 1] {
                        TokType::MemAddress(addr) => memory.get(addr).unwrap(),
                        _ => panic!("Invalid address"),
                    };

                    let second = match &tokens[pos + 2] {
                        TokType::MemAddress(addr) => memory.get(addr).unwrap(),
                        _ => panic!("Invalid address"),
                    };

                    let to = match &tokens[pos + 2] {
                        TokType::MemAddress(addr) => memory.get(addr).unwrap(),
                        _ => panic!("Invalid address"),
                    };

                    match (first, second, to) {
                        (
                            TokValue::Integer(first),
                            TokValue::Integer(second),
                            TokValue::Integer(to),
                        ) => {
                            if first < second {
                                memory.insert(*to, TokValue::Integer(0));

                                return;
                            }

                            memory.insert(*to, TokValue::Integer(first - second));
                        }
                        _ => panic!("Invalid types"),
                    }
                }
                //
                // Multiply two numbers
                //
                "m" => {
                    let first = match &tokens[pos + 1] {
                        TokType::MemAddress(addr) => memory.get(addr).unwrap(),
                        _ => panic!("Invalid address"),
                    };

                    let second = match &tokens[pos + 2] {
                        TokType::MemAddress(addr) => memory.get(addr).unwrap(),
                        _ => panic!("Invalid address"),
                    };

                    let to = match &tokens[pos + 2] {
                        TokType::MemAddress(addr) => memory.get(addr).unwrap(),
                        _ => panic!("Invalid address"),
                    };

                    match (first, second, to) {
                        (
                            TokValue::Integer(first),
                            TokValue::Integer(second),
                            TokValue::Integer(to),
                        ) => {
                            memory.insert(*to, TokValue::Integer(first * second));
                        }
                        _ => panic!("Invalid types"),
                    }
                }

                _ => panic!("Invalid internal function call"),
            },
            //
            // Comment
            //
            TokType::Comment(val) => match val.as_str() {
                "cfg dbg" => {
                    println!("Debug mode enabled");
                    println!("Memory: {:?}", memory);
                }
                _ => {}
            },
            //
            // nothing
            //
            _ => {}
        }

        pos += 1;
    }
}
