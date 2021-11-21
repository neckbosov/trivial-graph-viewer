use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug)]
pub struct GraphVertex<T: FromStr + Display> {
    pub id: usize,
    pub value: T,
}

impl<T: FromStr + Display> GraphVertex<T> {
    pub fn new(id: usize, value: T) -> Self {
        Self { id, value }
    }
}
