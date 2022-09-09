use crate::runner::run;
use crate::tokenizer::tokenize;
use std::time::Instant;

pub fn start(code: &String) {
    let all = String::new() + code + "# EOL \n\n\n";
    let tokens = tokenize(&all);

    let start_time = Instant::now();

    run(&tokens);

    println!("Done in {:.2?}", start_time.elapsed());
}
