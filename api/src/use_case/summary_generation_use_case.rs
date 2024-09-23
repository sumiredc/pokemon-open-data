use crate::domain::model::repository::SummaryRepository;

use super::output::summary_output::SummaryOutput;

pub struct SummaryGenerationUseCase<T>
where
    T: SummaryRepository,
{
    summary_repository: T,
}

impl<T: SummaryRepository> SummaryGenerationUseCase<T> {
    pub fn new(summary_repository: T) -> Self {
        Self { summary_repository }
    }

    pub fn execute(&self) -> Result<SummaryOutput, ()> {
        let data = self.summary_repository.generation().unwrap_or_default();
        Ok(SummaryOutput::new(data))
    }
}
