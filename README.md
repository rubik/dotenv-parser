<div align="center">
  <h1>dotenv-parser</h1>
  <p>Pest-based parser for `.env` files.</p>
  <a target="_blank" href="https://travis-ci.org/rubik/dotenv-parser">
    <img src="https://img.shields.io/travis/rubik/dotenv-parser?style=for-the-badge" alt="Build">
  </a>
  <a target="_blank" href="https://coveralls.io/github/rubik/dotenv-parser">
    <img src="https://img.shields.io/coveralls/github/rubik/dotenv-parser?style=for-the-badge" alt="Code Coverage">
  </a>
  <a target="_blank" href="https://crates.io/crates/dotenv-parser">
   <img src="https://img.shields.io/crates/d/dotenv-parser?style=for-the-badge" alt="Downloads (all time)">
  <a>
  <a href="https://github.com/rubik/dotenv-parser/blob/master/LICENSE">
    <img src="https://img.shields.io/crates/l/dotenv-parser?style=for-the-badge" alt="ISC License">
  </a>
  <br>
  <br>
</div>

dotenv-parser is a minimal crate that exposes an `.env` file parser generated
by [Pest](https://pest.rs).

# Usage
The API is minimal: this crate exposes a single `parse_dotenv` function which
accepts a string reference and returns a
[`BTreeMap`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html)
wrapped in a `Result`. The parser handles comments, strings and the `export`
syntax automatically. This program

```rust
use dotenv_parser::parse_dotenv;

fn main() {
    let source = r#"
        ENV_FOR_HYDRO='testing 2' # another one here
        export USER_ID=5gpPN5rcv5G41U_S
        API_TOKEN=30af563ccc668bc8ced9e24e  # relax! these values are fake
        APP_SITE_URL=https://my.example.com
    "#;
    println!("{:#?}", parse_dotenv(source).unwrap());
}
```

prints

```rust
{
    "API_TOKEN": "30af563ccc668bc8ced9e24e",
    "APP_SITE_URL": "https://my.example.com",
    "ENV_FOR_HYDRO": "testing 2",
    "USER_ID": "5gpPN5rcv5G41U_S",
}
```
