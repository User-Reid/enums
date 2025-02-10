#[derive(Debug)]
enum Milk {
    Lowfat(i32),
    Whole,
    Nondairy { kind: String },
}

fn main() {
    let my_beverage: Milk = Milk::Whole;

    if let Milk::Nondairy { kind } = my_beverage {
        println!("Im a fuckin whole milk! and its {kind}")
    } else {
        println!("Your shit wack")
    }
}
