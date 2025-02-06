#[derive(Debug)]
enum Beans {
    Pinto,
    Black,
}

#[derive(Debug)]
enum Meat {
    Chicken,
    Steak,
}

#[derive(Debug)]
enum RestaurantItem {
    Burrito { meat: Meat, beans: Beans },
    Bowl { meat: Meat, beans: Beans },
    VeganPlate,
}

fn main() {
    let lunch: RestaurantItem = RestaurantItem::Burrito {
        meat: (Meat::Chicken),
        beans: (Beans::Pinto),
    };
    let dinner: RestaurantItem = RestaurantItem::Bowl {
        meat: (Meat::Steak),
        beans: (Beans::Black),
    };
    let abandoned_meal: RestaurantItem = RestaurantItem::VeganPlate;

    println!(
        "Lunch was {:?}, and dinner was {:?}! Nobody had {:?}",
        lunch, dinner, abandoned_meal
    )
}
