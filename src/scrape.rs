use crate::schema::Domain;
use site_icons::{Icon, SiteIcons};

pub async fn scrape_one(
    domain: Domain,
    best_matches_only: Option<bool>,
) -> Result<Vec<Icon>, Box<dyn std::error::Error>> {
    let best_matches_only = best_matches_only.unwrap_or(false);

    let uri = if domain.domain.starts_with("http") {
        domain.domain.to_string()
    } else {
        format!("https://{domain_str}", domain_str = domain.domain)
    };

    let mut icons = SiteIcons::new();

    let entries: Vec<Icon> = icons.load_website(uri, best_matches_only).await?;

    Ok(entries)
}
