#[derive(Debug)]
enum Media {
    Book {title: String, author: String},
    Movie {title: String, director: String},
    Audiobook {title: String}
}

fn print_media(media: Media) {
    println!("{:#?}", media)

}



fn main() {
    println!("Hello, world!");
    let audiobook = Media::Audiobook {
        title: String::from("An Audiobook"),
    };
    let good_movie = Media::Movie { title: String::from("The Matrix"), director: String::from("Wachowski brothers") };

    print_media(audiobook);
    print_media(good_movie);
}
