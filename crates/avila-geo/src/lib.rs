// AvilaGeo - Native Geolocation Implementation
// Zero External Dependencies 🦀

#[derive(Debug, Clone, Copy)]
pub struct Coordinate {
    pub latitude: f64,
    pub longitude: f64,
}

impl Coordinate {
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self { latitude, longitude }
    }

    /// Calculate distance between two coordinates using Haversine formula (in km)
    pub fn distance_to(&self, other: &Coordinate) -> f64 {
        const EARTH_RADIUS_KM: f64 = 6371.0;

        let lat1_rad = self.latitude.to_radians();
        let lat2_rad = other.latitude.to_radians();
        let delta_lat = (other.latitude - self.latitude).to_radians();
        let delta_lon = (other.longitude - self.longitude).to_radians();

        let a = (delta_lat / 2.0).sin().powi(2)
            + lat1_rad.cos() * lat2_rad.cos() * (delta_lon / 2.0).sin().powi(2);

        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

        EARTH_RADIUS_KM * c
    }
}

#[derive(Debug)]
pub struct Location {
    pub name: String,
    pub coordinate: Coordinate,
    pub category: LocationCategory,
}

#[derive(Debug, Clone)]
pub enum LocationCategory {
    Residential,
    Commercial,
    Office,
    Landmark,
}

impl Location {
    pub fn new(name: String, coordinate: Coordinate, category: LocationCategory) -> Self {
        Self {
            name,
            coordinate,
            category,
        }
    }
}

// Dubai landmarks and areas
pub mod dubai {
    use super::*;

    pub const BURJ_KHALIFA: Coordinate = Coordinate {
        latitude: 25.1972,
        longitude: 55.2744,
    };

    pub const DUBAI_MALL: Coordinate = Coordinate {
        latitude: 25.1981,
        longitude: 55.2789,
    };

    pub const DUBAI_MARINA: Coordinate = Coordinate {
        latitude: 25.0805,
        longitude: 55.1399,
    };

    pub const DOWNTOWN_DUBAI: Coordinate = Coordinate {
        latitude: 25.1932,
        longitude: 55.2760,
    };

    pub const BUSINESS_BAY: Coordinate = Coordinate {
        latitude: 25.1869,
        longitude: 55.2649,
    };

    pub const PALM_JUMEIRAH: Coordinate = Coordinate {
        latitude: 25.1124,
        longitude: 55.1390,
    };

    pub const DIFC: Coordinate = Coordinate {
        latitude: 25.2138,
        longitude: 55.2824,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_calculation() {
        let burj = dubai::BURJ_KHALIFA;
        let marina = dubai::DUBAI_MARINA;

        let distance = burj.distance_to(&marina);
        assert!(distance > 10.0 && distance < 20.0); // Approximately 15km
    }

    #[test]
    fn test_coordinate_creation() {
        let coord = Coordinate::new(25.1972, 55.2744);
        assert_eq!(coord.latitude, 25.1972);
        assert_eq!(coord.longitude, 55.2744);
    }
}
