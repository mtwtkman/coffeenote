use uuid::Uuid;

pub struct Shop {
    pub id: Uuid,
    pub name: String,
    pub url: String,
}

impl Shop {
    pub fn new(name: String, url: String) -> Self {
        let id = Uuid::new_v4();
        Self { id, name, url }
    }
}
