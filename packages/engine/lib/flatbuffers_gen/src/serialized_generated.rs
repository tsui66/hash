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

use std::cmp::Ordering;
use std::mem;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

pub enum SerializedOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Serialized<'a> {
    pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Serialized<'a> {
    type Inner = Serialized<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf, loc },
        }
    }
}

impl<'a> Serialized<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Serialized { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args SerializedArgs<'args>,
    ) -> flatbuffers::WIPOffset<Serialized<'bldr>> {
        let mut builder = SerializedBuilder::new(_fbb);
        if let Some(x) = args.inner {
            builder.add_inner(x);
        }
        builder.finish()
    }

    pub const VT_INNER: flatbuffers::VOffsetT = 4;

    #[inline]
    pub fn inner(&self) -> &'a [u8] {
        self._tab
            .get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(
                Serialized::VT_INNER,
                None,
            )
            .map(|v| v.safe_slice())
            .unwrap()
    }
}

impl flatbuffers::Verifiable for Serialized<'_> {
    #[inline]
    fn run_verifier(
        v: &mut flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
        use self::flatbuffers::Verifiable;
        v.visit_table(pos)?
            .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u8>>>(
                &"inner",
                Self::VT_INNER,
                true,
            )?
            .finish();
        Ok(())
    }
}
pub struct SerializedArgs<'a> {
    pub inner: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
}
impl<'a> Default for SerializedArgs<'a> {
    #[inline]
    fn default() -> Self {
        SerializedArgs {
            inner: None, // required field
        }
    }
}
pub struct SerializedBuilder<'a: 'b, 'b> {
    fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> SerializedBuilder<'a, 'b> {
    #[inline]
    pub fn add_inner(&mut self, inner: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u8>>) {
        self.fbb_
            .push_slot_always::<flatbuffers::WIPOffset<_>>(Serialized::VT_INNER, inner);
    }
    #[inline]
    pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> SerializedBuilder<'a, 'b> {
        let start = _fbb.start_table();
        SerializedBuilder {
            fbb_: _fbb,
            start_: start,
        }
    }
    #[inline]
    pub fn finish(self) -> flatbuffers::WIPOffset<Serialized<'a>> {
        let o = self.fbb_.end_table(self.start_);
        self.fbb_.required(o, Serialized::VT_INNER, "inner");
        flatbuffers::WIPOffset::new(o.value())
    }
}

impl std::fmt::Debug for Serialized<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ds = f.debug_struct("Serialized");
        ds.field("inner", &self.inner());
        ds.finish()
    }
}
#[inline]
#[deprecated(since = "2.0.0", note = "Deprecated in favor of `root_as...` methods.")]
pub fn get_root_as_serialized<'a>(buf: &'a [u8]) -> Serialized<'a> {
    unsafe { flatbuffers::root_unchecked::<Serialized<'a>>(buf) }
}

#[inline]
#[deprecated(since = "2.0.0", note = "Deprecated in favor of `root_as...` methods.")]
pub fn get_size_prefixed_root_as_serialized<'a>(buf: &'a [u8]) -> Serialized<'a> {
    unsafe { flatbuffers::size_prefixed_root_unchecked::<Serialized<'a>>(buf) }
}

#[inline]
/// Verifies that a buffer of bytes contains a `Serialized`
/// and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_serialized_unchecked`.
pub fn root_as_serialized(buf: &[u8]) -> Result<Serialized, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::root::<Serialized>(buf)
}
#[inline]
/// Verifies that a buffer of bytes contains a size prefixed
/// `Serialized` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `size_prefixed_root_as_serialized_unchecked`.
pub fn size_prefixed_root_as_serialized(
    buf: &[u8],
) -> Result<Serialized, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::size_prefixed_root::<Serialized>(buf)
}
#[inline]
/// Verifies, with the given options, that a buffer of bytes
/// contains a `Serialized` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_serialized_unchecked`.
pub fn root_as_serialized_with_opts<'b, 'o>(
    opts: &'o flatbuffers::VerifierOptions,
    buf: &'b [u8],
) -> Result<Serialized<'b>, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::root_with_opts::<Serialized<'b>>(opts, buf)
}
#[inline]
/// Verifies, with the given verifier options, that a buffer of
/// bytes contains a size prefixed `Serialized` and returns
/// it. Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_serialized_unchecked`.
pub fn size_prefixed_root_as_serialized_with_opts<'b, 'o>(
    opts: &'o flatbuffers::VerifierOptions,
    buf: &'b [u8],
) -> Result<Serialized<'b>, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::size_prefixed_root_with_opts::<Serialized<'b>>(opts, buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a Serialized and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid `Serialized`.
pub unsafe fn root_as_serialized_unchecked(buf: &[u8]) -> Serialized {
    flatbuffers::root_unchecked::<Serialized>(buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a size prefixed Serialized and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid size prefixed `Serialized`.
pub unsafe fn size_prefixed_root_as_serialized_unchecked(buf: &[u8]) -> Serialized {
    flatbuffers::size_prefixed_root_unchecked::<Serialized>(buf)
}
#[inline]
pub fn finish_serialized_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<Serialized<'a>>,
) {
    fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_serialized_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<Serialized<'a>>,
) {
    fbb.finish_size_prefixed(root, None);
}
