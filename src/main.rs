#[derive(Debug)]
enum PaymentMethodType {
    CreditCard(String),
    DebitCard(String),
    PayPal { username: String, password: String },
}

fn main() {
    let visa: PaymentMethodType = PaymentMethodType::CreditCard(String::from("2342-2323-3232"));

    let paypal_credentials: PaymentMethodType = PaymentMethodType::PayPal {
        username: (String::from("BalloonBoy21")),
        password: (String::from("Password1!")),
    };

    println!("{:#?}, Visa: {:#?}", paypal_credentials, visa)
}
