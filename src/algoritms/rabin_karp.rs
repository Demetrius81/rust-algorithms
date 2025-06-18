#![allow(dead_code)]

const P: u64 = 31; // Polynomial hash base
const MOD: u64 = 1_000_000_007; // Prime number to prevent overflow

fn compute_hash(s: &str) -> u64 {
    s.chars().fold(0, |hash, c| (hash * P + (c as u64)) % MOD)
}

fn rabin_karp(text: &str, pattern: &str) -> Vec<usize> {
    let n = text.len();
    let m = pattern.len();

    if m > n {
        return vec![];
    }

    let pattern_hash = compute_hash(pattern);
    let mut current_hash = compute_hash(&text[..m]);
    let mut results = vec![];

    let highest_pow = P.pow((m - 1) as u32) % MOD;

    for i in 0..=n - m {
        if current_hash == pattern_hash {
            if &text[i..i + m] == pattern {
                results.push(i);
            }
        }

        if i < n - m {
            let first_char = text.chars().nth(i).unwrap() as u64;
            let new_char = text.chars().nth(i + m).unwrap() as u64;

            current_hash =
                ((current_hash + MOD - (first_char * highest_pow) % MOD) * P + new_char) % MOD;
        }
    }

    results
}

pub fn run() {
    let text = "sdfsdfsfsgregfgegdfgsrtyfgdfgdfgdfgdfgdgdfg";
    let pattern = "rty";

    let positions = rabin_karp(text, pattern);
    println!("Substring found at positions: {:?}", positions);
}
