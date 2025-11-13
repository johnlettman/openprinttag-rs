//! Context system for schema loading, caching, and contextual deserialization.
//!
//! # Overview
//!
//! This module defines three main types used during YAML schema resolution:
//!
//! - [`Context`]: Owns a loader and registry. Knows how to load schemas from
//!   disk/URL.
//! - [`SharedContext`]: [`Arc<Context>`][Context] for inexpensive cloning and
//!   sharing.
//! - [`LocalContext`]: A borrowed, transient contextual frame that adds:
//!     * current schema name
//!     * optional parent name
//!     * optional documentation string
//!     * optional name remapping
//!
//! Together these provide a layered environment used during deserialization to
//! resolve nested schemas recursively, cache results, and allow contextual
//! operations such as inherited `name_map` or debug descriptions.

mod context;
mod local_context;
mod shared_context;
pub mod registry;

pub use context::*;
pub use local_context::*;
pub use shared_context::*;
