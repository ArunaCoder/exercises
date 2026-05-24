// FIZZBUZZ
// Este é o problema clássico FizzBuzz.
// Imprima números de 1 a N, mas:
// - Para múltiplos de 3, imprima "Fizz"
// - Para múltiplos de 5, imprima "Buzz"
// - Para múltiplos de ambos (3 e 5), imprima "FizzBuzz"
// - Caso contrário, imprima o número

fn main() {
    // Esta função não exige nenhuma edição

    println!("FizzBuzz de 1 a 100:");
    fizzbuzz(100);

    let result = fizzbuzz_vec(100);

    println!("{result:?}")
}

/// FizzBuzz de 1 to n
pub fn fizzbuzz(n: u32) {
    todo!(
        "Use match com uma tupla (count % 3 == 0, count % 5 == 0) para cobrir todos os casos de forma declarativa."
    );
}
/// Alternativa: retorna Vec<String> em vez de print
pub fn fizzbuzz_vec(n: u32) -> Vec<String> {
    todo!(
        "Transforme o range (1..=n) em um iterador, aplique a lógica via .map() e materialize o resultado com .collect()."
    )
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
