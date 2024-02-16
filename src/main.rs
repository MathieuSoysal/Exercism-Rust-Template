#![feature(test)]
extern crate test;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let i = args
        .get(1)
        .expect("Give one argument")
        .parse::<u32>()
        .expect("Given argument should be a number.");
    play_game(i);
}

pub fn play_game(n: u32) {
    println!("{}", fizz_buzz_fibonacci(n));
}

fn is_fibonacci_number(n: u32) -> bool {
    let (mut previous, mut current) = (0, 1);
    while current < n {
        let next = previous + current;
        previous = current;
        current = next;
    }
    current == n
}

pub fn fizz_buzz_fibonacci(n: u32) -> String {
    if is_fibonacci_number(n) {
        "Fibonacci".to_string()
    } else {
        match (n % 3, n % 5) {
            (0, 0) => "FizzBuzz".to_string(),
            (0, _) => "Fizz".to_string(),
            (_, 0) => "Buzz".to_string(),
            (_, _) => n.to_string(),
        }
    }
}

#[cfg(test)]
mod benchmarks {
    use test::Bencher;

    use super::play_game;
    use std::hint::black_box;

    #[bench]
    fn bench_play_game(b: &mut Bencher) {
        b.iter(|| {
            black_box(for i in 1..=100 {
                play_game(i)
            });
        });
    }

    #[bench]
    fn bench_play_game_100(b: &mut Bencher) {
        b.iter(|| std::hint::black_box(play_game(100)));
    }

    #[bench]
    fn bench_play_game_1_000_000(b: &mut Bencher) {
        b.iter(|| std::hint::black_box(play_game(1_000_000)));
    }
}
