use std::io;

// Esse é um conversor de temperatura
fn main() {

    println!("######################################################");
    println!("1 - Converter Celsius para Fahrenheit");
    println!("2 - Converter Fahrenheit para Celsius");
    println!();
    println!("Escolha uma opcao: ");

    let mut opcao = String::new();
    let mut temperatura = String::new();

    io::stdin().read_line(&mut opcao)
        .expect("Não foi possível ler a opção ;-(");

    let opcao: u32 = opcao.trim().parse()
        .expect("Não foi possível converter o número!");
    
    println!("Digite o valor da temperatura: ");

    io::stdin().read_line(&mut temperatura)
        .expect("Não foi possível ler a temperatura ;-)");

    let temperatura: f32 = temperatura.trim().parse()
        .expect("Não foi possível converter a temperatura ;-)");


    println!();

    if opcao == 1 {
        converter_para_fahrenheit(temperatura);
    } else if opcao == 2 {
        converter_para_celsius(temperatura);
    }else{
        println!("Opção Inválida!!!!");
    }
}

fn converter_para_fahrenheit(temp: f32){
    let calculo = (temp/5.0) * 9.0 + 32.0;

    println!("A Temperatura {} ºC em ºF é {}", temp, calculo);
}

fn converter_para_celsius(temp: f32){
    let calculo = (temp - 32.0) / 9.0 * 5.0;

    println!("A temperatura {} ºF em ºC é {}", temp, calculo);
}
