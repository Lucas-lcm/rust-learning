/* 
char
    Definição: Representa um único caractere Unicode (4 bytes).

    Características: Pode armazenar qualquer caractere Unicode, não apenas ASCII.

    Exemplo:
    let c: char = 'a';
    let heart_eyed_cat: char = '😻';

&str (String Slice ou Referência de String)
    Definição: Uma fatia de string imutável. Referencia uma parte de uma string que já está alocada em algum lugar.

    Características: Leve e rápida, usada para acessar substrings sem copiar dados.

    Exemplo:
    let s: &str = "hello";

String
    Definição: Uma string alocada na heap que pode ser modificada.

    Características: Mais flexível que &str, pode crescer e ser modificada em tempo de execução.

    Exemplo:
    let mut s = String::from("hello");
    s.push_str(", world!");
    Comparação:

char é para representar um único caractere.

&str é uma referência a uma string imutável.

String é uma string alocada na heap, mutável e expansível.

Cada um tem seus usos específicos, dependendo da necessidade de imutabilidade, eficiência ou capacidade de modificação. 
*/

use std::io;

fn main() {
    let mut s = String::new();

    println!("Digite um texto: ");

    io::stdin()
        .read_line(&mut s)
        .expect("Erro reading console");

    println!("Você digitou: {s}");
}

/* 
O módulo std::io faz parte da biblioteca padrão de Rust e fornece funcionalidades para entrada e saída (I/O). Ele lida com operações como leitura e escrita em arquivos, entrada e saída de dados através de fluxos, entre outros.
    Principais componentes:
        BufReader e BufWriter: Para leitura e escrita bufferizada.
        Read e Write: Traits fundamentais para leitura e escrita.
        Error: Tipo usado para indicar erros de I/O.
        Result: Tipo usado para resultados de operações de I/O que podem falhar.

io::stdin()
Definição: A função io::stdin() obtém a entrada padrão (standard input) e retorna um handle para leitura.
*/