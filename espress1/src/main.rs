/* // IMPORTANTE --> HO FATTO ERRORI E GLI HO CAPITI
fn main(){
    let tupla0 = (3.45, false);
    let t1 = tupla0.0;
    println!("{}", t1);

    
    let mut tupla1 = (4, true); // il compilatore decide che la tupla è formata da due interi (tipo i32)

    tupla1.1 = 1.5; // non posso assegnare un valore floating point se prima avevamo detto che l'elemento 1 della tupla deve essere un intero'

    println!("{}", tupla1) //le tuple non implementano il tratto display, serve il tratto debug. 
} */

/* fn main(){
    let tupla = (100, 200, 300);
    let (_x,y,_z) = tupla; // precedi le variabili con un underscore se non le usi

    println!("The value of y is: {}", y );
}
 */

// -------------------------------------------------------------------------------------------------------------------------------------------

// MIGLIORIE DEL COMPILATORE RUST RISPETTO AL C

// MIGLIORIA 1: DEALLOCAZIONE AUTOMATICA 

/* fn main() {
    let i = 5;

    let j = 2;

    let result = i+j;

    println("{}", i); // a questa riga, il compilatore dealloca dallo stack la variabile i perchè il suo tempo di vita è terminato. Il compilatore C riserva ancora spazio in memoria per la variabile. 

    println("{}", result);

} */

// MIGLIORIA 2 : ESPANSIONE DELLE MACRO
// Il compilatore Rust espande il codice delle macro per renderlo più efficiente e sicuro in base al tipo di dato scritto nella macro. Il compilatore espande il codice su misura del tipo di dato inserito nella macro. Il compilatore C, che lavora con gli specificatori di formato, esegue la formattazione in tempo di run-time, che può eventualmente mandare in crush il programma se il programmatore ha inserito dati sbagliati. 

// MIGLIORIA 3: IL PANIC
// Il compilatore Rust fornisce un livello di sicurezza per codici che sono pericolosi o che non possono proseguire in sicurezza in certe situazioni. Questo impedisce situazioni in cui il processore esegua istruzioni che non sono possibili (come la divisione per zero).

//--------------------------------------------------------------------------------------------------------------------------------------------

// RIFERIMENTI MUTABILI ED IMMUTABILI

// MUTABILE: posso creare un solo riferimento mutabile alla volta e a questo posso accedervi in lettura e scrittura; tuttavia non posso in alcun modo modificare il valore del riferimento fintanto che esiste il riferimento mutabile. 
/* fn main() {
    let mut i = 32;

    let r = &mut i;

    *r = *r + 1; // modifico il riferimento mutabile
    println!("{}", *r);

    i = i - 1;
    println!("{}", i);

} */

// IMMUTABILE: È possibile creare N riferimenti immutabili per un valore o un'espressione e accedervi in sola lettura, ma durante il tempo di vita di questi non è possibile in alcun modo modificare il dato

/* fn main(){
    let mut i = 30;

    let r = & i;

    let r2 = & i;

    println!("Riferimento immutabile 1: {}", r);
    println!("Riferimento immutabile 2: {}", r2);

    let r_mut = &mut i;

    *r_mut = *r_mut + 1;

    println!("Riferimento mutabile 1: {}", *r_mut);

    println!("Valore originale {}", i);



} */

