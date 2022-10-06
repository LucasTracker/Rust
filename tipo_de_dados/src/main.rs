#![allow(non_snake_case)]
fn main() {

    // RUST tem dois tipos de variáveis as escalares e as compostas
    // Escalares -> inteiros, ponto flutuante, booleanos e caracteres
    
    // inteiros

    let _a = 2;        //i32 -> inteiro com sinal (padrão)
    let _a: u32 = 2;  //u32 -> inteiro sem sinal
                      //
    // Pontos flutuante
    
    let _x = 2.0;  // f64 -> float de 64 bits (padrão)

    let _x: f32 = 3.0; // f32 -> float de 32 bits


    // Booleano

    let _b = false;
    let _b = true;

    // Caracter

    let _c = 'A';

    // Operações Aritméticas

    // adição
    
    let _soma = 5 + 10;

    // subtração
    
    let _diferenca = 95.5 - 4.3;

    // multiplicação
    
    let _produto = 4 * 30;

    // divisão
    
    let _quociente = 56.7 / 32.2;

    // resto
    
    let _resto = 43 % 5;

    // Tipos compostos
    // tuplas e vetores
    
    // Tupla
    
    let _tup: (i32, f64,u8) = (500, 6.4, 1);

    let tup = (500,6.4,1);

    // desestruturação
    let (_x,y,_z) =  tup;

    println!("O valor do y é: {}", y);

    // Acessando elementos da tupla atráves do '.'
    
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let _quinhentos = x.0;

    let _seis_ponto_quatro = x.1;

    let _um = x.2;

    // O tipo Matriz
    
    let _m = [1,2,3,4,5];

    let _meses = ["Janeiro", "Fevereiro", "Março", "Abril",
        "Maio", "Junho", "Julho", "Agosto", "Setembro", "Outubro",
        "Novembro", "Dezembro"];
}


