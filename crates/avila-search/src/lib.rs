// AvilaSearch - Native Search Engine
// Zero External Dependencies 🦀

use std::collections::HashMap;

pub struct SearchIndex {
    documents: HashMap<String, String>,
    index: HashMap<String, Vec<String>>, // word -> document IDs
}

impl SearchIndex {
    pub fn new() -> Self {
        Self {
            documents: HashMap::new(),
            index: HashMap::new(),
        }
    }

    pub fn add_document(&mut self, id: String, content: String) {
        // Tokenize and index
        let words = tokenize(&content);
        for word in words {
            self.index.entry(word).or_insert_with(Vec::new).push(id.clone());
        }
        self.documents.insert(id, content);
    }

    pub fn search(&self, query: &str) -> Vec<String> {
        let words = tokenize(query);
        let mut results = Vec::new();
        
        for word in words {
            if let Some(doc_ids) = self.index.get(&word) {
                results.extend(doc_ids.clone());
            }
        }

        results.sort();
        results.dedup();
        results
    }

    pub fn get_document(&self, id: &str) -> Option<&String> {
        self.documents.get(id)
    }
}

impl Default for SearchIndex {
    fn default() -> Self {
        Self::new()
    }
}

fn tokenize(text: &str) -> Vec<String> {
    text.to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let mut index = SearchIndex::new();
        index.add_document("1".to_string(), "Dubai Marina apartment".to_string());
        index.add_document("2".to_string(), "Downtown Dubai villa".to_string());

        let results = index.search("Dubai");
        assert_eq!(results.len(), 2);
    }
}
