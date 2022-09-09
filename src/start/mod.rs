use crate::runner::run;
use crate::tokenizer::tokenize;
use std::time::Instant;

pub fn start(code: &String) {
    let tokens = tokenize(code);

    let start_time = Instant::now();

    run(&tokens);

    println!("Done in {:.2?}", start_time.elapsed());
}
