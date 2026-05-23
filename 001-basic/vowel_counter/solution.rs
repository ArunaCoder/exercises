// 1º Lugar: Imperativa com ASCII (Clara e Eficiente)
// ⚠️ LIMITAÇÃO: is_ascii_alphabetic() IGNORA acentos (á, é, ã, ç, etc.). NÃO funciona para português!
// Vantagens: Sem alocações de memória. Converte cada char individualmente com to_ascii_lowercase(). Usa is_ascii_alphabetic() (mais específico que is_alphabetic). Lógica imperativa clara e fácil de debugar. Perfeita para texto ASCII/inglês. Performance excelente.
pub fn count_vowels_and_consonants(text: &str) -> (usize, usize) {
    let mut vowel_count = 0;
    let mut consonant_count = 0;

    for c in text.chars() {
        if c.is_ascii_alphabetic() {
            match c.to_ascii_lowercase() {
                'a' | 'e' | 'i' | 'o' | 'u' => vowel_count += 1,
                _ => consonant_count += 1,
            }
        }
    }

    (vowel_count, consonant_count)
}

// 2º Lugar: Funcional com fold (Elegante mas Densa)
// Vantagens: Estilo funcional idiomático. Sem alocações. Pipeline de iteradores expressivo. Desvantagens: fold com tupla mutável pode ser menos legível. Dificulta debugging (não há pontos de parada claros). Escolha por preferência de estilo funcional.
pub fn count_vowels_and_consonants_functional(text: &str) -> (usize, usize) {
    text.chars()
        .filter(|c| c.is_ascii_alphabetic())
        .fold((0, 0), |(mut v, mut c), x| {
            match x.to_ascii_lowercase() {
                'a' | 'e' | 'i' | 'o' | 'u' => v += 1,
                _ => c += 1,
            }
            (v, c)
        })
}

// 3º Lugar: Com to_lowercase (Alocação Desnecessária)
// Desvantagens: Aloca String inteira na heap com to_lowercase(). Para textos longos, desperdiça memória. Usa is_alphabetic() (suporta Unicode, pode ser excessivo para inglês). Vantagens: Suporte completo a Unicode se necessário. Evite para texto ASCII comum.
pub fn count_vowels_and_consonants_allocated(text: &str) -> (usize, usize) {
    let mut vowel_count = 0;
    let mut consonant_count = 0;

    let lower_text = text.to_lowercase();
    let lower_text_chars = lower_text.chars();

    for k in lower_text_chars {
        if k.is_alphabetic() {
            match k {
                'a' | 'e' | 'i' | 'o' | 'u' => vowel_count += 1,
                _ => consonant_count += 1,
            }
        }
    }

    (vowel_count, consonant_count)
}

// ═══════════════════════════════════════════════════════════════════════════
// PARA PORTUGUÊS: Implementação Unicode-Aware
// ═══════════════════════════════════════════════════════════════════════════

// Versão Correta para Português (Suporta Acentos: á, é, í, ó, ú, ã, õ, ê, etc.)
// Problema: As implementações ASCII acima IGNORAM completamente letras acentuadas!
// Exemplo: "Meditação é ausência" conta apenas "Meditao e ausncia" (sem ã, é, ê).
// Solução: Usar is_alphabetic() (Unicode-aware) + to_lowercase() para cada char.
// Trade-off: Ligeiramente mais lento que ASCII, mas CORRETO para português/outros idiomas.
//
// ⚠️ DIFERENÇA: to_ascii_lowercase() retorna char, to_lowercase() retorna ITERADOR!
// Por quê? Unicode permite 1 char maiúsculo virar MÚLTIPLOS chars minúsculos (ex: 'ẞ' → "ss")
// Para português comum (á, é, ã), é sempre 1:1, então usamos .next().unwrap().
pub fn count_vowels_and_consonants_unicode(text: &str) -> (usize, usize) {
    let mut vowel_count = 0;
    let mut consonant_count = 0;

    for c in text.chars() {
        if c.is_alphabetic() {
            // .next().unwrap() pega o primeiro char do iterador
            // Para português/inglês, sempre há exatamente 1 char
            let lower = c.to_lowercase().next().unwrap();

            match lower {
                'a' | 'e' | 'i' | 'o' | 'u' | // ASCII vogais
                'á' | 'à' | 'â' | 'ã' | // A com acentos
                'é' | 'ê' |               // E com acentos
                'í' |                     // I com acentos
                'ó' | 'ô' | 'õ' |         // O com acentos
                'ú' | 'ü'                 // U com acentos
                => vowel_count += 1,
                _ => consonant_count += 1,
            }
        }
    }

    (vowel_count, consonant_count)
}

// ═══════════════════════════════════════════════════════════════════════════
// Resumo: Quando usar cada implementação
// ═══════════════════════════════════════════════════════════════════════════
//
// ASCII (1º/2º):
//   ✓ Texto puramente inglês/números
//   ✓ Máxima performance (sem overhead Unicode)
//   ✗ IGNORA acentos (á, é, ã → invisíveis!)
//
// Unicode (acima):
//   ✓ Português, espanhol, francês, alemão, etc.
//   ✓ CORRETO para acentos
//   ✗ ~10-20% mais lento (trade-off aceitável)
//
// Por que não pudemos só trocar os métodos?
//   to_ascii_lowercase() → retorna char (direto no match)
//   to_lowercase()       → retorna iterador (precisa .next().unwrap())
