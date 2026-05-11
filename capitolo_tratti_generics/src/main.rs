// slide 2
/* #[derive(Debug)]

struct Player {
    name : String,
    health : i32,
    age: i32,
}

fn main() {
    let p1 = Player { name : "Ale".to_string(), health : 100, age: 23 };
    // println!("player 1 data : {:?}, {}, {}", p1); --> NON CONVIENE
    println!("P1 : {:?}", p1);
} */

// ------------------------------------------------------------------------------------------------------------------------------------------------------------------------

// slide 5 --> instanziare una struct a partire da una esistente

/* #[derive(Debug)]

struct Player {
    name : String,
    health : i32,
    age: i32,
}

fn main() {
    let p1 = Player { name : "Ale".to_string(), health : 100, age: 23 };
    // ERRORE let p2 = Player { name : "Marta".to_string(), p1..};
    let p2 = Player { name : "Marta".to_string(), ..p1};
    println!("p1 : {:?}", p1);
    println!("p2 : {:?}", p2);
} */

// ------------------------------------------------------------------------------------------------------------------------------------------------------------------------

// utilizzo del tratto Default 

/* #[derive(Debug,Default)]

struct Player {
    name : String,
    health : i32,
    age: i32,
}

fn main() {
    let p1 = Player { name : "Ale".to_string(), health : 100, age: 23 };
    let p2 = Player { name : "Marta".to_string(), ..Player::default()}; 
    println!("p1 : {:?}", p1);
    println!("p2 : {:?}", p2);
} */

// ------------------------------------------------------------------------------------------------------------------------------------------------------------------------

// slide 9 --> riepilogo dei concetti : Struct, Tuple Struct e Unit Struct

/* #[derive(Debug,Defualt)]

struct DocumentID {
    name: String,
    surname : String,
    age: i32,
    tax_id_code : String, 
}
 
struct BasicInfo(String,String); // name, surname

struct Empty;

fn main() {
    let d1 = DocumentID {name: "ale".to_string(), surname : "puli".to_string(), age: 23, tax_id_code: "PLTNLSN"};
    let d1_basic_info = BasicInfo("ale", "puli");
    let unit_struct = Empty;
    let tuple_info = (d1_basic_info.0, d1_basic_info.1);

    let d2 = DocumentID::default(); // questa struct può utilizzare default perchè tutti i suoi campi implementano a loro volta il tratto Default
    let d3 = BasicInfo::default();

    println!("d1 : {:?}", d1);
    println!("d1 name : {:?}", d1.name) // per accedere ad un campo specifico uso la notazione .var
    println!("d1_basic_info : {:?}", d1);
    println!("tupla : {:?}", tuple_info);
} */

// ------------------------------------------------------------------------------------------------------------------------------------------------------------------------

// slide 22

/* pub mod module1 {
    #[derive(Debug)]
    pub struct Test {
        pub a : i32,
        pub b : bool,
    }
}

use module1::Test; // se non scrivessi la parola "use" per l'importazione nel main dovrei scrivere ogni volta il percorso completo: let t = Module1::Test { ... };
// Utilizzare la parole "use" equivale a creare un "alias" o una scorciatoia nel tuo scope attuale. Da quel punto in poi, puoi chiamare la struct semplicemente scrivendo Test. Ingegneristicamente, è come aggiungere una libreria specifica al tuo ambiente di lavoro per non doverne digitare l'indirizzo completo ogni volta

fn main() {
    let t1 = Test {a : 5, b : false};
    println!("t1 : {:?}", t1);
} */

// da qui sto scrivendo da mac control f intelligente------------------------------------------------------------------------------------------------------------------------------------------------------------------------

// definire il blocco impl serve per specificare i comportamenti di una struct. Sono le funzioni che hanno accesso ai campi della struct. Ogni funzione scritta dentro un blocco impl richiede l'utilizzo della parola chiave "self", che prende il possesso dell'istanza. È possibile sostituire self con &self, che prende un prestito immutabile, oppure con &mut self, che permette di modificare i dati dell'istanza di struct a cui si fa riferimento. Quindi il parametro self definisce il livello di accesso del metodo sul ricevitore (con ricevitore si intende un altro nome per chiamare l'istanza su cui applico il metodo)

// i comportamenti definiti nel blocco impl possono essere scritti in due modi
// 1) UTILIZZO METODI --> Prevedono l'utilizzo del parametro self e si chiamano con la dot notation --> oggetto.metodo()
// 2) FUNZIONI ASSOCIATE  --> Non utilizzano il parametro self e sono l'equivalente dei metodi statici, per cui  non c'è bisogno di creare un oggetto per poter essere usate. Tendenzialmente una funzione associata è un costruttore, quindi un metodo statico che crea direttamente un'istanza della struct. Per convenzione si usa il metodo "new" per creare un costruttore di una struct. I metodi statici sono definiti su un tipo anzichè su un'istanza di un tipo. I metodi statici sono usati anche per convertire un'istanza di un altro tipo nel tipo corrente.

// la chiamata di un metodo statico è fatta così --> <Tipo>::metodo()

// Le struct possono comunicare tra loro attraverso i tratti, e non esiste il concetto di ereditarietà 

// qui sotto creo un esempio di codice che usa sia una funzione associata, sia un metodo, tutti e due dentro il blocco impl, che come dicevamo descrivono il comportamento di una struct. 

/* #[derive(Debug)]

struct Punto {
    x : i32,
    y : i32,
}

impl Punto {
    fn new(x:i32, y:i32) -> Self { // creo un'istanza della struct Punto
        Punto {x,y}
    }

    fn distanza_origine(&self) -> f32 {
        let somma_quadrati = self.x.pow(2) + self.y.pow(2);
        (somma_quadrati as f32).sqrt()
    }
}

fn main() {
    let p1 = Punto::new(2,2);
    println!("p1 : {:?}", p1);
    let distanza = p1.distanza_origine();
    println!("distanza p1 da origine : {}", distanza);
} */

// ricordati sempre che se vuoi modificare il valore dei campi di self in un metodo nel blocco impl devi specificare un riferimento mutabile nell'intestazione del metodo, utilizzando &mut self

// ------------------------------------------------------------------------------------------------------------------------------------------------------------------------

// slide 32 specifica che in Rust non esiste un costruttore chiamato dal compilatore per creare un'istanza (cosa che succede in Java o C++). In Rust io creo una funzione associata perchè io programmatore so che quella crea un'istanza, ma per il compilatore, essa è una funzione come un'altra, siamo noi a specificare che crea un'istanza. Nelle slide c'è scritto che il metodo statico new() per inizializzare un'istanza favorisce l'incapsulamento

// slide 33 --> IMPORTANTE --> un metodo costruttore non prende parametri in input e inizializza una Struct con dei valori di default che gli assegni tu. È la versione custom del metodo default che importi tu scrivendo #[derive(Default)]. Quindi preferisci l'utilizzo di un metodo costruttore new(). 

// qui sotto provo a scrivere il codice di slide 34 --> inizializzo un'istanza della struct Player, che spcifica i valori di health e level

/* #[derive(Debug)]

struct Player {
    name : String,
    health : i32,
    age : i32,
}

impl Player {
    fn with_details(name : String) -> Self {
        Player { name, health: 25, age: 20 }
        }
    }


fn main() {
    let p1 = Player::with_details("Mario".to_string());
    println!("p1 : {:?}", p1);
} */

// qui scrivo un codice che istanzia una struct Test con dei valori di defualt che definisco con un metodo costruttore e provo a stampare se soddisfa o meno una certa condizione

/* #[derive(Debug)]

struct Test {
    a : i32,
    b : bool,
}

impl Test {
    fn new() -> Self { // il metodo costruttore standard funziona come Default, con l'unica differenza che gli specifico io quali valori mettere
        Test { a : 0, b : false}
    }

    fn evaluate(&self) -> bool {
        if self.a == 0 {false}
        else if self.b == true {true} else {false}
    }
}

fn main() {
    let mut t1 = Test::new();
    println!(" t1 : {:?}, soddisfa : {:?}", t1, t1.evaluate());
    t1.a = 1;
    t1.b = false;
    println!(" t1 : {:?}, soddisfa : {:?}", t1, t1.evaluate());
} */

// ------------------------------------------------------------------------------------------------------------------------------------------------------------------------

// slide 33-37
// se sto usando una struct che chiamo all'interno di un blocco sintattico, il compilatore Rust dealloca la memoria (stack o heap a seconda del tipo di dati dentro la struct) chiamando in automatico la funzione Drop una volta che sono giunto al fondo del blocco con l'esecuzione. È quindi possibile implementare (similmente ad un'interfaccia) il tratto drop per una struct. La deallocazione della memoria per una struct (o un qualsiasi oggetto) corrisponde alla distruzione dello stesso
// quindi rust garantisce che ad ogni operazione di allocazione della memoria per una struct corrisponda un'operazione di deallocazione, che avviene nel momento in cui l'oggetto esce dal suo scope. 
// IMPORTANTE --> la regola fondamentale da ricordare è che oggetti che implementano il tratto Drop non possono mai implementare Copy. Questo è vero perchè se una struct implementasse Copy significa che potrei avere tante variabili indipendenti di Struct che possono puntare allo stesso indirizzo di memoria heap. Al contrario, un oggetto che implementa copy permette al programmatore di fare una copia bit a bit SOLO DEI PARAMETRI DELLO STACK. Ad esempio, se per assurdo due String implmentassero copy, significa che sarebbe lecito avere due triplette nello stack che hanno lo stesso valore del parametro pointer. Quindi, se così fosse, si avrebbero due puntatori allo stesso indirizzo di memoria heap. Ma quando poi chiamo Drop, libero la memoria una volta in corrispondenza della fine dello scope della variabile e poi la dovrei eliminare di nuovo quando in realtà è già vuota, ma questo è un controsenso. 
// Questo dimostra che un oggetto che implementa copy non può implementare anche drop. 
// Quindi, per gli oggetti che implementano Drop, scrivere let b = a significa passare il possesso del dato da "a" a "b". Il risultato è che la variabile a viene consumata, e b diventa il possessore, ovvero l'unico pointer che posso usare per accedere al dato in memoria heap. 

// qui sotto provo a riscrivere un programma stile slide 39. Devo creare una struct Rectangle con i campi height e width e creare due istanze: una di queste è un rettangolo, l'altra un quadrato. È possibile calcolare l'area della figura

/* use std::process::Output;

#[derive(Debug)]

struct Rectangle {
    height : i32,
    width : i32,
}

impl Rectangle {
    fn new_rectangle(h : i32 , w : i32) -> Self {
        Rectangle {height : h, width : w}
    }

    fn with_square(edge : i32) -> Self {
        Rectangle { height : edge, width : edge}
    }

    fn calculate_area(&self) -> i32 {
        self.height * self.width
    }
}

fn main() {
    let r1 = Rectangle::new_rectangle(5, 2);
    let r1_area = r1.calculate_area();
    println!("r1 : {:?}", r1);
    println!("r1_area : {}", r1_area);
    let r2 = Rectangle::with_square(5);
    let r2_area = r2.calculate_area();
    println!("r2 : {:?}", r2);
    println!("r2_area : {}", r2_area);

} */

// ------------------------------------------------------------------------------------------------------------------------
// DA QUI IN POI È POLIMORFISMO
// slide 33

//  Il succo del discorso riguarda dei vincoli che sussistono per la creazione di oggetti tratto.

//1) una funzione che accetta un parametro di tipo oggetto-tratto significa che il parametro è un fat pointer. 8 byte puntano al dato nella memoria; gli altri 8 puntano alla VTABLE, una tabella statica che contiene l'implementazione dei metodi per tutti i tipi che godono del tratto

//2) quando una funzione accetta un oggetto-tratto, il compilatore non conosce la dimensione del parametro. Si vuole nascondere la dimensione al compilatore. Questo implica che i metodi del tratto richiedano sempre come parametri o &self o &mut self.

//3) un metodo di un oggetto-tratto non può contenere un metodo che accetta un generics, perchè si dovrebbero generare infinite fuzioni, per tutti i tipi esistenti (è corretto?) 

// se ho una funzione esterna al tratto che riceve in input un parametro, e voglio specificare al compilatore che la dimensione non è nota a priori devo  scrivere :?Sized, altrimenti il compilatore accetterebbe solo parametri di cui conosce la dimen

// in Rust è possibile specificare l'implementazione dei tratti forniti dalla libreria standard per tipi di dato specifico, come struct Punto o struct Vettore (ad esempio).
// I tratti della libreria standard prevedono operatori di confronto, operatori binari e unari.
// Dal punto di vista del codice, la slide di questo messaggio descrive come avviene l'implementazione del tratto Add.


/* trait Add<Rhs = Self> {

    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;

} */

// Rust permette di specificare il tipo di dato che compare "a destra", quindi come secondo operando (è corretto?); il tipo del secondo operando dipende su quale tipo di dato si sta implementando il tratto, se è ad esempio un i32, Rhs indica un i32.
// Poichè c'è scritto self, l'operazione consuma il dato contenuto dal primo operando. Self::Output obbliga il risultato del tipoad essere Output, quindi il tipo su cui si sta implementando il tratto 

// --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

/* use std::ops::Add;

struct Point{
    x : i32,
    y : i32,
}


impl Add for Point { // il parametro Rhs puoi scriverlo solo nella definizione del tratto, non nei metodi del tratto. 
    type Output = Self;
    fn add(self, op2 : Self ) -> Self::Output {
        Point {
            x : self.x + op2.x,
            y : self.y + op2.y,
        }
    }
}

fn main(){
    let op1 = Point {x:3, y:2};
    let op2 = Point{x:4, y:2};
    let result = op1 + op2;
    println!("result.x {}. result.y {}", result.x, result.y);
} */

// --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

// In informatica esistono dei tipi che non godono della propietà di riflessività, come i NaN. Per questo motivo, quando voglio implementare un tratto di uguaglianza per i floating point, devo ricordarmi di usare PartialEq e non Eq, perchè con Eq sto conferendo la propietà di riflessività di cui i floating point non possono godere. 
// Quando scrivo il tratto PartialEq posso decidere io di quale tipo deve essere il secondo operando, in questo Rust è flessibile. Di default, il compilatore assume che i tipi degli operandi sia uguale
// Nella definizione del tratto, conviene anche specificare che il secondo tipo di operando potrebbe avere una dimensione non nota a tempo di compilazione, altrimenti non sarebbe possibile confrontare un oggetto con una stringa

/* trait PartialEq <Rhs = Self> 
where Rhs :?Sized { // qui specifichiamo al compilatore che a lui non serve sapere quanto è grande il dato a cui sta puntanto il parametro other, in quanto stiamo maneggiando dei riferimenti
    fn eq(&self, other : &Rhs ) -> bool;
    fn ne(&self, other : &Rhs ) -> bool {
        !self.eq(other)
    }
} */


// Il vincolo ?Sized serve nella definizione del tratto (nella libreria standard) per permettere a Rust di funzionare anche con tipi "strani" come le stringhe (str). Nella tua implementazione per Point, non serve perché Point è una struct con una dimensione ben nota (8 byte: due interi da 32 bit)

// --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

// negli esempi pratici in cui si usano strutture dati semplici (che contengono numeri o stringhe) spesso i tratti Eq e PartialEq si ereditano tramite #[derive(Eq,PartialEq)] facendo sempre attenzione a non ereditare PartialEq per le struct che contengono floating point. 
// ricorda anche che se devi sommare, sottrarre, o in generale fare un'operazione tra due struct, il metodo add applicato alle struct consuma le struct, cioè non prende in input riferimenti. Al contrario, quando confrontiamo struct conviene prendere in input al metodo eq dei riferimenti alle struct.


// --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

// vedi slide 50

// --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
// slide 54
// quando voglio accedere ad un array o una matrice, l'operatore di indicizzazione non corrisponde ad un accesso diretto alla memoria in Rust. Se ad esempio ho:
// t[i] --> viene tradotto in *t.index(i). Significa che viene chiamato il metodo .index del tratto Index. Per poter fare questo serve che la struttura dati t implementi il tratto Index. Notiamo che poichè il metodo .index restituisce &Self::Output serve l'* per interagire con il dato. 

/* impl PartialEq for Point {
    fn eq(&self, other : &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

struct Point {
    x: i32,
    y: i32,
}



fn main() {
    let p1 = Point{x:2,y:2};
    let p2 = Point{x:2,y:2};
    let p3 = Point {x:2, y:3};
    println!("{}", p1 == p2);
    println!("{}", p1 == p3);

} */

// --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
/* struct Point {
    x : f64,
    y : f64,
}

impl PartialOrd for Point {
    fn partial_cmp(p1 : &self, p2: &Self) -> 

}

fn confronta(p1 : &Point, p2 : &Point) {
    match p1.partial_cmp(&p2) {
        Some(ordering) => {

        }
    }
}

fn main() {
    let p1 = Point {x:1.0, y:4.0};
    let p2 = Point {x:3.0, y:2.0};
    let p3 = Point {x10.0, y:5.0};

    confronta(&p1, &p2);
} */

// --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
// slide 54-59 --> riprovo a rifare i codici
/* use std::ops::{Index,IndexMut};

#[derive(Debug)]
struct MyData {
    data : Vec<String>,
}

impl MyData {
    fn new() -> Self {
        MyData {data : 
            vec![
            "banana".to_string(),
            "mela".to_string(),
            "arancia".to_string(),
            "ananas".to_string(),
        ]}
    }
    fn add(&mut self, stringa : String) {
        self.data.push(stringa)
    }
}

impl Index<usize> for MyData {
    type Output = String;
    fn index(&self, index : usize) -> &Self::Output { // ERRORE --> String
        &self.data[index]
    } 
}

impl IndexMut<usize> for MyData {
    fn index_mut(&mut self, indice : usize) -> &mut Self::Output {
        &mut self.data[indice]
    }
}

fn main() {
    let mut m1 = MyData::new();
    println!("m1 before adding albicocca : {:?}", m1);
    m1.data[1] = "albicocca".to_string();
    m1.add("pera".to_string());
    println!("m1 after adding albicocca : {:?}", m1);
} */

// --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
// il programma chiede in input all'utente di scrivere un elenco di numeri in i32 e il programma stampa il primo elemento dell'elenco salvandolo in un array
// vedi quaderno verde acqua per commenti, pagina

/* use std::env;
use std::process;
use std::ops::Index;

#[derive(Debug)]
struct MyVec {
    data: Vec<i32>,
    year: i32,
}

impl Index<usize> for MyVec {
    type Output = i32;
    fn index(&self, indice : usize) -> &Self::Output {
        &self.data[indice]
    }
}

fn get_numbers() -> Result<Vec<i32>, String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err("Errore: Inserire almeno un valore numerico".to_string());
    }

    // Qui collezioniamo in un Vec<i32>. 
    // Se il parsing fallisce, usiamo un messaggio personalizzato.
    let mut numeri = Vec::new();
    for s in args.iter().skip(1) {
        match s.parse::<i32>() {
            Ok(n) => numeri.push(n),
            Err(_) => return Err(format!("Input '{}' non è un numero valido", s)),
        }
    }

    Ok(numeri) // Restituiamo il successo "impacchettato"
}

fn main() {
    let user_input = get_numbers();

    let numeri = match user_input {
        Ok(v) => v, // Estrae il valore e lo assegna a 'numeri'
        Err(e) => {
            eprintln!("Errore: {}", e);
            return; // Esci dal main in modo pulito
        }
    };
    let my_vec = MyVec {data:numeri , year : 2026};
    println!("Primo elemento vettore : {}", my_vec[0]);
    println!("Vettore intero: {:?}", my_vec);
} */

// --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

// slide 61