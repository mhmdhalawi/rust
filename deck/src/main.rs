use rand::{ thread_rng, seq::SliceRandom };

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        let suits = ["Hearts", "Diamonds", "Spades"];
        let values = ["ace", "two", "three"];

        let mut cards = vec![];

        for suit in suits.iter() {
            for value in values.iter() {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        Deck {
            cards,
        }
    }

    fn shuffle(&mut self) {
        // Shuffle the deck
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        // Deal a number of cards
        let cards = self.cards.split_off(self.cards.len() - num_cards);
        cards
    }
}

fn main() {
    let mut deck = Deck::new();

    deck.shuffle();

    let hand = deck.deal(2);

    print!("Deck: {:#?}", deck);
    print!("Hand: {:#?}", hand);
}
