use std::io;

// Gere o enésimo número de fibonacci

fn main() {
	println!("Digite o valor n: ");

	let mut a = 1;
	let mut b = 1;
	let mut number = String::new();
	let mut cont = 1;

	io::stdin().read_line(&mut number)
		.expect("Erro ao tentar ler a entrada :-(");

	let number: u32 = number.trim().parse()
		.expect("Por favor entre com um número!");
	
	println!();
	println!("Resposta:");
	while cont <= number{
		if cont % 2 == 0 {
			print!("{} ", b);
			b += 2;
		} else {
			print!("{} ", a);
			a += 1;
		}
		cont += 1;
	}
	println!();
}
