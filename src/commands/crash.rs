use crate::{Result, SocorroClient};
use crate::output::{OutputFormat, compact, json, markdown};

fn extract_crash_id(input: &str) -> &str {
    if input.starts_with("http://") || input.starts_with("https://") {
        input.rsplit('/').next().unwrap_or(input)
    } else {
        input
    }
}

pub fn execute(
    client: &SocorroClient,
    crash_id: &str,
    depth: usize,
    full: bool,
    all_threads: bool,
    _modules: bool,
    format: OutputFormat,
) -> Result<()> {
    let crash_id = extract_crash_id(crash_id);
    let crash = client.get_crash(crash_id)?;

    let output = if full {
        json::format_crash(&crash)?
    } else {
        match format {
            OutputFormat::Compact => {
                let summary = crash.to_summary(depth, all_threads);
                compact::format_crash(&summary)
            }
            OutputFormat::Json => json::format_crash(&crash)?,
            OutputFormat::Markdown => {
                let summary = crash.to_summary(depth, all_threads);
                markdown::format_crash(&summary)
            }
        }
    };

    print!("{}", output);
    Ok(())
}
