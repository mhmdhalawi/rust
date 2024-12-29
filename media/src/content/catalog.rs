use super::media::Media;

pub struct Catalog {
    media: Vec<Media>,
}

impl Catalog {
    pub fn new() -> Self {
        Catalog { media: Vec::new() }
    }

    pub fn add(&mut self, media: Media) {
        self.media.push(media);
    }

    pub fn display(&self) {
        for media in &self.media {
            println!("{}", media.description());
        }
    }

    pub fn get_by_index(&self, index: usize) -> Option<&Media> {
        self.media.get(index)
    }
}
