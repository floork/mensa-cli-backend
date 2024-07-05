//! This module provides data structures and functions for interacting with canteens
//! and their meals using the OpenMensa API.

use reqwest::Error;
use serde::Deserialize;

/// Represents a canteen with its details.
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Canteen {
    /// The unique identifier for the canteen.
    pub id: u32,
    /// The name of the canteen.
    pub name: String,
    /// The city where the canteen is located.
    pub city: String,
    /// The address of the canteen.
    pub address: String,
    /// The geographical coordinates of the canteen (latitude, longitude).
    pub coordinates: Option<[f64; 2]>,
}

/// Represents the price structure for different user groups.
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Prices {
    /// Price for students.
    pub students: Option<f64>,
    /// Price for employees.
    pub employees: Option<f64>,
    /// Price for pupils.
    pub pupils: Option<f64>,
    /// Price for others.
    pub others: Option<f64>,
}

/// Represents a meal with its details.
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Meal {
    /// The unique identifier for the meal.
    pub id: u64,
    /// The name of the meal.
    pub name: String,
    /// The category of the meal (e.g., "Vegetarian", "Vegan").
    pub category: String,
    /// The price structure for the meal.
    pub prices: Prices,
    /// Additional notes about the meal.
    pub notes: Vec<String>,
}

/// Fetches data from the given URL and deserializes it into the specified type.
///
/// # Arguments
///
/// * `url` - The URL to fetch data from.
///
/// # Returns
///
/// A result containing the deserialized data or a request error.
async fn fetch_from_api<T: for<'de> Deserialize<'de>>(url: &str) -> Result<T, Error> {
    let response = reqwest::get(url).await?.json::<T>().await?;
    Ok(response)
}

/// Gets the meals available at a specified canteen on a given date.
///
/// # Arguments
///
/// * `canteen` - The canteen for which to fetch meals.
/// * `date` - The date for which to fetch meals in YYYY-MM-DD format.
///
/// # Returns
///
/// A result containing a vector of meals or a request error.
pub async fn get_meals(canteen: &Canteen, date: &str) -> Result<Vec<Meal>, Error> {
    let canteen_id = canteen.id;
    let menu_url = format!(
        "https://openmensa.org/api/v2/canteens/{}/days/{}/meals",
        canteen_id, date
    );
    fetch_from_api(&menu_url).await
}

/// Gets a canteen by its unique identifier.
///
/// # Arguments
///
/// * `id` - The unique identifier of the canteen.
///
/// # Returns
///
/// A result containing an option with the canteen or a request error.
pub async fn get_canteen_by_id(id: u32) -> Result<Option<Canteen>, Error> {
    let canteens = get_all_canteens().await?;
    Ok(canteens.into_iter().find(|c| c.id == id))
}

/// Gets a list of canteens by their unique identifiers.
///
/// # Arguments
///
/// * `ids` - A vector of unique identifiers for the canteens.
///
/// # Returns
///
/// A result containing a vector of canteens or a request error.
pub async fn get_canteens_by_ids(ids: Vec<u32>) -> Result<Vec<Canteen>, Error> {
    let canteens = get_all_canteens().await?;
    Ok(canteens
        .into_iter()
        .filter(|c| ids.contains(&c.id))
        .collect())
}

/// Gets a canteen by its name.
///
/// # Arguments
///
/// * `name` - The name of the canteen.
///
/// # Returns
///
/// A result containing an option with the canteen or a request error.
pub async fn get_canteen_by_name(name: &str) -> Result<Option<Canteen>, Error> {
    let all_canteens = get_all_canteens().await?;
    Ok(all_canteens.into_iter().find(|c| c.name == name))
}

/// Gets a list of canteens by their names.
///
/// # Arguments
///
/// * `names` - A vector of names for the canteens.
///
/// # Returns
///
/// A result containing a vector of canteens or a request error.
pub async fn get_canteens_by_names(names: Vec<&str>) -> Result<Vec<Canteen>, Error> {
    let all_canteens = get_all_canteens().await?;
    Ok(all_canteens
        .into_iter()
        .filter(|c| names.contains(&c.name.as_str()))
        .collect())
}

/// Gets a list of canteens in a specified location (city).
///
/// # Arguments
///
/// * `location` - The city where the canteens are located.
///
/// # Returns
///
/// A result containing a vector of canteens or a request error.
pub async fn get_canteens_by_location(location: &str) -> Result<Vec<Canteen>, Error> {
    let canteens = get_all_canteens().await?;
    Ok(canteens
        .into_iter()
        .filter(|c| c.city == location)
        .collect())
}

/// Gets a list of canteens in specified locations (cities).
///
/// # Arguments
///
/// * `locations` - A vector of cities where the canteens are located.
///
/// # Returns
///
/// A result containing a vector of canteens or a request error.
pub async fn get_canteens_by_locations(locations: Vec<&str>) -> Result<Vec<Canteen>, Error> {
    let canteens = get_all_canteens().await?;
    Ok(canteens
        .into_iter()
        .filter(|c| locations.contains(&c.city.as_str()))
        .collect())
}

/// Gets all available canteens.
///
/// # Returns
///
/// A result containing a vector of all canteens or a request error.
pub async fn get_all_canteens() -> Result<Vec<Canteen>, Error> {
    let canteens_url = "https://openmensa.org/api/v2/canteens";
    fetch_from_api(&canteens_url).await
}
