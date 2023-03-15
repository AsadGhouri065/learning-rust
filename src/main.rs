mod main_test;

fn linear_search_for_i32(arr: &[i32], target_value: i32) -> i32 {
    for (i, item) in arr.iter().enumerate() {
        if arr[i] == target_value {
            return arr[i];
        }
    }
    return -1;
}

fn linear_search_for_i16(arr: &[i16], target_value: i16) -> i16 {
    for (i, item) in arr.iter().enumerate() {
        if arr[i] == target_value {
            return arr[i];
        }
    }
    return -1;
}

fn main() {
    // what i know so far in rust
    // variable declaration
    let variable: i32 = 100;
    // printing (debug, normal)
    println!("{}", variable);
    // data types (memory types also)
    // arrays (tuples, fixed arrays)
    let i32_array: [i32; 5] = [10, 20, 30, 40, 60];
    // functions
    // for loops
    for i in i32_array.iter().enumerate() {
        println!("{:?}", i);
    }
    // if statements
    if variable == 100 {
        println!("hello world");
    } else {
        println!("goodbye world");
    }


    let arr_i32: [i32; 5] = [1, 44, 32, 100, 932];
    let target_value_i32: i32 = 932;

    let arr_i16: [i16; 5] = [1, 44, 32, 100, 932];
    let target_value_i16: i16 = 100;

    let found_index_i32: i32 = linear_search_for_i32(&arr_i32, target_value_i32);
    let found_index_i16: i16 = linear_search_for_i16(&arr_i16, target_value_i16);

    println!("Value of the found element => {:?}", found_index_i32);
    println!("Value of the found element => {:?}", found_index_i16);
}