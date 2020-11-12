use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("advinhe o numero");

    let numero_secreto = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("digite seu palpite");

        let mut palpite = String::new();

        io::stdin()
            .read_line(&mut palpite)
            .expect("falha ao ler entrada");

        // trim remove o enter do usuario
        // o metodo parse de strings
        // converte string pra algum
        // tipo de dado
        //
        // aqui trocamos uma chamada expect por
        // uma chamada match
        let palpite: u32 = match palpite.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("voce disse: {}", palpite);

        match palpite.cmp(&numero_secreto) {
            // braco               codigo que deve ser executado
            // de codigo           caso seja verdadeira
            Ordering::Less => println!("muito baixo"),
            Ordering::Greater => println!("muito alto"),
            Ordering::Equal => {
                println!("acertou");

                break;
            }
        }
    }
}
