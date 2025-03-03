//! TODO: DOC

use std::sync::Arc;

pub use packages::{InitTask, InitTaskMessage, Name, PACKAGE_CREATORS};

use super::{
    deps::Dependencies,
    ext_traits::{GetWorkerSimStartMsg, MaybeCpuBound},
    prelude::*,
};
pub use crate::{config::Globals, hash_types::Agent};
use crate::{
    datastore::schema::{accessor::FieldSpecMapAccessor, RootFieldSpec, RootFieldSpecCreator},
    simulation::{comms::package::PackageComms, package::ext_traits::GetWorkerExpStartMsg},
    SimRunConfig,
};

pub mod packages;

#[async_trait]
pub trait Package: MaybeCpuBound + GetWorkerSimStartMsg + Send + Sync {
    async fn run(&mut self) -> Result<Vec<Agent>>;
}

pub trait PackageCreator: GetWorkerExpStartMsg + Sync + Send {
    /// We can't derive a default as that returns Self which implies Sized which in turn means we
    /// can't create Trait Objects out of PackageCreator
    fn new(experiment_config: &Arc<ExperimentConfig>) -> Result<Box<dyn PackageCreator>>
    where
        Self: Sized;

    /// Create the package.
    fn create(
        &self,
        config: &Arc<SimRunConfig>,
        system: PackageComms,
        accessor: FieldSpecMapAccessor,
    ) -> Result<Box<dyn Package>>;

    fn dependencies() -> Dependencies
    where
        Self: Sized,
    {
        Dependencies::empty()
    }

    fn get_state_field_specs(
        &self,
        _config: &ExperimentConfig,
        _globals: &Globals,
        _field_spec_map_builder: &RootFieldSpecCreator,
    ) -> Result<Vec<RootFieldSpec>> {
        Ok(vec![])
    }
}
