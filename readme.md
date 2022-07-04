# Vet

A library for validation of arbitrary types.

## Usage

Add a dependency entry in `Cargo.toml`.

```toml
[dependencies]
vet = "0.1"
```

```rust
use vet::{Vet, Vetted};

#[derive(Debug)]
struct Username(String);

#[derive(Debug, PartialEq)]
enum UsernameVetError {
    TooShort,
    TooLong,
    InvalidChar,
}

impl Vet for Username {
    type VetError = UsernameVetError;
    fn is_valid(&self) -> Result<(), Self::VetError> {
        if self.0.len() < 3 {
            return Err(Self::VetError::TooShort);
        }
        if self.0.len() > 32 {
            return Err(Self::VetError::TooLong);
        }
        if self.0.chars().any(|c| !c.is_alphanumeric()) {
            return Err(Self::VetError::InvalidChar);
        }
        Ok(())
    }
}

fn main() {
    let username = Username(String::from("hi"));
    assert_eq!(username.is_valid(), Err(UsernameVetError::TooShort));
    
    let username = Username(String::from("benjamin"));
    match username.vet() {
        Ok(username) => create_account(username),
        Err(error) => println!("Could not create account: {:?}", error),
    }
}

fn create_account(username: Vetted<Username>) {
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
