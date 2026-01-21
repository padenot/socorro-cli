use clap::{Parser, Subcommand};
use socorro_cli::{Result, SocorroClient, OutputFormat};

#[derive(Parser)]
#[command(name = "socorro-cli")]
#[command(about = "Query Mozilla's Socorro crash reporting system", long_about = None)]
struct Cli {
    #[arg(long, value_enum, default_value = "compact", global = true)]
    format: OutputFormat,

    #[arg(long, env = "SOCORRO_API_TOKEN", global = true)]
    token: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Crash {
        crash_id: String,

        #[arg(long, default_value = "10")]
        depth: usize,

        #[arg(long, help = "Output full crash data without omissions (forces JSON format)")]
        full: bool,

        #[arg(long, help = "Show stacks from all threads (useful for diagnosing deadlocks)")]
        all_threads: bool,

        #[arg(long)]
        modules: bool,
    },
    Search {
        #[arg(long)]
        signature: Option<String>,

        #[arg(long, default_value = "Firefox")]
        product: String,

        #[arg(long)]
        version: Option<String>,

        #[arg(long)]
        platform: Option<String>,

        #[arg(long, default_value = "7")]
        days: u32,

        #[arg(long, default_value = "10")]
        limit: usize,

        #[arg(long)]
        facet: Vec<String>,

        #[arg(long, default_value = "-date")]
        sort: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let client = SocorroClient::new(
        "https://crash-stats.mozilla.org/api".to_string(),
        cli.token,
    );

    match cli.command {
        Commands::Crash { crash_id, depth, full, all_threads, modules } => {
            socorro_cli::commands::crash::execute(&client, &crash_id, depth, full, all_threads, modules, cli.format)?;
        }
        Commands::Search { signature, product, version, platform, days, limit, facet, sort } => {
            socorro_cli::commands::search::execute(
                &client,
                signature,
                product,
                version,
                platform,
                days,
                limit,
                facet,
                sort,
                cli.format,
            )?;
        }
    }

    Ok(())
}
