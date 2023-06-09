#![no_std]

//! This is a backport of the `SyncUnsafeCell` type from the standard library. The backport allows
//! it to be used in older Rust versions, where it either does not exist yet or is not stable. Its
//! minimum supported Rust version is 1.59, though
//! it may work with older versions too.
//!
//! A few changes have been made accordingly:
//! * `UnsafeCell::into_inner` is not stably `const`, so `SyncUnsafeCell::into_inner` is also not
//!   `const`.
//! * `const_mut_refs` is not stable, so `SyncUnsafeCell::get_mut` is not `const`.
//! * `CoerceUnsized` is not stable, so `SyncUnsafeCell` does not implement it.
//!
//! Thanks to Mara Bos (m-ou-se) for [the standard library
//! implementation](https://github.com/rust-lang/rust/pull/95438) of which this is a copy.

use core::cell::UnsafeCell;

/// [`UnsafeCell`], but [`Sync`].
///
/// This is just an `UnsafeCell`, except it implements `Sync` if `T` implements `Sync`.
///
/// `UnsafeCell` doesn't implement `Sync`, to prevent accidental mis-use. You can use
/// `SyncUnsafeCell` instead of `UnsafeCell` to allow it to be shared between threads, if that's
/// intentional. Providing proper synchronization is still the task of the user, making this type
/// just as unsafe to use.
///
/// See [`UnsafeCell`] for details.
#[repr(transparent)]
pub struct SyncUnsafeCell<T: ?Sized> {
	value: UnsafeCell<T>,
}

unsafe impl<T: ?Sized + Sync> Sync for SyncUnsafeCell<T> {}

impl<T> SyncUnsafeCell<T> {
	/// Constructs a new instance of `SyncUnsafeCell` which will wrap the specified value.
	#[inline]
	pub const fn new(value: T) -> Self {
		Self {
			value: UnsafeCell::new(value),
		}
	}

	/// Unwraps the value.
	#[inline]
	pub fn into_inner(self) -> T {
		self.value.into_inner()
	}
}

impl<T: ?Sized> SyncUnsafeCell<T> {
	/// Gets a mutable pointer to the wrapped value.
	///
	/// This can be cast to a pointer of any kind. Ensure that the access is unique (no active
	/// references, mutable or not) when casting to `&mut T`, and ensure that there are no
	/// mutations or mutable aliases going on when casting to `&T`
	#[inline]
	pub const fn get(&self) -> *mut T {
		self.value.get()
	}

	/// Returns a mutable reference to the underlying data.
	///
	/// This call borrows the `SyncUnsafeCell` mutably (at compile-time) which guarantees that we
	/// possess the only reference.
	#[inline]
	pub fn get_mut(&mut self) -> &mut T {
		self.value.get_mut()
	}

	/// Gets a mutable pointer to the wrapped value.
	///
	/// See [`UnsafeCell::get`] for details.
	#[inline]
	pub const fn raw_get(this: *const Self) -> *mut T {
		// We can just cast the pointer from `SyncUnsafeCell<T>` to `T` because
		// of #[repr(transparent)] on both SyncUnsafeCell and UnsafeCell.
		// See UnsafeCell::raw_get.
		this as *const T as *mut T
	}
}

impl<T: Default> Default for SyncUnsafeCell<T> {
	/// Creates an `SyncUnsafeCell`, with the `Default` value for T.
	fn default() -> SyncUnsafeCell<T> {
		SyncUnsafeCell::new(Default::default())
	}
}

impl<T> From<T> for SyncUnsafeCell<T> {
	/// Creates a new `SyncUnsafeCell<T>` containing the given value.
	fn from(t: T) -> SyncUnsafeCell<T> {
		SyncUnsafeCell::new(t)
	}
}
