//Tipos primitivos: Escalares e composotos

//Escalares
    //Representam um unico valor contido dentro de uma escala conhecida
    //Permitem a comparação direta entre valores

    //Tipos:

    //Inteiros ex: 5 ... n, por padrão sempre usa i32
         //   Bits  | Signed | Unsigned
        //  --------------------------
        //    8     | i8     | u8
        //    16    | i16    | u16
        //    32    | i32    | u32
        //    64    | i64    | u64
         //   128   | i128   | u128

        //signed: range: -(2^(n-1)) até 2^(n-1)-1
        //usigned: range: 0 até 2^(n-1)-1

        //Overflow: Quando aquela variavel passar do seu limite determinado
        //Literal: Podemos usar _ para separar dezenas de um numero
        //Bases:
            //hex 0x...
            //oct 0o...
            //binary 0b...
            //Bity b'...'
        
    //Flutuante ex: 42.1 ... n.n
        //f64 e f32

    //Booleano: true, false
    //Caractere 'a' ... 'emoji' até 4 bites da tabela unicode

//Compostos
    //Servem pra agregar multiplos valores

    //Tipos:
    //Tupla (tuples) ex: (5, true, 42,1, a), são heterogenas e aceitam combinação de valores dentro da tupla, porém é imutável na qtd de dados/tipo, mas podemos alterar seus elementos
    //Matriz (array) ex: [1, 2, 3, ..., n], não é heterogeneo

fn main(){
    //tupla
    let numbers_t= (1, 2, 3);
    println!("{:?}", numbers_t);

    let numbers_a= [1, 2, 3];
    println!("{:?}", numbers_a);

    //slice
    println!("{:?}", &numbers_a[..1]);

}