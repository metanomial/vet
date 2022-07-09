//! Provides an interface for validation of arbitrary types.
//!
//! The `Vet` trait requires an implementation of a `VetError` type alias and
//! an `is_valid` method, which executes arbitrary validation logic and returns
//! a result indicating whether the instance is valid.
//!
//! The `Vet` trait also provides a default implementation of a `vet` method
//! which, dependant on the result of `is_valid`, either wraps the instance with
//! a `Valid<T>` struct or propagates the validation error.
//!
//! The `Valid<T>` wrapper guarantees that the inner value was successfully
//! validated and remains immutable as long as it is wrapped.
//!
//! Implementations for the common standard library types `Vec<T>` and
//! `Option<T>` are provided.
//!
//! # Examples
//!
//! ```
//! use vet::{Valid, Vet};
//!
//! #[derive(Debug)]
//! struct Username(String);
//!
//! #[derive(Debug, PartialEq)]
//! enum UsernameVetError {
//!     TooShort,
//!     TooLong,
//!     InvalidChar,
//! }
//!
//! impl Vet for Username {
//!     type VetError = UsernameVetError;
//!
//!     fn is_valid(&self) -> Result<(), Self::VetError> {
//!         if self.0.len() < 3 {
//!             return Err(Self::VetError::TooShort);
//!         }
//!         if self.0.len() > 32 {
//!             return Err(Self::VetError::TooLong);
//!         }
//!         if self.0.chars().any(|c| !c.is_alphanumeric()) {
//!             return Err(Self::VetError::InvalidChar);
//!         }
//!         Ok(())
//!     }
//! }
//!
//! fn main() {
//!     let username = Username(String::from("hi"));
//!     assert_eq!(username.is_valid(), Err(UsernameVetError::TooShort));
//!
//!     let username = Username(String::from("benjamin"));
//!     match username.vet() {
//!         Ok(username) => create_account(username),
//!         Err(error) => println!("Could not create account: {:?}", error),
//!     }
//! }
//!
//! fn create_account(username: Valid<Username>) {
//!     println!("Account {:?} created", username);
//! }
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(test)]
mod tests;

/// A wrapper around a validated instance
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Valid<T>(T);

impl<T> core::ops::Deref for Valid<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// An interface for arbitrary type validation
pub trait Vet {
    /// The error returned by failed validation
    type VetError;

    /// Executes arbitrary validation logic on this instance.
    fn is_valid(&self) -> Result<(), Self::VetError>;

    /// Validates this instance and results in a wrapped instance if successful.
    fn vet(self) -> Result<Valid<Self>, Self::VetError>
    where
        Self: Sized,
    {
        match self.is_valid() {
            Ok(()) => Ok(Valid(self)),
            Err(e) => Err(e),
        }
    }
}

impl<T: Vet> Vet for Option<T> {
    type VetError = T::VetError;

    fn is_valid(&self) -> Result<(), Self::VetError> {
        match self {
            Some(t) => t.is_valid(),
            None => Ok(()),
        }
    }
}

#[cfg(feature = "alloc")]
impl<T: Vet> Vet for alloc::vec::Vec<T> {
    type VetError = T::VetError;

    fn is_valid(&self) -> Result<(), Self::VetError> {
        self.iter().try_for_each(|t| t.is_valid())
    }
}
