fn main() {
    println!("Hello, world!");

    outra_funcao();

    outra_funcao2(5);

    let x = cinco();

    println!("O valor de x é: {}",x);
}

fn outra_funcao(){
    println!("Outra função.");
}

fn outra_funcao2(x: i32){
    println!("O valor de x é: {}", x);
}

// função com retorno

fn cinco() -> i32 {
    5
}
