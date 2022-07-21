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

## Built-in implementations

Implementations are provided for generic arrays `[T: Vet; N]` and the standard
library types `Vec<T: Vet>` and `Option<T: Vet>`.

Arrays and `Vec`s are only valid if all of their individual elements are valid:

```rust
let usernames = vec![
    Username("日向".to_string()),
    Username("seán462".to_string()),
    Username("lone wolf".to_string())
].vet();
// Invalid, whitespace in the third element

let contact_numbers = [
    PhoneNumber("427-313-0255"),
    PhoneNumber("+1 (708) 484-0523")
].vet();
// Valid, all elements passed vetting
```

Options containing `None` are always valid:

```rust
let mut email: Option<EmailAddress> = None;
email.vet(); // Valid

let mut email: Option<EmailAddress> = Some("benjamin@@metanomial.com");
email.vet(); // Invalid, regex test failed
```

## No-std support

The default `std` feature flag can be disabled to use this library in no_std
contexts. Modify your dependency entry in `Cargo.toml` like so:

```toml
[dependencies]
vet = { version = "0.1", default-features = false }
```

In no_std contexts with a memory allocator, implementations for `Vec` can be
reenabled with the `alloc` feature flag:

```toml
[dependencies]
vet = { version = "0.1", default-features = false, features = ["alloc"] }
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
