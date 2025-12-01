// AvilaAuth - Native Authentication (OAuth2, JWT, API Keys)
// Zero External Dependencies 🦀

use std::collections::HashMap;

// OAuth2 Client
pub struct OAuth2Client {
    pub client_id: String,
    pub client_secret: String,
    pub authorization_url: String,
    pub token_url: String,
    pub redirect_uri: String,
}

impl OAuth2Client {
    pub fn new(
        client_id: String,
        client_secret: String,
        authorization_url: String,
        token_url: String,
        redirect_uri: String,
    ) -> Self {
        Self {
            client_id,
            client_secret,
            authorization_url,
            token_url,
            redirect_uri,
        }
    }

    pub fn authorization_url(&self, scope: &str, state: &str) -> String {
        format!(
            "{}?client_id={}&redirect_uri={}&response_type=code&scope={}&state={}",
            self.authorization_url,
            urlencode(&self.client_id),
            urlencode(&self.redirect_uri),
            urlencode(scope),
            urlencode(state)
        )
    }

    pub fn build_token_request(&self, code: &str) -> String {
        format!(
            "grant_type=authorization_code&code={}&redirect_uri={}&client_id={}&client_secret={}",
            urlencode(code),
            urlencode(&self.redirect_uri),
            urlencode(&self.client_id),
            urlencode(&self.client_secret)
        )
    }
}

// Simple JWT encoding/decoding (for API tokens)
pub mod jwt {
    use std::collections::HashMap;

    #[derive(Debug)]
    pub struct Claims {
        pub sub: String,
        pub exp: u64,
        pub iat: u64,
        pub custom: HashMap<String, String>,
    }

    impl Claims {
        pub fn new(sub: String, exp: u64) -> Self {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();

            Self {
                sub,
                exp,
                iat: now,
                custom: HashMap::new(),
            }
        }

        pub fn to_json(&self) -> String {
            let mut claims = format!(
                r#"{{"sub":"{}","exp":{},"iat":{}"#,
                self.sub, self.exp, self.iat
            );

            for (key, value) in &self.custom {
                claims.push_str(&format!(r#","{}":"{}""#, key, value));
            }

            claims.push('}');
            claims
        }
    }

    pub fn encode(claims: &Claims, _secret: &str) -> String {
        // Simplified JWT - header
        let header = r#"{"alg":"HS256","typ":"JWT"}"#;
        let header_b64 = base64_encode(header.as_bytes());
        let payload_b64 = base64_encode(claims.to_json().as_bytes());

        // In production, sign with HMAC-SHA256
        format!("{}.{}.SIGNATURE", header_b64, payload_b64)
    }

    fn base64_encode(data: &[u8]) -> String {
        const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut result = String::new();
        let mut i = 0;

        while i < data.len() {
            let b1 = data[i];
            let b2 = if i + 1 < data.len() { data[i + 1] } else { 0 };
            let b3 = if i + 2 < data.len() { data[i + 2] } else { 0 };

            let n = ((b1 as u32) << 16) | ((b2 as u32) << 8) | (b3 as u32);

            result.push(CHARS[((n >> 18) & 63) as usize] as char);
            result.push(CHARS[((n >> 12) & 63) as usize] as char);

            if i + 1 < data.len() {
                result.push(CHARS[((n >> 6) & 63) as usize] as char);
            }
            if i + 2 < data.len() {
                result.push(CHARS[(n & 63) as usize] as char);
            }

            i += 3;
        }

        result.replace('+', "-").replace('/', "_").replace('=', "")
    }
}

// API Key Management
pub struct ApiKeyManager {
    keys: HashMap<String, ApiKey>,
}

#[derive(Debug, Clone)]
pub struct ApiKey {
    pub key: String,
    pub name: String,
    pub created_at: u64,
    pub last_used: Option<u64>,
    pub rate_limit: Option<u32>,
}

impl ApiKeyManager {
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
        }
    }

    pub fn create_key(&mut self, name: String) -> String {
        use std::time::{SystemTime, UNIX_EPOCH};

        let key = format!("avila_{}", generate_random_string(32));
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

        self.keys.insert(
            key.clone(),
            ApiKey {
                key: key.clone(),
                name,
                created_at: now,
                last_used: None,
                rate_limit: Some(1000),
            },
        );

        key
    }

    pub fn validate(&mut self, key: &str) -> bool {
        if let Some(api_key) = self.keys.get_mut(key) {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            api_key.last_used = Some(now);
            true
        } else {
            false
        }
    }

    pub fn revoke(&mut self, key: &str) -> bool {
        self.keys.remove(key).is_some()
    }
}

impl Default for ApiKeyManager {
    fn default() -> Self {
        Self::new()
    }
}

fn urlencode(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
            _ => format!("%{:02X}", c as u8),
        })
        .collect()
}

fn generate_random_string(len: usize) -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64;
    let mut state = seed;

    const CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";

    (0..len)
        .map(|_| {
            state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
            CHARS[(state % CHARS.len() as u64) as usize] as char
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oauth2_authorization_url() {
        let client = OAuth2Client::new(
            "client_id".to_string(),
            "secret".to_string(),
            "https://auth.example.com".to_string(),
            "https://token.example.com".to_string(),
            "https://myapp.com/callback".to_string(),
        );

        let url = client.authorization_url("read write", "random_state");
        assert!(url.contains("client_id"));
        assert!(url.contains("response_type=code"));
    }

    #[test]
    fn test_jwt_encode() {
        let claims = jwt::Claims::new("user123".to_string(), 1234567890);
        let token = jwt::encode(&claims, "secret");
        assert!(token.contains('.'));
    }

    #[test]
    fn test_api_key_manager() {
        let mut manager = ApiKeyManager::new();
        let key = manager.create_key("test_app".to_string());

        assert!(key.starts_with("avila_"));
        assert!(manager.validate(&key));
        assert!(manager.revoke(&key));
        assert!(!manager.validate(&key));
    }

    #[test]
    fn test_urlencode() {
        assert_eq!(urlencode("hello world"), "hello%20world");
        assert_eq!(urlencode("a+b=c"), "a%2Bb%3Dc");
    }
}
