pub mod compact;
pub mod json;
pub mod markdown;

use clap::ValueEnum;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum OutputFormat {
    Compact,
    Json,
    Markdown,
}
