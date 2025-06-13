#![allow(dead_code)]

pub fn run() {
    println!("vec_fibonacci(7) = {:?}", fibonacci_calc_recirsive(7));
    println!("vec_fibonacci(7) = {:?}", fibonacci_calc_loop(7));
}

fn fibonacci_calc_recirsive(n: usize) -> Vec<u64> {
    let mut list_fibonacci: Vec<u64> = Vec::with_capacity(n);

    match n {
        1 => {
            list_fibonacci.push(0);
        }
        2 => {
            list_fibonacci.push(0);

            list_fibonacci.push(1);
        }
        _ => {
            list_fibonacci.append(&mut fibonacci_calc_recirsive(n - 1));

            list_fibonacci.push(
                list_fibonacci.last().unwrap()
                    + list_fibonacci.get(list_fibonacci.len() - 2).unwrap(),
            );
        }
    }

    list_fibonacci
}

fn fibonacci_calc_loop(n: usize) -> Vec<u64> {
    let mut list_fibonacci: Vec<u64> = Vec::with_capacity(n);

    match n {
        1 => {
            list_fibonacci.push(0);
        }
        2 => {
            list_fibonacci.push(0);
            list_fibonacci.push(1);
        }
        _ => {
            list_fibonacci.push(0);
            list_fibonacci.push(1);
            (2..n).into_iter().for_each(|i| {
                list_fibonacci
                    .push(list_fibonacci.get(i - 1).unwrap() + list_fibonacci.get(i - 2).unwrap())
            });
        }
    }

    list_fibonacci
}
