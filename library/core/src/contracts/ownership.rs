//! Ownership assertions for use in contracts.

use crate::alloc::Layout;
use crate::mem::ManuallyDrop;

// TODO expand on these docs quite a bit and write top-level documentation.
// Just like any good documentation, include usage examples etc.

/// [`owned`] asserts exclusive ownership of memory pointed at by the given raw
/// pointer `ptr`. It returns the value pointed to by `ptr` for further use in
/// the contract. Since contracts should not affect the behaviour of the
/// function, we wrap the returned value in a [`ManuallyDrop`] for the caller's
/// convenience.
pub const fn owned<T>(ptr: *const T) -> ManuallyDrop<T> {
    let _ = ptr;
    unimplemented!()
}

/// [`owned_unaligned`] asserts exclusive ownership of memory pointed at by the
/// given raw pointer `ptr`. Unlike [owned], this function does not require
/// `ptr` to be aligned, and uses `read_unaligned` to safely read the value
/// pointed to by `ptr` if it is safe to do so (i.e. the ownership assertion
/// holds).
pub const fn owned_unaligned<T>(ptr: *const T) -> ManuallyDrop<T> {
    let _ = ptr;
    unimplemented!()
}

/// [`alloc_block`] assert the necessary (but not sufficient) permission to free
/// an allocated block of memory starting at a given memory address and of a
/// certain size.
///
/// This function asserts a single-cell [allocation](crate::ptr#allocation) at
/// address `ptr` of type `T`.
pub fn alloc_block<T>(ptr: *const T) {
    let _ = ptr;
    unimplemented!()
}

/// [`alloc_block_array`] assert the necessary (but not sufficient) permission
/// to free an allocated block of memory starting at a given memory address and
/// of a certain size.
///
/// This function asserts an [allocation](crate::ptr#allocation) with multiple
/// contiguous `T`s, laid out as an array in memory.
pub fn alloc_block_array<T>(ptr: *const T, count: usize) {
    let _ = ptr;
    let _ = count;
    unimplemented!()
}

/// [`alloc_block_array`] assert the necessary (but not sufficient) permission
/// to free an allocated block of memory starting at a given memory address and
/// of a certain size.
///
/// This function asserts an [allocation](crate::ptr#allocation) of an arbitrary
/// [`Layout`]. The type `T` is not relevant to the assertion, but is
/// parameterised for the convenience of the caller, i.e. so that any pointer
/// type is accepted without the need to do an explicit cast.
pub fn alloc_block_layout<T>(ptr: *const T, layout: Layout) {
    let _ = ptr;
    let _ = layout;
    unimplemented!()
}
