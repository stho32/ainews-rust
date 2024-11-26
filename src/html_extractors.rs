use scraper::{Html, Selector};
use url;

/// Extracts all links (href attributes) from the provided HTML content
/// 
/// # Arguments
/// * `content` - A string slice containing the HTML content
/// 
/// # Returns
/// A vector of strings containing all found URLs
pub fn extract_links(content: &str) -> Vec<String> {
    // Parse the HTML string
    let document = Html::parse_document(content);
    
    // Create a selector for <a> tags
    let selector = Selector::parse("a[href]").unwrap();
    
    // Extract all href attributes
    document
        .select(&selector)
        .filter_map(|element| {
            element.value().attr("href").map(String::from)
        })
        .collect()
}

/// Converts relative URLs to absolute URLs based on the source URL
/// 
/// # Arguments
/// * `base_url` - The URL from which the links were scraped
/// * `links` - A vector of links to normalize
/// 
/// # Returns
/// A vector of normalized absolute URLs
pub fn normalize_urls(base_url: &str, links: Vec<String>) -> Vec<String> {
    let base = url::Url::parse(base_url).unwrap();
    
    links.into_iter()
        .map(|link| {
            if link.starts_with("http://") || link.starts_with("https://") {
                // Already absolute URL
                link
            } else if link.starts_with('/') {
                // Relative URL starting with /
                let base_domain = format!("{}://{}", base.scheme(), base.host_str().unwrap_or(""));
                format!("{}{}", base_domain, link)
            } else {
                // Relative URL without /
                base.join(&link)
                    .map(|u| u.to_string())
                    .unwrap_or(link)
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_links() {
        let html = r#"
            <html>
                <body>
                    <a href="https://example.com">Example</a>
                    <a href="/relative/path">Relative</a>
                    <a>No href</a>
                    <a href="https://another.com">Another</a>
                </body>
            </html>
        "#;

        let links = extract_links(html);
        
        assert_eq!(links.len(), 3);
        assert!(links.contains(&"https://example.com".to_string()));
        assert!(links.contains(&"/relative/path".to_string()));
        assert!(links.contains(&"https://another.com".to_string()));
    }

    #[test]
    fn test_extract_links_empty_html() {
        let html = "<html><body></body></html>";
        let links = extract_links(html);
        assert!(links.is_empty());
    }

    #[test]
    fn test_normalize_urls() {
        let base_url = "https://example.com/page/article";
        let links = vec![
            "https://absolute.com/path".to_string(),
            "/relative/path".to_string(),
            "local/resource".to_string(),
        ];

        let normalized = normalize_urls(base_url, links);
        
        assert_eq!(normalized.len(), 3);
        assert_eq!(normalized[0], "https://absolute.com/path");
        assert_eq!(normalized[1], "https://example.com/relative/path");
        assert_eq!(normalized[2], "https://example.com/page/local/resource");
    }
}