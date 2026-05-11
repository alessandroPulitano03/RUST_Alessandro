// ISTRUZIONI ED ESPRESSIONI

// let , let mut  -->  ISTRUZIONI
// {...}  -->  ESPRESSIONE
// costrutto if ... else  -->  ESPRESSIONE
// costrutto loop...  -->  ESPRESSIONE interrompibile con un break seguido da un valore di ritorno se presente. Si può saltare un'iterazione tramite continue

// -------------------------------------------------------------------------------------------------------------

// BLOCCO COME ESPRESSIONE
/* fn main() {
    let y = {
        let x = 3; 
        x + 1
    };
    println!("risultato : {}", y)
} */

// -------------------------------------------------------------------------------------------------------------

/* fn main() {
    let x = 10;
    let y = if x < 5 {0} else {1};
    println!("risultato di y: {}", y);
    } */

// -------------------------------------------------------------------------------------------------------------

/* fn main() {
    let mut counter = 0;
    let mut sum = 0;

    let result = loop {
        counter += 1;
        sum += counter;

        if counter == 10 {
            break sum * 2
        }
    };

    println!("The result is {}", result);
} */

// -------------------------------------------------------------------------------------------------------------

// codice consigliato da gemini sui loop annidati con le etichette


/* fn main() {
    let mut trovato = false;
    'outer : for i in 1..=10 {
        println!("Ciclo esterno #{}", i);

        'inner : for j in 1..=10 {
            println!("Ciclo interno #{}", j);

            if i * j > 10 {
                println!("Risultato trovato: {} x {} = {}", i,j,i*j);
                trovato = true;
                println!("Esco dal loop esterno e printo l'ultima riga del main")
                break 'outer;
            }

            if j == 3 {
                println!("Salto ad outer e continuo il programma al prossimo valore di i");
                continue 'outer;
            }
            println!("Printo questa riga e salto di nuovo ad inner per ripartire con il loop interno");
        }
    }
    println!("Codice finito");
    
} */

// -------------------------------------------------------------------------------------------------------------

//INTERVALLI
// a.. tutti i valori a partire da "a" (incluso)
// ..b tutti i valori prima di b fino a "b" (escluso)
// ..=c tutti i valori prima di c fino a "c" (escluso)
// d..e tutti i valori tra "d" ed "e" (d incluso, e escluso)
// f..=g tutti i valori tra "f" e "g" (estremi entrambi inclusi)



