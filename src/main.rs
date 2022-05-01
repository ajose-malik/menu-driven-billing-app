use std::io;

struct Bill {
    name: String,
    amount: f64,
}

struct Bills {
    inner: Vec<Bill>,
}

impl Bills {
    fn new() -> Self {
        Self { inner: vec![] }
    }
}

fn main() {
    return;
}
