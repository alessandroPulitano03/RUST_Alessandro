// per questi codici ho scritto degli appunti molto importanti sul tablet

/* fn main() {
    // COMPORTAMENTO PER I TIPI CHE NON GODONO DI COPY
    let mut s1 = String::from("hello");
    let s2 = s1; // qui Rust crea un buco nello stack per la tripletta (pointer,size,capacity) di s1 distruggento quei parametri e utilizza nuovo spazio nello stack per i parametri di s2; inoltre il possesso del dato di s1 nello heap è puntato dal puntatore di s2. Questo significa che non posso interagire al dato "hello" tramite s1, ma posso riassegnare una nuova tripletta nello sipazio di stack di s1. 
    println("s2 : {:?}", s2);
    s1 = s2.to_uppercase();

    // COMPORTAMENTO PER I TIPI CHE  GODONO DI COPY
    let num1 = 12;
    let num2 = num1 // non ci sono vincoli su num1 perchè la CPU esegue una copia bit a bit di num1 sulla variabile num2 che vive nello stack perchè è di tipo i32
    println!("num1 : {}", num1); // posso accedere in lettura a num1 senza problemi
    println!("num2 : {}", num2);
} */

/* fn main() {
    let mut s1 = "Dante Alighieri".to_string(); // Stack(tripletta --> pointer, size, capacity), Heap ("Dante Alighieri") ; String ; Move e Drop --> RICORDA SEMPRE CHE SE UNA VARIABILE GODE DEL TRATTO MOVE ALLORA NON PUO' GODER DEL TRATTO COPY, SONO IN MUTUA ESCLUSIONE (IMPORTANTE)
    println!("s1 : {:?}", s1);
    let mut s2 = s1; // Rust distrugge logicamente la tripletta di s1 creando un buco. Il possesso del dato "Dante Alighieri" viene dato a s2 e nello Stack viene creata una nuova tripletta (pointer, size, capcity) per s2. Il pointer di s2 punta alla stessa memoria heap che prima era puntata da s1
    s1 = s2.to_uppercase(); // Posso assegnare al buco nello stack un dato che deve essere necessariamente preso da una variabile di sitpo String che logicamente ha ancora in vita la tripletta nello Stack. Non posso accedere il lettura ad s1 perchè il dato lo possiede s2
    s2 = s2.to_uppercase();
    s2.push_str(" HA IL NASO CORVINO!");
    println!("s2 : {:?}", s2);
} */

//  la differenza tra Move e Clone e che Move si verifica sui tipi CHE NON IMPLEMENTANO IL TRATTO COPY quando il possesso di un dato viene spostato da una variabile ad un'altra, e questo significa che la tripletta (pointer, size, capacity) della prima variabile viene logicamente annullata per creare una nuova tripletta per la seconda variabile che riceve il possesso. Questo non modifica in alcun modo lo heap, è un'operazione che avviene a livello di stack.
// Clone invece è un metodo che copia sia la tripletta sia i dati in un'altra zona dello heap 
// I TIPI CHE IMPLEMENTANO IL TRATTO COPY VENGONO SEMPRE DUPLICATI NELLO STACK SENZA INVALIDARE L'ORIGINALE. 


// SLIDE 50
/* fn cambia(new_ref : &mut Box<i32>) ->&mut Box<i32> {
    *new_ref = Box::new(100);
    new_ref
}

//vedi video telefono per ripassare
fn main() {
    let mut my_box = Box::new(100); // creo un box, cioè un puntatore nello stack che punta ad un indirizzo heap che contiene il valore 100
    let mut z = &mut my_box; //un riferimento è un puntatore ad una variabile puntatore: qui z punta alla variabile my_box, che a sua volta punta allo heap
    *z = Box::new(200); // accedo all'indirizzo puntato da z e modifico il valore a 200:

    z = cambia(z); // il riferimento viene mosso al parametro new_ref nella funzione, e a tutto il diritto di modificare my_box. IMPROTANTE --> nota che avevo dichiarato z come mutabile perchè qui z viene riassegnata con un valore restiuito dalla funzione "cambia"

    let first_ref = & *z; //questo è un riferimento al dato puntato da z. Nota che il dato è di dimensione fissa (Sized) e first_ref è un puntatore nello stack che punta allo stesso indirizzo heap di z
    let second_ref = first_ref;

    println!("first_ref : {}", first_ref);
    println!("second_ref : {}", second_ref);

    println!("my_box : {}", my_box);
} */


// SLIDE 51
/* fn cambia (myref: & mut Box<i32>) -> &mut Box<i32>
{
    *myref = Box::new(200);
    myref
}

fn main() {
    let mut mybox = Box::new(150);
    let mut z = &mut mybox;
    *z = Box::new(100);

    z = cambia(z);

    let newref = & *z; // qui creo un riferimento immutabile al dato posseduto da z. 

    println!("{:?}", newref);
    println!("{:?}", mybox);

    println!("{:?}", newref); // qui per il compilatore newref è ancora vivo, il codice non compila perchè il possessore originale del dato tenta di accedere al dato quando esiste ancora un riferimento immutabile al dato stesso
} */

/* fn passaggio (my_ref : &Box<i32>) {
    println!(" my_box : {:?}", my_ref);
}

fn main() {
    let mut my_box = Box::new(80);
    let z = &mut my_box;

    *z = Box::new(100);

    passaggio(&*z);

    *z = Box::new(200);
    println!("z : {:?}" *z);
    println!("z : {:?}" *my_box);
} */

//SLIDE 54

/* fn main() {
    let mut my_box = Box::new(100);
    let mut z = & my_box;

    for i in 0..10 {
        println!("z : {:?}", *z);
        my_box = Box::new(i);
        z = &my_box; // se questa riga non ci fosse, avrei z che alla seconda iterazione del for legge il dato nel riferimento che è stato cambiato nell'iterazione precedente, ma questo è assurdo perchè per definizione un riferimento prevede che il dato originale a cui punta non cambi fino a che il tempo di vita del riferimento muore o il riferimento cambia. Aggiungendo questa riga creo un nuovo riferimento che annulla la z nel blocco esterno.
    }
    println!("{:?}", *z);
} */

// SLIDE 71,72
// Esistono dei metodi per passare da una struttura dati ad un'altra, tipicamente da un vec ad un box. Quando faccio questo avviene un movimento, perchè i dati che prima erano puntati dal puntatore del vec vengono messi dentro il box. il metodo into_boxed_slice() converte l'allocazione dinamica del Vec in una più snella, per cui la variabile vec viene consumata.

/* fn main() {
    let vec = vec![1,2,3,4];
    let boxed_slice : Box<[i32]>= vec.into_boxed_slice(); //quindi il Box<[i32]> è un tipo in rust. Questo tipo è un Box che contiene uno slice
   
    // let v = &*boxed_slice[1..3]; quando scrivi boxed_slice[1..3], Rust prende direttamente una porzione dei dati dello slice. Aggiungere la deferenziazione con asterisco è sbagliato perchè non ha senso deferenziare un dato --> L'ISTRUZIONE È ERRATA
    let v : &[i32] = &boxed_slice[1..3];
    println!("{:?}", v);
    println!("{:?}", boxed_slice);
} */

// in questo programma creo un vettore ed estreggo primatutto il vettore con uno slice e poi una porzione di esso

/* fn main() {
    let v = vec![1,2,3,4,5];

    let s1 = &v[..];
    let s2 = &v[1..3];

    println!("s1 : {:?}", s1);
    println!("s2 : {:?}", s2);  
} */

// in questo programma passo da un box ad un vec

/* fn main() {
    let my_box = Box::new([1,2,3,4,5]);
    let my_vec : Vec<i32> = my_box.to_vec();
    println!("{:?}", my_vec);
} */

// in questo programma faccio pratica con la scrittura di slice. Il trucco è che uno slice lo devo scrivere usando l'operatore ref
// 1) scrivo un boxed_slice

/* fn main() {
    let boxed_slice : Box<[i32]> = Box::new([1,2,3,3,4,5]);
    let slice1 : &[i32] = &*boxed_slice;
    let slice2 : &[i32] = &boxed_slice[2..4];
    println!("slice1 : {:?}", slice1);
    println!("slice2 : {:?}", slice2);
} */