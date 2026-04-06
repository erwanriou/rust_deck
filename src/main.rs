#[derive(Debug)]

struct Deck {
    cards: Vec<String>,
}

fn main() {
    // LIST OF SUITS & VALUES
    let suits = ["hearts", "diamonds", "clubs", "spades"];
    let values = [
        "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "jack", "queen",
        "king", "ace",
    ];

    let mut cards = vec![];

    for suit in suits {
        for value in values {
            let card = format!("{} of {}", value, suit);
            cards.push(card);
        }
    }

    let deck = Deck { cards };
    println!("Here is your deck: {:#?}", deck);
}
