# `megamind`

![Crates.io](https://img.shields.io/crates/v/megamind?logo=rust)
 ![docs.rs (with version)](https://img.shields.io/docsrs/megamind/latest?logo=docsdotrs)

A library for interacting with the [Genius API](https://docs.genius.com).

This library revolves around providing two things:

1. Idiomatic Rust structures (a.k.a "data models" or whatever else you like to call them) that represent API endpoint data.

2. A simple HTTP client for interacting with the API.

## Usage

```toml
megamind = "*"

# enable the "catchall" feature
megamind = { version = "*", features = ["catchall"] }
```

```rust
use std::{env::var, error::Error};

use megamind::{ClientBuilder, models::Response};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = ClientBuilder::new().auth_token(var("GENIUS_TOKEN")?).build()?;
    let response = client.account().await?;
    if let Response::Success { meta, response } = response {
        assert_eq!(meta.status, 200);
        println!("Howdy, {}!", response.account.user.core.name);
    }
    Ok(())
}
```

## Features

- [X] Get current user
- [X] Get an annotation
- [X] Get an artist
- [X] Get a list of referents
- [X] Get search results
- [X] Get a user
- [X] Get a web page
- [X] Get a song
- [ ] Get an album
- [ ] Get a referent
- [ ] Create an annotation
- [ ] Update an annotation
- [ ] Delete an annotation
- [ ] Upvote/downvote/unvote for an annotation

## Ongoing Improvements

There are a few general areas in which the library can be improved:

1. Data model documentation
    - The Genius documentation lacks information for many data fields
    - The surface area of the web API is large, even without some of the undocumented endpoints
2. Data model specificity
    - Many fields could be `enum`s instead of `String`s, but they lack the documentation necessary to be confidently converted
    - Some fields are simply `()` or `Vec<()>` because no endpoints have been found to hold data in them
    - Data model field presence isn't consistent across endpoints and endpoint values
3. Data model ergonomics
    - How nested is too nested for users of this library?
    - Naming conventions for the shared and nested structures can be awkward at times (`response.account.user.core.name` isn't as intuitive as `response.account.name`)
    - Users might need more derived traits than what is currently offered (e.g., `Eq`, `Hash`, etc.)
4. Error handling
    - The JSON parsing error given by `Reqwest` isn't always helpful
5. Testing
    - Endpoints with edge cases aren't always covered by the integration tests, so users will need to encounter them in the wild
    - Tests functions could be simplified with better fixtures and/or added macros
    - How should this library approach unit tests?

## Issues and Pull Requests

Issues and pull requests are welcome, but please be respectful. It's also generally a good idea to include an endpoint with values if the issue or pull request aims to resolve an issue with parsing a particular endpoint response, or a simple explanation of how the issue or pull request improves efforts in documentation, ergonomics, and/or testing.

## Can you call it an FAQ if the only person asking the questions is the maintainer?

> What makes this library different from libraries like
> [`genius_rs`](https://github.com/alt-art/genius-rs) and [`genius_rust`](https://github.com/tsirysndr/genius-rust)?

The big difference is that `genius_rs` and `genius_rust` both make use of large, general models for multiple endpoints, resulting in many fields being `Option<...>`. This could be avoided by simply omitting the field in certain endpoints and including it in others. To achieve this, `megamind` makes use of highly nested structures that are flattened during serialization and deserialization. This means fewer `unwrap`s for you, the API consumer!

> What is the `catchall` feature, and why is it a thing?

`catchall` enables an extra field on most of the data models that catches all missing JSON fields. This is useful for two reasons:

1. it improves this library's development process by pooling unimplemented data into one place (Serde will simply drop them if fields are not defined for them)
2. and it gives users flexibility by letting them access a new or missing field without waiting for a library update.

It's also unfortunately just a consequence of the web API itself being a bit unwieldy and underdocumented.

> Why is the crate called `megamind`?

Genius... Big-Brained Person... [Megamind](https://en.wikipedia.org/wiki/Megamind).
