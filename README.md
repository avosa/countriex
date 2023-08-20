# Countriex

Countriex is a Rust library that provides information about countries around the world. It includes data on a wide range of topics, including population, location, and official languages. It includes data from ISO 3166 (countries and states/subdivisions), ISO 4217 (currency), and E.164 (phone numbers).

## Usage

To use the Countriex library in your Rust project, add the following line to your Cargo.toml file:

```toml
[dependencies]
countriex = "0.1.1"
```

Or install it directly as:

```bash
cargo add countriex
```

Then, import the library into your Rust code using the `use` keyword:

```rust
use countriex::{all, get_by, filter};
```

## Examples

```rust
use countriex::{all, get_by, filter};

fn main() {
    // Get the total number of countries
    let total_countries = all();
    println!("Number of countries: {:?}", total_countries);

    // Find a country by its 2-letter ISO code (alpha2)
    let country_alpha2 = get_by("alpha2", "US");
    println!("Country by alpha2: {:?}", country_alpha2);

    // Find a country by its 3-letter ISO code (alpha3)
    let country_alpha3 = get_by("alpha3", "USA");
    println!("Country by alpha3: {:?}", country_alpha3);

    // Find a country by its common name
    let country_name = get_by("name", "United States");
    println!("Country by name: {:?}", country_name);

    // Find all countries in a specific continent
    let countries_in_asia = filter("continent", "Asia");
    println!("Countries in Asia: {:?}", countries_in_asia);

    // Find all countries in a specific region
    let countries_in_southern_europe = filter("region", "Southern Europe");
    println!("Countries in Southern Europe: {:?}", countries_in_southern_europe);

    // Find all EU member countries
    let eu_member_countries = filter("eu_member", "true");
    println!("EU member countries: {:?}", eu_member_countries);
}
```

## Functions

Countriex provides the following functions:

- `all()`: Returns the total number of countries.

* `get_by(key: &str, value: &str) -> Option<Country>`: Returns the first matching country with the given criteria, or None if a country with that data does not exist.
* `filter(key: &str, value: &str) -> Vec<Country>`: Returns all countries matching the given criteria, or [] if the criteria does not match any countries.

## Credits

This library uses data from the [restcountries](https://restcountries.com/) API and is inspired by Ruby's [countriex](https://github.com/navinpeiris/countriex).
