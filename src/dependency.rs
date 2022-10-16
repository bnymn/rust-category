mod repository;

pub use self::repository::CategoryRepository;

pub struct CategoryDependency<'a> {
    pub category_repository: &'a Box<dyn CategoryRepository>,
}