/* 
Ownership (Propriedade)
    Definição: Em Rust, cada valor tem um único "dono" (owner), que é responsável por gerenciar sua memória. 
Quando o dono sai de escopo, o valor é automaticamente desalocado.
    Regra de Movimentação: Ao mover um valor de uma variável para outra, a primeira variável perde a posse do valor. 
Isso previne problemas como double-free e uso após a liberação.

    Exemplo:

    fn main() {
        let s1 = String::from("hello");
        let s2 = s1; // 's1' move a propriedade para 's2'
        // 's1' não é mais válido aqui
    }

Borrowing (Empréstimo)
    Definição: Permite que você crie referências a valores sem transferir a propriedade. 
Existem dois tipos de empréstimos: imutáveis e mutáveis.
    Empréstimo Imutável: Você pode ter várias referências imutáveis a um valor ao mesmo tempo.
    Empréstimo Mutável: Você pode ter apenas uma referência mutável a um valor por vez, 
o que previne condições de corrida em cenários multi-thread.

    Exemplo:

    fn main() {
        let s = String::from("hello");
        empresta_imutavel(&s);
        println!("s ainda é válido aqui: {}", s);
    }

    fn empresta_imutavel(s: &String) {
        println!("empresta_imutavel: {}", s);
    }

Lifetimes (Tempo de Vida)
    Definição: Rust utiliza 'lifetimes' para garantir que todas as referências são válidas durante a vida útil dos dados a que se referem. 
Isso é feito de forma estática, durante a compilação.

    Exemplo:

    fn main() {
        let r;
        {
            let x = 5;
            r = &x;
            // 'x' não é mais válido aqui, então 'r' causaria um erro
        }
        // println!("r: {}", r); // Isso resultaria em erro de compilação
    }

Benefícios
    Segurança Sem Penalidade de Performance: 
Ao resolver problemas de memória em tempo de compilação, Rust oferece segurança sem comprometer o desempenho, diferentemente de linguagens que dependem de garbage collection (GC).
    
    Concurrência Segura: 
Através de suas regras de borrowing, Rust previne condições de corrida e outros problemas de concorrência de forma segura e eficiente.
    
    Comparação com Outras Linguagens
        C/C++: Em C/C++, os desenvolvedores precisam manualmente alocar e liberar memória, o que pode levar a erros 
    como vazamentos de memória, uso após liberação e double-free. Rust evita esses problemas ao garantir segurança de memória 
    através de seu sistema de propriedade.
        Linguagens com GC: Linguagens como Java e C# dependem de garbage collection para gerenciar memória automaticamente. 
    Embora isso simplifique o desenvolvimento, pode introduzir pausas imprevisíveis na execução do programa devido à coleta de lixo. 
    Rust oferece controle mais previsível e eficiente sobre a memória.

Em resumo, Rust revolucionou o gerenciamento de memória ao combinar segurança, eficiência e 
previsibilidade através de seus conceitos únicos de Ownership, Borrowing e Lifetimes. 
*/

//Código RAII

fn main(){
    let a = 1; //Valores do tipo copy (Primitivos escalares), que ficam na stack
    let b = a; //Para valores primitivos escalares, o Rust, da um COPY do valor a p/ o valor b

    

}