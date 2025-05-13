// Vec stands for vector
#[derive(Debug)]
struct Deck {
    cards: Vec<String>
}


fn main() {
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

    let deck = Deck { cards };

    println!("Deck: {:#?}", deck);
}

