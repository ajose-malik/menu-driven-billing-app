use std::io;

#[derive(Debug)]
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

    fn add(&mut self, bill: Bill) {
        self.inner.push(bill)
    }

    fn get_all(&self) -> &Vec<Bill> {
        &self.inner
    }
}

fn get_input () -> String {
    let mut buffer: String = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please enter your data again");
    }

    buffer.trim().to_owned()
}

fn get_bill_amount() -> f64 {
    println!("Amount:");

    loop {
        let input = get_input();
        let parsed_input = input.parse();
        match parsed_input {
            Ok(amount) => return amount,
            Err(_) => println!("Please enter a number"),
        }
    }
}

fn add_bill_menu(bills: &mut Bills) {
    let name = get_input();
    let amount = get_bill_amount();
    let bill = Bill {name, amount};
    bills.add(bill);
    println!("Bills added")
}

fn view_bills_menu(bills: &Bills) {
    for bill in bills.get_all() {
        println!("{:?}", bill);
    }
}

fn main_menu() {
    fn show() {
        println!("");
        println!("== Manage Bills ==");
        println!("1. Add bill");
        println!("2. View bills");
        println!("");
        println!("Enter selection:");
    }

    let mut bills = Bills::new();

    loop {
        show();
        let input = get_input();
        match input.as_str() {
            "1" => add_bill_menu(&mut bills),
            "2" => view_bills_menu(&bills),
            _ => break,
        }
    }    
}

fn main() {
    main_menu();
}
