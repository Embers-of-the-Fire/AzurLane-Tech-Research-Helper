use super::error::PsoError;

#[derive(Debug, Clone, Copy)]
pub struct PsoInertia {
    pub start: f64,
    pub end: f64,
    pub generation: usize,
    now_generation: usize,
}

impl PsoInertia {
    pub fn empty() -> PsoInertia {
        PsoInertia {
            start: 0.0,
            end: 0.0,
            generation: 0,
            now_generation: 0,
        }
    }

    pub fn new(start: f64, end: f64, generation: usize) -> PsoInertia {
        PsoInertia {
            start,
            end,
            generation,
            now_generation: 0,
        }
    }

    pub fn get(&mut self, generation: usize) -> Result<f64, PsoError> {
        if generation > self.generation {
            Err(PsoError::new(format!(
                "Unexpected generation value {}: should be less than {}",
                generation, self.generation
            )))
        } else {
            Ok(self.start
                - (self.start - self.end) * ((generation as f64 / self.generation as f64).powi(2)))
        }
    }
}
