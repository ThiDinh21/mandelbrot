use std::{f64, str::FromStr};

use num_complex::Complex;

fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }
    None
}

/// Parse str s into a pair of coord x and y, separated by a delim
/// i.e. "300x400", "5.0,8.0"
/// x and y must have FromStr implemented. Returns a pair Some(x, y), any other cases return None
fn parse_pair<T: FromStr>(s: &str, delim: &str) -> Option<(T, T)> {
    s.find(delim).map_or(None, |index| {
        match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(x), Ok(y)) => Some((x, y)),
            _ => {
                dbg!(&s[index + 1..]);
                None
            }
        }
    })
}

/// Parse a pair of floating point into a complex num
fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ",") {
        Some((re, im)) => Some(Complex { re, im }),
        _ => None,
    }
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ","), None);
    assert_eq!(parse_pair::<i32>("10,", ","), None);
    assert_eq!(parse_pair::<i32>("10,100", ","), Some((10, 100)));
    assert_eq!(parse_pair::<i32>(",100", ","), None);
    assert_eq!(parse_pair::<f64>("10,20xy", ","), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", "x"), Some((0.5, 1.5)));
}

#[test]
fn test_parse_complex_num() {
    assert_eq!(
        parse_complex("-8,17.5"),
        Some(Complex::<f64> { re: -8.0, im: 17.5 })
    );
}
