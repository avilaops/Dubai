// AvilaDB - Native Key-Value Database
// Zero External Dependencies 🦀

use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write, Seek, SeekFrom};
use std::path::Path;

#[derive(Debug)]
pub struct Database {
    path: String,
    index: HashMap<String, u64>, // key -> file offset
    file: File,
}

impl Database {
    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let path_str = path.as_ref().to_string_lossy().to_string();

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)?;

        let mut db = Self {
            path: path_str,
            index: HashMap::new(),
            file,
        };

        db.load_index()?;
        Ok(db)
    }

    fn load_index(&mut self) -> io::Result<()> {
        self.file.seek(SeekFrom::Start(0))?;
        let mut offset = 0u64;

        loop {
            // Read key length
            let mut len_buf = [0u8; 4];
            match self.file.read_exact(&mut len_buf) {
                Ok(_) => {}
                Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => break,
                Err(e) => return Err(e),
            }

            let key_len = u32::from_be_bytes(len_buf) as usize;

            // Read key
            let mut key_buf = vec![0u8; key_len];
            self.file.read_exact(&mut key_buf)?;
            let key = String::from_utf8_lossy(&key_buf).to_string();

            // Read value length
            let mut val_len_buf = [0u8; 4];
            self.file.read_exact(&mut val_len_buf)?;
            let val_len = u32::from_be_bytes(val_len_buf) as usize;

            // Skip value
            self.file.seek(SeekFrom::Current(val_len as i64))?;

            // Store in index
            self.index.insert(key, offset);
            offset = self.file.stream_position()?;
        }

        Ok(())
    }

    pub fn set(&mut self, key: &str, value: &[u8]) -> io::Result<()> {
        // Seek to end
        let offset = self.file.seek(SeekFrom::End(0))?;

        // Write key length
        let key_bytes = key.as_bytes();
        self.file.write_all(&(key_bytes.len() as u32).to_be_bytes())?;

        // Write key
        self.file.write_all(key_bytes)?;

        // Write value length
        self.file.write_all(&(value.len() as u32).to_be_bytes())?;

        // Write value
        self.file.write_all(value)?;

        self.file.flush()?;

        // Update index
        self.index.insert(key.to_string(), offset);

        Ok(())
    }

    pub fn get(&mut self, key: &str) -> io::Result<Option<Vec<u8>>> {
        match self.index.get(key) {
            Some(&offset) => {
                self.file.seek(SeekFrom::Start(offset))?;

                // Read key length
                let mut len_buf = [0u8; 4];
                self.file.read_exact(&mut len_buf)?;
                let key_len = u32::from_be_bytes(len_buf) as usize;

                // Skip key
                self.file.seek(SeekFrom::Current(key_len as i64))?;

                // Read value length
                let mut val_len_buf = [0u8; 4];
                self.file.read_exact(&mut val_len_buf)?;
                let val_len = u32::from_be_bytes(val_len_buf) as usize;

                // Read value
                let mut value = vec![0u8; val_len];
                self.file.read_exact(&mut value)?;

                Ok(Some(value))
            }
            None => Ok(None),
        }
    }

    pub fn delete(&mut self, key: &str) -> io::Result<bool> {
        if self.index.remove(key).is_some() {
            // Mark as deleted by setting empty value
            self.set(key, b"")?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn keys(&self) -> Vec<String> {
        self.index.keys().cloned().collect()
    }

    pub fn len(&self) -> usize {
        self.index.len()
    }

    pub fn is_empty(&self) -> bool {
        self.index.is_empty()
    }

    pub fn compact(&mut self) -> io::Result<()> {
        // Create temporary file
        let temp_path = format!("{}.tmp", self.path);
        let mut temp_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(&temp_path)?;

        let mut new_index = HashMap::new();

        // Copy all non-deleted entries
        let keys: Vec<String> = self.index.keys().cloned().collect();
        for key in keys {
            if let Some(value) = self.get(&key)? {
                if !value.is_empty() {
                    let offset = temp_file.stream_position()?;

                    // Write to temp file
                    let key_bytes = key.as_bytes();
                    temp_file.write_all(&(key_bytes.len() as u32).to_be_bytes())?;
                    temp_file.write_all(key_bytes)?;
                    temp_file.write_all(&(value.len() as u32).to_be_bytes())?;
                    temp_file.write_all(&value)?;

                    new_index.insert(key.clone(), offset);
                }
            }
        }

        temp_file.flush()?;
        drop(temp_file);

        // Replace original file
        std::fs::rename(&temp_path, &self.path)?;

        // Reopen file
        self.file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&self.path)?;

        self.index = new_index;

        Ok(())
    }
}

// JSON value storage helper
pub mod json {
    use super::Database;
    use std::io;

    pub fn set_json(db: &mut Database, key: &str, json: &str) -> io::Result<()> {
        db.set(key, json.as_bytes())
    }

    pub fn get_json(db: &mut Database, key: &str) -> io::Result<Option<String>> {
        match db.get(key)? {
            Some(bytes) => Ok(Some(String::from_utf8_lossy(&bytes).to_string())),
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_database_create() {
        let path = "test_create.db";
        let db = Database::open(path);
        assert!(db.is_ok());
        fs::remove_file(path).ok();
    }

    #[test]
    fn test_set_get() {
        let path = "test_set_get.db";
        let mut db = Database::open(path).unwrap();

        db.set("name", b"Dubai").unwrap();
        let value = db.get("name").unwrap();
        assert_eq!(value, Some(b"Dubai".to_vec()));

        fs::remove_file(path).ok();
    }

    #[test]
    fn test_delete() {
        let path = "test_delete.db";
        let mut db = Database::open(path).unwrap();

        db.set("temp", b"value").unwrap();
        assert!(db.delete("temp").unwrap());

        let value = db.get("temp").unwrap();
        assert_eq!(value, Some(vec![]));

        fs::remove_file(path).ok();
    }

    #[test]
    fn test_keys() {
        let path = "test_keys.db";
        let mut db = Database::open(path).unwrap();

        db.set("key1", b"val1").unwrap();
        db.set("key2", b"val2").unwrap();

        let keys = db.keys();
        assert_eq!(keys.len(), 2);

        fs::remove_file(path).ok();
    }

    #[test]
    fn test_persistence() {
        let path = "test_persist.db";

        {
            let mut db = Database::open(path).unwrap();
            db.set("persist", b"data").unwrap();
        }

        {
            let mut db = Database::open(path).unwrap();
            let value = db.get("persist").unwrap();
            assert_eq!(value, Some(b"data".to_vec()));
        }

        fs::remove_file(path).ok();
    }

    #[test]
    fn test_json_helpers() {
        let path = "test_json.db";
        let mut db = Database::open(path).unwrap();

        json::set_json(&mut db, "config", r#"{"city":"Dubai"}"#).unwrap();
        let json = json::get_json(&mut db, "config").unwrap();
        assert_eq!(json, Some(r#"{"city":"Dubai"}"#.to_string()));

        fs::remove_file(path).ok();
    }
}
