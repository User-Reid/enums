#[derive(Debug)]
enum OnlineOrderStatus {
    Ordered,
    Packed,
    Shipped,
    Delivered,
}

impl OnlineOrderStatus {
    fn check(&self) {
        match self {
            OnlineOrderStatus::Delivered => {
                println!("Your item has been delivered!")
            }
            other_status => {
                println!("No effin clue dog.🤷 aparently your shit is {other_status:?}")
            }
        }
    }
}

fn main() {
    OnlineOrderStatus::Delivered.check();
    OnlineOrderStatus::Ordered.check();
    OnlineOrderStatus::Packed.check();
}
