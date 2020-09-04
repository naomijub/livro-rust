fn main() {
    println!("Hello, world!");
}

// Número de Armstrong 
const RADIX: u32 = 10;

pub fn is_armstrong_number(num: u32) -> bool {
    let pow = num.to_string().len() as u32;
    num.to_string()
      .chars()
      .into_iter()
      // or:  .map(|c| c.to_digit(RADIX).unwrap())
      .filter_map(|c| c.to_digit(RADIX).ok_or("err").ok())
      .map(|n| n.pow(pow))
      .fold(0, |acc, n| acc + n) == num
}

#[test]
fn armstrong_numbers() {
    assert!(is_armstrong_number(153));
    assert!(!is_armstrong_number(154));
}

// Isograma
use std::collections::HashMap;

pub fn check(candidate: &str) -> bool {
    candidate
      .to_lowercase()
      .chars()
      .filter(|c| c.is_alphanumeric())
      .fold(HashMap::new(), |mut acc: HashMap<char, i32>, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
        })
      .values()
      .fold(true,|acc,x| 
        if x > &1 { acc && false } 
        else { acc } 
      )
}

#[test]
fn isograma() {
    assert_eq!(
        check("six-Year-old"),
        true,
    );

    assert_eq!(
        check("Alphabet"),
        false,
    );
}

// n-ésimo primo

fn is_prime(n: u32) -> bool {
    use rayon::prelude::*;

    !(2u32..n.div_euclid(2) + 1)
        .collect::<Vec<u32>>()
        .par_iter()
        .any(|i| n % i == 0)
}

pub fn nth(n: u32) -> u32 {
    match n {
        0 => 2u32,
        n => (1u32..)
            .filter(|num| is_prime(*num))
            .nth((n + 1) as usize)
            .unwrap(),
    }
}

#[test]
fn test_sixth_prime() {
    assert_eq!(nth(5), 13);
}

// #[test]
// fn test_big_prime() {
//     assert_eq!(nth(10_000), 104_743);
// }

// Series
pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len <= 0 { return vec!["".to_string(); digits.len() + 1]; }
    digits.chars()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .windows(len)
        .map(|v| v.join(""))
        .collect::<Vec<String>>()
}

#[test]
fn series_with_length_2() {
    let expected = vec![
        "92".to_string(),
        "20".to_string(),
        "01".to_string(),
        "17".to_string(),
    ];
    assert_eq!(series("92017", 2), expected);
}

// Series ordem
pub fn series_ord(digits: &str, len: usize) -> Vec<usize> {
    if len <= 0 { return Vec::new(); }
    let mut serie = digits.chars()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .windows(len)
        .map(|v| v.join(""))
        .filter_map(|e| e.parse::<usize>().ok())
        .collect::<Vec<usize>>();
    serie.sort();
    //serie.reverse();
    serie
}

#[test]
fn series_ord_test() {
    let expected = vec![
        01usize,
        17usize,
        20usize,
        92usize,
    ];
    assert_eq!(series_ord("92017", 2), expected);
}

// Max by second series char
pub fn max_by_second_char_series(digits: &str, len: usize) -> usize {
    digits.chars()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .windows(len)
        .max_by(|a, b| a[1].parse::<usize>().unwrap_or(0).cmp(&b[1].parse::<usize>().unwrap_or(0)))
        .unwrap()
        .to_vec()
        .join("")
        .parse::<usize>().unwrap_or(0)
}

#[test]
fn max_by_series_test() {
    assert_eq!(max_by_second_char_series("92017", 2), 17usize);
}

// Soma de multiplos
pub fn sum_of_multiples(limite: u32, fatores: &[u32]) -> u32 {
    (1..limite)
        .filter(|numero| fatores.iter().filter(|x| **x > 0).any(|x| numero % x == 0))
        .sum()
}

#[test]
fn some_pairs_of_factors_relatively_prime_and_some_not() {
    assert_eq!(4419, sum_of_multiples(150, &[5, 6, 8]))
}

#[test]
fn one_factor_is_a_multiple_of_another() {
    assert_eq!(275, sum_of_multiples(51, &[5, 25]))
}

// Luhn
pub fn is_valid(code: &str) -> bool {
    if code.trim().len() <= 1 {
        return false;
    }

    code
        .replace(" ", "")
        .chars()
        .rev()
        .map(|ch| ch.to_digit(10))
        .enumerate()
        .try_fold(0u32, |acc, (i, n)| 
            match (i % 2, n) {
                (0, Some(v)) => Some(acc + v),
                (1, Some(v)) if v == 9 => Some(acc + v),
                (1, Some(v)) => Some(acc + ((v * 2) % 9)),
                _ => None
            })
        .map_or(false, |v| v % 10 == 0)
}

#[test]
fn test_invalid_credit_card() {
    assert_eq!(is_valid("8273 1232 7352 0569"), false);
}

#[test]
fn test_valid_number_with_an_even_number_of_digits() {
    assert_eq!(is_valid("095 245 88"), true);
}

#[test]
fn strings_that_contain_non_digits_are_invalid() {
    assert_eq!(is_valid("055a 444 285"), false);
}