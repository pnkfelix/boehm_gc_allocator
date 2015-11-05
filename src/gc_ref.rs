use core::intrinsics::{needs_drop};

use super::{FinalizerMode, gc_allocate, register_finalizer};
use core::{mem, ptr};
use core::nonzero::NonZero;

pub const GC: GcHeapSingleton =
    GcHeapSingleton { _force_singleton: () };

#[derive(Copy, Clone)]
pub struct GcHeapSingleton {
    _force_singleton: (),
}

/// A smart-pointer for GC-allocated references.
///
/// There are three reasons why we need this (and cannot just rely on
/// using `&'a T` for some `'a` when passing around references to GC
/// data):
///
///   1. We need something to use with the placement protocols
///
///   2. A `&'a T` reference represents a *promise* that the
///      referenced `T` will remain alive, even if that reference
///      itself goes away before the end of `'a`. Passing around
///      a `&'a T` alone will expose a problem if a client then
///      casts it to `usize`, blames games with its underlying
///      address, and later (but before the end of `'a`) reconstructs
///      this original `&'a T`.
///
///   3. We need a way to register a finalizer for non-Copy `T`.
#[allow(raw_pointer_derive)]
#[derive(Copy, Clone)]
pub struct Gc<T>(NonZero<*const T>);

extern "C" fn finalize<T>(obj: *mut u8, _api_extra_nulled: *mut u8) {
    unsafe {
        let t = obj as *mut T as *const T;
        let t: T = ptr::read(t);
        drop(t);
    }
}

impl<T> Gc<T> {
    pub fn new(t: T) -> Gc<T> {
        let size = mem::size_of::<T>();
        // First we try to allocate the exact size necessary to hold T ...
        let mut ptr: *mut T = gc_allocate(size) as *mut T;
        // ... but its possible that it won't have the alignment we need.
        let align = mem::align_of::<T>();
        if 0 == ((ptr as usize) & (align - 1)) {
            // Ah, we got lucky; we can use the `ptr` we got.
        } else {
            // We were not so lucky. Re-do the allocation, but this
            // time add enough buffer that we can be guaranteed enough
            // room to adjust the result as needed.
            let new_ptr = gc_allocate(size + align);
            let delta = align - ((new_ptr as usize) & (align - 1));
            ptr = (new_ptr as usize + delta) as *mut T;
        }

        assert!(0 == ((ptr as usize) & (align - 1)));

        unsafe {
            ptr::write(ptr, t);

            // Almost done. But we still need to register a finalizer
            // for any `T` that is non-copy.

            if needs_drop::<T>() {
                // FIXME: it is not clear to pnkfelix from the
                // documentation in gc.h whether one can actually pass
                // an interior address (as we may be doing here, if
                // original was not aligned) or if we need to use the
                // original gc_allocate return value and then adjust
                // it in the finalization routine itself.
                //
                // For now I will assume that interior pointers will
                // work, but I definitely need to test this case.
                register_finalizer(
                    ptr as *mut u8,
                    finalize::<T>,
                    ptr::null_mut(),
                    // this is the conservative option; perhaps
                    // NoOrder will end up being necessary, but I
                    // think that decision is probably better left put
                    // at a higher level (perhaps via distinct Gc<T>
                    // smart pointers, or maybe an defaulted type
                    // parameter on it).
                    FinalizerMode::Standard);
            }

            assert!(ptr != ptr::null_mut());
            return Gc(NonZero::new(ptr as *const T));

        }

    }
}
