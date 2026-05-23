// Parte 1: Versão de Impressão (fizzbuzz)
// Nota: Para esta função, não focamos em performance. O println! domina o tempo de execução,
// tornando as diferenças algorítmicas negligíveis. A análise de performance está em fizzbuzz_vec.

// MENÇÃO ESPECIAL: BufWriter para I/O Pesado (Otimização Real)
// === PERFORMANCE FIZZBUZZ ===
// FizzBuzz (print) - n = 100000:
// • Tempo: 265.5341ms <--- milissegundos
// ===================

// Quando usar: Volume massivo de impressões (milhares/milhões de linhas). Reduz syscalls agrupando writes em buffer. O .lock() evita travar/destravar stdout repetidamente. Esta é uma otimização de sistema apropriada, diferente das micro-otimizações algorítmicas. Em FizzBuzz com n pequeno, a diferença é imperceptível. Com n=1_000_000+, pode ser 10-50x mais rápido que println!.
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

// 1º Lugar: Casamento de Padrões (Idiomático Rust)
// === PERFORMANCE FIZZBUZZ ===
// FizzBuzz (print) - n = 100000:
// • Tempo: 5.5292047s <--- segundos
// ===================

// Vantagens: Pattern matching explícito e declarativo. Expressa claramente a lógica do problema. Idiomático em Rust. O compilador otimiza para código assembly eficiente. Performance idêntica ao if/else, mas mais elegante.
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

// 2º Lugar: If/Else Clássico (Familiar e Direto)
// Vantagens: Familiar para quem vem de outras linguagens. Código imperativo direto. Checar % 15 primeiro evita duas checagens subsequentes (micro-otimização irrelevante aqui). Performance praticamente idêntica ao match. Escolha por preferência pessoal.
pub fn fizzbuzz(n: u32) {
    for count in 1..=n {
        if count % 15 == 0 {
            println!("FizzBuzz");
        } else if count % 3 == 0 {
            println!("Fizz");
        } else if count % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{count}");
        }
    }
}

// 3º Lugar: Concatenação de Strings (Alocação Desnecessária)
// Desvantagens: Aloca String na heap apenas para imprimir e descartar. Força o alocador a trabalhar sem necessidade. Ainda assim, a diferença de performance é pequena porque println! é a operação mais lenta. Evite por princípio, não por performance.
pub fn fizzbuzz(n: u32) {
    for count in 1..=n {
        let mut out = String::new();
        if count % 3 == 0 {
            out.push_str("Fizz");
        }
        if count % 5 == 0 {
            out.push_str("Buzz");
        }
        if out.is_empty() {
            println!("{count}");
        } else {
            println!("{out}");
        }
    }
}

// Parte 2: Versão com Vetor (fizzbuzz_vec)
// Comparando implementações: performance similar, mas diferenças críticas em idiomaticidade, manutenibilidade e corretude.

// 1º Lugar: Funcional Pura (Abstração Zero-Cost)
// === PERFORMANCE (100 iterações) ===
// FizzBuzz (vetor):
// • Média: 33.014854ms
// • Mínimo: 30.895ms
// • Máximo: 40.5282ms
// ===================

// === PERFORMANCE (100 iterações) === RELEASE MODE
// FizzBuzz (vetor):
// • Média: 14.591528ms
// • Mínimo: 12.8187ms
// • Máximo: 17.4809ms
// ===================

// Vantagens: Código idiomático Rust, sem estado mutável, menor variação de performance. O .collect() pré-aloca via TrustedLen. Fácil de compor e paralelizar. Melhor equilíbrio entre legibilidade e performance para código de produção.
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

// 2º Lugar: For Loop com Vec::with_capacity
// === PERFORMANCE (100 iterações) ===
// FizzBuzz (vetor):
// • Média: 33.446561ms
// • Mínimo: 30.8137ms
// • Máximo: 58.0763ms
// ===================

// === PERFORMANCE (100 iterações) === RELEASE MODE
// FizzBuzz (vetor):
// • Média: 13.561708ms
// • Mínimo: 12.3693ms
// • Máximo: 16.3187ms
// ===================

// Vantagens: Pré-alocação explícita, zero realocações, controle fino. Desvantagens: Estado mutável, maior variação de performance (branch misprediction?). Boa para quando precisa debugar ou entender cada passo.
pub fn fizzbuzz_vec(n: u32) -> Vec<String> {
    let mut v = Vec::with_capacity(n as usize);
    for count in 1..=n {
        let res = match (count % 3 == 0, count % 5 == 0) {
            (true, true) => String::from("FizzBuzz"),
            (true, false) => String::from("Fizz"),
            (false, true) => String::from("Buzz"),
            _ => count.to_string(),
        };
        v.push(res);
    }
    v
}

// 3º Lugar: Vec::new() sem pré-alocação
// === PERFORMANCE (100 iterações) ===
// FizzBuzz (vetor):
// • Média: 31.751142ms
// • Mínimo: 30.2632ms
// • Máximo: 37.4417ms
// ===================

// === PERFORMANCE (100 iterações) === RELEASE MODE
// FizzBuzz (vetor):
// • Média: 16.247652ms
// • Mínimo: 13.7166ms
// • Máximo: 21.2504ms
// ===================

// Vantagens: Surpreendentemente rápido (alocador eficiente). Desvantagens: Múltiplas realocações e cópias desperdiçam ciclos. Código imperativo verboso. Evite em produção mesmo que seja rápido neste caso específico.
pub fn fizzbuzz_vec(n: u32) -> Vec<String> {
    let mut v = vec![];
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

// 4º Lugar: Loop Manual Desenrolado (Corrigido)
// === PERFORMANCE (100 iterações) ===
// FizzBuzz (vetor):
// • Média: 29.3609ms
// • Mínimo: 26.5256ms
// • Máximo: 34.0177ms
// ===================

// === PERFORMANCE (100 iterações) === RELEASE MODE
// FizzBuzz (vetor):
// • Média: 13.281397ms
// • Mínimo: 11.9514ms
// • Máximo: 15.8463ms
// ===================

// Exemplo de otimização prematura: código ilegível, difícil de manter, propenso a erros, não generalizável. O ganho de performance não justifica a complexidade. Evite. Desejável apenas em contextos muito específicos como processamento de pixels, DSP, simulações físicas.
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
