// FIZZBUZZ
// This is the classic FizzBuzz problem.

fn main() {
    println!("FizzBuzz from 1 to 20 (Stream):");
    fizzbuzz(20);

    println!("\nFizzBuzz de 1 a 20 (Vetor de Arena):");
    let mut arena = String::with_capacity(20 * 6);
    let mut v = Vec::with_capacity(20);

    fizzbuzz_vec(20, &mut arena, &mut v);

    // Renderização eficiente consumindo a arena pelos índices salvos
    for item in &v {
        match item {
            FizzItem::Static(s) => println!("{s}"),
            FizzItem::NumRange(start, end) => {
                let num_str = &arena[*start as usize..*end as usize];
                println!("{num_str}");
            }
        }
    }
}

/// Representação cirúrgica no Stack. Tamanho previsível e cópia via memmove.
#[derive(Debug, Clone, Copy)]
pub enum FizzItem {
    Static(&'static str),
    NumRange(u32, u32), // offsets na arena (u32 corta o uso de memória pela metade vs usize)
}

/// FizzBuzz de alta performance enviando direto para o buffer do stdout lockado
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

/// Loop Unrolling puro com Arena de String. Alocação zero por item.
pub fn fizzbuzz_vec(n: u32, arena: &mut String, v: &mut Vec<FizzItem>) {
    use std::fmt::Write;
    let mut count = 1;

    // Bloco principal: Processa fatias completas de 15 sem avaliar restos olimpicamente
    while count + 14 <= n {
        // 1. Número dinâmico
        let start = arena.len() as u32;
        let _ = write!(arena, "{}", count);
        v.push(FizzItem::NumRange(start, arena.len() as u32));

        // 2. Número dinâmico
        let start = arena.len() as u32;
        let _ = write!(arena, "{}", count + 1);
        v.push(FizzItem::NumRange(start, arena.len() as u32));

        v.push(FizzItem::Static("Fizz"));

        // 4. Número dinâmico
        let start = arena.len() as u32;
        let _ = write!(arena, "{}", count + 3);
        v.push(FizzItem::NumRange(start, arena.len() as u32));

        v.push(FizzItem::Static("Buzz"));
        v.push(FizzItem::Static("Fizz"));

        // 7. Número dinâmico
        let start = arena.len() as u32;
        let _ = write!(arena, "{}", count + 6);
        v.push(FizzItem::NumRange(start, arena.len() as u32));

        // 8. Número dinâmico
        let start = arena.len() as u32;
        let _ = write!(arena, "{}", count + 7);
        v.push(FizzItem::NumRange(start, arena.len() as u32));

        v.push(FizzItem::Static("Fizz"));
        v.push(FizzItem::Static("Buzz"));

        // 11. Número dinâmico
        let start = arena.len() as u32;
        let _ = write!(arena, "{}", count + 10);
        v.push(FizzItem::NumRange(start, arena.len() as u32));

        v.push(FizzItem::Static("Fizz"));

        // 13. Número dinâmico
        let start = arena.len() as u32;
        let _ = write!(arena, "{}", count + 12);
        v.push(FizzItem::NumRange(start, arena.len() as u32));

        // 14. Número dinâmico
        let start = arena.len() as u32;
        let _ = write!(arena, "{}", count + 13);
        v.push(FizzItem::NumRange(start, arena.len() as u32));

        v.push(FizzItem::Static("FizzBuzz"));

        count += 15;
    }

    // Sobras do final (se n não for múltiplo exato de 15)
    for c in count..=n {
        match (c % 3 == 0, c % 5 == 0) {
            (true, true) => v.push(FizzItem::Static("FizzBuzz")),
            (true, false) => v.push(FizzItem::Static("Fizz")),
            (false, true) => v.push(FizzItem::Static("Buzz")),
            _ => {
                let start = arena.len() as u32;
                let _ = write!(arena, "{}", c);
                v.push(FizzItem::NumRange(start, arena.len() as u32));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn benchmark_fizzbuzz() {
        let n = 500_000;

        // Pré-alocação cirúrgica baseada na estimativa de bytes para evitar reallocs na Heap
        let mut arena = String::with_capacity(n as usize * 6);
        let mut v = Vec::with_capacity(n as usize);

        let start_vec = Instant::now();
        fizzbuzz_vec(n, &mut arena, &mut v);
        let duration_vec = start_vec.elapsed();

        println!(
            "\n\n=== PERFORMANCE ===\nFizzBuzz (Vetor + Arena Baseada em Índices): {:?}\n===================\n",
            duration_vec
        );

        assert_eq!(v.len(), n as usize);
    }
}
