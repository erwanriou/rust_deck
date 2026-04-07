#[derive(Debug)]

// STRUCTURE
struct Deck {
    cards: Vec<String>,
}

// IMPLEMENTATION (CLASS)
impl Deck {
    // ASSOCIATED FUNCTION
    fn new() -> Self {
        let suits = ["hearts", "diamonds", "clubs", "spades"];
        let values = [
            "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "jack",
            "queen", "king", "ace",
        ];

        let mut cards = vec![];

        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }
        // IMPLICIT RETURN
        Self { cards }
    }

    // METHOD
    fn suffle(&self) {}
}

fn main() {
    let deck = Deck::new();
    deck.suffle();
    println!("Here is your deck: {:#?}", deck);
}
