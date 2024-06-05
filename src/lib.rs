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

pub async fn get_meals(canteen: &Canteen, date: &str) -> Result<Vec<Meal>, Error> {
    let canteen_id = canteen.id;
    let menu_url = format!(
        "https://openmensa.org/api/v2/canteens/{}/days/{}/meals",
        canteen_id, date
    );
    let response = reqwest::get(&menu_url).await?.json::<Vec<Meal>>().await?;
    Ok(response)
}

pub async fn get_canteens_by_id(id: u32) -> Result<Vec<Canteen>, Error> {
    let mut city = vec![];

    let canteens = get_all_canteens().await?;

    for canteen in canteens {
        if canteen.id == id {
            city.push(canteen);
        }
    }

    Ok(city)
}

pub async fn get_canteens_by_ids(ids: Vec<u32>) -> Result<Vec<Canteen>, Error> {
    let mut city = vec![];

    let canteens = get_all_canteens().await?;

    for canteen in canteens {
        if ids.contains(&canteen.id) {
            city.push(canteen);
        }
    }

    Ok(city)
}

pub async fn get_canteens_by_location(location: &str) -> Result<Vec<Canteen>, Error> {
    let mut city = vec![];

    let canteens = get_all_canteens().await.unwrap();

    for canteen in canteens {
        if canteen.city == location {
            let name_parts: Vec<&str> = canteen.name.split(',').collect();
            let name = name_parts.get(1).unwrap_or(&"").to_owned();
            city.push(Canteen {
                id: canteen.id,
                name: name.to_string(),
                city: canteen.city.clone(),
                address: canteen.address.clone(),
                coordinates: canteen.coordinates,
            });
        }
    }
    Ok(city)
}

pub async fn get_all_canteens() -> Result<Vec<Canteen>, Error> {
    let canteens_url = "https://openmensa.org/api/v2/canteens";

    let response = reqwest::get(canteens_url)
        .await?
        .json::<Vec<Canteen>>()
        .await?;
    Ok(response)
}
