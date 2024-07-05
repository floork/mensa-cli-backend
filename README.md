# OpenMensa Rust Interface

This Rust module provides data structures and functions for interacting with canteens and their meals using the OpenMensa API. It allows you to retrieve information about canteens, their locations, and the meals they offer on specific dates.

## Table of Contents

1. [Installation](#installation)
2. [Usage](#usage)
   - [Fetching Meals](#fetching-meals)
   - [Fetching Canteens](#fetching-canteens)
3. [Data Structures](#data-structures)
   - [Canteen](#canteen)
   - [Prices](#prices)
   - [Meal](#meal)
4. [Functions](#functions)
   - [fetch_from_api](#fetch_from_api)
   - [get_meals](#get_meals)
   - [get_canteen_by_id](#get_canteen_by_id)
   - [get_canteens_by_ids](#get_canteens_by_ids)
   - [get_canteen_by_name](#get_canteen_by_name)
   - [get_canteens_by_names](#get_canteens_by_names)
   - [get_canteens_by_location](#get_canteens_by_location)
   - [get_canteens_by_locations](#get_canteens_by_locations)
   - [get_all_canteens](#get_all_canteens)
5. [Examples](#examples)
6. [License](#license)

## Installation

Add the following dependencies to your `Cargo.toml` file:

```toml
[dependencies]
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
```

## Usage

To use this module, you need to have an asynchronous runtime such as `tokio` set up in your project. Below are examples of how to fetch canteens and meals from the OpenMensa API.

### Fetching Meals

Fetch meals from a specific canteen on a given date:

```rust
use openmensa::{get_meals, Canteen};
use tokio;

#[tokio::main]
async fn main() {
    let canteen = Canteen {
        id: 1,
        name: "Canteen 1".to_string(),
        city: "City".to_string(),
        address: "Address".to_string(),
        coordinates: None,
    };

    match get_meals(&canteen, "2024-07-05").await {
        Ok(meals) => println!("{:?}", meals),
        Err(e) => eprintln!("Error fetching meals: {}", e),
    }
}
```

### Fetching Canteens

Fetch all available canteens:

```rust
use openmensa::get_all_canteens;
use tokio;

#[tokio::main]
async fn main() {
    match get_all_canteens().await {
        Ok(canteens) => println!("{:?}", canteens),
        Err(e) => eprintln!("Error fetching canteens: {}", e),
    }
}
```

## Data Structures

### Canteen

Represents a canteen with its details.

```rust
#[derive(Deserialize, Debug)]
pub struct Canteen {
    pub id: u32,
    pub name: String,
    pub city: String,
    pub address: String,
    pub coordinates: Option<[f64; 2]>,
}
```

### Prices

Represents the price structure for different user groups.

```rust
#[derive(Debug, Deserialize)]
pub struct Prices {
    pub students: Option<f64>,
    pub employees: Option<f64>,
    pub pupils: Option<f64>,
    pub others: Option<f64>,
}
```

### Meal

Represents a meal with its details.

```rust
#[derive(Debug, Deserialize)]
pub struct Meal {
    pub id: u64,
    pub name: String,
    pub category: String,
    pub prices: Prices,
    pub notes: Vec<String>,
}
```

## Functions

### fetch_from_api

Fetches data from the given URL and deserializes it into the specified type.

```rust
async fn fetch_from_api<T: for<'de> Deserialize<'de>>(url: &str) -> Result<T, Error>
```

### get_meals

Gets the meals available at a specified canteen on a given date.

```rust
pub async fn get_meals(canteen: &Canteen, date: &str) -> Result<Vec<Meal>, Error>
```

### get_canteen_by_id

Gets a canteen by its unique identifier.

```rust
pub async fn get_canteen_by_id(id: u32) -> Result<Option<Canteen>, Error>
```

### get_canteens_by_ids

Gets a list of canteens by their unique identifiers.

```rust
pub async fn get_canteens_by_ids(ids: Vec<u32>) -> Result<Vec<Canteen>, Error>
```

### get_canteen_by_name

Gets a canteen by its name.

```rust
pub async fn get_canteen_by_name(name: &str) -> Result<Option<Canteen>, Error>
```

### get_canteens_by_names

Gets a list of canteens by their names.

```rust
pub async fn get_canteens_by_names(names: Vec<&str>) -> Result<Vec<Canteen>, Error>
```

### get_canteens_by_location

Gets a list of canteens in a specified location (city).

```rust
pub async fn get_canteens_by_location(location: &str) -> Result<Vec<Canteen>, Error>
```

### get_canteens_by_locations

Gets a list of canteens in specified locations (cities).

```rust
pub async fn get_canteens_by_locations(locations: Vec<&str>) -> Result<Vec<Canteen>, Error>
```

### get_all_canteens

Gets all available canteens.

```rust
pub async fn get_all_canteens() -> Result<Vec<Canteen>, Error>
```

## Examples

Fetch and print all available canteens:

```rust
use openmensa::get_all_canteens;
use tokio;

#[tokio::main]
async fn main() {
    match get_all_canteens().await {
        Ok(canteens) => {
            for canteen in canteens {
                println!("{:?}", canteen);
            }
        }
        Err(e) => eprintln!("Error fetching canteens: {}", e),
    }
}
```

Fetch and print meals from a specific canteen on a given date:

```rust
use openmensa::{get_meals, Canteen};
use tokio;

#[tokio::main]
async fn main() {
    let canteen = Canteen {
        id: 1,
        name: "Canteen 1".to_string(),
        city: "City".to_string(),
        address: "Address".to_string(),
        coordinates: None,
    };

    match get_meals(&canteen, "2024-07-05").await {
        Ok(meals) => {
            for meal in meals {
                println!("{:?}", meal);
            }
        }
        Err(e) => eprintln!("Error fetching meals: {}", e),
    }
}
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
