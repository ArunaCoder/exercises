// FIZZBUZZ
// This is the classic FizzBuzz problem.
// Print numbers from 1 to N, but:
// - For multiples of 3, print "Fizz"
// - For multiples of 5, print "Buzz"
// - For multiples of both 3 and 5, print "FizzBuzz"
// - Otherwise, print the number

fn main() {
    println!("FizzBuzz from 1 to 100:");
    fizzbuzz(20);

    let result = fizzbuzz_vec(20);

    println!("{result:?}")
}

/// FizzBuzz from 1 to n
pub fn fizzbuzz(n: u32) {
    use std::io::{self, Write};

    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout.lock());

    for count in 1..=n {
        match (count % 3 == 0, count % 5 == 0) {
            (true, true) => {
                let _ = writeln!(handle, "FizzBuzz");
            }
            (true, false) => {
                let _ = writeln!(handle, "Fizz");
            }
            (false, true) => {
                let _ = writeln!(handle, "Buzz");
            }
            _ => {
                let _ = writeln!(handle, "{count}");
            }
        }
    }
}
/// Alternative: return Vec<String> instead of printing
pub fn fizzbuzz_vec(n: u32) -> Vec<String> {
    (1..=n)
        .map(|count| match (count % 3 == 0, count % 5 == 0) {
            (true, true) => String::from("FizzBuzz"),
            (true, false) => String::from("Fizz"),
            (false, true) => String::from("Buzz"),
            _ => count.to_string(),
        })
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn benchmark_fizzbuzz_print() {
        let n = 100_000;

        let start = Instant::now();
        fizzbuzz(n);
        let duration = start.elapsed();

        println!(
            "\n\n=== PERFORMANCE FIZZBUZZ ===\n\
            FizzBuzz (print) - n = {}:\n\
            • Tempo: {:?}\n\
            ===================\n",
            n, duration
        );

        assert!(true);
    }

    #[test]
    fn benchmark_fizzbuzz() {
        let n = 500_000;
        let iterations = 100;
        let mut durations = Vec::with_capacity(iterations);

        // Executa o benchmark múltiplas vezes
        for _ in 0..iterations {
            let start = Instant::now();
            let _result = fizzbuzz_vec(n);
            durations.push(start.elapsed());
        }

        // Calcula estatísticas
        let total: std::time::Duration = durations.iter().sum();
        let avg = total / iterations as u32;
        let min = durations.iter().min().unwrap();
        let max = durations.iter().max().unwrap();

        println!(
            "\n\n=== PERFORMANCE ({} iterações) ===\n\
            FizzBuzz (vetor):\n\
            • Média: {:?}\n\
            • Mínimo: {:?}\n\
            • Máximo: {:?}\n\
            ===================\n",
            iterations, avg, min, max
        );

        // Garante que o teste passa
        assert!(true)
    }
}
