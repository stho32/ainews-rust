use reqwest::blocking::Client;

/// Fetches content from a website with proper headers and error handling
/// 
/// Returns a tuple of (Option<String>, Option<String>) where:
/// - First element is the website content if successful, None if failed
/// - Second element is the error message if failed, None if successful
pub fn get_website_content(url: &str) -> (Option<String>, Option<String>) {
    println!("[DEBUG] Attempting to fetch content from {}", url);

    let client = Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();
    let response = client
        .get(url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36")
        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7")
        .header("Accept-Language", "de-DE,de;q=0.9,en-US;q=0.8,en;q=0.7")
        .header("Cache-Control", "max-age=0")
        .header("DNT", "1")
        .header("Connection", "keep-alive")
        .header("Upgrade-Insecure-Requests", "1")
        .header("Sec-Fetch-Dest", "document")
        .header("Sec-Fetch-Mode", "navigate")
        .header("Sec-Fetch-Site", "none")
        .header("Sec-Fetch-User", "?1")
        .header("Sec-Ch-Ua", "\"Chromium\";v=\"122\", \"Not(A:Brand\";v=\"24\", \"Google Chrome\";v=\"122\"")
        .header("Sec-Ch-Ua-Mobile", "?0")
        .header("Sec-Ch-Ua-Platform", "\"Windows\"")
        .timeout(std::time::Duration::from_secs(30))
        .send();

    match response {
        Ok(res) => {
            if res.status().as_u16() != 200 {
                let error_msg = format!("HTTP {}", res.status());
                println!("[DEBUG] Error fetching the website {}: {}", url, error_msg);
                (None, Some(error_msg))
            } else {
                match res.text() {
                    Ok(text) => {
                        println!("[DEBUG] Successfully fetched content from {}", url);
                        (Some(text), None)
                    }
                    Err(e) => {
                        let error_msg = e.to_string();
                        println!("[DEBUG] Error reading content from {}: {}", url, error_msg);
                        (None, Some(error_msg))
                    }
                }
            }
        }
        Err(e) => {
            let error_msg = e.to_string();
            println!("[DEBUG] Error fetching the website {}: {}", url, error_msg);
            (None, Some(error_msg))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_website_content_success() {
        let (content, error) = get_website_content("https://www.rust-lang.org");
        assert!(content.is_some());
        assert!(error.is_none());
    }

    #[test]
    fn test_get_website_content_error() {
        let (content, error) = get_website_content("https://this-does-not-exist.example.com");
        assert!(content.is_none());
        assert!(error.is_some());
    }
}
