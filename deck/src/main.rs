// Vec stands for vector
#[derive(Debug)]
struct Deck {
    cards: Vec<String>

}


fn main() {
    let deck = Deck {cards: vec![] };

    println!("Deck: {:?}", deck);
}

