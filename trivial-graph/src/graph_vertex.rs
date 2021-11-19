pub struct GraphVertex<T> {
    pub id: usize,
    pub value: T,
}

impl<T> GraphVertex<T> {
    pub fn new(id: usize, value: T) -> Self {
        Self { id, value }
    }
}
