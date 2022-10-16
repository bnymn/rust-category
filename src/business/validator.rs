use crate::CategoryError;
use crate::CategorySaveEntity;

pub struct CategoryValidator {

}

impl CategoryValidator {
    pub fn new() -> Self {
        Self {}
    }
    
    pub fn validate(&self, _category: &CategorySaveEntity) -> Result<bool, CategoryError> {
        Ok(true)
    }
}