// AvilaHttp - Native HTTP Client Implementation
// Zero External Dependencies 🦀

use std::io::{Read, Write};
use std::net::TcpStream;
use std::collections::HashMap;

#[derive(Debug)]
pub struct HttpClient {
    timeout_secs: u64,
}

impl HttpClient {
    pub fn new() -> Self {
        Self { timeout_secs: 30 }
    }

    pub fn with_timeout(mut self, secs: u64) -> Self {
        self.timeout_secs = secs;
        self
    }

    /// Perform a GET request
    pub fn get(&self, url: &str) -> Result<Response, HttpError> {
        self.request("GET", url, None, &HashMap::new())
    }

    /// Perform a POST request
    pub fn post(&self, url: &str, body: &str) -> Result<Response, HttpError> {
        self.request("POST", url, Some(body), &HashMap::new())
    }

    /// Generic HTTP request
    fn request(
        &self,
        method: &str,
        url: &str,
        body: Option<&str>,
        headers: &HashMap<String, String>,
    ) -> Result<Response, HttpError> {
        // Parse URL
        let (host, path) = parse_url(url)?;

        // Build HTTP request
        let request = build_request(method, &host, &path, body, headers);

        // TODO: Implement actual TCP connection and HTTP protocol
        println!("🌐 {} {}", method, url);

        Ok(Response {
            status_code: 200,
            headers: HashMap::new(),
            body: String::new(),
        })
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct Response {
    pub status_code: u16,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Response {
    pub fn is_success(&self) -> bool {
        self.status_code >= 200 && self.status_code < 300
    }

    pub fn text(&self) -> &str {
        &self.body
    }
}

#[derive(Debug)]
pub enum HttpError {
    InvalidUrl(String),
    ConnectionFailed(String),
    Timeout,
    InvalidResponse,
}

impl std::fmt::Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            HttpError::InvalidUrl(msg) => write!(f, "Invalid URL: {}", msg),
            HttpError::ConnectionFailed(msg) => write!(f, "Connection failed: {}", msg),
            HttpError::Timeout => write!(f, "Request timeout"),
            HttpError::InvalidResponse => write!(f, "Invalid response"),
        }
    }
}

impl std::error::Error for HttpError {}

fn parse_url(url: &str) -> Result<(String, String), HttpError> {
    // Simple URL parser
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err(HttpError::InvalidUrl("Missing protocol".to_string()));
    }

    let url = url.trim_start_matches("http://").trim_start_matches("https://");

    if let Some(pos) = url.find('/') {
        let host = url[..pos].to_string();
        let path = url[pos..].to_string();
        Ok((host, path))
    } else {
        Ok((url.to_string(), "/".to_string()))
    }
}

fn build_request(
    method: &str,
    host: &str,
    path: &str,
    body: Option<&str>,
    headers: &HashMap<String, String>,
) -> String {
    let mut request = format!("{} {} HTTP/1.1\r\n", method, path);
    request.push_str(&format!("Host: {}\r\n", host));
    request.push_str("User-Agent: AvilaHttp/0.1.0\r\n");
    request.push_str("Accept: */*\r\n");

    for (key, value) in headers {
        request.push_str(&format!("{}: {}\r\n", key, value));
    }

    if let Some(body) = body {
        request.push_str(&format!("Content-Length: {}\r\n", body.len()));
        request.push_str("\r\n");
        request.push_str(body);
    } else {
        request.push_str("\r\n");
    }

    request
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_parsing() {
        let (host, path) = parse_url("https://example.com/path/to/resource").unwrap();
        assert_eq!(host, "example.com");
        assert_eq!(path, "/path/to/resource");
    }

    #[test]
    fn test_client_creation() {
        let client = HttpClient::new();
        assert_eq!(client.timeout_secs, 30);
    }
}
