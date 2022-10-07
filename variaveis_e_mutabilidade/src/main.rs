fn main() {
    
    // let x -> cria uma variável imutável que não pode ter seu valor trocado por outro .
    let mut x = 5; // a keyword mut indica que essa variável pode ser alterada ao longo do nosso
                   // código
    
    println!("O valor de x é {}",x);
    x = 6 ;
    println!("O valor de x é {}", x);

    // Shadowing
    
    let x = 5;
    let x = x + 1;
    let _x = x * 2;


}
