// slide 7
//
/* trait Summarizabile {
    fn summary(&self) -> String;
}

impl Summarizabile for f64 {
    fn summary(&self) -> String {
        format!("{:.4}", self)
    }
}

impl Summarizabile for &str{
    fn summary(&self) -> String {
        if self.len() > 5 {
            format!("{}", &self[..=5])
        } else {
            self.to_string()
        }
    }
}

fn main() {
    let n = 5.3/3.4;
    println!("n : {}", n.summary());
    println!("{}", "Hello, World!".summary());
} */

// -------------------------------------------------------------------------------------------------------------------------------------------------------------

// slide 10
// in questo codice devo scalare un punto attraverso un metodo definito come comportamento di un tratto. Il metodo deve restituire una nuova istanza del punto scalato di un valore inserito in input al metodo

/* trait Scalable {
    fn scale(&self, scale_factor:f64) -> Self;
}

struct Punto {
    x: f64,
    y: f64,
}
impl Scalable for Punto {
    fn scale(&self, scale_factor: f64) -> Self {
        Self { x : self.x *scale_factor , y: self.y *scale_factor}
    }
}

fn main() {
    let p1 = Punto {x:3.0 , y:5.0};
    println!("Il punto p1 si originale si trova in coordinate : ({},{})", p1.x , p1.y);
    let p1_scaled = p1.scale(0.5);
    println!("Il punto p1 scalato si trova in coordinate : ({},{})", p1_scaled.x, p1_scaled.y);
}  */




// -------------------------------------------------------------------------------------------------------------------------------------------------------------

// poichè un tipo i32 gode del tratto Default, posso chiamare la funzione statica su un tipo i32. 
/* fn main() {
    let n : i32 = i32::default();
    println!("{}", n);
} */

// -------------------------------------------------------------------------------------------------------------------------------------------------------------
//silde 12 


/* trait Inizializzabile {
    fn inizializza(x : f64, y : f64) -> Punto; // poichè questo è un metodo statico che serve per creare un'istanza, devo prendere in input i valori che voglio mettere d'inizio
}

#[derive(Default)]
struct Punto {
    x : f64,
    y : f64,
}
impl Inizializzabile for Punto {
    fn inizializza(x : f64, y : f64) -> Punto {
        Punto{x :x , y : y}
    }
}

fn main() {
    let p1 = Punto::inizializza(5.0,10.0);
    let p2 : Punto = Default::default();
    println!("p1 ha coordinate ({}, {})", p1.x, p1.y);
    println!("p2 ha coordinate ({}, {})", p2.x, p2.y);
} */

// ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

// i codici da slide 13 a slide 33 li ho studiati sulla chat di gemini e ho provato a fare degli esercizi che ripropongono gli stessi argomenti di quelli proposti nelle slide in codici nella cartella chiamata codici_prova_polimorfismo







// ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
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

// negli esempi pratici in cui si usano strutture dati semplici (che contengono numeri o stringhe) spesso i tratti Eq e PartialEq si ereditano tramite #[derive(Eq,PartialEq)] facendo sempre attenzione a non ereditare PartialEq per le struct che contengono floating point. 
// ricorda anche che se devi sommare, sottrarre, o in generale fare un'operazione tra due struct, il metodo add applicato alle struct consuma le struct, cioè non prende in input riferimenti. Al contrario, quando confrontiamo struct conviene prendere in input al metodo eq dei riferimenti alle struct.


// --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

// vedi slide 50

// --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
// slid 54
// quando voglio accedere ad un array o una matrice, l'operatore di indicizzazione non corrisponde ad un accesso diretto alla memoria in Rust. Se ad esempio ho:
// t[i] --> viene tradotto in *t.index(i). Significa che viene chiamato il metodo .index del tratto Index. Per poter fare questo serve che la struttura dati t implementi il tratto Index. Notiamo che poichè il metodo .index restituisce &Self::Output serve l'* serve per interagire con il dato. 

// slide 56 --> il tipo Vec[T] può implementare il tratto Index più volte, a patto che si cambia il tipo di indice (Idx). 
// INDICIZZAZIONE SINGOLA → Quando usi un intero per indicizzare, il tipo associato Output è il singolo elemento T. vec[0] → restituisce un riferimento al singolo elemento.
// INDICIZZAZIONE PER INTERVALLO → Se usi la sintassi 1..4. In questo caso, l'indice non è un numero, ma un oggetto di tipo Range.  → L' Output non è più un singolo i32, ma una Slice ([i32]).
// IMPORTANTE -->  la stessa sintassi [] produce oggetti di natura diversa a seconda di cosa metti tra le parentesi.

/* use std::ops::{Index,IndexMut};

#[derive(Debug)]
struct VecString {
    data : Vec<String>
}

// definisco i metodi di VecString
impl VecString {
    fn new() -> Self {
        VecString { data: Vec::new() }
    }
    fn add(&mut self, stringa : String) {
        self.data.push(stringa);
    }
}

impl Index<usize> for VecString {
    type Output = String;
    fn index(&self, index:usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for VecString {
    fn index_mut(&mut self, index :usize) -> &mut Self::Output {
        &mut self.data[index]
    }

}

fn main() {
    let mut vec_string = VecString::new();
    let s1 = "banana".to_string();
    let s2 = "orange".to_string();
    let s3 = "apple".to_string();
    vec_string.add(s1); // devo implementare il metodo add come comprtamento del tipo VecString
    vec_string.add(s2);
    vec_string.add(s3);
    // se voglio modificare un valore dell'array di stringhe devo specificare al compilatore come voglio indicizzare i valori e che è possibile modificare il vettore
    vec_string[1] = "lemon".to_string();
    println!("{:?}", vec_string);

} */


// --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
// PROVO A RIFARE UN CODICE DELLE SLIDE PRECEDENTI (SLIDE 12)

/* trait Stampabile {
    fn stampa_contenuto(&self);
}

trait Intestazione : Stampabile {
    fn stampa_totale(&self);
}

struct Documento {
    title : String,
    content : String,
}

impl Stampabile for Documento {
    fn stampa_contenuto(&self)  {
        println!("contenuto {:?}", self.content);
    }
}

impl Intestazione for Documento {
    fn stampa_totale(&self) {
        println!(" --- {} --- ", self.title);
        self.stampa_contenuto();
    }
}

 // impl Documento {} --> posso inizializzare la struct Documento direttamente nel main se voglio, senza implementare il metodo che crea un'istanza di Documento

fn main() {
    let d = Documento{
        title : "titolo1".to_string(),
        content : "contenuto1".to_string(),
    };

    d.stampa_contenuto();
    d.stampa_totale();
} */

// --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
//codice slide 58
/* use std::ops::Index;

struct MyVec {
    data : Vec<i32>,
    year : i32,
}

impl MyVec {
    fn new() -> Self{
        MyVec {
            data : vec![1,2,3,4,5,6],
            year : 10,
        }
    }
}


impl Index<std::ops::Range<usize>> for MyVec {
    type Output = [i32]; // indico uno slice di i32 
    fn index(&self, slice :  std::ops::Range<usize>)  -> &Self::Output { // qui specifico sempre in che modo ho intenzione di accedere ai dati della struttura dati
       &self.data[slice]
    }
}

fn main() {
    let my_vec = MyVec::new();

    let slice = &my_vec[1..4]; // se posso accedere tramite slice ad un attributo di una struttura dati devo importare il tratto Range per dire al compilatore in che modalità voglio accedere agli indici e applicarlo alla mia struttura dati
    println!("{:?}", slice);
} */

// l'errore che hai fatto nella scrittura nel codice è scrivere la macro vec! utilizzando le tonde. La macro vec! non usa le tonde, si aspetta solo un vettore di numeri
// NO vec!([1,2,3,4,5,6])
// SI vec![1,2,3,4,5,6]
/* use std::ops::Index;

#[derive(Debug)]
struct MyVec {
    vettore : Vec<i32>,
}

impl Index<std::ops::Range<usize>> for MyVec {
    type Output = [i32];
    fn index(&self, index : std::ops::Range<usize> ) -> &Self::Output {
        &self.vettore[index]
    }
}
fn main() {
    let my_vec = MyVec{vettore:vec![1,2,3,4,5]};
    let slice = &my_vec[2..4];
    println!("vettore intero : {:?}", my_vec);
    println!("slice : {:?}", slice);
} */

// --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

// slide 60 --> voglio istanziare due elementi di Point utilizzando il tratto From

/* struct Point {
    x : i32,
    y : i32,
}

impl From<(i32,i32)> for Point{ // nella signature dell'applicazione del tratto From a Point devo specificare compilatore qual è la struttura dati di partenza
    fn from( (x,y) : (i32,i32)) -> Self { // è un errore scrivere &self come parametro perchè questo è un metodo statico per istanziare, e quindi non è applicato ad un'istanza ma serve per crearla
        Point {
            x : x,
            y : y,
        }
    } 
}

impl From<[i32;2]> for Point {
    fn from([x, y] : [i32;2]) -> Self {
        Point { x, y}
    }
}

fn main() {
    // from
    let p1 : Point = Point::from((5,2));
    let p2 : Point = Point::from([6,3]);

    //into
    let p3 : Point = (1,2).into();
    let p4 : Point = [3,4].into();

    println!("p1 {:?}", p1);
    println!("p2 {:?}", p2);
    println!("p3 {:?}", p3);
    println!("p4 {:?}", p4);
} */

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
use std::ops::Index;

#[derive(Debug)]
struct MyVec {
    data: Vec<i32>,
    _year: i32,
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
        match s.parse() {
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
    let my_vec = MyVec {data:numeri , _year : 2026};
    println!("Primo elemento vettore : {}", my_vec[0]);
    println!("Vettore intero: {:?}", my_vec);
} */



// --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
// slide 62-63 utilizzo dei tratti TryFrom e TryInto.

// Mentre i tratti From e Into che abbiamo visto prima sono pensati per conversioni che non possono mai fallire (infallibili), queste slide introducono i loro "fratelli maggiori": TryFrom e TryInto.
// Ingegneristicamente, questi sono fondamentali quando la conversione dipende da una condizione logica o di business che potrebbe essere violata

// La differenza tecnica sta tutta nel valore di ritorno. Invece di restituire direttamente l'oggetto, i metodi try_from e try_into restituiscono un Result<T, E>

// CASO 1 --> Ok(T): La conversione è riuscita.

// CASO 2 --> Err(E): La conversione è fallita e ti restituisce un oggetto errore con i dettagli

// Proprio come per From, se implementi TryFrom per un tipo, Rust ti regala automaticamente l'implementazione di TryInto

// struct EvenNumber(i32); // L'ingegnere qui vuole creare un tipo EvenNumber che garantisca, per costruzione, di contenere solo numeri pari. --> VOGLIO CHE LA STRUCT CONTENGA SOLO NUMERI PARI

// struct EvenNumber è una tuple Struct. SI usa perchè se usassi un semplice i32 per rappresentare un numero pari, potresti accidentalmente passargli un numero dispari e il compilatore non direbbe nulla. Creando EvenNumber, crei un "muro" di sicurezza: un EvenNumber non è più un i32, è un tipo diverso. In memoria, però, occupa esattamente lo stesso spazio di un i32 (4 byte), quindi non c'è overhead.

// IMPORTANTE --> Result<T, E> è un tipo nativo di Rust, ed è un'enumerazione (enum). È così fondamentale che è incluso nel "Prelude" (viene importato automaticamente in ogni file).
// La sua definizione logica è: Result<T,E>={Ok(T),Err(E)}. È lo strumento standard per la gestione degli errori: invece di lanciare eccezioni o restituire codici d'errore numerici (come in C), una funzione restituisce una "scatola" che contiene o il successo (Ok) o l'errore (Err).

// Spiegazione di even.0 -->  Poiché la Tuple Struct non ha nomi per i campi (non c'è un .valore o un .x), Rust assegna loro dei numeri basati sulla posizione, proprio come nelle tuple. even.0 accede al primo (e in questo caso unico) elemento dentro la struct EvenNumber. Se avessi scritto struct Punto(i32, i32), avresti usato .0 per la X e .1 per la Y. even è quindi una signature della struct perchè è ciò che si trova dentro la scatola d.

// quando scrivo Err(err) sto dicendo al compilatore che voglio accedere al contenuto del branch di errore del parametro result di tipo Result, e ciò non toglie che non posso usare altri parametri passati alla funzione di verifica

// IMPORTANTE --> &' static str --> &str è un riferimento. Poiché è un riferimento, il compilatore deve sapere per quanto tempo quella stringa rimarrà valida in memoria (Lifetime). static è un tempo di vita speciale: significa che il dato esiste per tutta la durata dell'esecuzione del programma. Le stringhe scritte tra virgolette nel codice (string literals) vengono memorizzate direttamente nel segmento "data" (sola lettura) dell'eseguibile. Dato che sono scritte nel binario, non vengono mai deallocate. Specifichiamo 'static per dire al compilatore: "Non preoccuparti, questo messaggio di errore non sparirà mai dalla memoria". Il fatto è che l'output del metodo try_from dipende da un riferimento di una stringa e quindi serve uno static.

/* use std::convert::TryFrom;

struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = & 'static str;
    fn try_from(number:i32) -> Result<Self,Self::Error> {
        if number % 2 == 0 {
            Ok(EvenNumber(number))
        } else {
            Err("il numero non è pari")
        }
    }
}

fn verifica(valore : i32, risultato : Result<EvenNumber,&str>) {
    match risultato {
        Ok(even_struct) => println!("numero resitito pari : {}", even_struct.0),
        Err(e) => println!("numero inserito {} ma {}", valore, e),
    }
}


fn main() {
    let even_number = 12;
    let result = EvenNumber::try_from(even_number);
    verifica(even_number,result);
    let odd_number = 13;
    let result_odd = EvenNumber::try_from(odd_number);
    verifica(odd_number,result_odd);

} */
// --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

// slide 64-65 --> uso di .parse() come metodo del tratto FromStr

// Guarda questa riga nel main:
// let numer_str = "42"
// let number = number_str.parse();
// Rust capisce che deve convertire la stringa in un i32 perché la funzione verifica si aspetta un Result<i32, ...>. Se cambiassi la firma di verifica per accettare un f64, parse() convertirebbe la stringa in un float senza cambiare una riga di codice nel main

// nella funzione di verifica vogliamo controllare che il metodo parse() che è implementato nel tratto FromStr abbia correttamente convertito un literal string in un i32. Se la funzione di verifica che scriviamo noi vede che il valore di ritorno che ha restituito parse è un ParseIntError, viene eseguito il ramo negativo o di fallimento.

// --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

// slide 66-67 tratto Deref e DerefMut
// se ho una struttura dati che implementa il tratto Deref e scrivo *t, il compilatore traduce l'istruzione in *(t.deref()) sulla struttura dati t
// 1) deref() restituisce un riferimento al dato interno (&Target)
// 2) L'asterisco esterno effettua la dereferenziazione finale su quel riferimento
// DerefMut: È la versione mutabile. Permette di modificare il dato interno scrivendo *t = nuovo_valore. Come per gli altri tratti che abbiamo visto, DerefMut eredita da Deref

// il tratto Deref utilizza un Target a dimensione ignota;; un Target è un tipo a dimensione ignota che definisce cosa c'è dentro lo smart pointer
/* use std::ops::Deref;

#[derive(Debug)]
struct CustomStruct{
    number : i32,
    boxed_number : Box<i32>,
}

impl Deref for CustomStruct {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &self.number
    }
}

fn main() {
    let n = 10;
    let box_number = Box::new(12);
    let structt = CustomStruct {number : n, boxed_number : box_number};
    println!("deferencing number {}", *structt); // scrivere l'asterisco si traduce nell'applicare il metodo deref. se la struct implementa il tratto Deref posso recuperare un valore dentro la struct
} */

// la deref coercion permette anche di tramutare una String in un &str

/* fn print_str(stringa : &str) {
    println!("{}", stringa);
}

fn main() {
    let stringa = String::from("Hello world!");

    print_str(&stringa);

    let string_slice_from_string = &stringa;
    println!("Lunghezza della stringa {}", string_slice_from_string.len());
    println!("Porzione della stringa {}", &string_slice_from_string[2..5]);
    println!("Conversione in maiuscolo {}", string_slice_from_string.to_uppercase());

} */

// --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
/* use std::ops::Deref;

#[derive(Debug)]
struct MyData {
    value : i32,
}

struct Container{
    data : Box<MyData>,
}

impl Deref for Container {
    type Target = MyData;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

fn main() {
    let my_data = MyData { value : 3};
    let container = Container {data : Box::new(my_data)};
    println!("valore estreatto con deref : {:?}", container.value);
} */

// --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
// slide 98 sul concetto della programmazione generica (vedi slide mac)

/* fn get_value<T>(value : T , condition : bool) -> Option<T> {
    if condition {
        Some(value)
    } else {
        None
    }
}

fn main() {
    let value = "Hello world!";
    let condition = true;
    match get_value(value, condition) {
        Some(val) => println!("Valore : {}", val),
        None => println!("Nessun valore trovato"),
    }

    // attraverso la monomorfizzazione posso utilizzare la funzione generica per tipi diversi presi in input
    match get_value(42,condition) {
        Some(val) => println!("valore {}", val),
        None => println!("Nessun valore trovato"),
    }
} */

// --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------


/* fn max<T:PartialOrd>(vettore : &[T]) -> Option<T> {
    if vettore.is_empty() {
        None
    } else {
        let mut max = &vettore[0];
        for i in vettore.iter() {
            if i > max {
                max = i;
            }
        }
    }
    Some(max)
}



fn main() {
    let numbers = vec![1,2,3,4,5,6];
    my_max(max(&numbers));
} */