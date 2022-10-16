use crate::{CategoryRepository, CategoryEntity};

pub struct CategoryReader<'a> {
    category_repository: &'a Box<dyn CategoryRepository>,
}

impl<'a> CategoryReader<'a> {
    pub fn new(category_repository: &'a Box<dyn CategoryRepository>) -> Self {
        Self {
            category_repository
        }
    }

    pub fn find_by_id(&self, category_id: u64) -> Option<CategoryEntity> {
        self.category_repository.find_by_id(category_id)
    }
}