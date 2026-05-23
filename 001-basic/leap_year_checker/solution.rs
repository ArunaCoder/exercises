// 1º Lugar: Expressão Idiomática (A matemática direta)
// Por que é a melhor? É imbatível. Ela respeita como a CPU funciona. O compilador (rustc via LLVM) transforma isso em instruções lógicas diretas e curtos-circuitos nativos extremamente rápidos.
// O trunfo: O teste mais comum (divisibilidade por 4) vem primeiro. Se falhar, a CPU descarta o resto imediatamente. Sem alocação, sem metadados, sem saltos (branches) complexos. É Rust em seu estado puro: declarativo, seguro e veloz.
pub fn is_leap_year(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

// 2º Lugar: Abordagem Monádica (then_some)
// Por que está aqui? Como discutimos antes, ela é sofisticada. Do ponto de vista de design, transforma fluxo em dados tipados (Option). A avaliação preguiçosa (||) salva a performance de hardware.
// Por que não é a primeira? Embora o compilador seja inteligente o suficiente para otimizar e inlinear os métodos de Option, você está adicionando abstrações de tipos na assinatura dos métodos internos para resolver uma aritmética booleana simples. É elegante, mas a primeira solução é mais crua e direta.
pub fn is_leap_year(year: u32) -> bool {
    (year % 400 == 0)
        .then_some(true)
        .or_else(|| (year % 100 == 0).then_some(false))
        .unwrap_or(year % 4 == 0)
}

// 3º Lugar: if / else Tradicional (O imperativo verboso)
// Por que está no fim da fila? Isso aqui parece C escrito com sintaxe de Rust. Rust é uma linguagem baseada em expressões; usar if/else para retornar literais de true e false é redundante e amador.
// Problema de Hardware: Cria uma árvore de desvios condicionais explícitos. Se o otimizador do compilador não intervir, você gera múltiplos saltos na CPU que podem confundir o preditor de desvios (branch predictor).
pub fn is_leap_year(year: u32) -> bool {
    if year % 400 == 0 {
        true
    } else if year % 100 == 0 {
        false
    } else if year % 4 == 0 {
        true
    } else {
        false
    }
}

// 4º Lugar: Pattern Matching com Tupla (Inchaço Oculto)
// Por que é a pior? Desperdício de ciclos. Para montar essa tupla (bool, bool, bool) no início do match, a CPU é obrigada a calcular os três restos da divisão (% 4, % 100, % 400) todas as vezes, para qualquer ano.
// O crime: Você jogou o curto-circuito no lixo. Se o ano não for divisível por 4 (caso mais comum), a função ainda assim gastou instruções calculando se ele é divisível por 100 e 400 à toa, apenas para preencher a tupla antes de avaliar o padrão. Bonito no papel, ineficiente no silício.
pub fn is_leap_year(year: u32) -> bool {
    match (year % 4 == 0, year % 100 == 0, year % 400 == 0) {
        (_, _, true) => true,
        (_, true, _) => false,
        (true, _, _) => true,
        _ => false,
    }
}
