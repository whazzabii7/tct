use std::io::{self, Write};

fn print_banner() {
    print!("\033[1;36m");
    println!("╔════════════════════════════╗");
    println!("║        TCT CLI Tool        ║");
    println!("╚════════════════════════════╝");
    print!("\033[0m");
    match io::stdout().flush() {
        Ok(_) => {},
        Err(e) => panic!("{:?}", e),
    }
}

fn main() {
    print_banner();

    print!("\033[1;32m[TCT]\033[0m Starte interaktive Sitzung...\n");
    print!("\033[1;34m[INFO]\033[0m Gib ':q' ein zum Beenden.\n\n");

    let mut input = String::new();
    loop {
        print!("> ");
        if io::stdin().read_line(&mut input).unwrap() == 0 { break;}
        input = input.trim().to_owned();

        if input.as_str() == ":q" { break; }
        print!("\033[1;33m[Du hast eingegeben:]\033[0m {}", input);
    }
}
