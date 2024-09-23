use crate::domain::entity::uuid::Uuid;

pub struct Generation {
    id: Uuid,
    value: u8,
}

impl Generation {
    pub fn new(id: String, value: u8) -> Self {
        Self {
            id: Uuid::new(id),
            value,
        }
    }

    pub fn id(&self) -> &str {
        &self.id.value()
    }

    pub fn value(&self) -> &u8 {
        &self.value
    }
}
