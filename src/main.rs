#[derive(Debug)]
enum Milk {
    Lowfat(i32),
    Whole,
    Nondairy { kind: String },
}

fn main() {
    let my_beverage: Milk = Milk::Nondairy {
        kind: String::from("Taci"),
    };

    let Milk::Nondairy { kind } = my_beverage else {
        println!("You dont have any damn milk, Clown");
        return;
    };

    println!("{kind}% Milk is available here.")
}
