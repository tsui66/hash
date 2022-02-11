#![allow(
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::cast_sign_loss,
    clippy::empty_enum,
    clippy::used_underscore_binding,
    clippy::redundant_static_lifetimes,
    clippy::redundant_field_names,

    unused_imports
)]
// automatically generated by the FlatBuffers compiler, do not modify

use super::batch_generated::*;
use super::metaversion_generated::*;
use super::package_config_generated::*;
use super::serialized_generated::*;
use super::shared_context_generated::*;
use std::cmp::Ordering;
use std::mem;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

// struct ExperimentId, aligned to 1
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct ExperimentId(pub [u8; 16]);
impl Default for ExperimentId {
    fn default() -> Self {
        Self([0; 16])
    }
}
impl std::fmt::Debug for ExperimentId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ExperimentId")
            .field("inner", &self.inner())
            .finish()
    }
}

impl flatbuffers::SimpleToVerifyInSlice for ExperimentId {}
impl flatbuffers::SafeSliceAccess for ExperimentId {}
impl<'a> flatbuffers::Follow<'a> for ExperimentId {
    type Inner = &'a ExperimentId;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        <&'a ExperimentId>::follow(buf, loc)
    }
}
impl<'a> flatbuffers::Follow<'a> for &'a ExperimentId {
    type Inner = &'a ExperimentId;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        flatbuffers::follow_cast_ref::<ExperimentId>(buf, loc)
    }
}
impl<'b> flatbuffers::Push for ExperimentId {
    type Output = ExperimentId;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(self as *const ExperimentId as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}
impl<'b> flatbuffers::Push for &'b ExperimentId {
    type Output = ExperimentId;

    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(*self as *const ExperimentId as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}

impl<'a> flatbuffers::Verifiable for ExperimentId {
    #[inline]
    fn run_verifier(
        v: &mut flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
        use self::flatbuffers::Verifiable;
        v.in_buffer::<Self>(pos)
    }
}
impl<'a> ExperimentId {
    #[allow(clippy::too_many_arguments)]
    pub fn new(inner: &[i8; 16]) -> Self {
        let mut s = Self([0; 16]);
        s.set_inner(&inner);
        s
    }

    pub fn inner(&'a self) -> flatbuffers::Array<'a, i8, 16> {
        flatbuffers::Array::follow(&self.0, 0)
    }

    pub fn set_inner(&mut self, items: &[i8; 16]) {
        flatbuffers::emplace_scalar_array(&mut self.0, 0, items);
    }
}

pub enum InitOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Init<'a> {
    pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Init<'a> {
    type Inner = Init<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf, loc },
        }
    }
}

impl<'a> Init<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Init { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args InitArgs<'args>,
    ) -> flatbuffers::WIPOffset<Init<'bldr>> {
        let mut builder = InitBuilder::new(_fbb);
        builder.add_worker_index(args.worker_index);
        if let Some(x) = args.package_config {
            builder.add_package_config(x);
        }
        if let Some(x) = args.shared_context {
            builder.add_shared_context(x);
        }
        if let Some(x) = args.experiment_id {
            builder.add_experiment_id(x);
        }
        builder.finish()
    }

    pub const VT_EXPERIMENT_ID: flatbuffers::VOffsetT = 4;
    pub const VT_WORKER_INDEX: flatbuffers::VOffsetT = 6;
    pub const VT_SHARED_CONTEXT: flatbuffers::VOffsetT = 8;
    pub const VT_PACKAGE_CONFIG: flatbuffers::VOffsetT = 10;

    #[inline]
    pub fn experiment_id(&self) -> Option<&'a ExperimentId> {
        self._tab.get::<ExperimentId>(Init::VT_EXPERIMENT_ID, None)
    }
    #[inline]
    pub fn worker_index(&self) -> u64 {
        self._tab
            .get::<u64>(Init::VT_WORKER_INDEX, Some(0))
            .unwrap()
    }
    #[inline]
    pub fn shared_context(&self) -> SharedContext<'a> {
        self._tab
            .get::<flatbuffers::ForwardsUOffset<SharedContext>>(Init::VT_SHARED_CONTEXT, None)
            .unwrap()
    }
    #[inline]
    pub fn package_config(&self) -> PackageConfig<'a> {
        self._tab
            .get::<flatbuffers::ForwardsUOffset<PackageConfig>>(Init::VT_PACKAGE_CONFIG, None)
            .unwrap()
    }
}

impl flatbuffers::Verifiable for Init<'_> {
    #[inline]
    fn run_verifier(
        v: &mut flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
        use self::flatbuffers::Verifiable;
        v.visit_table(pos)?
            .visit_field::<ExperimentId>(&"experiment_id", Self::VT_EXPERIMENT_ID, false)?
            .visit_field::<u64>(&"worker_index", Self::VT_WORKER_INDEX, false)?
            .visit_field::<flatbuffers::ForwardsUOffset<SharedContext>>(
                &"shared_context",
                Self::VT_SHARED_CONTEXT,
                true,
            )?
            .visit_field::<flatbuffers::ForwardsUOffset<PackageConfig>>(
                &"package_config",
                Self::VT_PACKAGE_CONFIG,
                true,
            )?
            .finish();
        Ok(())
    }
}
pub struct InitArgs<'a> {
    pub experiment_id: Option<&'a ExperimentId>,
    pub worker_index: u64,
    pub shared_context: Option<flatbuffers::WIPOffset<SharedContext<'a>>>,
    pub package_config: Option<flatbuffers::WIPOffset<PackageConfig<'a>>>,
}
impl<'a> Default for InitArgs<'a> {
    #[inline]
    fn default() -> Self {
        InitArgs {
            experiment_id: None,
            worker_index: 0,
            shared_context: None, // required field
            package_config: None, // required field
        }
    }
}
pub struct InitBuilder<'a: 'b, 'b> {
    fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> InitBuilder<'a, 'b> {
    #[inline]
    pub fn add_experiment_id(&mut self, experiment_id: &ExperimentId) {
        self.fbb_
            .push_slot_always::<&ExperimentId>(Init::VT_EXPERIMENT_ID, experiment_id);
    }
    #[inline]
    pub fn add_worker_index(&mut self, worker_index: u64) {
        self.fbb_
            .push_slot::<u64>(Init::VT_WORKER_INDEX, worker_index, 0);
    }
    #[inline]
    pub fn add_shared_context(
        &mut self,
        shared_context: flatbuffers::WIPOffset<SharedContext<'b>>,
    ) {
        self.fbb_
            .push_slot_always::<flatbuffers::WIPOffset<SharedContext>>(
                Init::VT_SHARED_CONTEXT,
                shared_context,
            );
    }
    #[inline]
    pub fn add_package_config(
        &mut self,
        package_config: flatbuffers::WIPOffset<PackageConfig<'b>>,
    ) {
        self.fbb_
            .push_slot_always::<flatbuffers::WIPOffset<PackageConfig>>(
                Init::VT_PACKAGE_CONFIG,
                package_config,
            );
    }
    #[inline]
    pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> InitBuilder<'a, 'b> {
        let start = _fbb.start_table();
        InitBuilder {
            fbb_: _fbb,
            start_: start,
        }
    }
    #[inline]
    pub fn finish(self) -> flatbuffers::WIPOffset<Init<'a>> {
        let o = self.fbb_.end_table(self.start_);
        self.fbb_
            .required(o, Init::VT_SHARED_CONTEXT, "shared_context");
        self.fbb_
            .required(o, Init::VT_PACKAGE_CONFIG, "package_config");
        flatbuffers::WIPOffset::new(o.value())
    }
}

impl std::fmt::Debug for Init<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ds = f.debug_struct("Init");
        ds.field("experiment_id", &self.experiment_id());
        ds.field("worker_index", &self.worker_index());
        ds.field("shared_context", &self.shared_context());
        ds.field("package_config", &self.package_config());
        ds.finish()
    }
}
