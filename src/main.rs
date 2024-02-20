#![feature(test)]
extern crate test;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let i = args
        .get(1)
        .expect("Give one argument")
        .parse::<u32>()
        .expect("Given argument should be a integer.");
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
mod tests {
    use super::*;
    use rstest::*;
    use std::time::Duration;

    #[rstest]
    #[case(1, "Fibonacci")]
    #[case(2, "Fibonacci")]
    #[case(3, "Fibonacci")]
    #[case(4, "4")]
    #[case(5, "Fibonacci")]
    #[case(6, "Fizz")]
    #[case(7, "7")]
    #[case(8, "Fibonacci")]
    #[case(9, "Fizz")]
    #[case(10, "Buzz")]
    #[case(15, "FizzBuzz")]
    #[timeout(Duration::from_millis(80))]
    fn test_fizz_buzz_fibonacci(#[case] input: u32, #[case] expected: &str) {
        assert_eq!(fizz_buzz_fibonacci(input), expected);
    }

    #[rstest]
    #[case(1, true)]
    #[case(2, true)]
    #[case(3, true)]
    #[case(4, false)]
    #[case(5, true)]
    #[case(6, false)]
    #[case(7, false)]
    #[case(8, true)]
    #[case(9, false)]
    #[case(10, false)]
    #[timeout(Duration::from_millis(80))]
    fn test_is_fibonacci_number(#[case] input: u32, #[case] expected: bool) {
        assert_eq!(is_fibonacci_number(input), expected);
    }
}

#[cfg(test)]
mod benchmarks {
    use super::*;
    use std::hint::black_box;
    use test::Bencher;

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
