#![allow(non_snake_case)]
fn main() {
    let numero = 3;

    if numero < 5 {
        println!("Condição era verdadeira");
    } else {
        println!("Condição era falsa");
    }

    
    // Múltiplas Condições com else if

    let numero = 6;

    if numero % 4 == 0 {
        println!("Número é divisível por 4");
    } else if numero % 3 == 0 {
        println!("Número é divisível por 3");
    } else if numero % 2 == 0 {
        println!("Número é divisível por 2");
    } else {
        println!("Número não é divisível por 4,3 e 2");
    }

    let condicao = true;
    let numero = if condicao {
        5
    } else {
        6
    };

    println!("O valor do número é: {}", numero);



    // while


    let mut numero = 3;

    while numero != 0 {
        println!("{}", numero);

        numero = numero -1;
    }

    println!("LIFTOFF");


    let a = [10,20,30,40,50];
    let mut indice = 0;

    while indice < 5 {
        println!("O valor é: {}",a[indice]);

        indice = indice + 1;
    }


    // for
    
    for elemento in a.iter(){
        println!("O valor é: {}", elemento);
    }


    for numero in (1..4).rev(){
        println!("{}!", numero);
    }

    println!("LIFTOFF!!!");
}
