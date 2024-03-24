pub mod cli;

pub use cli::Arguments;

pub mod prelude {
    pub use super::Arguments;
    pub use clap::Parser;
}
