use crate::domain::entity::uuid::Uuid;

pub struct Type {
    id: Uuid,
    name: String,
}

impl Type {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id: Uuid::new(id),
            name,
        }
    }

    pub fn id(&self) -> &str {
        &self.id.value()
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
