static TEXTO: &str = "hola";
use std::fmt::Display;
use std::io::prelude::*;
use std::path::Path;
use std::fs::File;
fn main() {
    let path = Path::new("hola.txt");
    let display = path.display();
    let mut file = match File::create(&path) {
        Err(why) => panic!("no se puede crear {}: {}", display, why),
        Ok(file) => file,
    };
    match file.write_all(TEXTO.as_bytes()) {
        Err(why) => panic!("no se puede leer {}: {}", display, why),
        Ok(_) => println!("Se creó {}", display),
    }
}
