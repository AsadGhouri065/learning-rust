mod main_test;

use std::{fmt, mem};
use std::fmt::{Formatter, write};

#[allow(dead_code)]
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
struct Person {
    name: String,
    age: u8,
}

struct Unit;

struct Pair(i32, f32);

struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
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

impl fmt::Debug for Pair {
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
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };
    let pair: Pair = Pair { 0: 10, 1: 5.0 };

    println!("{:?}", peter);
    println!("This is a pair: {:?}", pair);

    let point: Point = Point { x: 10.3, y: 0.4 };

    println!("point coordinates: ({}, {})", point.x, point.y);

    /*
        An array is a list of items of homogeneous type. You can iterate over it and index or slice it with dynamic indices. It should be used for homegeneous collections of items that play the same role in the code. In general, you will iterate over an array at least once in your code.
        A tuple is a fixed-length agglomeration of heterogeneous items. It should be thought of as a struct with anonymous fields. The fields generally have different meaning in the code, and you can't iterate over it.
    */
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);

    let tuple_of_tuple = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    let pair = (1, true);

    let tuple = (1, "hello", 4.5, true, 'A');
    let (a, b, c, d, e) = tuple;

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);


    // fixed size array
    let fixed_size_array: [i32; 5] = [1, 2, 3, 4, 5];
    // all 10 values will be set to 10 in this syntax
    let second_fixed_size_array_again: [i16; 10] = [10; 10];
    // all elements can be initialized to the same value


    analyze_slice(&fixed_size_array);

    println!("{}", fizz_buzz(15));
    println!("{}", fizz_buzz_another_way(15));
}