use crate::domain::model::summary::Summary;

use super::schema::SummarySchema;

pub struct SummaryResponder {}

impl SummaryResponder {
    pub fn create_all(&self, data: &Vec<Summary>) -> Vec<SummarySchema> {
        data.iter()
            .map(|pokemon| SummarySchema::new(pokemon))
            .collect()
    }
}
