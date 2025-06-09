#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
    Podcast (u32),
    Placeholder
}

impl Media {
    fn description(&self) -> String {
        match self {
            Media::Book { title, author } => {
                format!("Book: {} {}", title, author)
            },
            Media::Movie { title, director } => {
                format!("Book: {} {}", title, director)
            },
            Media::Audiobook { title } => {
                format!("Book: {}", title)
            }
            Media::Podcast(episode_number) => {
                format!("Podcast {}", episode_number)
            }

            Media::Placeholder => {
                format!("Placeholder")
            }
        }

    }

}
#[derive(Debug)]
struct Catalogue {
    items: Vec<Media>
}

impl Catalogue {
    fn new() -> Self {
        Catalogue { items: vec![] }
    }

    fn add(&mut self, media: Media) {
        self.items.push(media);

    }

    fn get_by_index(&self, index: usize) -> Option<&Media> {
        if self.items.len() > index {
            Some(&self.items[index])
        } else {
            None    
        }
        

    }

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

    let podcast = Media::Podcast(33);
    let placeholder = Media::Placeholder;
    // print_media(audiobook);
    // print_media(good_movie);

    let mut catalogue = Catalogue::new();
    catalogue.add(audiobook);
    catalogue.add(good_movie);
    catalogue.add(podcast);
    catalogue.add(placeholder);

    let item = catalogue.get_by_index(0);
}
