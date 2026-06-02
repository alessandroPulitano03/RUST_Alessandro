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


/* use std::sync::{Arc, Mutex};
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
} */

// No, con std::thread::spawn non puoi passare shared_data per riferimento se è una variabile locale del main.

// Il motivo è che spawn può far vivere il thread più a lungo dello scope in cui shared_data è stato creato, quindi la closure deve possedere dati che siano validi per tutto il tempo di esecuzione del thread. Per questo spawn richiede, di fatto, una closure move con dati Send + 'static. Un riferimento a shared_data non soddisfa quel vincolo.

// La soluzione corretta con spawn è clonare l’Arc dentro il ciclo:

/* use std::sync::{Arc, Mutex};
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
} */

// ------------------------------------------------------------------------------------------------------------------------------------
// slide 80

/* use std::sync::{Arc,Mutex};
use std::thread;

fn main() {
    let data = Arc::new(Mutex::new(0));
    let cloned_data = Arc::clone(&data);
    let thread = thread::spawn(move|| {
        let mut num = cloned_data.lock().unwrap();
        *num += 1;
        panic!("Il thread ha avvelenato il mutex.") 
    });
    let wait_join = thread.join();
    let result = data.lock(); //qui accedo al MutexGuard
    // IMPORTANTE: anche se il thread secondario ha modificato la variabile clonata, la risorsa è pur sempre puntata da due puntatori SULLA STESSA RISORSA
    match result {
        Ok(guard) =>  {println!("Mutex non avvelenato. Valore : {}", *guard);}
        Err(poisoned) => {
            // step 1) Accedo all'involucro del dato nel mutex utilizzando .into_inner() su poisoned 
            // IMPORTANTE : il metodo .into_inner() applicato a poisoned mi permette di estrarre l'involucro della risorsa nel ramo negativo, a cui posso sempre  accedere usando l'asterisco perchè gode del tratto Deref
            let mut guard = poisoned.into_inner();
            println!("Mutex avvelenato. Valore recuperato : {}", *guard);
            *guard += 1;
            println!("Stato del mutex resettato. Nuovo valore : {}", *guard); 
        }
    }
} */

// ------------------------------------------------------------------------------------------------------------------------------------
// slide 83
// creo un Arc per permettere la condivisione della risorsa tra più thread
// creo un RWLock per incapsulare una risorsa per thread specificatamente in lettura e/o scrittura


/* use std::sync::{Arc,RwLock};
use std::thread;

fn main() {
    let data = Arc::new(RwLock::new(vec![1,2,3,4,5]));

    let cloned_data1 = Arc::clone(&data);
    let cloned_data2 = Arc::clone(&data);

    // step 1) creo un thread passando con movimento la variabile clonata
    let reader_thread = thread::spawn(move|| {
        // step 2) accedo in lettura all'RWLock con .read() e spacchetto il lucchetto con .unwrap()
        let guard = cloned_data1.read().unwrap();
        // step 3) stampo la lettura del thread lettore
        println!("Lettura dal thread lettore : {:?}", guard); // ERRORE : in questo caso non serve mettere *guard, perchè il metodo .read() va direttamente sul dato contenuto nell'RWLock
    });
    // WORKFLOW PER SCRVERE IL THREAD SCRITTORE
    let writer_thread = thread::spawn(move|| {
        // step 1) accedo in scrittura all'RWLock e spacchetto il risultato con .unwrap()
        let mut guard = cloned_data2.write().unwrap();
        // step 2) modifico la risorsa
        guard.push(6);
        // step 3) stampo la risorsa modificata
        println!("Scrittura effettuata dal thread scrittore : {:?}", guard);
    });

    reader_thread.join().unwrap();
    writer_thread.join().unwrap();

} */

// riepilogo step
// step 0) incapsulo una risorsa in un lucchetto RWLock a sua volta contenuto in un Arc. L'RWLock mi permette di accedere in lettura e/o in scrittura alla risorsa tramite il metodo .read() e .write(). L'Arc permette di passare la risorsa tra più thread
// step 1) creo un thread lettore a cui passo CON MOVIMENTO (move) un clone della variabile Arc originale
// step 2) accedo in lettura alla risorsa richiamando .read() per leggere e .unwrap() per aprire il lucchetto
// step 3) stampo la risorsa letta dal thread
// ripeto un il workflow in maniera analoga per il thread scrittore
// step finale) nel thread princiapale attendo la terminazione dei due thread utilizzando .join().unwrap()


// ------------------------------------------------------------------------------------------------------------------------------------
// slide 85

/* use std::sync::{Arc,RwLock};
use std::thread;
use std::time::Duration;

fn main(){
    //se voglio creare un thread e fargli elaborare delle risorse, creo prima una risorsa e la proteggo da un RWLock, che a sua volta sta dentro un Arc per peermettere di passare la risorsa tra più thread
    let data = Arc::new(RwLock::new(vec![1,2,3]));

    // creo due cloni del dato perchè voglio avere un thread lettore e un thread scrittore
    let cloned_data1 = Arc::clone(&data);
    let cloned_data2 = Arc::clone(&data);

    let thread_reader = thread::spawn(move|| {
        // In questo esercizio vogliamo forzare uno stato di panico: faccio attendere il thread lettore in modo che l'esecuzione del thread scrittore inizi prima. In questo esempio, il thread scrittore simula un panico, e poichè IL PANICO È SCATENATO SULLA STESSA RISORSA A CUI IL LETTORE VUOLE ACCEDERE, anche il thread lettore entra nel ramo ERR
        thread::sleep(Duration::from_secs(1));
        let guard = cloned_data1.read();
        match guard {
            Ok(value) => {println!("Il thread lettore accede alla risorsa in lettura : {:?} ", value)},
            Err(poisoned) => {println!("Il thread lettore accede ad una risorsa avvelenata : {:?}", poisoned.into_inner());}
        }
        });

    let thread_writer = thread::spawn(move|| {
        let mut guard = cloned_data2.write().unwrap();
        guard.push(4);
        panic!("Attenzione, il thread scrittore ha commesso un errore nell'esecuzione");
    });

    thread_reader.join().unwrap_err();
    thread_writer.join().unwrap_err();
} */

// ------------------------------------------------------------------------------------------------------------------------------------
// slide 92


/* use std::sync::{Arc,Mutex, atomic::AtomicBool, atomic::Ordering::{Release,Acquire}};
use std::thread;

fn main() {
    // step 0) definisco la risorsa dentro un lucchetto che rendo Arc (atomic RC) per poterla rendere condivisibile tra thread; e definisco una variabile atomica per sincronizzare i thread
    let data = Arc::new(Mutex::new(vec![1,2,3,4]));
    let boolean_variable = Arc::new(AtomicBool::new(false));


    // creo due cloni del dato per passarli con movimento rispettivamente al thread produttore e al thread consumatore
    let cloned_data1 = Arc::clone(&data);
    let cloned_data2 = Arc::clone(&data);

    // IMPORTANTE: clono la variabile atomica
    let cloned_boolean_variable = Arc::clone(&boolean_variable);

    // creo il thread produttore a cui passo con possesso il dato
    let thread_producer = thread::spawn(move|| {
        for i in 0..10 {
            let mut guard = cloned_data1.lock().unwrap();
            guard.push(i);
            println!("Nuovo valore aggiunto nella risorsa: {:?}", *guard);
        }
        /* boolean_variable.store(true, Release); */ // ERRORE IMPORTANTE : La cosa sbagliata di questa istruzione è che stai lavorando sulla variabile booleana atomica originale senza aver creato un clone. Ogni volta che da un thread vuoi accedere ad una risorsa generata nel thread principale, devi prima clonare la risorsa nel thread principale mettendola in un Arc
        cloned_boolean_variable.store(true, Release);
    });

    let thread_consumer = thread::spawn(move|| {
        loop{
            // in questo thread consumer voglio creare un loop che continua a rimuovere valori dalla risorsa fino a che non viene avvisato che il produttore ha finito; questo avviene acquisendo la variabile atomica booleana con il metodo .load()
            let mut guard = cloned_data2.lock().unwrap();
            let length = guard.len();
            if length > 0 {
                let value = guard.remove(0);
                println!("Valore rimosso : {}", value);
            } else if boolean_variable.load(Acquire) {
                break;
        }
    }
    println!("Il thread ha consumato tutti gli elementi")
    });
    thread_producer.join().unwrap();
    thread_consumer.join().unwrap();
 
} */

// ------------------------------------------------------------------------------------------------------------------------------------
// slide 92

/* fn main() {
    // step 0) creo una tupla (mutex,condition variable) e clono per poterla passare al thread attivo
    let pair = (Mutex::new(false), Condvar::new()); 
    let pair2 = Arc::clone(&pair);
    
    // step 1) creo un thread attivo --> attivo significa che modifica la variabile booleana e manda la notifica DOPO CHE IL THREAD PASSIVO È ANDATO IN SLEEP
    let active_thread = thread::new(move|| {
        // step 2) destrutturo la tupla richiamando la notazione &*
        let (mutex,cond) = &*pair2;
        // step 3) faccio si che il thread attivo prenoti la risorsa affinchè ci possa lavorare
        let mut guard = mutex.lock().unwrap();
        // step 4) mando il thread in sleep solo per simulare il compimento di una certa task
        thread::sleep(Duration::from_secs(5));
        // step 5) una volt terminato, il thread attivo setta a true il booleano. Solo dopo aver settato a ture il booleano posso mandare la notifica al thread passivo di una notifica per mezzo della condition variable
        *guard = true;
        cond.notify_one();
    });

    // nel thread principale devo sempre spacchettare la tupla, perchè devo accedere al risultato aggiornato dal thread attivo
    let (mutex,cvar) = &*pair;
    let guard = mutex.lock().unwrap();

    println!("waiting");
    cvar.wait(guard).unwrap()
}
 */
 // ------------------------------------------------------------------------------------------------------------------------------------
// una variabile di tipo ConditionVariable può utilizzare tre metodi per attendere il completamento del task da parte del thread attivo
// 1) .wait(guard :MutexGuard) -> LockResult<MutexGuard<T>>
// 2) .wait_while()
// 3) .wait_timeout() per le attese temporarizzate
// ------------------------------------------------------------------------------------------------------------------------
// esercizio slide 119
/* use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::time::Duration;
// l'idea dell'esercizio è di mettere i dati a cui i thread accedono dentro un Mutex.
// Poichè in questo esempio i thread modificano un vettore e un booleano, metto i due valori in una struct, che a sua volta inglobo nel Mutex
struct SharedData {
    buffer:  Vec<i32>,
    finished : bool
}

// dato che il mutex protegge due variabili, mi serve un'altra struct che lega il Mutex ad una ConditionVariable
struct SharedController {
    // inglobo la struct che contiene le risorse dentro un Mutex
    data: Mutex<SharedData>,
    cv: Condvar,
}

impl SharedController {
    pub fn new() -> Self {
        SharedController {
            data : Mutex::new(SharedData  {
                buffer : Vec::new(),
                finished : false,
            }),
            cv : Condvar::new(),
        }
    }

    pub fn produce(&self) {
        for i in 0..10 {
            // Simuliamo un tempo di calcolo per produrre il dato
            thread::sleep(Duration::from_millis(200));

            // acquisisco il lock per modificare il buffer e stampo il valore inserito
            let mut shared = self.data.lock().unwrap();
            shared.buffer.push(i);
            println!("[Produttore] : Ho inserito il valore : {}", i);

            // Avviso i consumatori che c'è un nuovo dato
            self.cv.notify_all();
        }

        // segnalo il termine della produzione
        let mut shared = self.data.lock().unwrap();
        shared.finished = true;
        self.cv.notify_all();
        println!("[Produttore] : Produzione terminata.")
    }

    pub fn consume(&self, consumer_id : usize) {
        loop {
            // Prendiamo il lock sul Mutex
            let mut shared = self.data.lock().unwrap();

            // utilizzo wait_while che fa dormire il thread se il buffer è vuoto e la produzione non è finita
            shared = self.cv.wait_while(
                shared,
                // IMPORTANTE : il secondo parametro della funzione è il contenuto del mutex. 
                |state| { state.buffer.is_empty() && !state.finished }
            ).unwrap();

            // Se il buffer è vuoto e la produzione è finita, usciamo dal ciclo (lavoro completato)
            if shared.finished && shared.buffer.is_empty() {
                break;
            }

            if let Some(value) = shared.buffer.pop() {
                println!("[Consumatore {}] Ho rimosso e letto il valore: {}", consumer_id, value);
            }
        }
        println!("[Consumatore {}] Ho finito e mi spengo.", consumer_id);


    }
}


fn main() {
    let shared_controller = Arc::new(SharedController::new());

    let mut handles = vec![];

    // clono lo shared controller e creo il produttore
    let producer_clone = Arc::clone(&shared_controller);
    handles.push(thread::spawn(
        // nota che utilizzi la funzione produce sulla struct che contiene le risorse che vuoi elaborare
        move || { producer_clone.produce();}
    ));

    let cloned_data_consumer2 = Arc::clone(&shared_controller);
    // creo due thread consumer con i rispettivi id per distinguirli
    for id in 1..=2 {
        let cloned_data_consumer2 = Arc::clone(&cloned_data_consumer2);
        handles.push(thread::spawn ( 
           move || {cloned_data_consumer2.consume(id);}
        ));
    }
    for handle in handles {
        handle.join().expect("Errore durante la join del thread");
    }

    println!("[Main] Tutto il sistema ha terminato correttamente.");
} */


