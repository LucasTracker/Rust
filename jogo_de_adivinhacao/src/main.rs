extern crate rand;
// A linha Abaixo inclue a biblioteca padrão de 
// Entrada e Saída
use std::io;
use rand::Rng; // Biblioteca para gerar números pseudoaleatórios
use std::cmp::Ordering; // Biblioteca padrão para realizar a comparação dos números.

fn main() {
    //Println! é uma macro para printar caracteres
    //Na tela
    println!("Adivinhe o número!");

    let numero_secreto = rand::thread_rng().gen_range(1,101);

    println!("O número secreto é: {}", numero_secreto);
    loop{ 
        println!("Digite o seu palpite.");

        // A linha abaixo cria uma variável local do Tipo String mutável para armazenar uma String
        // Que está vinculada ao retorno da função String:::new() que retorna uma instância String vazia.
        let mut palpite = String::new();


        //Função de Entrada de Dados da biblioteca padrão
        io::stdin().read_line(&mut palpite)
            .expect("Falha ao ler entrada");

        // Faz o sombreamento da variável palpite para converte-la para um u32 (inteiro de 32 bits
        // sem sinal) a função trim() elimina espaço e parse() transforma a string para um tipo de
        // número específico no caso u32
        let palpite: u32 = palpite.trim().parse()
            .expect("Por favor, digite um número!");

        println!("Você disse {}", palpite);

        match palpite.cmp(&numero_secreto){
            Ordering::Less => println!("Muito baixo!"),
            Ordering::Greater => println!("Muito Alto!"),
            Ordering::Equal => {
                println!("Você acertou!"),
                break;
            }
        }
    }
}
