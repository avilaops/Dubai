// AvilaParser - Native HTML/XML Parser
// Zero External Dependencies 🦀

#[derive(Debug, Clone)]
pub struct Element {
    pub tag: String,
    pub attributes: Vec<(String, String)>,
    pub children: Vec<Node>,
}

#[derive(Debug, Clone)]
pub enum Node {
    Element(Element),
    Text(String),
}

impl Element {
    pub fn new(tag: String) -> Self {
        Self {
            tag,
            attributes: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn get_attribute(&self, name: &str) -> Option<&str> {
        self.attributes
            .iter()
            .find(|(k, _)| k == name)
            .map(|(_, v)| v.as_str())
    }

    pub fn find_by_class(&self, class_name: &str) -> Vec<&Element> {
        let mut results = Vec::new();
        self.find_by_class_recursive(class_name, &mut results);
        results
    }

    fn find_by_class_recursive<'a>(&'a self, class_name: &str, results: &mut Vec<&'a Element>) {
        if let Some(class) = self.get_attribute("class") {
            if class.split_whitespace().any(|c| c == class_name) {
                results.push(self);
            }
        }

        for child in &self.children {
            if let Node::Element(elem) = child {
                elem.find_by_class_recursive(class_name, results);
            }
        }
    }

    pub fn find_by_tag(&self, tag_name: &str) -> Vec<&Element> {
        let mut results = Vec::new();
        self.find_by_tag_recursive(tag_name, &mut results);
        results
    }

    fn find_by_tag_recursive<'a>(&'a self, tag_name: &str, results: &mut Vec<&'a Element>) {
        if self.tag == tag_name {
            results.push(self);
        }

        for child in &self.children {
            if let Node::Element(elem) = child {
                elem.find_by_tag_recursive(tag_name, results);
            }
        }
    }

    pub fn text_content(&self) -> String {
        let mut text = String::new();
        self.collect_text(&mut text);
        text
    }

    fn collect_text(&self, text: &mut String) {
        for child in &self.children {
            match child {
                Node::Text(t) => text.push_str(t),
                Node::Element(elem) => elem.collect_text(text),
            }
        }
    }
}

pub struct HtmlParser {
    input: Vec<char>,
    pos: usize,
}

impl HtmlParser {
    pub fn new(html: &str) -> Self {
        Self {
            input: html.chars().collect(),
            pos: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Element, ParseError> {
        self.skip_whitespace();
        self.parse_element()
    }

    fn parse_element(&mut self) -> Result<Element, ParseError> {
        // Skip to '<'
        while self.pos < self.input.len() && self.current_char() != '<' {
            self.pos += 1;
        }

        if self.pos >= self.input.len() {
            return Err(ParseError::UnexpectedEnd);
        }

        self.pos += 1; // skip '<'

        // Parse tag name
        let tag = self.parse_tag_name()?;
        let mut element = Element::new(tag.clone());

        // Parse attributes
        loop {
            self.skip_whitespace();

            if self.pos >= self.input.len() {
                return Err(ParseError::UnexpectedEnd);
            }

            if self.current_char() == '>' {
                self.pos += 1;
                break;
            }

            if self.current_char() == '/' {
                self.pos += 1;
                if self.current_char() == '>' {
                    self.pos += 1;
                    return Ok(element); // Self-closing tag
                }
            }

            let (name, value) = self.parse_attribute()?;
            element.attributes.push((name, value));
        }

        // Parse children (simplified)
        // TODO: Implement proper child parsing

        Ok(element)
    }

    fn parse_tag_name(&mut self) -> Result<String, ParseError> {
        let mut name = String::new();

        while self.pos < self.input.len() {
            let c = self.current_char();
            if c.is_whitespace() || c == '>' || c == '/' {
                break;
            }
            name.push(c);
            self.pos += 1;
        }

        if name.is_empty() {
            Err(ParseError::InvalidTag)
        } else {
            Ok(name)
        }
    }

    fn parse_attribute(&mut self) -> Result<(String, String), ParseError> {
        let mut name = String::new();

        // Parse attribute name
        while self.pos < self.input.len() {
            let c = self.current_char();
            if c == '=' || c.is_whitespace() {
                break;
            }
            name.push(c);
            self.pos += 1;
        }

        self.skip_whitespace();

        if self.current_char() != '=' {
            return Ok((name, String::new()));
        }

        self.pos += 1; // skip '='
        self.skip_whitespace();

        // Parse attribute value
        let quote = self.current_char();
        if quote != '"' && quote != '\'' {
            return Err(ParseError::InvalidAttribute);
        }

        self.pos += 1; // skip opening quote
        let mut value = String::new();

        while self.pos < self.input.len() {
            if self.current_char() == quote {
                self.pos += 1;
                break;
            }
            value.push(self.current_char());
            self.pos += 1;
        }

        Ok((name, value))
    }

    fn current_char(&self) -> char {
        self.input[self.pos]
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.input.len() && self.current_char().is_whitespace() {
            self.pos += 1;
        }
    }
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedEnd,
    InvalidTag,
    InvalidAttribute,
}

pub fn parse_html(html: &str) -> Result<Element, ParseError> {
    HtmlParser::new(html).parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_tag() {
        let html = r#"<div class="test">Content</div>"#;
        let result = parse_html(html);
        assert!(result.is_ok());
    }

    #[test]
    fn test_element_creation() {
        let mut elem = Element::new("div".to_string());
        elem.attributes.push(("class".to_string(), "test".to_string()));

        assert_eq!(elem.tag, "div");
        assert_eq!(elem.get_attribute("class"), Some("test"));
    }

    #[test]
    fn test_find_by_tag() {
        let mut parent = Element::new("div".to_string());
        let child = Element::new("span".to_string());
        parent.children.push(Node::Element(child));

        let spans = parent.find_by_tag("span");
        assert_eq!(spans.len(), 1);
    }
}
