use crate::domain::model::repository::SummaryRepository;

use super::output::summary_output::SummaryOutput;

pub struct SummaryTypeUseCase<T>
where
    T: SummaryRepository,
{
    summary_repository: T,
}

impl<T: SummaryRepository> SummaryTypeUseCase<T> {
    pub fn new(summary_repository: T) -> Self {
        Self { summary_repository }
    }

    pub fn execute(&self) -> Result<SummaryOutput, ()> {
        let data = self.summary_repository.r#type().unwrap_or_default();
        Ok(SummaryOutput::new(data))
    }
}
