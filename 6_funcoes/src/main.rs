/* 
Definindo e Chamando Funções em Rust

    Estrutura Básica de uma Função
    As funções em Rust são definidas com a palavra-chave fn, seguidas pelo nome da função, parâmetros (se houver) e 
    um corpo de função entre chaves {}. 

    Aqui está uma função simples que adiciona dois números: 
*/

fn main() {
    let resultado = soma(5, 3); // Não existem parametros opcionais em RUST, todos os parâmetros devem ser definidos
    println!("O resultado é: {}", resultado);
}

// Definição da função
fn soma(a: i32, b: i32) -> i32 {
    if a == 0{
        return b;
    };
    if b == 0{
        return b;
    };
    a + b
}