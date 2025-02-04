#[derive(Debug)]
enum PaymentMethodType {
    CreditCard(String),
    DebitCard(String),
    PayPal(String, String),
}

fn main() {
    let mut my_payment_method: PaymentMethodType =
        PaymentMethodType::CreditCard(String::from("1322-3123-3232"));

    my_payment_method =
        PaymentMethodType::PayPal(String::from("Bob@gmail.com"), String::from("Tacobell!"));

    println!("{:#?}", my_payment_method)
}
