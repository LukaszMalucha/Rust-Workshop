use super::media::Media;

#[derive(Debug)]
pub struct Catalogue {
    items: Vec<Media>
}

impl Catalogue {
    pub fn new() -> Self {
        Catalogue { items: vec![] }
    }

    pub fn add(&mut self, media: Media) {
        self.items.push(media);

    }

    pub fn get_by_index(&self, index: usize) -> Option<&Media> {
        if self.items.len() > index {
            Some(&self.items[index])
        } else {
            None    
        }
        

    }

}