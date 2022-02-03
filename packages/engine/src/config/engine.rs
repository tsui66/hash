use crate::types::WorkerIndex;

// Configuration specific to a single simulation engine
#[derive(Debug, Clone)]
pub struct Config {
    pub worker_allocation: WorkerAllocation,
    pub num_workers: usize,
}

/// Vec of the workers that are allocated to this simulation run
pub type WorkerAllocation = Vec<Worker>;

#[derive(derive_new::new, Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Worker(WorkerIndex);

impl Worker {
    pub fn index(&self) -> WorkerIndex {
        self.0
    }
}
