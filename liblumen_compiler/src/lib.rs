pub mod argparser;
mod compiler;
mod driver;
mod commands;

pub use self::driver::{run_compiler, run_compiler_with_emitter};

use clap::crate_version;

pub const LUMEN_RELEASE: &'static str = crate_version!();
pub const LUMEN_COMMIT_HASH: &'static str = env!("LUMEN_COMMIT_HASH");
pub const LUMEN_COMMIT_DATE: &'static str = env!("LUMEN_COMMIT_DATE");
