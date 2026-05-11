// capitolo sui tipi composti 10-03-2026 12:25
// tuple e struct possono avere tipi di dato non omogenei.
// le tuple richiedono che gli elementi siano passati in ordine. Il valore è associato alla tupla tramite posizione. 

/* #[derive(Debug)]

struct Player {
        name : String,
        health: i32,
        level: u8
    }

fn main() {
    let p1 = Player{name:"Mario".to_string(), health: 25, level:1};
    let p2 = Player{name:"Luca".to_string(), ..p1};
    println!("{:?}\n{:?}", p1, p2);
} */

// ------------------------------------------------------------

/* #[derive(Debug, Default)]

struct Player {
        name : String,
        health: i32,
        level: u8
    }

fn main() {
    let p1 = Player{name:"Mario".to_string(), health: 25, level:1};
    let p2 = Player{name:"Luca".to_string(), .. Player::default()};
    println!("{:?}\n{:?}", p1, p2);
} */


// ------------------------------------------------------------------------------------------------------------------------

// slide 8 --> una struct senza campi non ha uno spazio di allocazione in memoria. Questo serve perchè i tratti sono assocaiti ai tipi, per cui posso definire questo tipo particolari per cui posso associargli i metodi dei tratti che sto defininendo. 

// ------------------------------------------------------------------------------------------------------------------------

// slide 10

/* #[derive(Copy,Clone,Debug)]
struct Vuota;

fn main() {
    let p = Empty;

    let mut array = Vec::new();
    for i in 0..10{
        array.push(p);
    }
    println!("{}", array.len());
    println!("{}", array.capacity()); // la capacity viene impostata ad infinito perchè non ha dimensione; se è infinito soo sicuro che non viene allocato in memoria
    println!("{:p}", array.as_ptr()); // il valore del puntatore è inutilizzabile perchè il vettore è vuoto. 
    
    for i in 0..10{
        println!("{}", array[i]);
    }
} */




// La struct memorizzata in memoria ha sempre una dimensione pari ad una potenza di 2. 

// i riferimenti a tutti i tipi di strutture dati vale 8 byte. 

// se il compilatore conosce la dimensione del dato il puntatore è semplice. Il vec ha implicita la sua dimensione, quindi il compilatore conosce la dimensione del vec. Se ho uno slice quello è un riferimento di cui il compilatore non conosce la dimensione, e quindi mi serve un puntatore grasso. I puntatori semplici 

// --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

// STRUCT PARTICOLARI : tuple struct vs unit struct

// le TUPLE STRUCT sono delle struct in cui non si specifica il nome del campo. A differenza delle tuple normali, una tuple struct ha un nome proprio che la distingue. Questo significa che un Point(0,0,0) e un Colore(0,0,0) sono tipi diversi per il compilatore, anche se i dati interni sono identici (IL NOME CHE ASSEGNO ALLA TUPLE STRUCT CORRISPONDE AL TIPO DELLA STRUCT STESSA). Si accede agli elementi esattamente come nelle tuple, usando la notazione con il punto e l'indice (es. p1.0, p1.1). 

// Le UNIT STRUCT sono struct che non occupano byte in memoria (es struct Empty) perchè non hanno alcun dato da memorizzare. La loro utilità è quella di definire tratti per un determinato tipo

// --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
// la riga di codice #[derive(Debug)] serve per dire al compilatore come stampare un tipo non elementare, tra cui una Struct

// --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

// slide 11 prova a farla
// ERRORE: quando una funzione ritorna un valore non devi mai scrivere let nel valore che devi ritornare
/* #[derive(Debug)]
struct Metric;
struct Imperial;
fn calculate_speed_km(d : f64, _unit : Metric) -> f64 {
    let vel = d / 1.0
}

fn calculate_speed_mph(d: f64, _unit: Imperial) -> f64 {
    let km_to_mph = d * 0.8
}

fn main() {
    let m = Metric;
    let i = Imperial;
    let distance = 120.0;

    let result1 = calculate_speed_km(distance, m);
    println!("The speed in Km/h is {:.2} km/h", result1);

    let result2 = calculate_speed_mph(distance, i);
    println!("The speed in Km/h is {} km/h", result2);
} */
// CORREZIONE
/* #[derive(Debug)]

struct Metric;
struct Imperial;
fn calculate_speed_km(d : f64, _unit : Metric) -> f64 {
    d / 1.0
}

fn calculate_speed_mph(d: f64, _unit: Imperial) -> f64 {
    let fattore_di_conversione = 0.8;
    d * fattore_di_conversione
}

fn main() {
    let m = Metric;
    let i = Imperial;
    let distance = 120.0;

    let result1 = calculate_speed_km(distance, m);
    println!("The speed in Km/h is {:.2} km/h", result1);

    let result2 = calculate_speed_mph(distance, i);
    println!("The speed in Km/h is {} km/h", result2);
}  */


// --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

// DESTRUTTURAZIONE DI UNA STRUCT E DISPOSIZIONE DELLA STRUCT IN MEMORIA 

// --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

// INCAPSULAMENTO E MODULARITA'
//ERRORE RIGA 145 (devi scrivere il debug dentro il modulo se implementi moduli), ERRORE riga 156

//#[derive(Debug)] errore, devi definirla dentro module1

/* mod module1 {
    #[derive(Debug)]
    pub struct Test {
        pub a : i32,
        pub b : bool,
    }
}

// use::Module1 errore
use Module1::Test;


fn main() {
    let x = Test { a: 10, b : true};
    println!("{:?}", x);
} */

// --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
// COSTRUTTORI E FUNZIONI ASSOCIATE

/* #[derive(Debug)]

pub struct Test {
    a :i32, // la struttura è pubblica (accesibile dal main); i campi li lasciamo privati per nascondere l'implementazione
    b :bool,
}
// quindi impl Test crea un blocco sintattico dentro il quale definisco tutti i metodi della Struct Test. 
impl Test {
    // con new posso creare un'istanza di Test con valori predefiniti
    fn new() -> Self {
        Test {a : 0, b : true}
    }
    fn with_initial_values(i:i32, boo:bool) -> Test {
        Test { a : i, b : boo}
    }
    fn evaluate(&self) -> bool { // in questo modo posso accedere in lettura all'istanza su cui ho chiamato il metodo
    if self.b == false {false}
    else if self.a != 0 {true} else {false}
    }
}

fn main() {
    let mut t = Test::new(); // se volessi riassegnare t usando il secondo metodo statico che ho chiamato with_initial_values devo dichiarare la variabile t come mutabile
    println! ("result of {:?} is: {:?}", t, t.evaluate());
    t = Test::with_initial_values(100, true);
    println!("result of {:?} is : {:?}", t, t.evaluate())
    
} */

// --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
#[derive(Debug)]

struct Player {
    nome : String,
    id : i32,
    health : f64,
}

impl Player {
    fn with_string(n : String) -> Self {
        Player {nome : n, id : 1, health: 5.0}
    }
}

fn main() {
    let p = Player::with_string("Mario".to_string());
    println!("result:  {:?}", p);
}
