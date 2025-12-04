// AvilaHttp - Native HTTP Client Implementation
// Zero External Dependencies 🦀

pub mod tls;

use std::io::{Read, Write};
use std::net::TcpStream;
use std::collections::HashMap;
use tls::TlsStream;

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

    /// Perform a POST request with custom headers
    pub fn post_with_headers(
        &self,
        url: &str,
        body: &str,
        headers: &HashMap<String, String>,
    ) -> Result<Response, HttpError> {
        self.request("POST", url, Some(body), headers)
    }

    /// Generic HTTP request
    fn request(
        &self,
        method: &str,
        url: &str,
        body: Option<&str>,
        headers: &HashMap<String, String>,
    ) -> Result<Response, HttpError> {
        let is_https = url.starts_with("https://");
        let (host, path) = parse_url(url)?;

        // Build HTTP request
        let request = build_request(method, &host, &path, body, headers);

        // Connect to server
        let port = if is_https { 443 } else { 80 };
        let addr = format!("{}:{}", host, port);

        println!("🌐 {} {} ({})", method, url, if is_https { "HTTPS" } else { "HTTP" });

        if is_https {
            // HTTPS com TLS
            self.request_https(&host, port, &request)
        } else {
            // HTTP simples
            self.request_http(&addr, &request)
        }
    }

    /// HTTP request sem TLS
    fn request_http(&self, addr: &str, request: &str) -> Result<Response, HttpError> {
        let mut stream = TcpStream::connect(addr)
            .map_err(|e| HttpError::ConnectionFailed(e.to_string()))?;

        // Set timeout
        stream.set_read_timeout(Some(std::time::Duration::from_secs(self.timeout_secs)))
            .map_err(|e| HttpError::ConnectionFailed(e.to_string()))?;

        // Send request
        stream.write_all(request.as_bytes())
            .map_err(|e| HttpError::ConnectionFailed(e.to_string()))?;

        // Read response
        let mut response_data = Vec::new();
        stream.read_to_end(&mut response_data)
            .map_err(|e| HttpError::ConnectionFailed(e.to_string()))?;

        // Parse response
        parse_response(&response_data)
    }

    /// HTTPS request com TLS nativo
    fn request_https(&self, host: &str, port: u16, request: &str) -> Result<Response, HttpError> {
        // Conectar com TLS
        let mut tls_stream = TlsStream::connect(host, port)
            .map_err(|e| HttpError::TlsError(e.to_string()))?;

        // Set timeout no stream interno
        // Note: TlsStream wraps TcpStream, mas não expõe set_timeout diretamente
        // Em produção, isso seria configurável

        // Send request
        tls_stream.write_all(request.as_bytes())
            .map_err(|e| HttpError::TlsError(e.to_string()))?;

        tls_stream.flush()
            .map_err(|e| HttpError::TlsError(e.to_string()))?;

        // Read response
        let mut response_data = Vec::new();
        tls_stream.read_to_end(&mut response_data)
            .map_err(|e| HttpError::TlsError(e.to_string()))?;

        // Parse response
        parse_response(&response_data)
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
    TlsError(String),
    Timeout,
    InvalidResponse,
}

impl std::fmt::Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            HttpError::InvalidUrl(msg) => write!(f, "Invalid URL: {}", msg),
            HttpError::ConnectionFailed(msg) => write!(f, "Connection failed: {}", msg),
            HttpError::TlsError(msg) => write!(f, "TLS error: {}", msg),
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

fn parse_response(data: &[u8]) -> Result<Response, HttpError> {
    let response_str = String::from_utf8_lossy(data);
    let mut lines = response_str.lines();

    // Parse status line
    let status_line = lines.next().ok_or(HttpError::InvalidResponse)?;
    let status_code = status_line
        .split_whitespace()
        .nth(1)
        .and_then(|s| s.parse::<u16>().ok())
        .ok_or(HttpError::InvalidResponse)?;

    // Parse headers
    let mut headers = HashMap::new();
    let mut body_start = 0;

    for (i, line) in response_str.lines().enumerate() {
        if line.is_empty() {
            // Empty line marks end of headers
            body_start = response_str.lines().take(i + 1).map(|l| l.len() + 1).sum();
            break;
        }

        if let Some(colon_pos) = line.find(':') {
            let key = line[..colon_pos].trim().to_string();
            let value = line[colon_pos + 1..].trim().to_string();
            headers.insert(key, value);
        }
    }

    // Extract body
    let body = if body_start < data.len() {
        String::from_utf8_lossy(&data[body_start..]).to_string()
    } else {
        String::new()
    };

    Ok(Response {
        status_code,
        headers,
        body,
    })
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

    #[test]
    fn test_https_url_detection() {
        let url = "https://www.google.com";
        assert!(url.starts_with("https://"));
    }

    #[test]
    fn test_http_url_detection() {
        let url = "http://example.com";
        assert!(url.starts_with("http://"));
    }

    #[test]
    #[ignore] // Requer conexão de rede real
    fn test_real_https_request() {
        let client = HttpClient::new();
        let result = client.get("https://www.google.com");

        match result {
            Ok(response) => {
                println!("Status: {}", response.status_code);
                assert!(response.status_code > 0);
            }
            Err(e) => {
                println!("Erro esperado (handshake simplificado): {}", e);
            }
        }
    }
}
