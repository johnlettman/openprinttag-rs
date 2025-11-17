//! Unified `RwLock` abstraction.
//!
//! This module re-exports a lock type depending on whether the crate
//! is compiled with the `parking_lot` feature. When using `parking_lot`,
//! locking operations never fail. When using the standard library backend,
//! this module automatically unwraps poisoned locks, treating poisoning
//! as non-fatal.
#![allow(unused_imports)]

#[cfg(feature = "parking_lot")]
pub use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};

#[cfg(not(feature = "parking_lot"))]
mod std_wrap {
    use std::sync::{
        RwLock as StdRwLock, RwLockReadGuard as StdRead, RwLockWriteGuard as StdWrite,
    };

    /// A thin wrapper around [`std::sync::RwLock`] that unwraps poison errors.
    ///
    /// This matches `parking_lot::RwLock` behavior, enabling a unified API.
    pub struct RwLock<T>(StdRwLock<T>);

    impl<T> RwLock<T> {
        pub fn new(val: T) -> Self {
            Self(StdRwLock::new(val))
        }

        #[inline]
        pub fn read(&self) -> RwLockReadGuard<'_, T> {
            RwLockReadGuard(self.0.read().unwrap())
        }

        #[inline]
        pub fn write(&self) -> RwLockWriteGuard<'_, T> {
            RwLockWriteGuard(self.0.write().unwrap())
        }
    }

    /// Wrapper for poisoned-ignored read guards.
    pub struct RwLockReadGuard<'a, T>(StdRead<'a, T>);
    impl<'a, T> std::ops::Deref for RwLockReadGuard<'a, T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &*self.0
        }
    }

    /// Wrapper for poisoned-ignored write guards.
    pub struct RwLockWriteGuard<'a, T>(StdWrite<'a, T>);
    impl<'a, T> std::ops::Deref for RwLockWriteGuard<'a, T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &*self.0
        }
    }
    impl<'a, T> std::ops::DerefMut for RwLockWriteGuard<'a, T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut *self.0
        }
    }

    pub use self::{RwLock, RwLockReadGuard, RwLockWriteGuard};
}

#[cfg(not(feature = "parking_lot"))]
pub use std_wrap::*;
