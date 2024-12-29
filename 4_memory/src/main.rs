//Tipos de memória:

    /* 
    Memória Static
    Definição: Memória alocada durante o tempo de vida do programa e não pode ser alterada em tempo de execução.

    Uso: Utilizada para variáveis globais e constantes.

    Memória Stack
    Definição: Memória de rápido acesso, organizada em uma estrutura LIFO (Last In, First Out). Usada para variáveis locais e chamadas de função.

    Características: Rápida alocação e desalocação, tamanho fixo e conhecido em tempo de compilação.

    Memória Heap
    Definição: Memória para alocação dinâmica. Não possui uma ordem específica para alocação e desalocação.

    Características: Flexível, tamanho pode ser determinado em tempo de execução, mas é mais lenta que a stack. 
    */

static _Y: u32 = 13; //Alocada num espaço fixo e imutável que termina quando o programa terminar.

fn main() {
    let y = 5; //Alocada num espaço dinamico mas definido, e finaliza ao finalizar o escopo odne ela foi cirada.

//    let users = get_users(); //Alocada num espaço dinamico e idenfinido (depende do limite de hardware), pode ser limpa por GC ou RAII
}
