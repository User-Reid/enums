#[derive(Debug)]
enum CardSuit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

struct Card {
    rank: String,
    suit: CardSuit,
}

fn main() {
    let first_card: CardSuit = CardSuit::Hearts;
    let mut second_card: CardSuit = CardSuit::Spades;
    let third_card: CardSuit = CardSuit::Clubs;
    second_card = CardSuit::Diamonds;

    let card_suits: [CardSuit; 2] = [CardSuit::Hearts, CardSuit::Clubs];
    let card_suits2: (CardSuit, CardSuit) = (CardSuit::Hearts, CardSuit::Clubs);

    println!("{:#?}", second_card)
}
