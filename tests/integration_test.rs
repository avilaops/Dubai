// TESTES DE INTEGRAÇÃO END-TO-END
// Verifica que todas as ferramentas Avila* funcionam juntas

use dubai_project::integration::DubaiDataPipeline;
use dubai_project::property_search::*;

#[test]
fn test_full_pipeline() {
    println!("\n🧪 TESTE COMPLETO DE INTEGRAÇÃO");

    // Criar pipeline
    let pipeline = DubaiDataPipeline::new("test_integration.db");
    assert!(pipeline.is_ok(), "Pipeline deve ser criado com sucesso");

    println!("✅ Pipeline criado");
}

#[test]
fn test_distance_calculation_integration() {
    println!("\n🧪 TESTE: Cálculo de Distância");

    let pipeline = DubaiDataPipeline::new("test_distance.db").unwrap();

    // Testar várias localizações reais de Dubai
    let locations = vec![
        ("Dubai Marina", (25.0801, 55.1378)),
        ("Palm Jumeirah", (25.1124, 55.1390)),
        ("Downtown Dubai", (25.1972, 55.2744)),
        ("Business Bay", (25.1877, 55.2632)),
    ];

    for (name, coords) in locations {
        let distance = pipeline.calculate_distance_to_burj(coords);
        println!("  📍 {}: {:.2} km do Burj Khalifa", name, distance);
        assert!(distance >= 0.0, "Distância deve ser positiva");
    }

    println!("✅ Todas as distâncias calculadas corretamente");
}

#[test]
fn test_search_system_creation() {
    println!("\n🧪 TESTE: Sistema de Busca");

    let search = DubaiRealEstateSearch::new();
    assert_eq!(search.api_endpoints.len(), 3, "Deve ter 3 endpoints configurados");

    println!("✅ Sistema de busca inicializado");
}

#[test]
fn test_job_queue_integration() {
    println!("\n🧪 TESTE: Fila de Jobs");

    let mut pipeline = DubaiDataPipeline::new("test_jobs.db").unwrap();

    // Enfileirar várias áreas para scraping
    let areas = vec![
        "dubai-marina".to_string(),
        "downtown-dubai".to_string(),
        "palm-jumeirah".to_string(),
        "business-bay".to_string(),
        "jbr".to_string(),
    ];

    pipeline.queue_area_scraping(areas.clone());
    assert_eq!(pipeline.job_queue.len(), areas.len(), "Todos os jobs devem estar na fila");

    println!("✅ {} jobs enfileirados", areas.len());
}

#[test]
fn test_crypto_integration() {
    println!("\n🧪 TESTE: Criptografia");

    let mut pipeline = DubaiDataPipeline::new("test_crypto.db").unwrap();

    let stats = r#"{"total_properties": 1500, "average_price": 2500000}"#;
    let result = pipeline.save_encrypted_stats(stats);

    assert!(result.is_ok(), "Stats devem ser salvas com sucesso");
    println!("✅ Stats criptografadas e salvas");
}

#[test]
fn test_visa_requirements() {
    println!("\n🧪 TESTE: Requisitos de Visto");

    let search = DubaiRealEstateSearch::new();
    let visa = search.get_visa_requirements();

    assert_eq!(visa.entrepreneur_visa.duration_years, 10);
    assert!(!visa.entrepreneur_visa.requirements.is_empty());
    assert!(!visa.entrepreneur_visa.benefits.is_empty());

    println!("✅ Dados de visto disponíveis:");
    println!("   Nome: {}", visa.entrepreneur_visa.name);
    println!("   Duração: {} anos", visa.entrepreneur_visa.duration_years);
    println!("   Requisitos: {}", visa.entrepreneur_visa.requirements.len());
    println!("   Benefícios: {}", visa.entrepreneur_visa.benefits.len());
}

#[test]
fn test_free_zones() {
    println!("\n🧪 TESTE: Free Zones");

    let search = DubaiRealEstateSearch::new();
    let zones = search.get_free_zones();

    assert!(zones.len() >= 3, "Deve ter pelo menos 3 free zones");

    for zone in &zones {
        println!("  🏢 {}", zone.name);
        println!("     Localização: {}", zone.location);
        println!("     Custo: AED {:.0} - {:.0}", zone.cost_range_aed.0, zone.cost_range_aed.1);
        assert!(!zone.benefits.is_empty(), "Cada zona deve ter benefícios");
        assert!(!zone.business_types.is_empty(), "Cada zona deve ter tipos de negócio");
    }

    println!("✅ {} free zones validadas", zones.len());
}

#[test]
fn test_market_statistics() {
    println!("\n🧪 TESTE: Estatísticas de Mercado");

    let search = DubaiRealEstateSearch::new();
    let stats = search.get_market_statistics();

    assert!(!stats.source.is_empty(), "Deve ter fonte de dados");
    assert!(!stats.average_prices_aed_per_sqm.is_empty(), "Deve ter preços médios");

    println!("✅ Estatísticas de mercado:");
    println!("   Fonte: {}", stats.source);
    for (area, price) in &stats.average_prices_aed_per_sqm {
        println!("   {}: AED {:.0}/m²", area, price);
    }
}

#[test]
fn test_pdf_generation() {
    println!("\n🧪 TESTE: Geração de PDF");

    let pipeline = DubaiDataPipeline::new("test_pdf.db").unwrap();

    // Criar propriedades de exemplo
    let properties = vec![
        PropertyListing {
            title: "Luxury Apartment in Dubai Marina".to_string(),
            price: 2500000.0,
            currency: "AED".to_string(),
            location: "Dubai Marina".to_string(),
            coordinates: Some((25.0801, 55.1378)),
            bedrooms: 3,
            bathrooms: 2,
            area_sqm: 150.0,
            property_type: PropertyType::Apartment,
            url: "https://example.com/property/1".to_string(),
            distance_to_burj_khalifa: Some(12.5),
        },
        PropertyListing {
            title: "Penthouse with Sea View".to_string(),
            price: 5000000.0,
            currency: "AED".to_string(),
            location: "Palm Jumeirah".to_string(),
            coordinates: Some((25.1124, 55.1390)),
            bedrooms: 4,
            bathrooms: 3,
            area_sqm: 250.0,
            property_type: PropertyType::Penthouse,
            url: "https://example.com/property/2".to_string(),
            distance_to_burj_khalifa: Some(15.2),
        },
    ];

    let pdf_result = pipeline.generate_property_report(&properties);
    assert!(pdf_result.is_ok(), "PDF deve ser gerado com sucesso");

    let pdf_bytes = pdf_result.unwrap();
    assert!(pdf_bytes.len() > 0, "PDF deve ter conteúdo");
    assert!(pdf_bytes.starts_with(b"%PDF"), "Deve ser um PDF válido");

    println!("✅ PDF gerado: {} bytes", pdf_bytes.len());
}

#[test]
fn test_search_criteria() {
    println!("\n🧪 TESTE: Critérios de Busca");

    let criteria = SearchCriteria {
        min_price: Some(1000000.0),
        max_price: Some(3000000.0),
        min_bedrooms: Some(2),
        max_bedrooms: Some(4),
        property_type: Some(PropertyType::Apartment),
        min_area_sqm: Some(100.0),
    };

    println!("✅ Critérios criados:");
    println!("   Preço: AED {:.0} - {:.0}",
        criteria.min_price.unwrap(),
        criteria.max_price.unwrap());
    println!("   Quartos: {} - {}",
        criteria.min_bedrooms.unwrap(),
        criteria.max_bedrooms.unwrap());
    println!("   Área mín: {} m²", criteria.min_area_sqm.unwrap());
}

#[test]
#[ignore] // Requer configuração de API keys
fn test_real_bayut_scraping() {
    println!("\n🧪 TESTE: Scraping Real do Bayut (IGNORADO - requer API)");

    let mut pipeline = DubaiDataPipeline::new("test_bayut.db").unwrap();

    // Este teste só funcionará com configuração real
    match pipeline.scrape_bayut("dubai-marina") {
        Ok(properties) => {
            println!("✅ {} propriedades encontradas", properties.len());
            for prop in properties.iter().take(3) {
                println!("  🏢 {}", prop.title);
                println!("     AED {:.0} - {}", prop.price, prop.location);
            }
        }
        Err(e) => {
            println!("⚠️  Erro esperado (sem API configurada): {}", e);
        }
    }
}
