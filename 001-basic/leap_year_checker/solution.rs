// 1º Lugar: Expressão Booleana Direta (Idiomática e Otimizada)
// Vantagens: Código idiomático Rust, expressão booleana direta. A ordem de checagem é otimizada: testa % 4 primeiro (caso mais comum - 75% dos anos falham aqui), depois % 100 (centenários), por fim % 400 (raríssimo). Short-circuit natural elimina checagens desnecessárias. Performance idêntica às outras versões com curto-circuito.
pub fn is_leap_year(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

// 2º Lugar: Abordagem Monádica (then_some)
// Vantagens: Estilo funcional sofisticado, transforma lógica em fluxo de Option. Short-circuit via or_else. Desvantagens: Menos idiomática em Rust para lógica booleana simples. Ordem de checagem subótima (% 400 primeiro, caso mais raro). Performance similar à primeira após otimizações do compilador.
pub fn is_leap_year(year: u32) -> bool {
    (year % 400 == 0)
        .then_some(true)
        .or_else(|| (year % 100 == 0).then_some(false))
        .unwrap_or(year % 4 == 0)
}

// 3º Lugar: if/else Tradicional (Imperativo Verboso)
// Vantagens: Familiar para quem vem de outras linguagens. Desvantagens: Verboso e não idiomático em Rust. Retornar literais true/false explicitamente é redundante. Ordem subótima (% 400 primeiro). Performance idêntica às anteriores após otimização do compilador.
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

// 4º Lugar: Pattern Matching com Tupla (Perda de Short-Circuit)
// Desvantagem crítica: Calcula os 3 módulos (% 4, % 100, % 400) sempre, mesmo quando desnecessário. Perde o short-circuit que as outras versões têm. Para anos não divisíveis por 4 (75% dos casos), desperdiça cálculos de % 100 e % 400. Esta é uma diferença real de performance, não apenas estilo. Evite.
pub fn is_leap_year(year: u32) -> bool {
    match (year % 4 == 0, year % 100 == 0, year % 400 == 0) {
        (_, _, true) => true,
        (_, true, _) => false,
        (true, _, _) => true,
        _ => false,
    }
}
