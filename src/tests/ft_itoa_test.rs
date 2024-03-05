#[path = "../libft/ft_itoa.rs"] mod ft_itoa;
use std::i128;
use std::i64;
use std::i32;
use std::i16;
use std::i8;
use ft_itoa::ft_itoa;

 
#[test]
fn test_itoa_normal() {
    assert_eq!(ft_itoa(123), "123");
    assert_eq!(ft_itoa(01), "1");
    assert_eq!(ft_itoa(-124), "-124");
    assert_eq!(ft_itoa(-01), "-1");
    assert_eq!(ft_itoa(-00000000133370), "-133370");
    assert_eq!(ft_itoa(-0000000013337), "-13337");
}

#[test]
fn test_itoa_max_min_i128() {
    assert_eq!(ft_itoa(i128::MAX), i128::MAX.to_string());
    assert_eq!(ft_itoa(i128::MIN), i128::MIN.to_string());
}

#[test]
fn test_itoa_max_min_i64() {
    assert_eq!(ft_itoa(i128::from(i64::MAX)), i64::MAX.to_string());
    assert_eq!(ft_itoa(i128::from(i64::MIN)), i64::MIN.to_string());
}

#[test]
fn test_itoa_max_min_i32() {
    assert_eq!(ft_itoa(i128::from(i32::MAX)), i32::MAX.to_string());
    assert_eq!(ft_itoa(i128::from(i32::MIN)), i32::MIN.to_string());
}

#[test]
fn test_itoa_max_min_i16() {
    assert_eq!(ft_itoa(i128::from(i16::MAX)), i16::MAX.to_string());
    assert_eq!(ft_itoa(i128::from(i16::MIN)), i16::MIN.to_string());
}

#[test]
fn test_itoa_max_min_i8() {
    assert_eq!(ft_itoa(i128::from(i8::MAX)), i8::MAX.to_string());
    assert_eq!(ft_itoa(i128::from(i8::MIN)), i8::MIN.to_string());
}
