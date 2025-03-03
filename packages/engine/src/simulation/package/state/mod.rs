pub mod packages;

use std::sync::Arc;

pub use packages::{Name, StateTask, StateTaskMessage, PACKAGE_CREATORS};
use tracing::Span;

use super::{deps::Dependencies, ext_traits::GetWorkerSimStartMsg, prelude::*};
pub use crate::config::Globals;
use crate::{
    config::ExperimentConfig,
    datastore::{
        batch::change::ArrayChange,
        error::Result as DatastoreResult,
        schema::{accessor::FieldSpecMapAccessor, RootFieldSpec, RootFieldSpecCreator},
    },
    simulation::{
        comms::package::PackageComms, package::ext_traits::GetWorkerExpStartMsg, Error, Result,
    },
    SimRunConfig,
};

#[async_trait]
pub trait Package: GetWorkerSimStartMsg + Send + Sync {
    async fn run(&mut self, state: &mut State, context: &Context) -> Result<()>;

    fn span(&self) -> Span;
}

pub trait PackageCreator: GetWorkerExpStartMsg + Send + Sync {
    /// We can't derive a default as that returns Self which implies Sized which in turn means we
    /// can't create Trait Objects out of PackageCreator
    fn new(experiment_config: &Arc<ExperimentConfig>) -> Result<Box<dyn PackageCreator>>
    where
        Self: Sized;

    /// Create the package.
    fn create(
        &self,
        config: &Arc<SimRunConfig>,
        comms: PackageComms,
        accessor: FieldSpecMapAccessor,
    ) -> Result<Box<dyn Package>>;

    /// Get the package names that this package depends on.
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

pub struct StateColumn {
    inner: Box<dyn IntoArrowChange + Send + Sync>,
}

impl StateColumn {
    pub fn get_arrow_change(&self, range: std::ops::Range<usize>) -> DatastoreResult<ArrayChange> {
        self.inner.get_arrow_change(range)
    }

    pub fn new(inner: Box<dyn IntoArrowChange + Send + Sync>) -> StateColumn {
        StateColumn { inner }
    }
}

pub trait IntoArrowChange {
    fn get_arrow_change(&self, range: std::ops::Range<usize>) -> DatastoreResult<ArrayChange>;
}
