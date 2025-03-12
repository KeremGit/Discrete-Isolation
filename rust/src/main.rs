use std::fs::File;
use csv::{ReaderBuilder, WriterBuilder};
use geo::{Point, prelude::HaversineDistance};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct City {
    city: String,
    city_ascii: String,
    #[serde(deserialize_with = "deserialize_or_default")]
    lat: f64,
    #[serde(deserialize_with = "deserialize_or_default")]
    lng: f64,
    #[serde(deserialize_with = "deserialize_u64_or_default")]
    population: u64,
}

#[derive(Debug, Serialize)]
struct CityWithIsolation {
    city: String,
    city_ascii: String,
    lat: f64,
    lng: f64,
    population: u64,
    isolation_value: f64,
}

fn deserialize_or_default<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Ok(f64::deserialize(deserializer).unwrap_or(0.0))
}

fn deserialize_u64_or_default<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Ok(u64::deserialize(deserializer).unwrap_or(0))
}

fn calculate_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let start = Point::new(lon1, lat1);
    let end = Point::new(lon2, lat2);
    start.haversine_distance(&end) / 1000.0 // Convert to km (remove redundant 0s)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = ReaderBuilder::new().from_path("../../files/worldcities.csv")?;
    let mut cities: Vec<City> = reader.deserialize().collect::<Result<_, _>>()?;

    // Sort cities by population (descending)
    cities.sort_by(|a, b| b.population.cmp(&a.population));

    let mut cities_with_isolation = Vec::new();
    let mut higher_pop_cities: Vec<&City> = Vec::new(); // Only need to compare to higher population cities

    // Calculate isolation values
    for city in &cities {
        let mut isolation_value = 20037; // max isolation should be half the circumference of the earth

        for other_city in &higher_pop_cities {
            let distance = calculate_distance(city.lat, city.lng, other_city.lat, other_city.lng);
            if distance < isolation_value {
                isolation_value = distance;
            }
        }

        cities_with_isolation.push(CityWithIsolation {
            city: city.city.clone(),
            city_ascii: city.city_ascii.clone(),
            lat: city.lat,
            lng: city.lng,
            population: city.population,
            isolation_value,
        });

        higher_pop_cities.push(city);
    }

    // Save results
    let isolation_file = File::create("city_isolation_values.csv")?;
    let mut isolation_writer = WriterBuilder::new().from_writer(isolation_file);
    for city in cities_with_isolation {
        isolation_writer.serialize(city)?;
    }

    Ok(())
}
