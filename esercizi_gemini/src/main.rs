// ERRORE
/* Struct Rivelazione {
    id: i32
    valore: f64
} */

// SOLUZIONE
/* struct Rivelazione {
    id:i32,
    valore:f64,
} */

// ERRORE
/* impl Drop for Rivelazione {
    fn drop(&mut self) {
        println!("Sensore {id} rimosso dalla memoria", Rivelazione.id);
    }
} */

// SOLUZIONE
/* impl Drop for Rivelazione {
    fn drop(&mut self) {
        println!("Sensore {} rimosso dalla memoria", self.id);
    }
} */

// ERRORE
/* fn main() {
    let r1 = Rivelazione{ id:1 , valore:22.5};
    let b1 = Box::new(r1);
    let mut r1 = Rivelazione { id : 1, valore:25};
    let v : Vec<T> = Vec::new(); 
    v = r; // per muovere il possesso di un box all'interno del vettore occorre 1) UTILIZZARE UNA PUSH (non puoi usare una semplice assegnazione) 2) SPECIFICARE CHE IL VETTORE CONTIENE ELEMENTI DI TIPO BOX 3) SE DEVI SPOSTARE IL POSSESSO DI r1 NEL VETTORE DEVI PUSHARE IL POSSESSORE DI r1 NEL VETTORE, QUINDI b1
    let r2 = Rivelazione { id:2, valore:30};
    v.push(r2);
    for i in &v {
        println!("id : {}, valore {}", i.id, i.valore);
    }

} */

// SOLUZIONE
/* fn main() {
    // crea un'istanza di Rivelazione allocata nello Heap tramite un Box
    let r1 = Rivelazione{ id:1 , valore:22.5};
    let b1 = Box::new(r1);

    // uso lo shadowing per rendere r1 mutabile
    let mut r1 = Rivelazione { id : 1, valore:25.0};
    // crea un vec e muovi il possesso di r1 all'interno del vettore
    let v : Vec<Box<Rivelazione> = Vec::new();
    v.push(b1); // con questa istruzione la variabile b1 viene invalidata e il possesso del Box passa al vettore

    v.push(Box::new(r1)) // con questa istruzione prendo la seconda istanza (quella mutabile) di r1. 
    let r2 = Rivelazione { id:2, valore:30};
    v.push(Box::new(r2));

    for i in &v {
        println!("id : {}, valore {}", i.id, i.valore);
    }
} */

// ESERCIZI CONTROL+F "prevedere" chat di studio Rust

// esercizio 1

/* #[derive(Debug)]

struct Documento {
    titolo : String,
}

fn pubblica(p1 : Documento) {
    println!("Titolo documento : {:?}", p1.titolo );
}

fn main() {
    let d1 = Documento { titolo : "lettera avvocati".to_string()};
    pubblica(d1);
    println!("provo a stampare il titolo, senza successo : {:?}", d1.titolo);
} */

// esercizio 1 in cui si tenta di riprendere il possesso di d1 nel main
// gemini ha risposto che la soluzione è corretta

/* #[derive(Debug)]

struct Documento {
    titolo : String,
}

fn pubblica(p1 : & Documento) {
    println!("Titolo documento : {:?}", p1.titolo );
}

fn main() {
    let d1 = Documento { titolo : "lettera avvocati".to_string()};
    pubblica(&d1);
    //poichè ho passato un riferimento di d1 posso riusare d1 e non perdo il possesso nel main
    println!(" titolo d1 : {:?}", d1.titolo);
} */

// sotto riporto opzioni alternative per risolvere lo stesso esercizio. QUando ragioni su questa alternativa pensa al modo in cui i puntatori evolvono durante l'esecuzione
// alternativa 1: passo la variabile d1 e la restituisco, in modo che nel main riottengo il possesso di cosa ho passato
/* #[derive(Debug)]

struct Documento {
    titolo : String,
}

fn pubblica(p1 : Documento) -> Documento {
    println!("Titolo documento : {:?}", p1.titolo );
    p1
}

fn main() {
    let mut d1 = Documento { titolo : "lettera avvocati".to_string()};
    d1 = pubblica(d1);
    println!("titolo d1 : {:?}", d1.titolo);
} */

// alternativa 2: creo una nuova istanza utilizzando .clone(). Questa è la soluzione meno efficiente dal punto di vista delle risorse, perchè duplico i dati in un'altra area di memoria

/* #[derive(Debug, Clone)]

struct Documento {
    titolo : String,
}

fn pubblica(p1 : Documento) {
    println!("Titolo documento : {:?}", p1.titolo );
}

fn main() {
    let d1 = Documento { titolo : "lettera avvocati".to_string()};
    pubblica(d1.clone());
    println!("titolo d1 : {:?}", d1.titolo);
} */

// alternativa 3: uso un riferimento al campo della struct che mi interessa

/* #[derive(Debug)]

struct Documento {
    titolo : String,
}

fn pubblica(p1 : & String) {
    println!("Titolo documento : {:?}", p1 );
}

fn main() {
    let d1 = Documento { titolo : "lettera avvocati".to_string()};
    pubblica(&d1.titolo);
    println!("titolo d1 : {:?}", d1.titolo);
} */

// ------------------------------------------------------------------------------------------------------------------------------------------------------------------------
// esercizio 2 --> svolto perfettamente, utilizzalo per ripassare
/* #[derive(Debug)]
struct Nodo {
    valore : i32, // in questo caso la struct contiene solo un valore a dimensione fissa, pertanto la struct si trova nello stack
}

fn modifica_valore(p1 : &mut Nodo, nuovo:i32) {
    p1.valore = nuovo;
}

fn main() {
    let nodo : Nodo = Nodo { valore : 100 };
    let mut n1 : Box<Nodo> = Box::new(nodo); // qui si verifica movimento di nodo nel box. Significa che il compilatore annulla il puntatore nello stack di nodo, e utilizza il puntatore del box n1 nello stack per puntare al dato
    println!("nodo prima : {:?}", n1);
    let nuovo = 50;
    modifica_valore(&mut n1, nuovo);
    println!("nodo dopo : {:?}", n1);
    // se scrivo &Box<Nodo> significa che sto creando un puntatore nello stack che punta al puntatore di box che sta ancora nello stack. Quindi di fatto sto creando un puntatore di un puntatore. 
    // Se invece scrivo &mut Nodo sto creando un pointer nello stack che punta direttamente all'indirizzo di memoria heap in cui risiede n1. In questo senso il compilatore annulla la calidità del box n1 fino a che il riferimento è in vita, secondo quanto dice la regola del borrow checker. Poichè in questo codice sto definendo un riferimento mutabile, posso avere un solo riferimento mutabile che può sia leggere che scrivere il dato.
    //il doppio salto avviene con &Box<Nodo> 
} */

// ------------------------------------------------------------------------------------------------------------------------------------------------------------------------
// esercizio 3 

fn media(dati : &[f32] ) -> f32 {
    let mut sum : f32 = 0.0;
    for valore in dati {
        // sum += valore; --> IMPORTANTE : Quando scrivi for valore in dati, poiché dati è un riferimento a una sequenza (&[f32]), l'iteratore ti restituisce un riferimento a ogni singolo elemento. valore non è un f32, ma un &f32, per cui non puoi sommare un riferimento (&f32) a un numero (f32). Quindi la riga che hai scritto è sbagliata
        sum += *valore;
    }
    let lunghezza_vec : f32 = dati.len() as f32;
    // media = sum / lunghezza_vec --> IMPORTANTE : Quando buoi ritornare un valore da una funzione non devi assegnare il risultato dell'operazione ad una variabile. È sufficiente scrivere l'operazione che vuoi fare e omettere il ; Rust capisce che si tratta di un valore che vuoi ritornare al chiamante
    sum / lunghezza_vec
}

fn main() {
    let v_temeperature : Vec<f32> = vec![10.3, 9.4, 9.9, 9.0, 10.7, 11.0]; //stavo ragionando sul seguente concetto; mentre scrivo il codice sono in dubbio se passare alla funzione la variabile v_temperatures oppure un suo riferimento. Se passo la variabile significa che il main perde il possesso e potenzialmente posso rischiare di non poter più riutilizzare la variabile. Se invece passo un riferimento probabilmente ho più sicurezza, giusto? decido di passare un riferimento per ora, che per un vettore si tratta di uno slice, quindi un fat pointer --> Gemini dice che è sempre meglio passare il riferimento. Ha anche detto che il ragionamento che hai fatto è assolutamente lecito e anzi è ottimo ragionare come hai fatto. Good job!
    let s1 : &[f32] = &v_temeperature[..];
    let s2 : &[f32] = &v_temeperature[3..];
    let media_totale : f32 = media(s1);
    let media_parziale : f32 = media(s2);

    println!("media su tutto il vec : {}", media_totale);
    println!("media sulla seconda metà del vec : {}", media_parziale);
}