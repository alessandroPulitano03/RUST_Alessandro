// ripassa differenza tra Path e PathBuf su gemini per imparare la GESTIONE DEI PERCORSI SU RUST--> ctrl f Anima

// PathBuf è un contenitore di dati, tipo la String, che possiede il percorso. Il dato è il percorso, e può essere modificato con opportuni metodi
// Path è un riferimento ad un percorso, analogo a &str. Rappresenta quindi un riferimento ad un percorso, che ho creato tramite PathBuf

/* use std::path::{Path, PathBuf};

fn main() {
    // Creiamo un PathBuf (Owned)
    let mut percorso_base = PathBuf::from("/home/alessandro");
    
    // Lo modifichiamo (solo PathBuf può farlo)
    percorso_base.push("progetti");
    percorso_base.push("rust_lab");
    
    // Passiamo un riferimento (&Path) alla funzione
    stampa_percorso(&percorso_base);
}

// La funzione accetta &Path: è la pratica migliore!
fn stampa_percorso(p: &Path) {
    println!("Sto guardando il percorso: {:?}", p);
} */

// ----------------------------------------------------------------------------------------------------------------------------------------
// slide 5 NAVIGARE SUL FILE SYSTEM

// read_dir: Non restituisce una lista fissa, ma un iteratore. È un approccio "pigro" (lazy) ed efficiente: non carica in memoria tutti i nomi dei file contemporaneamente, ma te ne dà uno alla volta mentre scorri il ciclo

/* use std::fs;
use std::path::{Path, PathBuf};
fn main() -> std::io::Result<()> {
    let mut p = PathBuf::from("src"); // creo una variabile che detiene il percorso. Nota che hai utilizzato il cotenitore PathBuf dal modulo path per creare un percorso. Per creare usi anche from
    p.push("main.rs"); // aggiunge un segmento in modo portabile    
    println!("Path completo: {:?}", p);
    let path: &Path = p.as_path(); // trasformo un percorso in un riferimento per poter poi stampare se è un file o una cartella. Nel caso fosse un file, stampo poi i suoi attributi.
    // IMPORTANTE : Creare un riferimento ad un file con il metodo .ad_path() mi permette di accedere ai suoi metadati. Il riferimento mi permette di accedere ai dati del percorso senza usare moduli della liberria standard
    println!("Esiste? {}", path.exists());
    println!("È un file? {}", path.is_file());
    println!("È una directory? {}", path.is_dir());
    println!("Nome file: {:?}", path.file_name());
    println!("Estensione: {:?}", path.extension());
    println!("File stem: {:?}", path.file_stem());
    println!("Parent: {:?}", path.parent());
    // Itera sui componenti del path
    println!("Componenti:");
    for c in path.components() {
        println!(" {:?}", c);
    }
    if path.exists() {
        let metadata = fs::metadata(path)?; // segue eventuali symlink
        println!("Dimensione: {} byte", metadata.len());
        println!("Readonly? {}", metadata.permissions().readonly());
        if let Ok(modified) = metadata.modified() {
            println!("Ultima modifica: {:?}", modified);
        }
        if let Ok(created) = metadata.created() {
            println!("Creato il: {:?}", created);
        }
    }
    Ok(())
} */

// ----------------------------------------------------------------------------------------------------------------------------------------
// slide 7

/* fn main() -> std::io::Result<()> { // nota che qui usi come valore di ritorno un Result dal modulo IO perchè stai accedendo al file system e quindi stai facendo già così un'operazione IO
    // Ottieni il percorso della directory
    let directory_path = ".";

    // Leggi il contenuto della directory
    let entries = fs::read_dir(directory_path)?; // un'operazione di lettura di una cartella. il metodo create_dir() mi restituisce un riferimento per ogni elemento, che ti viene dato uno alla volta, al file o la directory. Con il riferimento, come visto nel primo esempio del capitolo, posso accedere ai metadati (nome --> file_name(), estensione --> .extension(), solo il nome senza estensione, se è una cartella --> .is_dir(), se è un file --> .is_file() )
    //richiede di utilizzare il modulo fs

    // Itera sugli elementi nella directory
    for entry in entries {
        // Gestisci eventuali errori nell'accesso ai file/directory --> mi serve per controllare se la cartella può potenzialmente non avere alcun elemento all'interno
        let entry = entry?;

        // Ottieni il nome dell'elemento
        let file_name = entry.file_name();

        // Stampa il nome dell'elemento
        println!("{:?}", file_name);
    }
    Ok(())
} */

// ----------------------------------------------------------------------------------------------------------------------------------------
// slide 9
/* use std::fs;

fn main() -> std::io::Result<()> {
    // Definisci il percorso della nuova directory da creare
    let p1 = "./mynewdir";

    // Crea la nuova directory
    // ERRORE let new_directory = p1.create_dir(); 
    fs::create_dir(new_directory_path)?;

    println!("Directory creata con successo!");
    Ok(())
} */

// ----------------------------------------------------------------------------------------------------------------------------------------
// codice di prova 1
/* use std::path::PathBuf;

fn main() {
    let mut p = PathBuf::from("src");
    p.push("main.rs");
    let ref_p = p.as_path();
    println!("{}", ref_p.exists());
    for c in p.components(){
        println!(" {:?}\n",c);
    }

} */

// ----------------------------------------------------------------------------------------------------------------------------------------
// codice di prova 2 --> codice con errori che ho provato a fare
/* use std::path::PathBuf;
use std::fs;

fn main() {
    // creo il percorso, verifico che sia una cartella e stampo le componenti del percorso
    let p = PathBuf::from("C:\Users\aless\Documents\Uni\Programmazione di Sistema\slide teoria");
    let ref_p = p.as_path();
    if ref_p.is_dir() {
        for c in ref_p.components {
            println!(" {:?}\n",c);
        }
    }
    // creo una cartella --> devo importare il file system nel codice
    let nd = fs::create_dir(ref_p)? // fornisce un Result --> se la cartella esiste già nel percorso specificato la funzione ritorna un errore 
    for el in nd{
        let elemento = el?
        let filename = elemento.file_name()?
        // Stampa il nome dell'elemento
        println!("{:?}", filename);
    }
} */

// ----------------------------------------------------------------------------------------------------------------------------------------
// codice di prova 2 funzionante
/* use std::fs;
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    // 1. Definiamo il percorso base
    let mut p = PathBuf::from(r"C:\Users\aless\Documents\Uni\Programmazione di Sistema\slide teoria");

    // 2. Per creare una cartella "A PARTIRE" da questo percorso, dobbiamo aggiungere un nome
    p.push("Esercitazione_Nuova"); // quuindi devo assegnare un nome nuovo e poi creo la cartella, controllando che prima non esista con lo stesso nome

    // 3. Creiamo la sottocartella (se non esiste)
    if !p.exists() {
        fs::create_dir(&p)?;
        println!("Cartella creata con successo!");
    }

   // Provo a rimuovere la cartella
    fs::remove_dir(&p)?;

    if !p.exists() {
        println!("cartella rimossa con successo!");
    }

    // 4. Se vuoi LEGGERE il contenuto (come nel tuo ciclo for), devi usare read_dir
    // Usiamo il percorso originale (togliendo l'ultima aggiunta)
    p.pop(); 
    let entries = fs::read_dir(&p)?; // Questo fornisce l'iteratore

    for entry in entries {
        let entry = entry?;
        let filename = entry.file_name();
        println!("Trovato: {:?}", filename);
    }
    Ok(())
} */

// ----------------------------------------------------------------------------------------------------------------------------------------
// codice di prova 4 errato
use std::path::PathBuf;
use std::File::BufReader; // ricordati BRR, il freddo per ricrodardi BufRead

fn main() {
    let p1 = PathBuf::from(r"C:\Users\aless\Documents\Uni\Programmazione di Sistema\slide teoria");
    p.push("nuova cartella");
    let ref_p1 = p.as_path();

    if !ref_p1.exists() {
        fs::create_dir(ref_p1)?;
        println!("cartella creata con successo");
    }

    // voglio creare un file e scriverci qualcosa sopra, ma non so come fare
    

    let file = File::open("filename.txt") // come faccio ad aprire un file a partire dal percorso?
    let buffer = BufReader::new(file);
    for l in buffer.lines() { // uso l per abbreviare la parola line; nella riga successiva uso proprio la parola line per intero per indicare che se la riga esiste allora fanne il binding con una variabile con un nome esplicativo, in cui si vede bene che si tratta di una line
        // ogni volta che accedo ad una struttura dati utilizzando un iteratore devo sempre controllare con il ? se l'elemento i-esimo esiste
        let line = l?;
        println!("{:?}"; line);
    }

    // qui vorrei copiare il file generato in un altro percorso con fs::copy
}
// ----------------------------------------------------------------------------------------------------------------------------------------
// codice di prova 4 funzionante

/* use std::fs; // Necessario per create_dir, write e copy
use std::fs::File; // Per aprire/creare file
use std::io::{self, BufRead, BufReader, Write}; // ricordati BRR, il freddo per ricordarti BufRead
use std::path::PathBuf;

fn main() -> io::Result<()> { // Aggiunto il tipo di ritorno per usare il ?
    // 1. Definiamo il percorso base
    let mut p1 = PathBuf::from(r"C:\Users\aless\Documents\Uni\Programmazione di Sistema\slide teoria");
    
    // Aggiungiamo la sottocartella al percorso
    p1.push("nuova cartella");

    if !p1.exists() {
        fs::create_dir(&p1)?; // Usiamo &p1 direttamente, è più idiomatico
        println!("cartella creata con successo");
    }

    // --- RISPOSTA AL DUBBIO: CREARE E SCRIVERE ---
    // Per creare un file dentro la cartella appena creata, spingiamo il nome del file nel percorso
    p1.push("filename.txt"); 
    
    // fs::write è il modo più veloce per creare un file e scriverci sopra in un colpo solo
    let contenuto = "Ciao! Questo è il contenuto del file.\nSeconda riga.\n";
    fs::write(&p1, contenuto)?; 
    println!("File creato e scritto con successo in: {:?}", p1);

    // --- RISPOSTA AL DUBBIO: APRIRE DAL PERCORSO ---
    // File::open accetta &p1 (che ora punta al file .txt)
    let file = File::open(&p1)?; 
    let buffer = BufReader::new(file); //accedo al contenuto del file. 

    for l in buffer.lines() { 
        // uso l per abbreviare la parola line; nella riga successiva uso proprio la parola line per intero 
        // per indicare che se la riga esiste allora fanne il binding con una variabile con un nome esplicativo
        let line = l?;
        println!("{}", line); // Usiamo {} per le stringhe pulite, {:?} per il debug
    }

    // --- RISPOSTA AL DUBBIO: COPIARE IL FILE ---
    // Creiamo un nuovo percorso per la copia (ad esempio, nella cartella superiore)
    let mut p_copia = p1.clone();
    p_copia.pop(); // Togliamo "filename.txt"
    p_copia.pop(); // Togliamo "nuova cartella"
    p_copia.push("copia_di_filename.txt");

    // fs::copy(sorgente, destinazione)
    fs::copy(&p1, &p_copia)?;
    println!("File copiato con successo in: {:?}", p_copia);

    Ok(())
} */