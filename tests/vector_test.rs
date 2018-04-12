extern crate radium;

use radium::vector::Vec3;

#[test]
fn adding_two_vectors() {
    assert_eq!(Vec3::new(1.0, 1.0, 1.0) + Vec3::new(2.0, 3.0, 4.0), Vec3::new(3.0, 4.0, 5.0));
}

