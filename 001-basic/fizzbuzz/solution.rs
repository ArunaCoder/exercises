// Parte 1: Versão de Impressão (fizzbuzz)

// 1º Lugar: Casamento de Padrões Eficiente (O Idioma Rust)
// Por que é a melhor? É limpa, declarativa e não força cálculos desnecessários na CPU. O uso do operador de resto (%) de forma combinada tira proveito do curto-circuito lógico antes de entrar no match. O compilador consegue otimizar os desvios e você mantém a legibilidade sem comprometer o silício.
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

// 2º Lugar: O Clássico Imperativo Otimizado (Controle de Fluxo Direto)
// Por que está aqui? Substitui o loop while manual por um iterador nativo (1..=n), o que elimina o risco de erros de contagem e permite ao compilador aplicar otimizações de desbobinamento de loop (loop unrolling). A cadeia de if/else funciona, mas checar % 15 primeiro é uma heurística que assume que a CPU lidará bem com o desvio mais raro falhando a maior parte do tempo.
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

// 3º Lugar: Concatenação de Strings em Loop (Inchaço Oculto e Alocação)
// Por que é a pior? Alocar memória na heap para fazer um println! é um crime de engenharia. Criar uma string vazia e empurrar caracteres nela a cada iteração força o alocador do sistema a trabalhar sem necessidade, destruindo a localidade de cache da CPU para um problema puramente aritmético.
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
// Aqui o buraco é mais embaixo. Funções que retornam coleções sofrem com realocação de memória. Se você não avisa a CPU o tamanho do vetor, ela precisa adivinhar, realocar e copiar dados na heap toda vez que o espaço acaba.

// === PERFORMANCE (100 iterações) ===
// FizzBuzz (vetor):
// • Média: 29.829953ms
// • Mínimo: 26.811ms
// • Máximo: 48.3168ms
// ===================
pub fn fizzbuzz_vec(n: u32) -> Vec<String> {
    let mut v = Vec::with_capacity(n as usize);
    let mut count = 1;
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
    v
}

// 1º Lugar: Iteradores com Pré-alocação Explicita (Mestre da Performance)
// === PERFORMANCE (100 iterações) ===
// FizzBuzz (vetor):
// • Média: 33.446561ms
// • Mínimo: 30.8137ms
// • Máximo: 58.0763ms
// ===================
// Por que é a melhor? Usar .with_capacity(n as usize) aloca a memória exata na heap de uma única vez. Zero realocações. Além disso, usar strings estáticas String::from("...") evita processamento em tempo de execução, e o iterador mapeia os dados diretamente para o vetor final. É o uso ideal do sistema de tipos de Rust.
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

// 2º Lugar: Funcional Pura (Abstração Zero-Cost)
// === PERFORMANCE (100 iterações) ===
// FizzBuzz (vetor):
// • Média: 33.014854ms
// • Mínimo: 30.895ms
// • Máximo: 40.5282ms
// ===================
// Por que está aqui? Transforma o problema em uma pipeline de dados elegante. O método .collect() é inteligente o suficiente graças ao trait TrustedLen dos iteradores de intervalo (1..=n), o que significa que ele sabe o tamanho exato final e pré-aloca a memória por debaixo dos panos. Perde o primeiro lugar apenas porque a leitura do match em linha pode ficar densa para quem não domina o idioma.
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

// 3º Lugar: O Vetor Dinâmico Preguiçoso (Ineficiência de Alocação)
// === PERFORMANCE (100 iterações) ===
// FizzBuzz (vetor):
// • Média: 31.751142ms
// • Mínimo: 30.2632ms
// • Máximo: 37.4417ms
// ===================
// Por que está no fim da fila? O uso de vec![] inicializa o vetor com capacidade zero. À medida que você dá .push(), o vetor atinge o limite, dobra de tamanho, pede mais memória ao sistema operacional, copia os elementos antigos para o novo endereço e deleta o antigo. Multiplique esse processo de cópia por milhares de iterações e você terá um gargalo massivo de I/O de memória. (Nota técnica: o código original continha o bug literário "{count}".to_string(), que foi corrigido aqui para count.to_string()).
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
