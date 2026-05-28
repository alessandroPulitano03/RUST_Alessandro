/* fn main() {
    use std::thread;
    let a = vec![1, 2, 3];
    let builder = thread::Builder::new()
        .name(String::from("t1"))
        .stack_size(100_000);
    let handler = builder
        .spawn( move || {
        println!("{}", a[4]); // panica. Viene fornito il nome del thread.
    });
    let result = handler.unwrap().join();
    match result {
        Ok(()) => {println!("Terminato Thread");},
        Err(err) => {println!("Errore {:?}", err);}
        }
    println!("Fine");
} */

// ------------------------------------------------------------------------------------------
// slide 52 

// thread::spawn(|| {...}) --> JoinHandle<T>
// .join() --> Result<T>
// buillder.spawn() --> io::Result<JoinHandle<T>>

// thread::current().id() --> id del thread

// ------------------------------------------------------------------------------------------
// Per le slide precedenti studia dalle slide


// slide 77


use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let shared_data = Arc::new(Mutex::new(Vec::new()));
    let mut threads = vec![];
    for i in 1..10 {
        /* let ref_share_data = &shared_data */ // <-- ERRORE
        threads.push( thread::spawn( move || { //data è ceduto al thread
            /* let mut v = ref_share_data.lock().unwrap();  */ // <-- con std::thread::spawn non puoi passare shared_data per riferimento se è una variabile locale del main.
            println!("{:?}", i);
            v.push(i); //quando v esce dallo scope, il lock  //viene rilasciato
            }) 
        );
    }
    for t in threads { t.join().unwrap(); }

    //v contiene i numeri da 1 a 9
    println!("\nResult: {:?}", *(shared_data.lock().unwrap()));
}

// No, con std::thread::spawn non puoi passare shared_data per riferimento se è una variabile locale del main.

// Il motivo è che spawn può far vivere il thread più a lungo dello scope in cui shared_data è stato creato, quindi la closure deve possedere dati che siano validi per tutto il tempo di esecuzione del thread. Per questo spawn richiede, di fatto, una closure move con dati Send + 'static. Un riferimento a shared_data non soddisfa quel vincolo.

// La soluzione corretta con spawn è clonare l’Arc dentro il ciclo:

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let shared_data = Arc::new(Mutex::new(Vec::new()));
    let mut threads = vec![];

    for i in 1..10 {
        let shared_data = Arc::clone(&shared_data);
        threads.push(thread::spawn(move || {
            let mut v = shared_data.lock().unwrap();
            println!("{:?}", i);
            v.push(i);
        }));
    }

    for t in threads {
        t.join().unwrap();
    }

    println!("\nResult: {:?}", *shared_data.lock().unwrap());
}