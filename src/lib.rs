use reqwest::Error;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Canteen {
    id: u32,
    name: String,
    city: String,
    address: String,
    coordinates: Option<[f64; 2]>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Prices {
    students: Option<f64>,
    employees: Option<f64>,
    pupils: Option<f64>,
    others: Option<f64>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Meal {
    id: u64,
    name: String,
    category: String,
    prices: Prices,
    notes: Vec<String>,
}

async fn fetch_from_api<T: for<'de> Deserialize<'de>>(url: &str) -> Result<T, Error> {
    let response = reqwest::get(url).await?.json::<T>().await?;
    Ok(response)
}

pub async fn get_meals(canteen: &Canteen, date: &str) -> Result<Vec<Meal>, Error> {
    let canteen_id = canteen.id;
    let menu_url = format!(
        "https://openmensa.org/api/v2/canteens/{}/days/{}/meals",
        canteen_id, date
    );
    fetch_from_api(&menu_url).await
}

pub async fn get_canteens_by_id(id: u32) -> Result<Vec<Canteen>, Error> {
    let canteens = get_all_canteens().await?;
    let city: Vec<Canteen> = canteens.into_iter().filter(|c| c.id == id).collect();
    Ok(city)
}

pub async fn get_canteens_by_ids(ids: Vec<u32>) -> Result<Vec<Canteen>, Error> {
    let canteens = get_all_canteens().await?;
    let city: Vec<Canteen> = canteens
        .into_iter()
        .filter(|c| ids.contains(&c.id))
        .collect();
    Ok(city)
}

pub async fn get_canteens_by_location(location: &str) -> Result<Vec<Canteen>, Error> {
    let canteens = get_all_canteens().await?;
    let city: Vec<Canteen> = canteens
        .into_iter()
        .filter(|c| c.city == location)
        .map(|c| Canteen {
            id: c.id,
            name: c.name.split(',').nth(1).unwrap_or("").to_owned(),
            city: c.city.clone(),
            address: c.address.clone(),
            coordinates: c.coordinates,
        })
        .collect();
    Ok(city)
}

pub async fn get_all_canteens() -> Result<Vec<Canteen>, Error> {
    let canteens_url = "https://openmensa.org/api/v2/canteens";
    fetch_from_api(&canteens_url).await
}
