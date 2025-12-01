// AvilaJson - Native JSON Implementation
// Zero External Dependencies 🦀

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

impl JsonValue {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            JsonValue::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            JsonValue::Number(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            JsonValue::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&Vec<JsonValue>> {
        match self {
            JsonValue::Array(arr) => Some(arr),
            _ => None,
        }
    }

    pub fn as_object(&self) -> Option<&HashMap<String, JsonValue>> {
        match self {
            JsonValue::Object(obj) => Some(obj),
            _ => None,
        }
    }

    /// Serialize to JSON string
    pub fn to_string(&self) -> String {
        match self {
            JsonValue::Null => "null".to_string(),
            JsonValue::Bool(b) => b.to_string(),
            JsonValue::Number(n) => n.to_string(),
            JsonValue::String(s) => format!("\"{}\"", escape_string(s)),
            JsonValue::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| v.to_string()).collect();
                format!("[{}]", items.join(","))
            }
            JsonValue::Object(obj) => {
                let items: Vec<String> = obj
                    .iter()
                    .map(|(k, v)| format!("\"{}\":{}", escape_string(k), v.to_string()))
                    .collect();
                format!("{{{}}}", items.join(","))
            }
        }
    }
}

fn escape_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

pub struct JsonParser {
    input: Vec<char>,
    pos: usize,
}

impl JsonParser {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    pub fn parse(&mut self) -> Result<JsonValue, JsonError> {
        self.skip_whitespace();
        self.parse_value()
    }

    fn parse_value(&mut self) -> Result<JsonValue, JsonError> {
        self.skip_whitespace();

        if self.pos >= self.input.len() {
            return Err(JsonError::UnexpectedEnd);
        }

        match self.current_char() {
            '{' => self.parse_object(),
            '[' => self.parse_array(),
            '"' => self.parse_string(),
            't' | 'f' => self.parse_bool(),
            'n' => self.parse_null(),
            c if c.is_numeric() || c == '-' => self.parse_number(),
            _ => Err(JsonError::InvalidValue),
        }
    }

    fn parse_object(&mut self) -> Result<JsonValue, JsonError> {
        let mut obj = HashMap::new();
        self.pos += 1; // skip '{'
        self.skip_whitespace();

        if self.current_char() == '}' {
            self.pos += 1;
            return Ok(JsonValue::Object(obj));
        }

        loop {
            self.skip_whitespace();

            // Parse key
            let key = match self.parse_string()? {
                JsonValue::String(s) => s,
                _ => return Err(JsonError::InvalidKey),
            };

            self.skip_whitespace();
            if self.current_char() != ':' {
                return Err(JsonError::ExpectedColon);
            }
            self.pos += 1;

            // Parse value
            let value = self.parse_value()?;
            obj.insert(key, value);

            self.skip_whitespace();
            match self.current_char() {
                ',' => {
                    self.pos += 1;
                    continue;
                }
                '}' => {
                    self.pos += 1;
                    break;
                }
                _ => return Err(JsonError::InvalidObject),
            }
        }

        Ok(JsonValue::Object(obj))
    }

    fn parse_array(&mut self) -> Result<JsonValue, JsonError> {
        let mut arr = Vec::new();
        self.pos += 1; // skip '['
        self.skip_whitespace();

        if self.current_char() == ']' {
            self.pos += 1;
            return Ok(JsonValue::Array(arr));
        }

        loop {
            arr.push(self.parse_value()?);
            self.skip_whitespace();

            match self.current_char() {
                ',' => {
                    self.pos += 1;
                    continue;
                }
                ']' => {
                    self.pos += 1;
                    break;
                }
                _ => return Err(JsonError::InvalidArray),
            }
        }

        Ok(JsonValue::Array(arr))
    }

    fn parse_string(&mut self) -> Result<JsonValue, JsonError> {
        self.pos += 1; // skip opening quote
        let mut result = String::new();

        while self.pos < self.input.len() {
            match self.current_char() {
                '"' => {
                    self.pos += 1;
                    return Ok(JsonValue::String(result));
                }
                '\\' => {
                    self.pos += 1;
                    if self.pos >= self.input.len() {
                        return Err(JsonError::UnexpectedEnd);
                    }
                    result.push(self.current_char());
                    self.pos += 1;
                }
                c => {
                    result.push(c);
                    self.pos += 1;
                }
            }
        }

        Err(JsonError::UnexpectedEnd)
    }

    fn parse_number(&mut self) -> Result<JsonValue, JsonError> {
        let start = self.pos;

        while self.pos < self.input.len() {
            let c = self.current_char();
            if !c.is_numeric() && c != '.' && c != '-' && c != 'e' && c != 'E' && c != '+' {
                break;
            }
            self.pos += 1;
        }

        let num_str: String = self.input[start..self.pos].iter().collect();
        num_str
            .parse::<f64>()
            .map(JsonValue::Number)
            .map_err(|_| JsonError::InvalidNumber)
    }

    fn parse_bool(&mut self) -> Result<JsonValue, JsonError> {
        if self.matches_str("true") {
            self.pos += 4;
            Ok(JsonValue::Bool(true))
        } else if self.matches_str("false") {
            self.pos += 5;
            Ok(JsonValue::Bool(false))
        } else {
            Err(JsonError::InvalidValue)
        }
    }

    fn parse_null(&mut self) -> Result<JsonValue, JsonError> {
        if self.matches_str("null") {
            self.pos += 4;
            Ok(JsonValue::Null)
        } else {
            Err(JsonError::InvalidValue)
        }
    }

    fn current_char(&self) -> char {
        self.input[self.pos]
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.input.len() && self.current_char().is_whitespace() {
            self.pos += 1;
        }
    }

    fn matches_str(&self, s: &str) -> bool {
        let chars: Vec<char> = s.chars().collect();
        if self.pos + chars.len() > self.input.len() {
            return false;
        }
        &self.input[self.pos..self.pos + chars.len()] == &chars[..]
    }
}

#[derive(Debug)]
pub enum JsonError {
    UnexpectedEnd,
    InvalidValue,
    InvalidKey,
    InvalidObject,
    InvalidArray,
    InvalidNumber,
    ExpectedColon,
}

pub fn parse(input: &str) -> Result<JsonValue, JsonError> {
    JsonParser::new(input).parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_string() {
        let json = r#""hello world""#;
        let value = parse(json).unwrap();
        assert_eq!(value, JsonValue::String("hello world".to_string()));
    }

    #[test]
    fn test_parse_number() {
        let json = "42.5";
        let value = parse(json).unwrap();
        assert_eq!(value, JsonValue::Number(42.5));
    }

    #[test]
    fn test_parse_bool() {
        assert_eq!(parse("true").unwrap(), JsonValue::Bool(true));
        assert_eq!(parse("false").unwrap(), JsonValue::Bool(false));
    }

    #[test]
    fn test_parse_null() {
        assert_eq!(parse("null").unwrap(), JsonValue::Null);
    }

    #[test]
    fn test_parse_array() {
        let json = r#"[1, 2, 3]"#;
        let value = parse(json).unwrap();
        assert!(matches!(value, JsonValue::Array(_)));
    }

    #[test]
    fn test_parse_object() {
        let json = r#"{"name": "Dubai", "country": "UAE"}"#;
        let value = parse(json).unwrap();
        assert!(matches!(value, JsonValue::Object(_)));
    }

    #[test]
    fn test_serialize() {
        let value = JsonValue::Object({
            let mut map = HashMap::new();
            map.insert("test".to_string(), JsonValue::Number(123.0));
            map
        });
        let json = value.to_string();
        assert!(json.contains("test"));
    }
}
