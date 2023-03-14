mod main_test;

fn linear_search_with_for(arr: &[i32], target_value: i32) -> i32 {
    for (i, item) in arr.iter().enumerate() {
        if arr[i] == target_value {
            return i as i32;
        }
    }
    return -1;
}

fn linearSearch(arr: &[i32], target: i32) -> Option<usize> {
    for (i, elem) in arr.iter().enumerate() {
        if *elem == target {
            return Some(i);
        }
    }
    None
}


fn main() {
    let arr: [i32; 5] = [1, 44, 32, 100, 932];
    let target_value: i32 = 932;

    let found_index = linearSearch(&arr, target_value);

    println!("Index of the found element => {:?}", found_index);
}