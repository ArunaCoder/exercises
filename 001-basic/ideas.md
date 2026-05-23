1

```rust
// Abordagem manual/procedural (Estilo C/Java)
let mut x;
if condicao {
x = 10; // Instrução: altera x, mas o bloco em si não retorna nada
} else {
x = 20;
}

// Abordagem idiomática em Rust (Expressão)
let x = if condicao {
10 // Sem ponto e vírgula: o bloco retorna 10
} else {
20 // Sem ponto e vírgula: o bloco retorna 20
};
//Isso elimina variáveis mutáveis desnecessárias e permite que o compilador garanta a segurança de memória e a inicialização correta em tempo de compilação.
```

2
