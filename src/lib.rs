//! # EOCLU
//!
//! The library for EOCLU, a command line utility for EVE Online by CCP Games.
//!
//! This crate is intended for use by the EOCLU binary.

use clap::Parser;

use cli::Cli;

mod cli;

/// Runs EOCLU.
///
/// This is the entrypoint for EOCLU and is intended to be called by `main` in
/// the EOCLU binary crate.
///
/// # Examples
///
/// ```
/// eoclu::run();
/// ```
pub fn run() {
    Cli::parse();
}
