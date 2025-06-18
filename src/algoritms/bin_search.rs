#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let (mut left, mut right) = (0, arr.len());

    while left < right {
        let mid = left + (right - left) / 2;

        match true {
            true if arr[mid] == target => return Some(mid),
            true if arr[mid] < target => left = mid + 1,
            _ => right = mid,
        }
    }

    None
}

fn binary_search_recursive(arr: &[i32], target: i32, left: usize, right: usize) -> Option<usize> {
    if left >= right {
        return None;
    }

    let mid = left + (right - left) / 2;

    match true {
        true if arr[mid] == target => Some(mid),
        true if arr[mid] < target => binary_search_recursive(arr, target, mid + 1, right),
        _ => binary_search_recursive(arr, target, left, mid),
    }
}

fn binary_search_first(arr: &[i32], target: i32) -> Option<usize> {
    let (mut left, mut right) = (0, arr.len());
    let mut result = None;

    while left < right {
        let mid = left + (right - left) / 2;

        if arr[mid] >= target {
            if arr[mid] == target {
                result = Some(mid);
            }
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    result
}

pub fn run() {
    let arr = [1, 3, 5, 7, 9, 11, 15];
    let target = 7;

    match binary_search(&arr, target) {
        Some(index) => println!("Element found on position: {}", index),
        None => println!("Element not found"),
    }

    let arr = [1, 3, 5, 7, 9, 11, 15];
    let target = 7;

    match binary_search_recursive(&arr, target, 0, arr.len()) {
        Some(index) => println!("Element found on position: {}", index),
        None => println!("Element not found"),
    }

    let arr = [1, 3, 3, 3, 5, 7, 9];
    let target = 3;

    match binary_search_first(&arr, target) {
        Some(index) => println!(
            "First occurrence of element {} at position {}",
            target, index
        ),
        None => println!("Element not found"),
    }
}
