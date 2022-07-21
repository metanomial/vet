# Vet

Arbitrary type validation.

## Usage

Add a dependency entry in `Cargo.toml`.

```toml
[dependencies]
vet = "0.1"
```

Implement the `Vet` trait on your type.

```rust
use vet::{Valid, Vet};

// A valid username consists of between 3 and 32 alphanumeric characters
#[derive(Debug)]
struct Username(String);

#[derive(Debug, PartialEq)]
enum InvalidUsername {
    TooShort, // Under 3 characters
    TooLong, // Over 3 characters
    InvalidChar, // Contains non-alphanumeric character
}

impl Vet for Username {
    type Error = InvalidUsername;

    // Arbitrary logic to validate the Username type
    fn is_valid(&self) -> Result<(), Self::Error> {
        if self.0.len() < 3 {
            return Err(Self::Error::TooShort);
        }
        if self.0.len() > 32 {
            return Err(Self::Error::TooLong);
        }
        if self.0.chars().any(|c| !c.is_alphanumeric()) {
            return Err(Self::Error::InvalidChar);
        }
        Ok(())
    }
}
```

`Valid`-wrapped types provide safety guarantees about their contents.

```rust
fn main() {
    let args: Vec<String> = env::args().collect();
    
    let username: Username = Username(args[1].clone());
    let username: Result<Valid<Username>, InvalidUsername> = username.vet();

    match username {
        Ok(n) => create_account(n),
        Err(e) => eprintln!("Invalid username: {:?}", e),
    }
}

// Any `Valid<Username>` passed is guaranteed to be 3-32 alphanumeric characters.
fn create_account(username: Valid<Username>) {
    let username = username.into_inner(); // Unwrap

    println!("Account {:?} created", username);
}
```

## License

Licensed under either of

- Apache License, Version 2.0 ([license-apache.txt](license-apache.txt) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([license-mit.txt](license-mit.txt) or
  http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
