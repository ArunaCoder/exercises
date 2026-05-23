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
    // BufWriter recebe a trava do stdout diretamente. Sem Box, sem overhead.
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
    let mut v = vec![]; // Capacidade inicial: 0. Péssimo.
    let mut count = 1;

    while count <= n {
        if count % 15 == 0 {
            v.push("FizzBuzz".to_string());
        } else if count % 3 == 0 {
            v.push("Fizz".to_string());
        } else if count % 5 == 0 {
            v.push("Buzz".to_string());
        } else {
            v.push(count.to_string());
        }
        count += 1;
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

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
