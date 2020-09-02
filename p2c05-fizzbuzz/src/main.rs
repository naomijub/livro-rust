fn main() {
    let val = 4i32;
    let result = fizzbuzz(val);
    println!("fizzbuzz for {} is {}", val, result);
}

// V1
// fn fizzbuzz(i: i32) -> &'static str {
//     if i % 15 == 0 {
//         "FizzBuzz"
//     } else if i % 5 == 0 {
//         "Buzz"
//     } else if i % 3 == 0 {
//         "Fizz"
//     } else {
//         i
//     }
// }

//V2
// fn fizzbuzz(i: i32) -> &'static str {
//     if i % 15 == 0 {
//         "FizzBuzz"
//     } else if i % 5 == 0 {
//         "Buzz"
//     } else if i % 3 == 0 {
//         "Fizz"
//     } else {
//         i.to_string()
//     }
// }

//V3
// fn fizzbuzz(i: i32) -> &'static str {
//     if i % 15 == 0 {
//         "FizzBuzz"
//     } else if i % 5 == 0 {
//         "Buzz"
//     } else if i % 3 == 0 {
//         "Fizz"
//     } else {
//         &i.to_string()
//     }
// }

// V4
// fn fizzbuzz(i: i32) -> &'static str {
//     let val;

//     if i % 15 == 0 {
//         "FizzBuzz"
//     } else if i % 5 == 0 {
//         "Buzz"
//     } else if i % 3 == 0 {
//         "Fizz"
//     } else {
//         val = i.to_string();
//         &val
//     }
// }

// V5
// fn fizzbuzz(i: i32) -> String {
//     if i % 15 == 0 {
//         "FizzBuzz".to_string()
//     } else if i % 5 == 0 {
//         "Buzz".to_string()
//     } else if i % 3 == 0 {
//         "Fizz".to_string()
//     } else {
//         i.to_string()
//     }
// }

// V6
use std::borrow::Cow;
fn fizzbuzz(i: i32) -> Cow<'static, str> {
    if i % 15 == 0 {
        "FizzBuzz".into()
    } else if i % 5 == 0 {
        "Buzz".into()
    } else if i % 3 == 0 {
        "Fizz".into()
    } else {
        i.to_string().into()
    }
}
