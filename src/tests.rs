use super::*;

#[derive(Debug, PartialEq)]
struct EvenUsize(usize);

#[derive(Debug)]
struct OddUsize;

impl Vet for EvenUsize {
    type VetError = OddUsize;
    fn is_valid(&self) -> Result<(), Self::VetError> {
        if self.0 % 2 == 0 {
            Ok(())
        } else {
            Err(OddUsize)
        }
    }
}

#[test]
fn vet_type() {
    let foo = EvenUsize(2);
    assert!(foo.is_valid().is_ok());

    let foo = EvenUsize(3);
    assert!(foo.is_valid().is_err());
}

#[test]
fn vet_array() {
    let foo: [EvenUsize; 0] = [];
    assert!(foo.is_valid().is_ok());

    let foo = [EvenUsize(2), EvenUsize(4)];
    assert!(foo.is_valid().is_ok());

    let foo = [EvenUsize(12), EvenUsize(0), EvenUsize(5)];
    assert!(foo.is_valid().is_err());
}

#[test]
fn vet_option() {
    let foo = None::<EvenUsize>;
    assert!(foo.is_valid().is_ok());

    let foo = Some(EvenUsize(4));
    assert!(foo.is_valid().is_ok());

    let foo = Some(EvenUsize(5));
    assert!(foo.is_valid().is_err());
}

#[test]
fn transpose_valid_option() {
    let foo = None::<EvenUsize>.vet().unwrap();
    assert_eq!(foo.transpose(), None);

    let foo = Some(EvenUsize(84)).vet().unwrap();
    assert_eq!(foo.transpose(), Some(Valid(EvenUsize(84))));
}

#[test]
#[cfg(feature = "alloc")]
fn vet_vec() {
    use alloc::{vec, vec::Vec};

    let foo = Vec::<EvenUsize>::new();
    assert!(foo.is_valid().is_ok());

    let foo = vec![EvenUsize(8)];
    assert!(foo.is_valid().is_ok());

    let foo = vec![EvenUsize(7)];
    assert!(foo.is_valid().is_err());

    let foo = vec![EvenUsize(8), EvenUsize(7)];
    assert!(foo.is_valid().is_err());
}
