//Variáveis e contantes

//Variaveis:
    //Para definir variáveis não podemos usar o escopo geral do programa, somente um escopo definido como o "main" por exemplo
    //Escrever let variavel = x; irá gerar erro, pois estará no escopo geral.
    //RAAI (Sigla para a forma como o Rust evita memory leek)

//Constantes:
    //p constante usamos sempre screaming snake case, sempre em maiusculo
    //Elas são imutáveis e não pode ser definida duas vezes
    //É obrigatoria ter o tipo
    //Ela pode ser definida fora do escopo

    
//Vamos desenvovler o programa referente a horas trabalahadas em segundos

const SECONDS_IN_MINUTE: u32 = 60; 
const MINUTES_IN_HOUR: u32 = 60;
const SECONDS_IN_HOUR: u32 = SECONDS_IN_MINUTE * MINUTES_IN_HOUR;

fn main(){ //escopo
    let total = 32; //Apesar de Rust ser estaticamente tipada, o compilador através do contexto/sintaxe ele entende o tipo da variável
    let total_em_segundos = total * SECONDS_IN_HOUR;
    //p/ definir o tipo use o let nome da variável: tipo = valor: let total: i32 = 32
    //toda variável do Rust é imutável, para ficar mutável use o "mut"
    //o tipo da varíavel é imutável, se desejar trocar, utilizar let (inicializando ele novamente) variable shadowing

    //Variable shadowing usando sub escopo
//    {
//        let total = total + 40;
//        println!("Trabalhou {} horas", total);
//   }
    //É possivel usar variaveis do contexto externo no contexto interno sem mudar a do externo
    
   println!("Trabalhou {} segundos", total_em_segundos); //println! Possuí interpolação de variáveis
}//fim
//Ao chegar no fim do escopo, ele executa o "drop" que finaliza as variáveis criadas