mod dependency;

use category::CategoryDependency;
use category::CategoryFacade;
use category::CategoryRepository;
use category::CategorySaveEntity;

use crate::dependency::repository::CategoryRepositoryNull;

#[test]
pub fn init() {
    let category_repository: Box<dyn CategoryRepository> = Box::new(CategoryRepositoryNull::new());
    let category_dependency = CategoryDependency {
            category_repository: &category_repository
    };
    let category_facade = CategoryFacade::new(&category_dependency);

    let saved_category = category_facade.save(CategorySaveEntity {
        id: Some(1),
        title: Some(String::new()),
        slug: Some(String::new()),
        order: Some(1),
    });

    assert_eq!(true, saved_category.is_ok());
}
