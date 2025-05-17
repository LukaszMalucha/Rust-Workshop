use rand::{thread_rng, seq::SliceRandom};


// Vec stands for vector
#[derive(Debug)]
struct Deck {
    cards: Vec<String>
}

impl Deck {
    fn new() -> Self {
        // List of 'suits' 
        let suits = ["Hearts", "Spades", "Diamonds", "Clubs"];
        // List of 'values'
        let values = ["Ace", "Two", "Three"];
        // Double nested for loop

        // mut = mutable - can be changed
        let mut cards = vec![];

        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);    
            }
        }

        // Return statement simplified, remove ; at the end
        Deck { cards }
    }
    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }
}


fn main() {
    // deck is changing via shuffling = has to be mutable
    let mut deck = Deck::new();
    deck.shuffle();

    println!("Deck: {:#?}", deck);
}

//  cargo run
//  crates.io
// cargo add rand