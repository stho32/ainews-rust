use scraper::{Html, Selector};

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
}