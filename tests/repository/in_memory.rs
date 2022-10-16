use category::CategoryEntity;
use category::{CategoryRepository, CategoryEntity, CategorySaveEntity};
use lazy_static::lazy_static;

pub struct CategoryRepositoryInMemory {
}

lazy_static! {
    static ref CATEGORIES: Vec<CategoryEntity> = {
         Vec::new()
    };
}

impl CategoryRepository for CategoryRepositoryInMemory {
    fn save(&self, mut category: CategorySaveEntity) -> CategoryEntity {
        if let None = category.id() {
            category.id = CATEGORIES.len().try_into().unwrap()
        }
        self.categories.push(CategoryEntity {
            id: category.id,
            title: category.title.clone(),
            slug: category.slug.clone(),
            order: category.order,
        });

        category
    }

    fn find_by_id(&self, category_id: u64) -> Option<CategoryEntity> {
        for current_category in self.categories.iter() {
            if let Some(current_category_id) = current_category.id {
                if current_category_id == category_id {
                    return Some(CategoryEntity {
                        id: current_category.id, 
                        title: current_category.title.clone(),
                        slug: current_category.slug.clone(),
                        order: current_category.order,
                    })
                }
            }
        }
        
        None
    }

    fn delete_by_id(&self, category_id: u64) -> () {
        let mut position_to_delete = None;
        self.categories.iter().enumerate().for_each(|(current_position, current_category)| {
            if current_category.id.unwrap() == category_id {
                position_to_delete = Some(current_position);
            }
        });

        if position_to_delete.is_some() {
            self.categories.remove(position_to_delete.unwrap());
        }
    }
}
