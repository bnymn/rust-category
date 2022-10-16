use crate::{CategoryRepository, CategoryEntity, CategorySaveEntity, CategoryError};

use super::validator::CategoryValidator;

pub struct CategoryWriter<'a> {
    category_repository: &'a Box<dyn CategoryRepository>,
    category_validator: CategoryValidator,
}

impl<'a> CategoryWriter<'a> {
    pub fn new(
        category_repository: &'a Box<dyn CategoryRepository>,
        category_validator: CategoryValidator
    ) -> Self {
        Self {
            category_repository,
            category_validator
        }
    }

    pub fn save(&self, category: CategorySaveEntity) -> Result<CategoryEntity, CategoryError> {
        match self.category_validator.validate(&category) {
            Ok(_) => Ok(self.category_repository.save(category)),
            Err(err) => Err(err)
        }
    }

    /**
     * Deletes the CategoryEntity with the given category_id
     */
    pub fn delete_by_id(&self, _category_id: u64) -> () {
        
    }
}