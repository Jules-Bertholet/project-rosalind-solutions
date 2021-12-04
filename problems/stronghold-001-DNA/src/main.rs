use std::io;

fn main() {
    let mut dna_string = String::new();
    io::stdin().read_line(&mut dna_string).unwrap();

    let [num_a, num_c, num_g, num_t]: [usize; 4] =
        dna_string
            .chars()
            .fold([0_usize, 0, 0, 0], |mut acc, char| {
                match char {
                    'A' => acc[0] += 1,
                    'C' => acc[1] += 1,
                    'G' => acc[2] += 1,
                    'T' => acc[3] += 1,
                    _ => (),
                };
                acc
            });

    println!("{} {} {} {}", num_a, num_c, num_g, num_t);
}
