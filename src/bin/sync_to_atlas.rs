// Sync Local JSON Data to MongoDB Atlas
// Uploads dubai-properties.json to MongoDB

use std::fs;
use avila_json::{parse, JsonValue};
use avila_mongo::{MongoAtlasClient, MongoDocument};

fn main() {
    println!("📤 Sincronizando dados locais com MongoDB Atlas...\n");
    
    // Connect to MongoDB Atlas
    let mongo = match MongoAtlasClient::from_env() {
        Ok(client) => {
            println!("✅ MongoDB Atlas conectado:");
            println!("   App: {}", client.app_id());
            println!("   Cluster: {}", client.cluster());
            println!("   Database: {}", client.database());
            println!("   Collection: {}\n", client.collection());
            client
        }
        Err(e) => {
            eprintln!("❌ Erro ao conectar MongoDB Atlas: {:?}", e);
            eprintln!("\n🔧 Configure as variáveis:");
            eprintln!("  MONGO_ATLAS_URI=mongodb+srv://...");
            eprintln!("  MONGODB_ATLAS_APP_ID=...");
            eprintln!("  MONGODB_ATLAS_API_KEY=...");
            eprintln!("  MONGODB_ATLAS_CLUSTER=cluster0");
            eprintln!("  MONGODB_ATLAS_DATABASE=dubai");
            eprintln!("  MONGODB_ATLAS_COLLECTION=properties");
            std::process::exit(1);
        }
    };
    
    // Test connection
    println!("🔌 Testando conexão...");
    match mongo.ping() {
        Ok(_) => println!("✅ Conexão bem-sucedida!\n"),
        Err(e) => {
            eprintln!("❌ Erro na conexão: {:?}", e);
            std::process::exit(1);
        }
    }
    
    // Load local JSON file
    let json_path = "docs/data/dubai-properties.json";
    println!("📂 Carregando arquivo: {}", json_path);
    
    let json_content = match fs::read_to_string(json_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("❌ Erro ao ler arquivo: {}", e);
            std::process::exit(1);
        }
    };
    
    let data = match parse(&json_content) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("❌ Erro ao parsear JSON: {:?}", e);
            std::process::exit(1);
        }
    };
    
    println!("✅ JSON parseado com sucesso\n");
    
    // Upload properties
    if let Some(obj) = data.as_object() {
        if let Some(properties_val) = obj.get("properties") {
            if let Some(properties) = properties_val.as_array() {
                println!("🏢 Enviando {} propriedades...", properties.len());
                
                let mut success_count = 0;
                let mut error_count = 0;
                
                for (idx, prop) in properties.iter().enumerate() {
                    let doc = convert_property_to_document(prop, idx);
                    
                    match mongo.insert_document(&doc) {
                        Ok(_) => {
                            success_count += 1;
                            print!(".");
                            if (idx + 1) % 10 == 0 {
                                println!(" {}/{}", idx + 1, properties.len());
                            }
                        }
                        Err(e) => {
                            error_count += 1;
                            eprintln!("\n❌ Erro ao inserir propriedade {}: {:?}", idx + 1, e);
                        }
                    }
                }
                
                println!("\n✅ Propriedades enviadas: {}", success_count);
                if error_count > 0 {
                    println!("⚠️ Erros: {}", error_count);
                }
            }
        }
        
        // Upload free zones
        if let Some(fz_val) = obj.get("free_zones") {
            if let Some(free_zones) = fz_val.as_array() {
                println!("\n🏗️ Enviando {} free zones...", free_zones.len());
                
                for zone in free_zones {
                    let doc = convert_free_zone_to_document(zone);
                    match mongo.insert_document(&doc) {
                        Ok(_) => print!("."),
                        Err(e) => eprintln!("\n❌ Erro: {:?}", e),
                    }
                }
                println!("\n✅ Free zones enviadas");
            }
        }
        
        // Upload landmarks
        if let Some(landmarks_val) = obj.get("landmarks") {
            if let Some(landmarks_obj) = landmarks_val.as_object() {
                println!("\n📍 Enviando landmarks...");
                
                for (key, landmark) in landmarks_obj {
                    let mut doc = MongoDocument::new();
                    doc.insert_string("doc_type", "landmark");
                    doc.insert_string("key", key);
                    
                    if let Some(landmark_obj) = landmark.as_object() {
                        if let Some(name) = landmark_obj.get("name").and_then(|n| n.as_str()) {
                            doc.insert_string("name", name);
                        }
                        
                        if let Some(coords_val) = landmark_obj.get("coordinates") {
                            doc.insert_value("coordinates", coords_val.clone());
                        }
                    }
                    
                    match mongo.insert_document(&doc) {
                        Ok(_) => print!("."),
                        Err(e) => eprintln!("\n❌ Erro: {:?}", e),
                    }
                }
                println!("\n✅ Landmarks enviados");
            }
        }
        
        // Upload statistics
        if let Some(stats_val) = obj.get("statistics") {
            if let Some(stats) = stats_val.as_object() {
                println!("\n📊 Enviando estatísticas de mercado...");
                
                let mut doc = MongoDocument::new();
                doc.insert_string("doc_type", "market_statistics");
                
                for (key, value) in stats {
                    match value {
                        JsonValue::Number(n) => {
                            doc.insert_number(key, *n);
                        }
                        JsonValue::Array(arr) => {
                            if key == "areas_covered" {
                                let areas: Vec<String> = arr.iter()
                                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                    .collect();
                                let areas_str = areas.join(", ");
                                doc.insert_string("areas_covered", &areas_str);
                            }
                        }
                        _ => {}
                    }
                }
                
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs_f64();
                doc.insert_number("updated_at", now);
                
                match mongo.insert_document(&doc) {
                    Ok(_) => println!("✅ Estatísticas enviadas"),
                    Err(e) => eprintln!("❌ Erro: {:?}", e),
                }
            }
        }
    }
    
    println!("\n🎉 Sincronização completa!");
    println!("💡 Acesse MongoDB Atlas para visualizar os dados");
    println!("🌐 https://cloud.mongodb.com");
}

fn convert_property_to_document(prop: &JsonValue, index: usize) -> MongoDocument {
    let mut doc = MongoDocument::new();
    
    doc.insert_string("doc_type", "property");
    doc.insert_number("index", index as f64);
    
    if let Some(obj) = prop.as_object() {
        if let Some(id) = obj.get("id").and_then(|i| i.as_str()) {
            doc.insert_string("id", id);
        }
        
        if let Some(title) = obj.get("title").and_then(|t| t.as_str()) {
            doc.insert_string("title", title);
        }
        
        if let Some(price) = obj.get("price").and_then(|p| p.as_f64()) {
            doc.insert_number("price", price);
        }
        
        if let Some(price_per_sqm) = obj.get("price_per_sqm").and_then(|p| p.as_f64()) {
            doc.insert_number("price_per_sqm", price_per_sqm);
        }
        
        if let Some(currency) = obj.get("currency").and_then(|c| c.as_str()) {
            doc.insert_string("currency", currency);
        }
        
        if let Some(location) = obj.get("location").and_then(|l| l.as_str()) {
            doc.insert_string("location", location);
        }
        
        if let Some(coords_val) = obj.get("coordinates") {
            doc.insert_value("coordinates", coords_val.clone());
        }
        
        if let Some(bedrooms) = obj.get("bedrooms").and_then(|b| b.as_f64()) {
            doc.insert_number("bedrooms", bedrooms);
        }
        
        if let Some(bathrooms) = obj.get("bathrooms").and_then(|b| b.as_f64()) {
            doc.insert_number("bathrooms", bathrooms);
        }
        
        if let Some(area_sqm) = obj.get("area_sqm").and_then(|a| a.as_f64()) {
            doc.insert_number("area_sqm", area_sqm);
        }
        
        if let Some(property_type) = obj.get("property_type").and_then(|pt| pt.as_str()) {
            doc.insert_string("property_type", property_type);
        }
        
        if let Some(url) = obj.get("url").and_then(|u| u.as_str()) {
            doc.insert_string("url", url);
        }
        
        if let Some(distance) = obj.get("distance_to_burj_khalifa_km").and_then(|d| d.as_f64()) {
            doc.insert_number("distance_to_burj_khalifa_km", distance);
        }
        
        if let Some(building) = obj.get("building").and_then(|b| b.as_str()) {
            doc.insert_string("building", building);
        }
        
        if let Some(year) = obj.get("year_built").and_then(|y| y.as_f64()) {
            doc.insert_number("year_built", year);
        }
        
        if let Some(ready) = obj.get("ready_to_move").and_then(|r| r.as_bool()) {
            doc.insert_string("ready_to_move", if ready { "yes" } else { "no" });
        }
        
        if let Some(features_val) = obj.get("features") {
            if let Some(features) = features_val.as_array() {
                let features_str: Vec<String> = features.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect();
                doc.insert_string("features", &features_str.join(", "));
            }
        }
    }
    
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs_f64();
    doc.insert_number("synced_at", now);
    
    doc
}

fn convert_free_zone_to_document(zone: &JsonValue) -> MongoDocument {
    let mut doc = MongoDocument::new();
    
    doc.insert_string("doc_type", "free_zone");
    
    if let Some(obj) = zone.as_object() {
        if let Some(name) = obj.get("name").and_then(|n| n.as_str()) {
            doc.insert_string("name", name);
        }
        
        if let Some(location) = obj.get("location").and_then(|l| l.as_str()) {
            doc.insert_string("location", location);
        }
        
        if let Some(website) = obj.get("website").and_then(|w| w.as_str()) {
            doc.insert_string("website", website);
        }
        
        if let Some(cost_range_val) = obj.get("cost_range_aed") {
            if let Some(cost_range) = cost_range_val.as_object() {
                if let Some(min) = cost_range.get("min").and_then(|m| m.as_f64()) {
                    doc.insert_number("cost_min_aed", min);
                }
                if let Some(max) = cost_range.get("max").and_then(|m| m.as_f64()) {
                    doc.insert_number("cost_max_aed", max);
                }
            }
        }
        
        if let Some(benefits_val) = obj.get("benefits") {
            if let Some(benefits) = benefits_val.as_array() {
                let benefits_str: Vec<String> = benefits.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect();
                doc.insert_string("benefits", &benefits_str.join("; "));
            }
        }
        
        if let Some(business_types_val) = obj.get("business_types") {
            if let Some(business_types) = business_types_val.as_array() {
                let types_str: Vec<String> = business_types.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect();
                doc.insert_string("business_types", &types_str.join(", "));
            }
        }
    }
    
    doc
}
