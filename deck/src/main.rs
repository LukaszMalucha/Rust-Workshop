// Vec stands for vector
struct Deck {
    cards: Vec<String>

}


fn main() {
    let deck = Deck {cards: vec![] };

    println!("Deck: {}", deck);
}
