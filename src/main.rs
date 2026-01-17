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

/// Convert RowxCol of img -> XxY on the complex plane
/// `bounds`: width and height of img in pixels
/// `pixel`: (col, row) of the point on img in pixels
/// `upper_left` and `lower_right`: determine the region of the complex plane that the image covers
fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );
    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
    }
}

/// `pixels`: a buffer to store a rectangle of the Mandelbrot set
/// `bounds`: width and height of the pixels puffer
/// `upper_left` and `lower_right`: specify the points on the complex plane corresponding to the
/// upper left and lower right of the pixels buffer
fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    assert!(pixels.len() == bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for col in 0..bounds.0 {
            let point = pixel_to_point(bounds, (col, row), upper_left, lower_right);
            pixels[row * bounds.0 + col] = match escape_time(point, 255) {
                Some(count) => 255 - count as u8,
                None => 0,
            }
        }
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

#[test]
fn test_pixel_to_point() {
    assert_eq!(
        pixel_to_point(
            (100, 200),
            (25, 175),
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 }
        ),
        Complex {
            re: -0.5,
            im: -0.75
        }
    );
}
