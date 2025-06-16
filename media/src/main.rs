mod content;

use content::media::Media;
use content::catalogue::Catalogue;


fn main() {
    println!("Hello, world!");
    let audiobook = Media::Audiobook {
        title: String::from("An Audiobook"),
    };
    let good_movie = Media::Movie { title: String::from("The Matrix"), director: String::from("Wachowski brothers") };

    let podcast = Media::Podcast(33);
    let placeholder = Media::Placeholder;


    let mut catalogue = Catalogue::new();
    catalogue.add(audiobook);
    catalogue.add(good_movie);
    catalogue.add(podcast);
    catalogue.add(placeholder);

    let item = catalogue.get_by_index(1);
    let placeholder = Media::Placeholder;

    println!("{:#?}", item.unwrap());
    println!("{:#?}", item.expect("expected item"));
    println!("{:#?}", item.unwrap_or(&placeholder));
}
