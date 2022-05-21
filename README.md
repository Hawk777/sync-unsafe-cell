This is a backport of the `SyncUnsafeCell` type from the standard library. The
backport allows it to be used in older Rust versions, where it either does not
exist yet or is not stable. Its minimum supported Rust version is 1.59, though
it may work with older versions too.

A few changes have been made accordingly:
* `UnsafeCell::into_inner` is not stably `const`, so
  `SyncUnsafeCell::into_inner` is also not `const`.
* `const_mut_refs` is not stable, so `SyncUnsafeCell::get_mut` is not `const`.
* `CoerceUnsized` is not stable, so `SyncUnsafeCell` does not implement it.

Thanks to Mara Bos (m-ou-se) for [the standard library
implementation](https://github.com/rust-lang/rust/pull/95438) of which this is
a copy.
