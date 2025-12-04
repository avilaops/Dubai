// Avila Mongo - MongoDB Atlas Integration
// Zero external dependencies, built on top of Avila HTTP/TLS stack 🦀

use std::collections::HashMap;

use avila_http::{HttpClient, HttpError};
use avila_json::{self, JsonValue};

/// MongoDB Atlas client powered by the Avila native stack.
///
/// This client communicates with Atlas using the HTTPS Data API,
/// leveraging the native TLS implementation (avila-http::tls) and
/// native JSON encoder/decoder (avila-json).
pub struct MongoAtlasClient {
    app_id: String,
    cluster: String,
    database: String,
    collection: String,
    api_key: String,
    base_url: String,
    http: HttpClient,
}

impl MongoAtlasClient {
    /// Construct a new client with explicit parameters.
    pub fn new(
        app_id: impl Into<String>,
        cluster: impl Into<String>,
        database: impl Into<String>,
        collection: impl Into<String>,
        api_key: impl Into<String>,
    ) -> Self {
        let app_id = app_id.into();
        let base_url = format!(
            "https://data.mongodb-api.com/app/{}/endpoint/data/v1/action",
            app_id
        );

        Self {
            app_id,
            cluster: cluster.into(),
            database: database.into(),
            collection: collection.into(),
            api_key: api_key.into(),
            base_url,
            http: HttpClient::new(),
        }
    }

    /// Instantiate the client reading credentials from environment variables.
    ///
    /// Required variables:
    /// - MONGODB_ATLAS_APP_ID
    /// - MONGODB_ATLAS_API_KEY
    /// - MONGODB_ATLAS_CLUSTER
    /// - MONGODB_ATLAS_DATABASE
    /// - MONGODB_ATLAS_COLLECTION
    pub fn from_env() -> Result<Self, MongoAtlasError> {
        let app_id = read_env("MONGODB_ATLAS_APP_ID")?;
        let api_key = read_env("MONGODB_ATLAS_API_KEY")?;
        let cluster = read_env("MONGODB_ATLAS_CLUSTER")?;
        let database = read_env("MONGODB_ATLAS_DATABASE")?;
        let collection = read_env("MONGODB_ATLAS_COLLECTION")?;

        Ok(Self::new(app_id, cluster, database, collection, api_key))
    }

    /// Change the target collection.
    pub fn with_collection(mut self, collection: impl Into<String>) -> Self {
        self.collection = collection.into();
        self
    }

    pub fn app_id(&self) -> &str {
        &self.app_id
    }

    pub fn cluster(&self) -> &str {
        &self.cluster
    }

    pub fn database(&self) -> &str {
        &self.database
    }

    pub fn collection(&self) -> &str {
        &self.collection
    }

    /// Insert a document into the configured collection.
    pub fn insert_document(
        &self,
        document: &MongoDocument,
    ) -> Result<MongoInsertOneResult, MongoAtlasError> {
        let mut payload = HashMap::new();
        payload.insert(
            "dataSource".to_string(),
            JsonValue::String(self.cluster.clone()),
        );
        payload.insert(
            "database".to_string(),
            JsonValue::String(self.database.clone()),
        );
        payload.insert(
            "collection".to_string(),
            JsonValue::String(self.collection.clone()),
        );
        payload.insert("document".to_string(), document.to_json());

        let response = self.post_json("insertOne", JsonValue::Object(payload))?;
        parse_insert_one_response(response)
    }

    /// Find a single document using the provided filter.
    pub fn find_one(&self, filter: &MongoDocument) -> Result<Option<JsonValue>, MongoAtlasError> {
        let mut payload = HashMap::new();
        payload.insert(
            "dataSource".to_string(),
            JsonValue::String(self.cluster.clone()),
        );
        payload.insert(
            "database".to_string(),
            JsonValue::String(self.database.clone()),
        );
        payload.insert(
            "collection".to_string(),
            JsonValue::String(self.collection.clone()),
        );
        payload.insert("filter".to_string(), filter.to_json());

        let response = self.post_json("findOne", JsonValue::Object(payload))?;
        parse_find_one_response(response)
    }

    /// Ping command to verify connectivity.
    pub fn ping(&self) -> Result<(), MongoAtlasError> {
        let mut payload = HashMap::new();
        payload.insert(
            "dataSource".to_string(),
            JsonValue::String(self.cluster.clone()),
        );
        payload.insert(
            "database".to_string(),
            JsonValue::String(self.database.clone()),
        );
        payload.insert(
            "collection".to_string(),
            JsonValue::String(self.collection.clone()),
        );

        // The Data API exposes a "ping" action via the admin endpoint.
        // We emulate it using an aggregation that returns a constant value.
        let pipeline = JsonValue::Array(vec![JsonValue::Object({
            let mut stage = HashMap::new();
            stage.insert(
                "$project".to_string(),
                JsonValue::Object({
                    let mut proj = HashMap::new();
                    proj.insert("alive".to_string(), JsonValue::Number(1.0));
                    proj
                }),
            );
            stage
        })]);

        payload.insert("pipeline".to_string(), pipeline);

        // The aggregate command validates credentials + permissions.
        let _ = self.post_json("aggregate", JsonValue::Object(payload))?;
        Ok(())
    }

    fn post_json(
        &self,
        action: &str,
        payload: JsonValue,
    ) -> Result<JsonValue, MongoAtlasError> {
        let url = format!("{}/{}", self.base_url, action);
        let body = payload.to_string();

        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("Accept".to_string(), "application/json".to_string());
        headers.insert("api-key".to_string(), self.api_key.clone());

        let response = self
            .http
            .post_with_headers(&url, &body, &headers)
            .map_err(MongoAtlasError::Http)?;

        if response.status_code >= 400 {
            return Err(MongoAtlasError::Api(format!(
                "HTTP {}: {}",
                response.status_code, response.body
            )));
        }

        avila_json::parse(&response.body).map_err(|_| {
            MongoAtlasError::Parse(format!("Invalid JSON response: {}", response.body))
        })
    }
}

/// Simple BSON-like document representation leveraging JsonValue.
#[derive(Debug, Clone, Default)]
pub struct MongoDocument {
    fields: HashMap<String, JsonValue>,
}

impl MongoDocument {
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
        }
    }

    pub fn insert_string(&mut self, key: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.fields
            .insert(key.into(), JsonValue::String(value.into()));
        self
    }

    pub fn insert_number(&mut self, key: impl Into<String>, value: f64) -> &mut Self {
        self.fields.insert(key.into(), JsonValue::Number(value));
        self
    }

    pub fn insert_bool(&mut self, key: impl Into<String>, value: bool) -> &mut Self {
        self.fields.insert(key.into(), JsonValue::Bool(value));
        self
    }

    pub fn insert_value(&mut self, key: impl Into<String>, value: JsonValue) -> &mut Self {
        self.fields.insert(key.into(), value);
        self
    }

    pub fn to_json(&self) -> JsonValue {
        JsonValue::Object(self.fields.clone())
    }
}

impl From<HashMap<String, JsonValue>> for MongoDocument {
    fn from(map: HashMap<String, JsonValue>) -> Self {
        Self { fields: map }
    }
}

#[derive(Debug, Clone)]
pub struct MongoInsertOneResult {
    pub inserted_id: String,
}

#[derive(Debug)]
pub enum MongoAtlasError {
    MissingEnv(&'static str),
    Http(HttpError),
    Api(String),
    Parse(String),
    NotFound,
}

impl std::fmt::Display for MongoAtlasError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MongoAtlasError::MissingEnv(var) => write!(f, "Missing environment variable: {}", var),
            MongoAtlasError::Http(err) => write!(f, "HTTP error: {}", err),
            MongoAtlasError::Api(msg) => write!(f, "Atlas API error: {}", msg),
            MongoAtlasError::Parse(msg) => write!(f, "Parse error: {}", msg),
            MongoAtlasError::NotFound => write!(f, "Document not found"),
        }
    }
}

impl std::error::Error for MongoAtlasError {}

fn read_env(name: &'static str) -> Result<String, MongoAtlasError> {
    std::env::var(name).map_err(|_| MongoAtlasError::MissingEnv(name))
}

fn parse_insert_one_response(value: JsonValue) -> Result<MongoInsertOneResult, MongoAtlasError> {
    match value {
        JsonValue::Object(mut obj) => {
            if let Some(id_value) = obj.remove("insertedId") {
                match id_value {
                    JsonValue::String(s) => Ok(MongoInsertOneResult { inserted_id: s }),
                    JsonValue::Object(map) => {
                        if let Some(JsonValue::String(oid)) = map.get("$oid") {
                            Ok(MongoInsertOneResult {
                                inserted_id: oid.clone(),
                            })
                        } else {
                            Err(MongoAtlasError::Parse(
                                "insertedId missing $oid attribute".to_string(),
                            ))
                        }
                    }
                    _ => Err(MongoAtlasError::Parse(
                        "Unexpected insertedId format".to_string(),
                    )),
                }
            } else {
                Err(MongoAtlasError::Parse(
                    "insertOne response missing insertedId".to_string(),
                ))
            }
        }
        _ => Err(MongoAtlasError::Parse(
            "insertOne response is not an object".to_string(),
        )),
    }
}

fn parse_find_one_response(value: JsonValue) -> Result<Option<JsonValue>, MongoAtlasError> {
    match value {
        JsonValue::Object(mut obj) => {
            if let Some(document) = obj.remove("document") {
                if document == JsonValue::Null {
                    Ok(None)
                } else {
                    Ok(Some(document))
                }
            } else {
                Ok(None)
            }
        }
        _ => Err(MongoAtlasError::Parse(
            "findOne response is not an object".to_string(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn document_builder_basic() {
        let mut doc = MongoDocument::new();
        doc.insert_string("name", "Dubai");
        doc.insert_number("value", 123.0);
        doc.insert_bool("active", true);

        if let JsonValue::Object(map) = doc.to_json() {
            assert_eq!(map.get("name"), Some(&JsonValue::String("Dubai".to_string())));
            assert_eq!(map.get("active"), Some(&JsonValue::Bool(true)));
        } else {
            panic!("Document should serialize to JsonValue::Object");
        }
    }

    #[test]
    fn parse_insert_response_with_string_id() {
        let mut obj = HashMap::new();
        obj.insert(
            "insertedId".to_string(),
            JsonValue::String("abc123".to_string()),
        );
        let result = parse_insert_one_response(JsonValue::Object(obj)).unwrap();
        assert_eq!(result.inserted_id, "abc123");
    }

    #[test]
    fn parse_insert_response_with_oid() {
        let mut oid = HashMap::new();
        oid.insert("$oid".to_string(), JsonValue::String("507f1f77bcf86cd799439011".to_string()));

        let mut obj = HashMap::new();
        obj.insert("insertedId".to_string(), JsonValue::Object(oid));

        let result = parse_insert_one_response(JsonValue::Object(obj)).unwrap();
        assert_eq!(result.inserted_id, "507f1f77bcf86cd799439011");
    }

    #[test]
    fn parse_find_response_document() {
        let mut doc = HashMap::new();
        doc.insert("name".to_string(), JsonValue::String("Dubai".to_string()));

        let mut resp = HashMap::new();
        resp.insert("document".to_string(), JsonValue::Object(doc.clone()));

        let parsed = parse_find_one_response(JsonValue::Object(resp)).unwrap();
        assert!(parsed.is_some());
        if let Some(JsonValue::Object(map)) = parsed {
            assert_eq!(map.get("name"), Some(&JsonValue::String("Dubai".to_string())));
        }
    }
}
