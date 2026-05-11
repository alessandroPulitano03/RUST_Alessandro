// IN QUESTO CODICE PROVIAMO I CONCETTI DI RIFERIMENTI MUTABILI E IMMURABILI

/* fn main() {
    let mut _i = 30;
    let p = &i;
    let pp = &p;
    let ppp = &pp;

    let stringa = ppp.to_string();
    println!("{}", stringa);
} */


// CODICE
/* fn main() {
    // ripasso riferimento mutabile
    let r= &mut 12; // è possibile definire un riferimento mutabile ad un valore 
    *r += 1; // posso sempre modificare un riferimento mutabile (lettura o scrittura)

    // riferimenti immutabili ed espressioni
    let a = 5;
    let r_mut = & (a + 2);
    let r_espressione = &mut (r_mut * 2); 
    println!("Risultato espressione: {}", r_espressione);

    // case use: riutilizzo i riferimenti per crearne altri, con offsett o prodotto per costante. 
    let b = 30;
    let r1 = & (b + 2); //qui creo una nuova variabile scrivendo b+2 (offsett di una variabile definita prima), e ne creo un riferimento immutabile

    let r2 = & (*r1 * 2); // qui posso usare il riferimento immutabile creato prima, in sola lettura, e lo uso per creare un altro riferimento immutabile

    println!("b : {}", b);
    println!("r1 : {}", r1);
    println!("r2 : {}", r2);
} */

// in questo codice creo un riferimento immutabile da una variabile i32.

/* fn main() {
    let x : i32 = 30; // il dato vive solo nello stack, perchè si tratta di un intero a 32 bit di dimensione fissa
    let r : &i32 = &x // il riferimento immutabile r è un puntatore del dato. Un puntatore è un indirizzo dello stack 
} */

// in questo codice creo un riferimento ad un box

/* fn main() {
    let b : Box<i32> = Box::new(100); // b è un puntatore (il metadato) nello stack al dato che sta nello heap
    let r : &Box<i32> = &b; // r è un puntatore alla variabile nello stack, non allo heap. La CPU prima va puntatore b nello stack, e poi accede ai dati nello Heap

    println!("r : {:?}", r);
} */

//in questo codice creo un riferimento ad un Vec tramite uno slice. Quindi creo un FAT POINTER

fn main(){
    let v : Vec<i32> = vec![1,2,3,4,5];
    let s : &[i32] = &v[1..3]; // la slice è un fat pointer, cioè un puntatore che indica direttamente i dati nello heap. Inoltre c'è anche l'informazione sulla lunghezza del dato puntato 

    println!("lo slice : {:?}", s);

}