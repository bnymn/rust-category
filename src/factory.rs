use crate::CategoryRepository;
use crate::business::reader::CategoryReader;
use crate::business::validator::CategoryValidator;
use crate::business::writer::CategoryWriter;
use crate::dependency::CategoryDependency;

pub struct CategoryFactory<'a> {
    category_dependency: &'a CategoryDependency<'a>,
}

impl<'a> CategoryFactory<'a> {
    pub fn new(category_dependency: &'a CategoryDependency) -> Self {
        CategoryFactory {
            category_dependency
        }
    }

    pub fn create_category_writer(&self) -> CategoryWriter {
        CategoryWriter::new(
            self.get_repository(),
            self.create_category_validator(),
        )
    }

    pub fn create_category_reader(&self) -> CategoryReader {
        CategoryReader::new(self.get_repository())
    }

    fn create_category_validator(&self) -> CategoryValidator {
        CategoryValidator::new()
    }

    fn get_repository(&self) -> &Box<dyn CategoryRepository> {
        self.category_dependency.category_repository
    }
}