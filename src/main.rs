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

    fn add(&mut self, bill:Bill) {
        self.inner.push(bill)
    }

    fn get_all(&self) -> &Vec<Bill> {
        &self.inner
    }
}

fn get_input () -> String {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please enter your data again");
    }

    buffer.trim().to_owned()
}


fn main() {
    return;
}
