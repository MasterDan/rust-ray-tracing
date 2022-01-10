use crate::Vec3;

#[test]
pub fn add() {
    let first = Vec3::new(1f64, 2f64, 3f64);
    let second = Vec3::new(4f64, 5f64, 6f64);
    let third = Vec3::new(5f64, 7f64, 9f64);
    assert_eq!(first + second, third);
}

#[test]
pub fn mul() {
    let first = Vec3::new(1f64, 2f64, 3f64);
    let second = Vec3::new(2f64, 4f64, 6f64);
    assert_eq!(first * 2.0, second);
}

#[test]
pub fn mul_assign() {
    let mut first = Vec3::new(1f64, 2f64, 3f64);
    first *= 2.0;
    let second = Vec3::new(2f64, 4f64, 6f64);
    assert_eq!(first, second);
}

#[test]
pub fn div() {
    let first = Vec3::new(1f64, 2f64, 3f64);
    let second = Vec3::new(2f64, 4f64, 6f64);
    assert_eq!(second / 2.0, first);
}

#[test]
pub fn div_assign() {
    let first = Vec3::new(1f64, 2f64, 3f64);
    let mut second = Vec3::new(2f64, 4f64, 6f64);
    second /= 2.0;
    assert_eq!(second, first);
}
