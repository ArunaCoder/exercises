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
    for count in 1..=n {
        match (count % 3 == 0, count % 5 == 0) {
            (true, true) => println!("FizzBuzz"),
            (true, false) => println!("Fizz"),
            (false, true) => println!("Buzz"),
            _ => println!("{count}"),
        }
    }
}
/// Alternative: return Vec<String> instead of printing
pub fn fizzbuzz_vec(n: u32) -> Vec<String> {
    let mut v = Vec::with_capacity(n as usize);
    let mut count = 1;

    // Loop desenrolado para blocos de 15
    while count + 14 <= n {
        v.push(count.to_string());
        v.push((count + 1).to_string());
        v.push(String::from("Fizz"));
        v.push((count + 3).to_string());
        v.push(String::from("Buzz"));
        v.push(String::from("Fizz"));
        v.push((count + 6).to_string());
        v.push((count + 7).to_string());
        v.push(String::from("Fizz"));
        v.push(String::from("Buzz"));
        v.push((count + 10).to_string());
        v.push(String::from("Fizz"));
        v.push((count + 12).to_string());
        v.push((count + 13).to_string());
        v.push(String::from("FizzBuzz"));
        count += 15
    }

    // Processa elementos restantes (remainder)
    while count <= n {
        let res = match (count % 3 == 0, count % 5 == 0) {
            (true, true) => String::from("FizzBuzz"),
            (true, false) => String::from("Fizz"),
            (false, true) => String::from("Buzz"),
            _ => count.to_string(),
        };
        v.push(res);
        count += 1;
    }

    v
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
