use std::io;

fn main() {
    loop {
        println!("Qual'è la posizione nella serie di fibonacci del numero che stai cercando?");

        let mut x = String::new();
        io::stdin().read_line(&mut x)
            .expect("Failed to read file");

        // Parse user input
        let x: u8 = match x.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // That's too much, man
        if x > 93 {
            println!("gfy");
            break
        }

        println!("Il numero è {}", fibonacci(x));
    };
}

fn fibonacci(x: u8) -> u64 {
    let mut curr = 1u64;
    let mut prev = 1u64;
    #[allow(unused_assignments)]
    let mut tmp = 1u64;

    for _i in 3..(x+1) {
        tmp = curr;
        curr = curr + prev;
        prev = tmp;
    }

    curr
}
