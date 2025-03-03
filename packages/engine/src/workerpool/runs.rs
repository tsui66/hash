use std::collections::HashMap;

use super::error::{Error, Result};
use crate::{config::WorkerAllocation, proto::SimulationShortId};

#[derive(Default)]
pub struct SimulationRuns {
    // Associates a simulation run with the workers available to it
    worker_allocations: HashMap<SimulationShortId, WorkerAllocation>,
}

impl SimulationRuns {
    pub fn push(
        &mut self,
        sim_id: SimulationShortId,
        worker_allocation: &WorkerAllocation,
    ) -> Result<()> {
        self.worker_allocations
            .try_insert(sim_id, worker_allocation.clone())
            .map_err(|_| Error::from("Occupied hashmap key"))?;
        Ok(())
    }

    pub fn get_worker_allocation(&self, id: SimulationShortId) -> Result<&WorkerAllocation> {
        self.worker_allocations
            .get(&id)
            .ok_or(Error::MissingSimulationWithId(id))
    }
}
