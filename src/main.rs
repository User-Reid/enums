enum LaundryCycle {
    Cold,
    Hot { temp: u32 },
    Delicate(String),
}

impl LaundryCycle {
    fn wash_laundry(&self) {
        match self {
            LaundryCycle::Cold => {
                println!("Running the laundry with cold water.")
            }
            LaundryCycle::Hot { temp } => {
                println!("Running the laundry with a temperature of {temp} degrees");
            }
            LaundryCycle::Delicate(fabric_type) => {
                println!("Running the laundry with {fabric_type}");
            }
        }
    }
}

fn main() {
    LaundryCycle::Cold.wash_laundry();
    let hot_cycle: LaundryCycle = LaundryCycle::Hot { temp: 34 };
    hot_cycle.wash_laundry();
    let fabric: LaundryCycle = LaundryCycle::Delicate(String::from("Tacos"));
    fabric.wash_laundry();
}
