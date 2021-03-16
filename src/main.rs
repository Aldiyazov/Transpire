// Transpire
// MIT Licence - 2021
// main.rs

mod prompt;

fn main() {
    let fflags = prompt::process_args();
    println!("Suck Ma Balls too!");
    println!("Do flags work? {}", fflags.src_lang);
}
