[![Crates.io](https://img.shields.io/crates/v/roboat.svg)](https://crates.io/crates/roboat)
[![Documentation](https://docs.rs/roboat/badge.svg)](https://docs.rs/roboat/)
[![dependency status](https://deps.rs/repo/github/chloe-woahie/roboat/status.svg)](https://deps.rs/repo/github/chloe-woahie/roboat)

[![](https://dcbadge.vercel.app/api/server/QmBEgPaFSD)](https://discord.gg/QmBEgPaFSD)

<img align="right" src="images/icon.png" height="150px" alt="roboat logo">

# roboat
A high performance interface for the Roblox API.

This library is designed to be high-performance capable, meaning that it supports proxies
and is capable of making requests in parallel.

# Documentation
Extensive documentation is used throughout this crate. 
All public methods in this crate are documented and have at least one corresponding example.

Documentation can be found [here](https://docs.rs/roboat/).

# Covered Endpoints
* Catalog API - [`catalog.roblox.com/*`]
    - Item Details - `/v1/catalog/items/details`
* Economy API - [`economy.roblox.com/*`]
    - Robux Balance - `/v1/users/{user_id}/currency`
    - Resellers - `/v1/assets/{item_id}/resellers`
    - User Sales - `/v2/users/{user_id}/transactions?transactionType=Sale`
    - Put Limited On Sale - `/v1/assets/{item_id}/resellable-copies/{uaid}`
    - Take Limited Off Sale - `/v1/assets/{item_id}/resellable-copies/{uaid}`
    - Purchase Limited - `/v1/purchases/products/{product_id}`
* Users API - [`users.roblox.com/*`]
    - User Information - `/v1/users/authenticated`
    - User Search - `/v1/users/search`
* Presence API - [`presence.roblox.com/*`]
    - Register Presence - `/v1/presence/register-app-presence`
* Trades API - [`trades.roblox.com/*`]
    - Trades List - `/v1/trades/{trade_type}`

# Setup
You can add the latest version of roboat to your project by running:
```bash
cargo add roboat
```

Alternatively, you can add a specific version of roboat to your project by adding the crate to your `Cargo.toml`:

```toml
[dependencies]
roboat = "0.11.1"
```

# Quick Start Examples

## Example 1

This code snippet allows you to get your current robux, id, username, and display name.

```rust
// Replace this value with your own roblosecurity token.
const ROBLOSECURITY: &str = "your-roblosecurity-token";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = roboat::ClientBuilder::new()
        .roblosecurity(ROBLOSECURITY.to_string())
        .build();

    let robux = client.robux().await?;
    let user_id = client.user_id().await?;
    let username = client.username().await?;
    let display_name = client.display_name().await?;    

    println!("Robux: {}", robux);
    println!("User ID: {}", user_id);
    println!("Username: {}", username);
    println!("Display Name: {}", display_name);

    Ok(())   
}
```

## Example 2

This code snippet allows you to view the lowest price of a limited item by
fetching a list of reseller listings.

```rust
// Replace this value with your own roblosecurity token.
const ROBLOSECURITY: &str = "your-roblosecurity-token";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = roboat::ClientBuilder::new()
        .roblosecurity(ROBLOSECURITY.to_string())
        .build();

    let item_id = 1365767;
    let limit = roboat::Limit::Ten;
    let cursor = None;

    let (resellers, _) = client.resellers(item_id, limit, cursor).await?;

    println!("Lowest Price for Valkyrie Helm: {}", resellers[0].price);  

    Ok(())   
}
```

## Example 3

This code snippet allows you to get the details of an item.

```rust
use roboat::catalog::avatar_catalog::{ItemArgs, ItemType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = roboat::ClientBuilder::new().build();

    let item = ItemArgs {
        item_type: ItemType::Asset,
        id: 1365767,
    };

    let details = &client.item_details(vec![item]).await?[0];

    let name = &details.name;
    let description = &details.description;
    let creator_name = &details.creator_name;
    let price = details.price.unwrap_or(0);

    println!("Name: {}", name);
    println!("Description: {}", description);
    println!("Creator Name: {}", creator_name);
    println!("Price: {}", price);

    Ok(())   
}
```

# More Examples
More examples can be found in the [examples](examples) directory.

# Related Crates
This crate is a sister crate of [roli](https://crates.io/crates/roli), an API wrapper for [Rolimons.com](https://www.rolimons.com/).

# Contributing
Pull requests and issues are welcome! 

Please refer to [CONVENTIONS.md](CONVENTIONS.md) for information on conventions used in this crate.

Additional resources used to help make this crate are available in [RESOURCES.md](RESOURCES.md).

# License
MIT License
