pub mod agent;
pub mod message;
pub mod proxy;

use std::sync::Arc;

use parking_lot::RwLock;

use self::proxy::{PoolReadProxy, PoolWriteProxy};
use crate::datastore::{
    batch::Batch,
    prelude::Result,
    table::proxy::{BatchReadProxy, BatchWriteProxy},
};

/// Internal trait to implement `BatchPool` without leaking `Arc<RwLock<B>>`
trait Pool<B> {
    fn new(batches: Vec<Arc<RwLock<B>>>) -> Self;
    fn get_batches(&self) -> &[Arc<RwLock<B>>];
    fn get_batches_mut(&mut self) -> &mut Vec<Arc<RwLock<B>>>;
}

/// A pool is an ordered collection of batches.
///
/// Each group of agents within a simulation run is associated to a [`Batch`] in the pool. Each
/// [`Batch`] in a pool can either be borrowed as shared ([`read_proxies()`] and
/// [`partial_read_proxies()`] returning [`PoolReadProxy`]) or mutable reference
/// ([`write_proxies()`] and [`partial_write_proxies()`] returning [`PoolReadProxy`]).
///
/// [`read_proxies()`]: Self::read_proxies
/// [`partial_read_proxies()`]: Self::partial_read_proxies
/// [`write_proxies()`]: Self::write_proxies
/// [`partial_write_proxies()`]: Self::partial_write_proxies
pub trait BatchPool<B: Batch>: Send + Sync {
    /// Creates a new pool from [`Batches`].
    ///
    /// Because of the way `BatchPools` are organized it's required that the [`Batch`]es are
    /// stored inside an [`RwLock`] behind an [`Arc`]. This is subject to change.
    fn new(batches: Vec<Arc<RwLock<B>>>) -> Self;

    /// Creates a new empty pool.
    fn empty() -> Self
    where
        Self: Sized,
    {
        Self::new(Vec::new())
    }

    /// Returns the number of [`Batch`]es inside this pool.
    fn len(&self) -> usize;

    /// Returns true if there are no [`Batch`]es in this pool.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Adds a [`Batch`] at the end of this pool.
    ///
    /// Note, that unlike in [`new()`](Self::new) this does not require the [`Batch`] to be wrapped
    /// inside of [`Arc`]`<RwLock<B>>`.
    fn push(&mut self, batch: B);

    /// Removes the [`Batch`] at position `index` within the pool, shifting all elements after it to
    /// the left.
    ///
    /// Returns the batch id of the removed [`Batch`].
    ///
    /// # Panics
    ///
    /// - If `index` is out of bounds, or
    /// - if the `Batch` is currently borrowed as a [`BatchReadProxy`] or [`BatchWriteProxy`].
    fn remove(&mut self, index: usize) -> String;

    /// Removes the [`Batch`] at position `index` within the pool and returns its id.
    ///
    /// The removed [`Batch`] is replaced by the last [`Batch`] of the pool. This does not preserve
    /// ordering, but is `O(1)`. If you need to preserve the element order, use
    /// [`remove()`](Self::remove) instead.
    ///
    /// # Panics
    ///
    /// - If `index` is out of bounds, or
    /// - if the `Batch` is currently borrowed as a [`BatchReadProxy`] or [`BatchWriteProxy`].
    fn swap_remove(&mut self, index: usize) -> String;

    /// Creates a [`PoolReadProxy`] for _all_ batches within the pool.
    ///
    /// # Errors
    ///
    /// Returns [`Error::ProxySharedLock`] if any of the [`Batch`]es is currently borrowed in a
    /// [`BatchWriteProxy`].
    ///
    /// [`Error::ProxySharedLock`]: crate::datastore::error::Error::ProxySharedLock
    fn read_proxies(&self) -> Result<PoolReadProxy<B>>;

    /// Creates a [`PoolReadProxy`] for a _selection_ of the batches within the pool, selected by
    /// the given `indices`.
    ///
    /// # Errors
    ///
    /// Returns [`Error::ProxySharedLock`] if any of the [`Batch`]es at one of the specified
    /// `indices` is currently borrowed in a [`BatchWriteProxy`].
    ///
    /// [`Error::ProxySharedLock`]: crate::datastore::error::Error::ProxySharedLock
    fn partial_read_proxies(&self, indices: &[usize]) -> Result<PoolReadProxy<B>>;

    /// Creates a [`PoolWriteProxy`] for _all_ batches within the pool.
    ///
    /// # Errors
    ///
    /// Returns [`Error::ProxyExclusiveLock`] if any of the [`Batch`]es at one of the specified
    /// `indices` is currently borrowed either as [`BatchReadProxy`] or as [`BatchWriteProxy`].
    ///
    /// [`Error::ProxyExclusiveLock`]: crate::datastore::error::Error::ProxyExclusiveLock
    fn write_proxies(&mut self) -> Result<PoolWriteProxy<B>>;

    /// Creates a [`PoolWriteProxy`] for a _selection_ of the batches within the pool, selected by
    /// the given `indices`.
    ///
    /// This can be used to create multiple mutable disjoint partitions of the pool.
    ///
    /// # Errors
    ///
    /// Returns [`Error::ProxyExclusiveLock`] if any of the [`Batch`]es at one of the specified
    /// `indices` is currently borrowed either as [`BatchReadProxy`] or as [`BatchWriteProxy`].
    ///
    /// [`Error::ProxyExclusiveLock`]: crate::datastore::error::Error::ProxyExclusiveLock
    fn partial_write_proxies(&mut self, indices: &[usize]) -> Result<PoolWriteProxy<B>>;
}

impl<P: Pool<B> + Send + Sync, B: Batch> BatchPool<B> for P {
    fn new(batches: Vec<Arc<RwLock<B>>>) -> Self {
        Pool::new(batches)
    }

    fn len(&self) -> usize {
        self.get_batches().len()
    }

    fn push(&mut self, batch: B) {
        self.get_batches_mut().push(Arc::new(RwLock::new(batch)))
    }

    fn remove(&mut self, index: usize) -> String {
        let mut batch_arc = self.get_batches_mut().remove(index);
        if let Some(rw_lock) = Arc::get_mut(&mut batch_arc) {
            // This can't deadlock as we just checked that the Arc owning this RwLock is unique
            rw_lock.write().get_batch_id().to_string()
        } else {
            panic!("Failed to remove Batch at index {index}, other Arcs to the Batch existed")
        }
    }

    fn swap_remove(&mut self, index: usize) -> String {
        let mut batch_arc = self.get_batches_mut().swap_remove(index);
        if let Some(rw_lock) = Arc::get_mut(&mut batch_arc) {
            // This can't deadlock as we just checked that the Arc owning this RwLock is unique
            rw_lock.write().get_batch_id().to_string()
        } else {
            panic!("Failed to swap remove Batch at index {index}, other Arcs to the Batch existed")
        }
    }

    fn read_proxies(&self) -> Result<PoolReadProxy<B>> {
        self.get_batches().iter().map(BatchReadProxy::new).collect()
    }

    fn partial_read_proxies(&self, indices: &[usize]) -> Result<PoolReadProxy<B>> {
        let batches = self.get_batches();
        indices
            .iter()
            .map(|i| BatchReadProxy::new(&batches[*i]))
            .collect()
    }

    fn write_proxies(&mut self) -> Result<PoolWriteProxy<B>> {
        self.get_batches()
            .iter()
            .map(BatchWriteProxy::new)
            .collect()
    }

    fn partial_write_proxies(&mut self, indices: &[usize]) -> Result<PoolWriteProxy<B>> {
        let batches = self.get_batches_mut();
        indices
            .iter()
            .map(|i| BatchWriteProxy::new(&batches[*i]))
            .collect()
    }
}
