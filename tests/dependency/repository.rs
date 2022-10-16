use category::{CategoryRepository, CategoryEntity, CategorySaveEntity};

pub struct CategoryRepositoryNull {
}

impl CategoryRepositoryNull {
    pub fn new() -> Self {
        Self {}
    }
}

impl CategoryRepository for CategoryRepositoryNull {
    fn save(&self, category: CategorySaveEntity) -> CategoryEntity {
        CategoryEntity::new(
            category.id().unwrap(), 
            category.title().unwrap().clone(), 
            category.slug().unwrap().clone(), 
            category.order().unwrap()
        )
    }

    fn find_by_id(&self, _category_id: u64) -> Option<CategoryEntity> {
        None
    }

    fn delete_by_id(&self, _category_id: u64) -> () {
        
    }
}
