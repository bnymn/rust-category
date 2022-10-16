mod business;
mod dependency;
mod entity;
mod error;
mod factory;

use dependency::CategoryDependency;

pub use crate::dependency::CategoryRepository;
pub use crate::entity::CategoryEntity;
pub use crate::entity::CategorySaveEntity;
pub use crate::error::Error as CategoryError;
use crate::factory::CategoryFactory;

pub struct CategoryFacade<'a> {
    category_factory: CategoryFactory<'a>,
}

impl<'a> CategoryFacade<'a> {
    pub fn new(category_dependency: CategoryDependency<'a>) -> Self {
        Self {
            category_factory: CategoryFactory::new(category_dependency)
        }
    }

    /**
     * Saves the given category
     */
    pub fn save(&self, category: CategorySaveEntity) -> Result<CategoryEntity, CategoryError> {
        self.category_factory.create_category_writer().save(category)
    }

    /**
     * Returns a CategoryEntity if found without children
     */
    pub fn find_by_id(&self, category_id: u64) -> Option<CategoryEntity> {
        self.category_factory.create_category_reader().find_by_id(category_id)
    }

    /**
     * Deletes the CategoryEntity with the given category_id
     */
    pub fn delete_by_id(&self, category_id: u64) -> () {
        self.category_factory.create_category_writer().delete_by_id(category_id)
    }
}