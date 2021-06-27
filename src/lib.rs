#![no_std]

use crate::ptr::Pointer;
use crate::result::Result;

use core::convert::From;
use core::ops::DerefMut;
use core::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};

/// An alias type for an array of `Descriptor`s.
pub type DescriptorRegion = [Descriptor];

pub trait Ralloc: DerefMut<Target = SuperBlockRegion> {
    /// create or remap heap in `path`, `size` of `SuperBlockRegion`, return `true` if heap files
    /// exist.
    fn init(&self, path: &str, size: usize) -> bool;
    /// issue offline GC and reconstruction if dirty, return `true` if GC occurs, otherwise `false`.
    fn recover(&self) -> bool;
    /// give back cached blocks and flush heap by setting heap as not dirty.
    fn close(&self);
    /// allocate `size` in bytes, return address of allocated block.
    fn malloc(&self, size: usize) -> Result<usize>;
    /// deallocate the pointer which cannot be done by the `Drop` trait.
    #[inline]
    fn free(&mut self, ptr: Pointer<u8>);
    /// set the pointer to be root `i`.
    fn set_root(&mut self, ptr: Pointer<u8>, i: usize);
    /// update root type info, return address of root `i`.
    fn get_root(&self, i: usize) -> Result<usize>;
}

/// The superblock region holds the actual data of the heap. After the initial `size` and `used`
/// fields, it holds an array of `SuperBlock`s.
pub struct SuperBlockRegion {
    size: usize,
    used: usize,
    superblocks: [SuperBlock],
}

/// A Treiber stack of `Descriptor`s, linked through their next free node fields.
/// Given the 1-to-1 coorespondence between superblocks and descriptors, Ralloc finds a free
/// superblock easily given a pointer to its descriptor.
pub struct SuperBlockFreeList {
    top: AtomicPtr<Descriptor>,
}

pub struct PartialList {
    top: AtomicPtr<Descriptor>,
}

/// A descriptor describes a `SuperBlock`, and is the locus of synchronization on that superblock.
/// Each descriptor is 32B in size, padded out to a 64B cache line. Within a given heap, the i-th
/// descriptor corresponds to the i-th superblock, allowing either to be found using simple bit
/// manipulation given the location of the other.
pub struct Descriptor {
    /// indicates the index of the first block on the block free list, the number of free blocks,
    /// and the state of the corresponding superblock.
    anchor: AtomicUsize,
    /// whether the superblock is entirely free, partially allocated, or fully allocated.
    state: BlockState,
    /// the field indicates which of several standard sizes is being used for blocks in the
    /// superblock, or `0` is the superblock comprises a single block that is larger than any
    /// standard size.
    size_class: usize,
    /// the field indicates the size of each block in this superblock, either fetched from a
    /// `size_class` or the actual size of the large block set during allocation.
    block_size: usize,
    next_free: *mut SuperBlock,
    next_partial: *mut SuperBlock,
}

/// Imply the allocation state of the `SuperBlock`.
pub enum BlockState {
    Empty,
    Partial,
    Full,
}

/// Each superblock is 64KB in size. If a block is free (not in use), its first word contains a
/// pointer to the next free block, otherwise in the same superblock.
pub struct SuperBlock {
    /// the size of this block, in bytes.
    size: usize,
    /// the pointer to the start of this block.
    ptr: Pointer<u8>,
    /// the pointer to the next superblock.
    pub next: Pointer<SuperBlock>,
}

impl From<SuperBlock> for Pointer<u8> {
    fn from(from: SuperBlock) -> Pointer<u8> {
        from.ptr
    }
}

/// A `Result` type with string error messages
pub mod result {
    pub type Result<T> = core::result::Result<T, &'static str>;
}

/// A `NonNull<T>` raw pointer wrapper around `*mut T`.
pub mod ptr {
    #[derive(PartialEq, Eq, Debug, Clone)]
    pub struct Pointer<T> {
        /// the internal pointer.
        ptr: core::ptr::NonNull<T>,
        /// this indicates the pointer owning `T`.
        _phantom: core::marker::PhantomData<T>,
    }
}
