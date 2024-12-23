#[derive(Debug)]
enum Media {
    Book {
        title: String,
        author: String,
    },
    Movie {
        title: String,
        director: String,
    },
    Audiobook {
        title: String,
    },
    Podcast(u32),
    Placeholder,
}

impl Media {
    fn description(&self) -> String {
        match self {
            Media::Book { title, author } => format!("Book {} by {}", title, author),
            Media::Movie { title, director } => format!("Movie {} by {}", title, director),
            Media::Audiobook { title } => format!("Audiobook {}", title),
            Media::Podcast(episode) => format!("Podcast episode {}", episode),
            Media::Placeholder => "Placeholder".to_string(),
        }
    }
}

struct Catalog {
    media: Vec<Media>,
}

impl Catalog {
    fn new() -> Self {
        Catalog { media: Vec::new() }
    }

    fn add(&mut self, media: Media) {
        self.media.push(media);
    }

    fn display(&self) {
        for media in &self.media {
            println!("{}", media.description());
        }
    }

    fn get_by_index(&self, index: usize) -> Option<&Media> {
        self.media.get(index)
    }
}

fn main() {
    let mut catalog = Catalog::new();

    let book = Media::Book {
        title: "The Hobbit".to_string(),
        author: "J.R.R. Tolkien".to_string(),
    };

    let movie = Media::Movie {
        title: "The Lord of the Rings".to_string(),
        director: "Peter Jackson".to_string(),
    };

    let audiobook = Media::Audiobook {
        title: "The Silmarillion".to_string(),
    };

    let podcast = Media::Podcast(42);
    let placeholder = Media::Placeholder;

    catalog.add(book);
    catalog.add(movie);
    catalog.add(audiobook);
    catalog.add(podcast);
    catalog.add(placeholder);

    catalog.display();

    let index = 2;

    match catalog.get_by_index(index) {
        Some(media) => println!("Media at index {}: {}", index, media.description()),
        None => println!("No media found at index {}", index),
    }
}
