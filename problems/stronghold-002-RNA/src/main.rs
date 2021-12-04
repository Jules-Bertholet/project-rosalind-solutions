use std::io;

fn main() {
    let mut dna_string = String::with_capacity(1000);
    io::stdin().read_line(&mut dna_string).unwrap();

    let rna_string = dna_string.replace('T', "U");

    println!("{}", &rna_string);
}
