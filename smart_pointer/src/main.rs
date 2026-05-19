
fn produce ( odd:bool) -> Box<i32> {
    let mut b = Box::new(0);
    if odd { *b = 5; }
    b
}


fn main() {
    let b1 = produce(false);
    println!("b1 : {:?}", b1);
    let b2 = produce(true);
    println!("b2 : {:?}", b2);
}

// ------------------------------------------------------------------------------------------------------------------------------------------------------------------
// slide 24




// ------------------------------------------------------------------------------------------------------------------------------------------------------------------
// slide 26
// Il box serve per implementare oggetti tratto

/* trait Animal {
    fn make_sound(&self) -> String;
}

struct Dog;
impl Animal for Dog {
    fn make_sound(&self) -> String 
    {"Bau!".to_string()}
}

struct Cat;
impl Animal for Cat {
    fn make_sound(&self) -> String {"Miao!".to_string()}
}

fn main() {
    let animals : Vec<Box<dyn Animal>> = vec![Box::new(Dog), Box::new(Cat)]
} */

// ------------------------------------------------------------------------------------------------------------------------------------------------------------------
// slide 27

/* use std::io;
fn main() {
    println!("Inserisci la dimensione (in byte) del dato puntato dal Box:");
    let mut input_line = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Errore nella lettura dell'input.");

    let num_bytes: usize = input_line.trim().parse().unwrap();
    // .trim() cancella tutti gli spazi bianchi e rimuove il carattere di "a capo"
    // .parse() tenta di convertire un testo in un numero in cui tipo deve essere specificato come tipo. Per questo motivo, se l'utente scrive una stringa del tipo "ciao", il parse inserisce dentro un Err la stringa, perchè si apsetta un input tipo "10" da poter convertire in un numero
    // .unwrap() estrae il risultato, e in caso positivo assegna il valore numerico alla variabile num_bytes

    // Alloca un Vec<u8> con la dimensione specificata.
    let buffer: Vec<u8> = vec![0; num_bytes];
    // Converti il Vec in un Box<[u8]> per avere un puntatore a un blocco di memoria di dimensione fissa sull'heap
    let boxed_slice: Box<[u8]> = buffer.into_boxed_slice();
    println!("Box<[u8]> allocato con successo. Dimensione: {} byte.", boxed_slice.len());
} */

// ------------------------------------------------------------------------------------------------------------------------------------------------------------------
// 1h 50' 13''