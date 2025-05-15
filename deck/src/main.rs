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
    fn shuffle(&self) {


    }
}


fn main() {

    let deck = Deck::new();
    deck.shuffle();

    println!("Deck: {:#?}", deck);
}

//  cargo run
//  crates.io
// cargo add rand