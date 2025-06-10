use dioxus::html::cite;
use serde::Deserialize;
use serde_json::json;

pub async fn get_address(address: &str) -> Result<Feature, String> {
    let url = format!(
        "https://nominatim.openstreetmap.org/search?q={}&format=jsonv2&addressdetails=1",
        urlencoding::encode(address)
    );

    let response = reqwest::get(&url).await;

    if response.is_err() {
        return Err(format!(
            "Request failed: {}",
            response.expect_err("Expected response error").to_string()
        ));
    }

    let response = response.expect("Expected successful response");
    let response = response.json::<Vec<Feature>>().await;

    if response.is_err() {
        return Err(format!(
            "Response parsing failed: {}",
            response.expect_err("Expected response error").to_string()
        ));
    }

    let response = response.expect("Expected successful JSON parsing");

    if !response.is_empty() {
        return Ok(response.get(0).expect("Expect one feature").clone());
    }

    Err("Address not found".to_string())
}

// --- Structs zur Deserialisierung ---

#[derive(Debug, Deserialize, Clone)]
pub struct Feature {
    #[serde(deserialize_with = "self::deserialize_f64")]
    pub lat: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    pub lon: f64,
    pub address: Address,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Address {
    pub house_number: Option<String>,
    pub road: Option<String>,
    pub postcode: Option<String>,
    village: Option<String>,
    town: Option<String>,
    city: Option<String>,
}

impl Address {
    pub fn get_city(&self) -> String {
        if self.city.is_some() {
            return self.city.clone().expect("Expect city to be set!");
        }
        if self.town.is_some() {
            return self
                .town
                .clone()
                .expect("Expect town to be set!")
                .to_string();
        }
        if self.village.is_some() {
            return self
                .village
                .clone()
                .expect("Expect village to be set!")
                .to_string();
        }
        "".to_string()
    }
}

fn deserialize_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = serde::Deserialize::deserialize(deserializer)?;
    s.parse::<f64>().map_err(serde::de::Error::custom)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geojson_response_parsing() {
        let json_data = json!([
          {
            "place_id": 126313729,
            "licence": "Data © OpenStreetMap contributors, ODbL 1.0. http://osm.org/copyright",
            "osm_type": "way",
            "osm_id": 449377753,
            "lat": "50.1127197",
            "lon": "8.6830441",
            "category": "tourism",
            "type": "attraction",
            "place_rank": 30,
            "importance": 0.250991283779456,
            "addresstype": "tourism",
            "name": "Kleinmarkthalle",
            "display_name": "Kleinmarkthalle, 5-7, Hasengasse, Altstadt, Innenstadt 1, Frankfurt, Hesse, 60311, Germany",
            "address": {
              "tourism": "Kleinmarkthalle",
              "house_number": "5-7",
              "road": "Hasengasse",
              "suburb": "Altstadt",
              "city_district": "Innenstadt 1",
              "city": "Frankfurt",
              "state": "Hesse",
              "ISO3166-2-lvl4": "DE-HE",
              "postcode": "60311",
              "country": "Germany",
              "country_code": "de"
            },
            "boundingbox": [
              "50.1124591",
              "50.1130170",
              "8.6822089",
              "8.6839645"
            ]
          }
        ]);

        let parsed: Result<Vec<Feature>, _> = serde_json::from_value(json_data);

        assert!(
            parsed.is_ok(),
            "Failed to parse GeoJSON response: {}",
            parsed.unwrap_err()
        );
        let features = parsed.unwrap();
        assert_eq!(features.len(), 1);

        let feature = &features[0];
        assert_eq!(feature.lat, 50.1127197);
        assert_eq!(feature.lon, 8.6830441);

        let address = &feature.address;
        assert_eq!(address.house_number.as_deref(), Some("5-7"));
        assert_eq!(address.road.as_deref(), Some("Hasengasse"));
        assert_eq!(address.postcode.as_deref(), Some("60311"));
    }
}
