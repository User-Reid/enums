enum LaundryCycle {
    Cold,
    Hot { temp: u32 },
    Delicate(String),
}

fn wash_laundry(cycle: LaundryCycle) {
    match cycle {
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

fn main() {
    wash_laundry(LaundryCycle::Delicate(String::from("Bleach")));
    wash_laundry(LaundryCycle::Hot { temp: 100 });
    wash_laundry(LaundryCycle::Cold);
}
