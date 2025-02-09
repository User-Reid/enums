#[derive(Debug)]
enum Milk {
    Lowfat(i32),
    Wholemilk,
}

impl Milk {
    fn drink(self) {
        match self {
            Milk::Lowfat(2) => {
                println!("Delicious! 2% milk is my favorite!")
            }
            Milk::Lowfat(percent) => {
                println!("You've got the lowfat {percent}% version!")
            }
            Milk::Wholemilk => {
                println!("You've got the whole milk!")
            }
        }
    }
}

fn main() {
    Milk::Lowfat(3).drink();
    Milk::Wholemilk.drink();
}
