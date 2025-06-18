#![allow(dead_code)]

fn merge_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    let mid = arr.len() / 2;
    let mut left = arr[..mid].to_vec();
    let mut right = arr[mid..].to_vec();

    merge_sort(&mut left);
    merge_sort(&mut right);

    merge(arr, &left, &right);
}

fn merge(arr: &mut [i32], left: &[i32], right: &[i32]) {
    let (mut i, mut j, mut k) = (0, 0, 0);

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        arr[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < right.len() {
        arr[k] = right[j];
        j += 1;
        k += 1;
    }
}

fn merge_sort_iterative(arr: &mut Vec<i32>) {
    let mut step = 1;
    let len = arr.len();

    while step < len {
        let mut temp = arr.clone();

        let mut left = 0;
        while left < len {
            let mid = usize::min(left + step, len);
            let right = usize::min(left + 2 * step, len);

            let (mut i, mut j, mut k) = (left, mid, left);

            while i < mid && j < right {
                if arr[i] <= arr[j] {
                    temp[k] = arr[i];
                    i += 1;
                } else {
                    temp[k] = arr[j];
                    j += 1;
                }
                k += 1;
            }

            while i < mid {
                temp[k] = arr[i];
                i += 1;
                k += 1;
            }

            while j < right {
                temp[k] = arr[j];
                j += 1;
                k += 1;
            }

            left += 2 * step;
        }

        arr.copy_from_slice(&temp);
        step *= 2;
    }
}

pub fn run() {
    let mut arr = [5, 2, 9, 1, 5, 6];
    merge_sort(&mut arr);
    println!("{:?}", arr);

    let mut arr = vec![5, 2, 9, 1, 5, 6];
    merge_sort_iterative(&mut arr);
    println!("{:?}", arr);
}
