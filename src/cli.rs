use clap::Parser;

/// A command line utility for EVE Online by CCP Games.
///
/// This tool seeks to offer a convenient command line interface for interacting
/// with the Tranquility server cluster. It is designed to be approachable by
/// users who are curious and may be new to this kind of thing, but also offer
/// more advanced features for users who want to use it to compose a larger
/// script or other program.
#[derive(Parser)]
pub struct Cli {}
