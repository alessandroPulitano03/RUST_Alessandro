// QUI SPIEGHIAMO IL CONCETTO DI SMART POINTER (BOX), EVOLUZIONE DELLO STACK E DELLO HEAP IN UN PROGRAMMA (VEDI ANCHE VIDEO TELEFONO)
// questi codici trattano gli argomenti presentati nel capitolo I01_2-Il linguaggio_RM

/* fn main() {
    let i = 4;
    let mut b = Box::new( (5,7) );

    b.1 = i;

    println!("{:?}", *b);

} // quando il compilatore arriva a questa riga di codice lo stack e l'heap vengono svuotati della memoria occupata dalle variabili b e i  */

// -------------------------------------------------------------------------------------------------------------------------------------------------

// In questo codice vediamo un movimento di dato, per cui il possesso dell'indirizzo di memoria che punta al Box nello heap è dato alla variabile b nel main. 

/* fn makeBox (a: i32) -> Box<(i32 , i32)>
{
    let r = Box::new ( (a,1) );
    return r;
}

fn main() {
    let mut b = makeBox(5);

    b.0 = b.0 * 2; // è possibile accedere al dato senza * perchè Rust sa che b è un Box

    println!("{:?}", b);

} */

// -------------------------------------------------------------------------------------------------------------------------------------------------

// In questo codice vediamo i tipi che implementano il tratto copy e quelli che implementano il tratto clone.

/* fn main() {
    let x = 123;
    let y = x; // è possibile copiare bit a bit un dato intero in un altro --> questo non determina movimento

    println!("{}", y);

    // i vec non godono del tratto copy ma godono del tratto clone.
    // let v : Vec = vec![1,2,3]; ---> ERRORE : i vettori richiedono di specificare il tipo di dato inserito

    let v : Vec<u8> = vec![1,2,3];
    

    let w = v; // poichè v non è un tipo elementare, l'operatore = sposta il possesso dell'indirizzo di memoria che punta al vettore v nello heap alla variabile w. Pertanto la variabile v non è più utilizzabile 
    // println!("{}", w); ---> ERRORE : i vettori non godono del tratto Display

    println!("vettore: {:?}", w);
}  */

// -------------------------------------------------------------------------------------------------------------------------------------------------
// slide 64
// In questo codice spieghiamo i primi concetti sugli array

// • collezione omogenea di elementi nello stack fissa in dimensione. 
/*  fn main() {
    let mut v1 = [1,2,3,4,5];

    println!("{:?}", v1);

    v1[0] = 5; // questa operazione è lecita solo se l'array è stato definito come mutabile

    let v2 = v1; // questo determina un copy perchè l'array gode del tratto copy

    println!("{:?}", v2);

    let v3 = [0;5];


 } */

// -------------------------------------------------------------------------------------------------------------------------------------------------

// Qui si verifica una situazione di panico: il compilatore non può prevedere il valore ritornato da una funzione, per cui l'index out of bound viene notato in tempo di esecuzione, per cui si verica il panic, a seguire l'unwinding delle risorse con il metodo drop
/* fn f(n : u8) -> usize {
    return n + 10
}

 fn main() {
    let v = [1,2,3,4,5];
    println!("{}",v[0]);
    println!("{}",v[f(0)]);
 } */

// -------------------------------------------------------------------------------------------------------------------------------------------------

// lettura sicura di un array

/* fn main() {
    let arr = [1,2,3,4];
    if let Some(val) = arr.get(10) {
        println!("Valore: {}", val);
    }
    else {println!("Elemento non esistente nell'array");}
}
 */

// -------------------------------------------------------------------------------------------------------------------------------------------------

// ARRAY E SLICE

/* fn main() {
    let arr1 = [1,2,3,4,5,6]; // l'array immutabile mi permette di creare soltanto slice immutabili, accessibili in sola lettura
    let s1 = &arr1[0..2]; // prendo arr[0] e arr[1] (escluso arr[2])
    println!("{:?}", s1);
    
    let mut arr2 = [1,2,3,4,5,6]; // se creo un array mutabile posso definire uno slice e modifcarne uno degli elementi; l'effetto si ripercuote anche sull'array originale perchè ho due puntatori alla stessa zona di memoria.
    let s2 = &mut arr2[3..];

    s2[0] = 40;

    println!("{:?}", s2);
    println!("{:?}", arr2);
} */


// -------------------------------------------------------------------------------------------------------------------------------------------------

// VETTORI + EVOLUZIONE STACK, HEAP 
// ricorda che quando provo ad aggiungere un nuovo elemento al vettore quando la capacity (ovvero il buffer di memoria allocato nello heap per salvare gli elementi del vettore) è uguale a size, Rust alloca un buffer più grande nello heap, copia gli elementi dal vecchio buffer a quello nuovo e aggiunge il valore che volevo aggiungere. 

/* fn main() {
    let mut v : Vec<i32> = Vec::new(); // nello stack vengono creati 3 parametri: pointer, length, capacity. Il valore di capacity dipende dal tipo di dati che il vettore può salvare
    v.push(1);
    v.push(2);

    let s = &mut v;
    s[0] = 10;

    println!("{}", s);
} // In corrispondenza di questa riga lo stack e lo heap vengono liberati. 
*/ 

// -------------------------------------------------------------------------------------------------------------------------------------------------

/* fn main() {
    let mut v : Vec<i32> = vec![1,2,3];
    println!("size: {}; capacity: {}", v.len(), v.capacity());

    v.push(4);
    println!("size: {}; capacity: {}", v.len(), v.capacity());
    v.push(5);
    v.push(6);
    v.push(7);
    println!("size: {}; capacity: {}", v.len(), v.capacity());

    while (v.len()) > 0 {
        v.pop();
    }

    println!("size: {}; capacity: {}", v.len(), v.capacity());


} */



// -------------------------------------------------------------------------------------------------------------------------------------------------

// slide 87

/* fn main() {
    let s = "Ciao mamma";

    println!("string literal : {:?}", s);
    println!("primo carattere : {}", s.chars().nth(1).unwrap());
} */
// .chars trasforma una string literal in un iteratore di caratteri tutti di dimensione pari a 4 byte. Ricordiamo che Rust esprime le string literal, quelle rappresentate da doppi apici (che sono memorizzate dal compilatore in un'area statica di memoria e sono diversi dagli oggetti dinamici di tipo String), come array di caratteri, dove ciasuno dei quali può avere una dimensione variabile da 1 a 4 byte. con .chars() tutti i caratteri dell'array diventano di 4 byte.
// il metodo .nth() scorre l'array byte dopo byte, quindi quel .nth indica l'iesimo byte (in inglese)
// il metodo .unwrap si assicura che il metodo .nth() abbia resituito un carattere, in quanto se il programmatore scrive un numero che esce aldilà dei byte occupati dalla stringa, il metodo .unwrap() manda il compilatore in panic!

// -------------------------------------------------------------------------------------------------------------------------------------------------

// slide 95 --> il programma printa una stringa e la sua lunghezza chiamando una funzione
/* fn compute_length(p1 : &str) ->usize {
    let length = p1.len();
    length
}
fn main() {
    let s : String = String::from("hello");

    let s_ref = &s;

    let length = compute_length(s_ref);
    println!("stringa : {:?}\nlunghezza : {} ", s, length);
} */

// -------------------------------------------------------------------------------------------------------------------------------------------------

//ESERCIZI GEMINI SU STRINGHE command f "mirate"
//Data una stringa letterale (string literal) definita nel main, scrivi un frammento di codice che tenti di estrarre e stampare il quarto carattere. Il programma deve gestire correttamente il caso in cui la stringa sia più corta di 4 caratteri (ad esempio "Cia"), stampando un messaggio di errore personalizzato invece di andare in panic.

/* fn main() {
    let s : &str = "Ciaomamma";
    if let Some(val) = s.chars().nth(4) {
        println!("quarto carattere : {}", val);
    }
    else {
        println!("stringa non contenente un quarto carattere");
    }
}
 */
// -------------------------------------------------------------------------------------------------------------------------------------------------
//Implementa una funzione chiamata misura_stringa che accetti un parametro in grado di ricevere sia una String (allocata nello heap) sia una &str (string literal). All'interno della funzione, stampa la lunghezza della stringa in byte. Dimostra nel main che la funzione può essere chiamata correttamente passando una volta una variabile di tipo String e una volta una stringa racchiusa tra virgolette direttamente.

/* fn misura_stringa(p : &str) -> usize{
    let length = p.len();
    length
}
fn main() {
    let s1 = String::from("hello world");
    let s2 = "ciao mondooooooo";
    let l1 = misura_stringa(&s1);
    let l2 = misura_stringa(s2); 
    println!("lunghezza 1: {}", l1);
    println!("lunghezza 2: {}", l2);
} */

// -------------------------------------------------------------------------------------------------------------------------------------------------

// ESERCIZIO 3 Crea una variabile di tipo String che contenga la parola "Caffè" (nota la 'è' finale). Scrivi un programma che:
// 1) Stampi la lunghezza della stringa usando il metodo .len().
// 2) Stampi la lunghezza della stringa contando i caratteri tramite l'iteratore .chars().
// 3) Spieghi (tramite un commento nel codice o una println!) perché i due numeri ottenuti sono diversi, facendo riferimento a come i caratteri non-ASCII vengono memorizzati nello stack/heap.

/* fn misura_stringa(p : &str) -> usize{
    let length = p.len();
    length
}

fn s_len_iter(p : &str) -> usize {
    let mut sum = 0;
    for _ in p.chars() { // in questo c
        sum += 1
    }
    sum
}

fn main() {
    let s : String = String::from("Caffè");
    let s_len = misura_stringa(&s);
    let s_len_iter = s_len_iter(&s);
    println!("s_len : {}", s_len);
    println!("s_len_iter : {}", s_len_iter);
} */

// -------------------------------------------------------------------------------------------------------------------------------------------------

// slide 129 

/* fn main() {
    let array = [10,20,30,40,50,60];
    for el in array { // IMPORTANTE : in questo caso, poichè l'array è composto da i32 che implementano il tratto copy, il compilatore può copiare il valore di ogni elemento nella variabile el. se ci fossero stati elementi di tipo String, il circlo avrebbe preso il possesso dell'array rendendolo inutilizzabile dopo il ciclo. 
        println!("elemento : {}", el);
    }

    let mut count = 0;
    let names = ["Bob", "Frank", "Jack","Carl"];
    for name in names.iter() { // IMPORTANTE: Il metodo .iter() tasforma l'array su cui è applicato in un iteratore che restituisce riferimenti agli elementi. QUesto permette di evitare il movimento e quindi se uso .iter() posso riusare l'array anche dopo il ciclo.
        println!("name#{} : {:?}" : count, name);
        count += 1;
    }
} */

// -------------------------------------------------------------------------------------------------------------------------------------------------
// CODICI CON LA CLAUSOLA MATCH
// slide 132



/* fn main() {
    for index in 0 .. 10 {
        println!("Index:{}", index);
        let s: &str = match index {
            0 ..= 4 => { "\tI'm in the first half" },
            5 => { "\tI'm in the middle" },
            _ => { "\tI'm in the second half..." }
        };
        println!("{}",s);
    }
} */