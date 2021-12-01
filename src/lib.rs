extern crate lazy_static;
extern crate regex;

use std::fs::File;
use std::str::FromStr;
use std::fmt::Debug;
use std::io::{self,Read};
pub use regex::Regex;
pub use lazy_static::lazy_static;

pub mod intcode;

#[macro_export]
macro_rules! regex_parser {
    ($fname:ident : $typ:ty { $($re_name:ident = $re:expr => |$($cap:ident : $capty:ty),*| $res:expr ),* }) =>
        {
            $crate::lazy_static! {
                $(
                pub static ref $re_name: $crate::Regex = $crate::Regex::new($re).unwrap();
                 )*
            }
            pub fn $fname(s: &str) -> $typ {
                $(
                    if let Some(cap) = $re_name.captures(s) {
                        return {
                            let mut capno = 0;
                            $(capno += 1; let $cap: $capty = cap[capno].parse().unwrap(); )*
                            $res
                        };
                    }
                )*
                panic!("Failed to parse: [[{}]]", s)
            }
            impl std::str::FromStr for $typ {
                type Err = ();
                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    Ok($fname(s))
                }
            }
        }
}

pub fn parse_lines<T:FromStr+Debug>(data: &str) -> Vec<T>
   where <T as FromStr>::Err: Debug
{
    data.lines()
        .map(|s| s.parse().expect("Failed to parse"))
        .collect()
}

fn get_input_str(s: &str) -> io::Result<String> {
    let filename = s;
    let mut f = File::open(&filename)?;
    let mut data = String::new();
    f.read_to_string(&mut data)?;
    Ok(data)
}

pub fn get_input(n: u32) -> io::Result<String> {
    get_input_str(&format!("data/day{}.txt", n))
}

pub fn get_gcd(a: isize, b: isize) -> isize {
    assert!(a >= 0);
    assert!(b >= 0);
    let (mut a, mut b) = if a > b {
        (a, b)
    } else {
        (b, a)
    };
    while b > 0 {
        a = a % b;
        std::mem::swap(&mut a, &mut b);
    }
    a
}

pub fn get_gcd_u(a: usize, b: usize) -> usize {
    let (mut a, mut b) = if a > b {
        (a, b)
    } else {
        (b, a)
    };
    while b > 0 {
        a = a % b;
        std::mem::swap(&mut a, &mut b);
    }
    a
}

#[test]
fn test_gcd() {
    assert_eq!(get_gcd(1, 24), 1);
    assert_eq!(get_gcd(35, 24), 1);
    assert_eq!(get_gcd(12, 24), 12);
    assert_eq!(get_gcd(12, 15), 3);
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
