#[derive(Debug)]
struct FoodBill {
    meal: String,
    price: f64,
    dessert: bool,
    total: f64,
}

impl FoodBill {
    fn new(meal: String, price: f64, dessert: bool, total: f64) -> Self {
        Self {
            meal,
            price,
            dessert,
            total,
        }
    }

    fn total_bill_calculator(&mut self) -> f64 {
        if self.dessert == false {
            self.total = (self.price * 0.2) + self.price;
            self.total
        } else {
            self.total = (self.price * 0.2) + self.price + 5.00;
            self.total
        }
    }
}

fn main() {
    let mut meal_one: FoodBill = FoodBill::new(String::from("Chicken Alfredo"), 32.00, true, 0.00);

    println!("{:#?}", meal_one.total_bill_calculator())
}
