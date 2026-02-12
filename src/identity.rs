//! Identity file loading (SOUL.md, IDENTITY.md, USER.md).

pub mod files;

pub use files::{Identity, Prompts, scaffold_default_prompts, scaffold_identity_files};
