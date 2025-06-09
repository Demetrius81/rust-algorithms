#![allow(dead_code)]

pub fn run() {
    println!("Enter number >");

    let number = match read_stdin() {
        Some(str) => match str.parse::<i64>() {
            Ok(num) => num,
            Err(e) => panic!("Parse error: {:?}", e),
        },
        None => panic!("Empty string!"),
    };

    let is_prime = check_is_prime_update(number);

    if is_prime {
        println!("Число {number} является простым");
    } else {
        println!("Число {number} не является простым");
    }
}

fn check_is_prime(number: i64) -> bool {
    let mut d = 0i64;

    (2..number).into_iter().for_each(|i| {
        if number % i == 0 {
            d += 1;
        }
    });

    if d == 0 { true } else { false }
}

fn check_is_prime_update(number: i64) -> bool {
    if number == 2 {
        return true;
    }

    for i in 2..number {
        if number % i == 0 {
            return false;
        }
    }

    true
}

fn read_stdin() -> Option<String> {
    let mut line = String::new();
    match std::io::stdin().read_line(&mut line) {
        Ok(_) => Some(line.trim().to_string()),
        Err(_) => None,
    }
}
