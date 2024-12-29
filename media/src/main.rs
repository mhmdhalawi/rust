mod content;

use content::{catalog::Catalog, media::Media};

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
