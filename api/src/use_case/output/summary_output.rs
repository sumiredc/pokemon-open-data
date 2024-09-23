use crate::domain::model::summary::Summary;

pub struct SummaryOutput {
    data: Vec<Summary>,
}

impl SummaryOutput {
    pub fn new(data: Vec<Summary>) -> Self {
        Self { data }
    }

    pub fn data(&self) -> &Vec<Summary> {
        &self.data
    }
}
