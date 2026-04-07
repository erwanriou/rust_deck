use rand::{rng, seq::SliceRandom};

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

    // METHODS
    fn suffle(&mut self) {
        let mut rng = rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - num_cards)
    }
}

fn main() {
    // CALLS
    let mut deck = Deck::new();
    deck.suffle();

    // TODO ADD ERROR HANDLING
    let cards = deck.deal(3);

    // PRINTINGS
    println!("Here is your hand: {:#?}", cards);
    println!("Here is your deck: {:#?}", deck);
}
