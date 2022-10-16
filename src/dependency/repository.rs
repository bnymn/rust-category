use crate::{CategoryEntity, CategorySaveEntity};

pub trait CategoryRepository {
    /**
     * Saves the given category without saving the children
     */
    fn save(&self, category: CategorySaveEntity) -> CategoryEntity;

    /**
     * Returns a CategoryEntity if found without children
     */
    fn find_by_id(&self, category_id: u64) -> Option<CategoryEntity>;

    /**
     * Deletes the CategoryEntity with the given category_id
     */
    fn delete_by_id(&self, category_id: u64) -> ();
}