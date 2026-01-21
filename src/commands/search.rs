use crate::{Result, SocorroClient};
use crate::models::SearchParams;
use crate::output::{OutputFormat, compact, json, markdown};

pub fn execute(
    client: &SocorroClient,
    signature: Option<String>,
    product: String,
    version: Option<String>,
    platform: Option<String>,
    days: u32,
    limit: usize,
    facets: Vec<String>,
    sort: String,
    format: OutputFormat,
) -> Result<()> {
    let params = SearchParams {
        signature,
        product,
        version,
        platform,
        days,
        limit,
        facets,
        sort,
    };

    let response = client.search(params)?;

    let output = match format {
        OutputFormat::Compact => compact::format_search(&response),
        OutputFormat::Json => json::format_search(&response)?,
        OutputFormat::Markdown => markdown::format_search(&response),
    };

    print!("{}", output);
    Ok(())
}
