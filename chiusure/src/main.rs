// ----------------------------------------------------------------------------------------------------
// slide 4 
// ci sono due approcci alla programmazione funzionale, applicabile nel contesto delle closure
// approccio 1) VARIABILI COME PUNTATORI A FUNZIONI

/* fn add_numbers(x :i32, y:i32) -> i32 {
    x + y
}

fn main() {
    let pointer = add_numbers;
    let call = pointer(2,1);
    println!("{}",call);
} */

// ----------------------------------------------------------------------------------------------------
// slide 5
/* fn add(a: i32, b: i32) -> i32 { a + b }
fn subtract(a: i32, b: i32) -> i32 { a - b }
fn multiply(a: i32, b: i32) -> i32 { a * b }

fn main() {

    let (x, y) = (10, 5);

    // Selezioniamo la funzione in base a una condizione.


    let operation_name = "add";
    // Assegnamo il puntatore alla funzione corrispondente.

    let operation: fn(i32, i32) -> i32 = match operation_name { // operation rappresenta un puntatore ad una funzione
        "add" => add,
        "subtract" => subtract,
        "multiply" => multiply,
        _ => panic!("Operazione non supportata"),
    };

    let result = operation(x, y);
    println!("Il risultato dell'operazione '{}' è: {}", operation_name, result);
} */

// ----------------------------------------------------------------------------------------------------
// slide 8 Posso definire la funzione lambda e assegnarla ad una variabile. Dal momento in cui avviene l'assegnazione, la variabile può essere chiamata come se fosse una funzione

/* fn main() {
    let answer = |v| {v+1};
    println!("the answer is : {}", answer(3));
} */

// ----------------------------------------------------------------------------------------------------

// slide 15, piccola variazione 

/* fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    let mut print_numbers = move || { // for n in numbers.iter(), stai creando un borrow immutabile dell'intero vettore per tutta la durata del ciclo
        for n in numbers.iter_mut() { // IMPORTANTE --> è richiesto un iteratore mutabile per modificare i valori del vettore
            *n+=1; 
        }
        println!("vettore modificato: {:?}", numbers);

    // La chiusura usa il vettore catturato
    // numbers è stato mosso (moved) dentro la chiusura

    };

    // non può più essere usato qui
    // println!("I numeri sono: {:?}", numbers);
    // Questo darebbe errore di compilazione in quanto un Vec non implementa il tratto Copy

    print_numbers(); // Chiamiamo la chiusura

} */

// ----------------------------------------------------------------------------------------------------
// slide 16 

/* fn main() {
    let mut count = 0;
    let mut increment = move|| { // questa closure implementa FnMut perchè se vedi non consuma, perchè count implementa il tratto Copy e quindi c'è solo una copia bit a bit. 
        count += 1; // Viene creato un clone della variabile count
        println!("Valore del conteggio: {}", count);
    };

    increment(); 
    println!("HELLO: {}", count);
    increment();
    println!("HELLO: {}", count);
} */


// ----------------------------------------------------------------------------------------------------
// slide 16 variazione 1

/* fn main() {
    let mut count = 0;
    let mut increment = || {
        count += 1; // Viene creato un clone della variabile count
        println!("Valore del conteggio: {}", count);
    };

    increment(); 
    increment();
    println!("HELLO: {}", count);
} */


// ----------------------------------------------------------------------------------------------------
// slide 16 variazione 2
// la variazione di questo codice rispetto a quello originale della slide 16 è che poichè non c'è la move, il compilatore non esegue la copia bit a bit dopo aver verificato che la variabile count implementi il tratto Copy, la closure acquisce un riferimento mutabile. 
/* fn main() {
    let mut count = 0;
    let mut increment = || {
        count += 1; // Viene creato un clone della variabile count
        println!("Valore del conteggio: {}", count);
    };

    increment(); 
    println!("HELLO: {}", count); //  IMPORTANTE l'errore è qui --> non posso accedere al dato originale e subito dopo modificare il riferimento. Devo prima distruggere tutti i riferimenti mutabili e poi posso accedere al dato originale
    increment();
    println!("HELLO: {}", count);
} */

// ----------------------------------------------------------------------------------------------------
// esempio utilizzo di un vettore in cui c'è un movimento dei dati e un consumo del vettore. È di fatto un iterazione in cui si prende il possesso del vettore e lo si consuma.  
/* fn main() {
    let v1 = vec![1,2,3,4];
    let mut v2: Vec<i32> = Vec::new();
    for n in v1{
        v2.push(n)
    }
    println!("Vettore 2 : {:?}", v2);
} */

// ----------------------------------------------------------------------------------------------------
// slide 28
// in questo esempio la closure implementa Fn perchè sta utilizzando un riferimento immutabile del vettore. 
/* fn main() {
    let v1 = vec![1,2,3,4,5];
    let calculate_sum = || {
        let sum : i32 = v1.iter().sum();
        println!("sum value : {}", sum);
    };
    calculate_sum();
    calculate_sum();
} */

// ----------------------------------------------------------------------------------------------------
// slide 34 
// poichè in questo codice la closure restituisce il dato e lo possiede, io posso chiamare la closure quante volte voglio, e ovviamente non posso accedere alla stringa s1. 
/* fn main() {
    let s1 = String::from("ciao");
    let print_string = move || {
        println!("stampa stringa prima: {}", s1);
        s1
    };
    print_string();
    // print_string(); sarebbe errore perchè la stringa è stata mossa nella closure
} */

// variante codice slide 34 IMPORTANTE
/* fn main() {
    let s1 = String::from("ciao");
    let print_string = move || {
        println!("stampa stringa prima: {}", s1);
    };
    print_string();
    print_string();
    // in questo caso la closure non restituisce il dato, e quindi non lo consuma, ne prende solo possesso
} */

// ----------------------------------------------------------------------------------------------------
// slide 34 variazione 
// poichè in questo codice la closure non restituisce il dato e lo possiede, io posso chiamare la closure quante volte voglio, e ovviamente non posso accedere alla stringa s1. 
/* fn main() {
    let mut s1 = String::from("ciao");
    let mut print_string = move|| {
        println!("stampa stringa prima: {}", s1);
        s1.push_str(" ciao");
        println!("stampa stringa dopo: {}", s1);
    };
    print_string();
    print_string();
} */

// ----------------------------------------------------------------------------------------------------
// slide 36 FUNZIONI DI ORDINE SUPERIORE
// Per una funzione di ordine superiore devo sempre specificare il tratto funzionale che implementa. 

/* fn multiply(n1 : i32, function : F) -> i32 
    where F : Fn(i32) ->i32, // qui specifico il tratto funzionale
    {
        function(n1) // n1 è il valore assunto da number quando la closure viene chiamata
    }


fn main() {
    let per_due = |number| {number * 2}; // il parmetro scritto tra parentesi viene preso da un input
    let per_tre = |number| {number * 3};
    println!("10 x 2 = {}", multiply(10, per_due));
    println!("20 x 3 = {}", multiply(20, per_due));
} */

// ----------------------------------------------------------------------------------------------------
// slide 41

/* fn genera_contatore() -> impl Fn(i32) -> i32 {
    let contatore = 0;
    move |incremento : i32| contatore+incremento
}


fn main() {
    conto = genera_contatore(); // conto è la closure restituita dalla funzione genera contatore
    println!("Il risultato è: {}", conto(3));
    println!("Il risultato è: {}", conto(10));
    println!("Il risultato è: {}", conto(10));
} */

// ----------------------------------------------------------------------------------------------------
// slide 42
/* fn generator(prefix: &str) -> impl FnMut() -> String {
    let mut i = 0;
    let b = prefix.to_string();
    return move || {i+=1; format!("{}{}",b,i)}
}

fn main() {
    let mut f = generator("id_");  // la stessa closure viene chiamata 4 volte, per cui il contatore i non viene ridefinito --> la funzione viene eseguita 4 volte con una chiamata
    for _ in 1..5 {
        println!("{}",f());
    }
} */


// ----------------------------------------------------------------------------------------------------
fn crea_operazione(tipo: &str) -> Box<dyn Fn(i32) -> i32> {
    match tipo {
        "raddoppia" => Box::new(|x| x * 2),
        "quadrato" => Box::new(|x| x * x),
        _ => Box::new(|x| x),
    }
}

fn main() {
    let op = crea_operazione("quadrato"); // la funzione crea_operazione accetta in input una certa stringa; se la stringa corrisponde ad una certa funzione  allora viene chiamata la closure
    println!("{}", op(4)); // 16
}