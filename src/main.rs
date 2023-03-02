mod main_test;

use std::{fmt, mem};
use std::fmt::Formatter;

#[derive(Debug)]
struct MinMax(i64, i64);

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", count, v)?;
        }
        write!(f, "]")
    }
}

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

impl fmt::Binary for Point2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "real: {} + imag: {}", self.real, self.imag)
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({} {})\n({} {})", self.0, self.1, self.2, self.3)
    }
}

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (int_param, bool_param) = pair;
    (bool_param, int_param)
}

fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn addition(number_one: i32, number_two: i32) -> i32 {
    number_one + number_two
}

fn subtraction(number_one: i32, number_two: i32) -> i32 {
    number_one - number_two
}

fn fizz_buzz(number: i32) -> &'static str {
    if number % 3 == 0 && number % 5 == 0 {
        "FizzBuzz"
    } else if number % 3 == 0 {
        "Fizz"
    } else if number % 5 == 0 {
        "Buzz"
    } else {
        "1"
    }
}

fn fizz_buzz_another_way(number: i32) -> String {
    if number % 3i32 == 0i32 && number % 5i32 == 0i32 {
        "FizzBuzz".to_string()
    } else if number % 3i32 == 0i32 {
        "Fizz".to_string()
    } else if number % 5i32 == 0i32 {
        "Buzz".to_string()
    } else {
        "1".to_string()
    }
}

fn main() {
    println!("1 + 2 = {}", 1u32 + 2u32);
    println!("1 + 22 = {}", 1u16 + 22u16);

    println!("1 - 2 = {}", 1i32 - 2);

    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    /*
        An array is a list of items of homogeneous type. You can iterate over it and index or slice it with dynamic indices. It should be used for homegeneous collections of items that play the same role in the code. In general, you will iterate over an array at least once in your code.
        A tuple is a fixed-length agglomeration of heterogeneous items. It should be thought of as a struct with anonymous fields. The fields generally have different meaning in the code, and you can't iterate over it.
    */
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);

    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    let tuple_of_tuple = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    println!("tuple of tuples: {:?}", tuple_of_tuple);

    let pair = (1, true);
    println!("pair is {:?}", pair);

    println!("the reversed pair is {:?}", reverse(pair));

    println!("one element tuple: {:?}", (5u32, ));
    println!("just an integer: {:?}", (5u32));

    let tuple = (1, "hello", 4.5, true, 'A');
    let (a, b, c, d, e) = tuple;

    println!("{:?}, {:?}, {:?}, {:?}, {:?}", a, b, c, d, e);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
    println!("{}", matrix);

    // fixed size array
    let fixed_size_array: [i32; 5] = [1, 2, 3, 4, 5];
    // all 10 values will be set to 10 in this syntax
    let second_fixed_size_array_again: [i16; 10] = [10; 10];
    // all elements can be initialized to the same value

    println!("first element of the array: {}", fixed_size_array[0]);
    println!("second element of the array: {}", fixed_size_array[1]);
    println!("first element of the second array: {}", second_fixed_size_array_again[0]);

    println!("number of elements in array: {}", fixed_size_array.len());

    println!("array occupies {} bytes", mem::size_of_val(&fixed_size_array));
    println!("borrow the whole array as a slice");
    analyze_slice(&fixed_size_array);

    println!("{}", addition(7, 7));
    println!("{}", subtraction(7, 7));

    println!("{}", fizz_buzz(15));
    println!("{}", fizz_buzz_another_way(15));
}