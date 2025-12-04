// Dubai Real Estate Atlas - Frontend Application
// Loads and displays Dubai property data

document.addEventListener('DOMContentLoaded', () => {
    loadDubaiData();
});

async function loadDubaiData() {
    try {
        const response = await fetch('data/dubai-properties.json');
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        const data = await response.json();

        // Update header badges
        updateHeaderBadges(data);

        // Render statistics
        renderStatistics(data.statistics);

        // Render properties
        renderProperties(data.properties);

        // Render free zones
        renderFreeZones(data.free_zones);

        // Render landmarks
        renderLandmarks(data.landmarks);

        // Update footer
        updateFooter(data.meta);

        console.log('✅ Dubai data loaded successfully');
    } catch (error) {
        console.error('❌ Error loading Dubai data:', error);
        showError('Failed to load property data. Please refresh the page.');
    }
}

function updateHeaderBadges(data) {
    const totalEl = document.getElementById('total-properties');
    const avgPriceEl = document.getElementById('avg-price');

    if (totalEl) totalEl.textContent = `${data.meta.total_properties} Properties`;
    if (avgPriceEl) avgPriceEl.textContent = `Avg: ${formatCurrency(data.statistics.average_price_aed)}`;
}

function renderStatistics(stats) {
    document.getElementById('stat-total').textContent = stats.areas_covered?.length || '0';
    document.getElementById('stat-avg-price').textContent = formatCurrency(stats.average_price_aed);
    document.getElementById('stat-median-price').textContent = formatCurrency(stats.median_price_aed);
    document.getElementById('stat-price-sqm').textContent = formatCurrency(stats.average_price_per_sqm_aed);
    document.getElementById('stat-min-price').textContent = formatCurrency(stats.min_price_aed);
    document.getElementById('stat-max-price').textContent = formatCurrency(stats.max_price_aed);
}

function renderProperties(properties) {
    const container = document.getElementById('properties-container');
    if (!container) return;

    container.innerHTML = properties.map(prop => `
        <div class="property-card">
            <div class="property-header">
                <h3 class="property-title">${escapeHtml(prop.title)}</h3>
                <span class="property-type">${prop.property_type}</span>
            </div>

            <div class="property-price">
                <span class="price-main">${formatCurrency(prop.price)}</span>
                <span class="price-detail">AED ${formatNumber(prop.price_per_sqm)}/m²</span>
            </div>

            <div class="property-details">
                <div class="detail-row">
                    <span class="detail-icon">📍</span>
                    <span>${escapeHtml(prop.location)}</span>
                </div>
                <div class="detail-row">
                    <span class="detail-icon">🏗️</span>
                    <span>${escapeHtml(prop.building)}</span>
                </div>
                <div class="detail-row">
                    <span class="detail-icon">🛏️</span>
                    <span>${prop.bedrooms} BR · ${prop.bathrooms} Bath · ${formatNumber(prop.area_sqm)}m²</span>
                </div>
                ${prop.distance_to_burj_khalifa_km ? `
                <div class="detail-row">
                    <span class="detail-icon">📏</span>
                    <span>${formatNumber(prop.distance_to_burj_khalifa_km)} km to Burj Khalifa</span>
                </div>
                ` : ''}
                ${prop.coordinates ? `
                <div class="detail-row">
                    <span class="detail-icon">🌐</span>
                    <span>${prop.coordinates.lat.toFixed(4)}, ${prop.coordinates.lon.toFixed(4)}</span>
                </div>
                ` : ''}
            </div>

            <div class="property-features">
                ${prop.features.slice(0, 3).map(f => `<span class="feature-tag">${escapeHtml(f)}</span>`).join('')}
            </div>

            <div class="property-footer">
                <span class="year-built">Built ${prop.year_built}</span>
                ${prop.ready_to_move ? '<span class="ready-badge">Ready to Move</span>' : '<span class="under-construction">Under Construction</span>'}
            </div>

            <a href="${escapeHtml(prop.url)}" target="_blank" class="property-link" rel="noopener noreferrer">
                View Details →
            </a>
        </div>
    `).join('');
}

function renderFreeZones(freeZones) {
    const container = document.getElementById('free-zones-container');
    if (!container) return;

    container.innerHTML = freeZones.map(zone => `
        <div class="free-zone-card">
            <h3 class="zone-name">${escapeHtml(zone.name)}</h3>
            <div class="zone-location">
                <span class="detail-icon">📍</span>
                ${escapeHtml(zone.location)}
            </div>

            <div class="zone-cost">
                <strong>Setup Cost:</strong>
                AED ${formatNumber(zone.cost_range_aed.min)} - ${formatNumber(zone.cost_range_aed.max)}
            </div>

            <div class="zone-benefits">
                <strong>Benefits:</strong>
                <ul>
                    ${zone.benefits.map(b => `<li>${escapeHtml(b)}</li>`).join('')}
                </ul>
            </div>

            <div class="zone-business-types">
                <strong>Business Types:</strong>
                ${zone.business_types.map(bt => `<span class="biz-tag">${escapeHtml(bt)}</span>`).join('')}
            </div>

            <a href="${escapeHtml(zone.website)}" target="_blank" class="zone-link" rel="noopener noreferrer">
                Visit Website →
            </a>
        </div>
    `).join('');
}

function renderLandmarks(landmarks) {
    const container = document.getElementById('landmarks-container');
    if (!container) return;

    const landmarkArray = Object.values(landmarks);

    container.innerHTML = landmarkArray.map(landmark => `
        <div class="landmark-card">
            <h3 class="landmark-name">${escapeHtml(landmark.name)}</h3>
            <div class="landmark-coords">
                <span class="detail-icon">🌐</span>
                Lat: ${landmark.coordinates.lat.toFixed(4)}, Lon: ${landmark.coordinates.lon.toFixed(4)}
            </div>
        </div>
    `).join('');
}

function updateFooter(meta) {
    const sourceEl = document.getElementById('data-source');
    const timestampEl = document.getElementById('data-timestamp');

    if (sourceEl) sourceEl.textContent = meta.source || 'Dubai Real Estate Atlas';
    if (timestampEl) {
        const date = new Date(meta.timestamp);
        timestampEl.textContent = date.toLocaleDateString('en-US', {
            year: 'numeric',
            month: 'long',
            day: 'numeric'
        });
    }
}

function formatCurrency(amount) {
    if (amount === null || amount === undefined) return 'N/A';
    return `AED ${formatNumber(amount)}`;
}

function formatNumber(num) {
    if (num === null || num === undefined) return 'N/A';
    return new Intl.NumberFormat('en-AE').format(Math.round(num));
}

function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}

function showError(message) {
    const main = document.querySelector('main');
    if (main) {
        main.innerHTML = `
            <div class="error-message">
                <h2>⚠️ Error</h2>
                <p>${escapeHtml(message)}</p>
            </div>
        `;
    }
}
