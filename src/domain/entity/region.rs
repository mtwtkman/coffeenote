use uuid::Uuid;

pub struct Region {
    pub id: Uuid,
    pub name: String,
}

impl Region {
    pub fn new(name: String) -> Self {
        let id = Uuid::new_v4();
        Self { id, name }
    }
}
