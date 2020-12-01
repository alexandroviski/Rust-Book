use std::io;

fn main() {
    println!("por favor insira um numero: ");
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    pub fn raindrops(n: u32) -> String {
        let mut resp = String::new();

        if n % 3 == 0 {
            resp.push_str("pling");
        }
        if n % 5 == 0 {
            resp.push_str("plang");
        }
        if n % 7 == 0 {
            resp.push_str("plong");
        }
        if resp.is_empty() {
            resp.push_str(&n.to_string());
        }
        resp
    }

    let unsigned_int = input_text.trim().parse::<u32>().unwrap();

    println!("{}", raindrops(unsigned_int));
}
