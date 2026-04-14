#![doc = include_str!("../README.md")]

// ============================================================================
// opeyml: A declarative, zero-overhead YAML surgery DSL
// ============================================================================

#[doc(hidden)]
pub use serde_yaml::{self, Mapping, Sequence, Value};

/// Errors encountered during surgical operations
#[derive(Debug, Clone)]
pub enum Error {
    PathNotFound(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::PathNotFound(path) => write!(f, "Path '{}' not found", path),
        }
    }
}

impl std::error::Error for Error {}

// Load internal engine
#[macro_use]
mod core;

// ============================================================================
// API Modules (Semantic Boundaries)
// ============================================================================

/// Mode A: Strict
/// The Surgeon: Operates precisely on existing anatomy.
pub mod strict {
    #[doc(inline)]
    pub use crate::_biopsy as biopsy;
    #[doc(inline)]
    pub use crate::_excise as excise;
    #[doc(inline)]
    pub use crate::_incise as incise;
    #[doc(hidden)]
    pub use crate::serde_yaml::{self, Mapping, Sequence, Value};
}

/// Mode B: Genesis
/// The Creator: Cultivates and grows new paths.
pub mod genesis {
    #[doc(inline)]
    pub use crate::_acquire as acquire;
    #[doc(inline)]
    pub use crate::_force_suture as force_suture;
    #[doc(inline)]
    pub use crate::_graft as graft;
    #[doc(inline)]
    pub use crate::_implant as implant;
    #[doc(inline)]
    pub use crate::_mesh as mesh;
    #[doc(inline)]
    pub use crate::_suture as suture;
    #[doc(hidden)]
    pub use crate::serde_yaml::{self, Mapping, Sequence, Value};
}

pub mod prelude;
