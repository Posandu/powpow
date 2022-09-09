#[derive(Debug)]
pub enum TokType {
    MemAddress(u64),
    String(String),
    Integer(u64),
    InternalFn(String),
    Fn(String),
    Comment(String),
}

#[derive(Debug)]
pub enum TokValue<'a> {
    String(&'a String),
    Integer(u64),
}

