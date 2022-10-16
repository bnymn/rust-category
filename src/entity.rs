#[derive(Debug)]
pub struct CategorySaveEntity {
    id: Option<u64>,
    title: Option<String>,
    slug: Option<String>,
    order: Option<u64>,
}

#[derive(Debug)]
pub struct CategoryEntity {
    id: u64,
    title: String,
    slug: String,
    order: u64,
}

impl CategoryEntity {
    pub fn new (id: u64, title: String, slug: String, order: u64) -> CategoryEntity {
        CategoryEntity {
            id,
            title,
            slug,
            order
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn slug(&self) -> &String {
        &self.slug
    }

    pub fn order(&self) -> u64 {
        self.order
    }
}

impl CategorySaveEntity {
    pub fn id(&self) -> Option<u64> {
        self.id
    }

    pub fn title(&self) -> Option<&String> {
        self.title.as_ref()
    }

    pub fn slug(&self) -> Option<&String> {
        self.slug.as_ref()
    }

    pub fn order(&self) -> Option<u64> {
        self.order
    }
}