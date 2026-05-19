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
// slide 14
/* use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
fn main() -> Result<(), Error> {
    // Definisci il percorso del file da aprire
    let file_path = r"C:\Users\aless\Documents\Uni\Programmazione di Sistema\slide teoria\copia_di_filename.txt";
    // Apri il file in modalità di lettura
    let  file = File::open(file_path)?;
    // Leggi il contenuto del file in una stringa
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    // Stampa il contenuto del file
    println!("Contenuto del file:");
    println!("{}", contents);
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
/* use std::path::PathBuf;
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
} */
// ----------------------------------------------------------------------------------------------------------------------------------------
// codice di prova 4 funzionante

/* use std::fs; // Necessario per create_dir, write e copy
use std::fs::File; // Per aprire/creare file
use std::io::{self, BufRead, BufReader}; // ricordati BRR, il freddo per ricordarti BufRead
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

// ----------------------------------------------------------------------------------------------------------------------------------------
//slide 26 --> in questo programma proviamo a leggere un file e scrivere due varianti di errore in caso di operazione impossibile

/* use std::io::{ErrorKind, Read};
use std::fs::File;

fn main() -> std::io::Result<()> {
    let fp = File::open(r"C:\\Users\\aless\\Documents\\Uni\\Programmazione di Sistema\\slide teoria\\copia_di_filename.txt"); 
    match fp {
    Ok(mut file) => {
        //tento di scrivere il contenuto del file su una stringa
        let mut contenuto = String::new();
        match file.read_to_string(&mut contenuto) {
            Ok(_) => println!("operazione avvenuta con successo."),
            Err(e) => match e.kind() {
                ErrorKind::NotFound => println!("il file non è stato trovato."),
                ErrorKind::PermissionDenied => println!("non disponi dei permessi necessari."),
                _ => println!("Impossibile aprire, a causa di : {:?}", e),
                },
            }
        },
    Err(e) => { 
        match e.kind() {
            ErrorKind::NotFound => println!("file non trovato."),
            ErrorKind::PermissionDenied => println!("non disponi dei permessi."),
            _=> println!("Impossibile aprire, a causa di : {:?}", e),
            }  
        }
    }
    Ok(())
} */

// ----------------------------------------------------------------------------------------------------------------------------------------
// slide 26
/* use std::fs::File;
use std::io::Read; // Mancava il punto e virgola

fn main() -> std::io::Result<()> {
    // Ho aggiunto 'mut' e l'assegnazione perché il match deve restituire il file alla variabile
    let mut file = match File::open(r"C:\Users\aless\Documents\Uni\Programmazione di Sistema\slide teoria\copia_di_filename.txt") {
        Ok(file) => file,
        Err(error) => {
            println!("Errore durante l'apertura del file: {}", error);
            return Err(error); // Bisogna ritornare l'errore per uscire
        }
    };

    let mut buffer = [0;10]; // questo è un array memorizzato nello stack.
    
    // let result = fs::read // ERRORE : il metodo read discende dal tratto Read
    // IMPORTANTE : la funzione read carica sul buffer i byte letti dal file sul vettore
    // Nota: in Rust è necessario passare il buffer come riferimento mutabile (&mut)
    let result = file.read(&mut buffer); 

    // IMPORTANTE: se un carattere ha una dimensione maggiore di 1 byte, allora vengono occupati due elementi del buffer
    match result {
        Ok(byte_letti) => {
            if byte_letti > 0 { // Corretto byte_Letti in byte_letti
                println!("Sono stati letti {} byte", byte_letti);
                // la funzione converte in stringa la codifica ASCII contenuta in ogni elemento del buffer, 
                // perchè con read ho preso ogni carattere dal file e ho messo la sua codifica binaria ASCII nell'elemento i-esimo del buffer
                let contenuto = String::from_utf8_lossy(&buffer[..byte_letti]); 
                println!("Contenuto : {}", contenuto);
            } else {
                println!("fine del file raggiunto.");
            }
        }
        Err(e) => println!("errore nell'apertura del file : {}", e),
    }; 
    
    let mut another_buffer = [0;5];
    let result2 = file.read(&mut another_buffer); // read restituisce un usize nel caso positivo
    match result2 {
        Ok(byte_letti2) => {
            println!("sono stati letti {} byte.", byte_letti2);
            if byte_letti2 > 0 {
                let s2 = String::from_utf8_lossy(&another_buffer[..byte_letti2]);
                println!(" Dati letti : {}", s2);
            } else {
                println!("fine del file raggiunto");
            }
        }
        Err(e) => {
            println!("errore nella seconda lettura del file : {}", e);
        }
    };
    Ok(())
} */

// ----------------------------------------------------------------------------------------------------------------------------------------
// slide 29 codice su read_to_end
/* use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut file = match File::open(r"C:\Users\aless\Documents\Uni\Programmazione di Sistema\slide teoria\copia_di_filename.txt")
    {
        Ok(f) => f,
        Err(e) => {
            println!("Impossibile aprire il file : {}", e);
            return Err(e)
        }
    };

    // definisco un vettore che può espandersi per poter utilizzare read_to_end
    // 3. Errore: non puoi usare 'file?' dopo aver già gestito l'errore con il match.
    // 'file' ora è un oggetto File, non un Result.
    let mut buffer = Vec::new();
    let result = file.read_to_end(&mut buffer);

    match result {
        // IMPORTANTE --> Anche se read_to_end restituisce il numero di byte letti nel file, posso non essere interessato a stamparli e quindi uso Ok(_)
        Ok(_) => {
            //qui i dati stanno nello heap, ma la validità UTF-8 dipende dal contenuto del file, non dalla locazione di memoria.
            let s = String::from_utf8(buffer); 
            match s {
                Ok(contenuto) => println!("Il contenuto nel file : {}", contenuto),
                Err(e) => println!("Errore nella decodifica : {}", e),
            }
        }
        Err(e) => println!("Errore nella lettura del file : {}", e),
    } // Rimosso punto e virgola superfluo dopo il match (opzionale)

    Ok(())
} */

// ----------------------------------------------------------------------------------------------------------------------------------------
// slide 31
// in questo codice voglio scrivere su un nuovo file e stampare la stringa corrispondente ai primi 30 byte del file. Per farlo bisogna convertire i byte in stringa

/* use std::fs::{self,File};
use std::io::Read;
use std::path::PathBuf;

fn main () -> std::io::Result<()> {
    let mut path1 = PathBuf::from(r"C:\Users\aless\Documents\Uni\Programmazione di Sistema\slide teoria");
    path1.push("new_file31.txt");
    let ref_path = path1.as_path();
    if !ref_path.exists() {
        let _file = File::create(ref_path);
        println!("file creato con successo")
    }

    // se il percorso del file è gia presente posso scrivere direttamente sopra un testo
    let contenuto = "ciao mamma, guarda come mi diverto con Rust!";
    fs::write(&path1,contenuto);

    let mut file = File::open(path1)?;
    let mut buffer = [0;30];
    // ERRORE IMPORTANTE --> let result = file.read_exact(&mut buffer)?; Qui io sto modificando il contenuto del buffer perchè sto copiando i primi 30 byte del file nel buffer. 
    let result1 = file.read_exact(&mut buffer); // IMPORTANTE Nel caso in cui l'operazione fatta da read_exact vada a buon fine, la funzione restituisce un Result<()>. Questo vuol dire che nel caso positivo la funzione non restituisce niente, si limita a modificare il buffer. IN caso negativo la funzione restituisce un errore, ovvero Err(e)
    match result1 {
        Ok(b) => println!("I byte letti corrispondono a : {:?}. {:?}", String::from_utf8_lossy(&buffer), b), // IMPORTANTE: dato che il tratto Read ha molte funzioni e ci sta non ricordarsi cosa restituisce ciascuna, puoi pensare di scrvere un blocco match in cui tenti di printare il contenuto restituito dalla funzione. 
        Err(_) => println!("Impossibile"),
    }

    Ok(())

}
 */


// ----------------------------------------------------------------------------------------------------------------------------------------
// slide 32


// voglio leggere un file e stampare la corrispondenza tra ogni carattere e il valore del byte associato

/* use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()>{
    let mut file = File::open(r"C:\Users\aless\Documents\Uni\Programmazione di Sistema\slide teoria\text.txt")?;

    for byte in file.bytes() {
        match byte {
            Ok(b) => println!("Byte : {} {}", b, char::from_u32(b as u32).unwrap()), // mi serve convertire il byte in un u32 se voglio rappresentarlo come char, che in byte sappiamo che possono avere dimensione variabile dagli 1 ai 4 byte
            Err(_) => println!("Impossibile eseguire la lettura di questo carattere"),
        };
    }
    Ok(())
} */

// ----------------------------------------------------------------------------------------------------------------------------------------

/* use std::fs::File;
use std::io;
use std::io::{Write, BufReader, BufRead};


fn main() -> io::Result<()> {
    let path = "myfile";

    let mut output = File::create(path)?;
    write!(output, "Rust\n💖\nFun")?;

    let input = File::open(path)?;
    let buffered = BufReader::new(input); //creo un buffer di dimensione predefinita di 8k che posso scandire riga per riga

    for line in buffered.lines() { // leggo una riga alla volta dell'intero file
        println!("{}", line?);
    }

    Ok(())
} */
// ----------------------------------------------------------------------------------------------------------------------------------------

/* use std::io::{BufReader,BufRead,Result};
use std::fs::File;

fn main() -> Result<()> {
    let file = File::open(r"C:\Users\aless\Documents\Uni\Programmazione di Sistema\slide teoria\text.txt")?;

    // Voglio leggere una riga del file --> uso il metodo read_line
    let mut buffer = BufReader::new(file);

    let mut contenuto = String::new();
    buffer.read_line(&mut contenuto)?;
    println!("{}", contenuto);

    Ok(())
} */

// ----------------------------------------------------------------------------------------------------------------------------------------
// slide 38
// questo codice legge la prima e la seconda riga di un file

/* use std::fs::File;
use std::io::{BufReader,BufRead};

fn main() -> std::io::Result<()> {
    let file = File::open(r"C:\Users\aless\Documents\Uni\Programmazione di Sistema\slide teoria\text.txt")?;

    let mut buffer = BufReader::with_capacity(1024,file); // il buffer creato con BufReader serve per minimizzare la lettura del file dal disco, perchè carica 1024 byte del file

    let mut contenuto = String::new();
    buffer.read_line(&mut contenuto);
    println!("Riga 1 : {:?}", contenuto);
    contenuto.clear(); // questo metodo dice a Rust di scrivere a partire dall'indice 0 della stringa

    // alla seconda chiamata, la read_line passa alla riga successiva
    buffer.read_line(&mut contenuto);
    println!("Riga 2 : {:?}", contenuto);
    Ok(())
} */

// ----------------------------------------------------------------------------------------------------------------------------------------
// slide 40

/* use std::fs::File;
use std::io::{self, BufReader, BufRead, Result};
use std::str;

fn main() -> Result<()> {
    let file = File::open(r"C:\Users\aless\Documents\Uni\Programmazione di Sistema\lezioni in classe PDS\RUST appunti\lezione del 3 marzo PDS RUST.txt")?;

    let mut reader = BufReader::new(file); // creo un buffer vuoto a partire dal file di testo aperto
    let mut total_bytes_read = 0;

    loop {
        let buffer = reader.fill_buf()?;
        let len = buffer.len();

        if len == 0 {
            break; // Fine del file
        }

        // Processa il buffer qui.
        // In questo esempio, lo stampiamo a blocchi.
        match str::from_utf8(buffer) {
            Ok(s) => print!("{}", s),
            Err(_) => {
                eprintln!("Warning: Invalid UTF-8 encountered in buffer.");
             }
        }
        // Indica al BufReader quanti byte abbiamo letto (in questo caso, l'intero buffer).
        reader.consume(len);
        total_bytes_read += len;

    }
    println!("\nLettura completata. Totale byte letti: {}", total_bytes_read);
    Ok(())
} */

// ----------------------------------------------------------------------------------------------------------------------------------------
// slide 42
// in questo codice voglio scrivere 5 volte Hello world su un file appena creato

/* se std::fs::File;
use std::io::{self,Write};

fn main() -> io::Result<()> {
    let mut file = File::create(r"C:\Users\aless\Documents\Uni\Programmazione di Sistema\slide teoria\text1.txt")?;

    let iterazioni = 5;
    let contenuto = b"Hello world!\n"; // scrivere b davanti ad un literal string trasforma la stringa in un array di byte in cui ogni elemento è la conversione ASCII di ciascuna lettera della stringa
    let mut total_byte = 0;
    for _ in 0..=iterazioni {
        match file.write(contenuto) { // il metodo write restituisc un Result che contiene nel ramo positivo il numero di byte scritti; nel ramo negativo un errore
            Ok(bytes_written) => {total_byte += bytes_written;},
            Err(e) => println!("Errore durante la scrittura : {}", e),
        }
    }
    println!("totale dei byte : {}", total_byte);
    Ok(())
} */
 

// ----------------------------------------------------------------------------------------------------------------------------------------
// slide 43
// in questo codice voglio scrivere 5 volte Hello world su un file appena creato

/* use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut file = File::open(r"C:\Users\aless\Documents\Uni\Programmazione di Sistema\slide teoria\text1.txt")?;
    let data = b"Hello world";
    
    // scrivo tutto il vettore nel file
    file.write_all(data)?;

    match file.flush() {
        Ok(b) => println!("sono statti scritti i dati sul disco. Numero byte : {}", b),
        Err(err) => {
            eprintln!("Errore durante il flushing dei dati nel file: {}", err);
            return Err(err);
        }
    }
    Ok(())
} */

// ----------------------------------------------------------------------------------------------------------------------------------------
// le slide da 44 a 48 il prof le reputa inutili, non le studio

// ----------------------------------------------------------------------------------------------------------------------------------------
// slide 49

use std::io::{self,Cursor,Read,Seek};

fn main() -> io::Result<()> {
    // Creo un buffer in memoria che simula un file, in questo caso un stringa rappresentata in byte
    let data = b"Hello world".to_vec();
    // Racchiudo il dato in un Cursor
    let mut cursor = Cursor::new(data);

    let mut buffer = [0;5];
    let byte_letti = match cursor.read(&mut buffer) {
            Ok(b) => {
            // in questo caso i dati sono stati scritti come ASCII puro, quindi UTF-8 valido. Quindi non occorre scrivere String::from_utf8_lossy  
            let contenuto_testo = str::from_utf8(&buffer[..b]);
            println!("byte letti : {}. Contenuto originale : {:?}",b,contenuto_testo);
            b // IMPORTANTE --> scrivendo così assegno il valore del contenitore restituito dalla read alla variabile byte_letti
        },
        Err(e) => {
            eprintln!("errore nella lettura : {}", e);
            return Err(e);
        }
    };

    let current_position = cursor.stream_position()?;
    println!("Posizione corrente del cursore : {}", current_position);

    cursor.rewind()?;
    println!("Cuesore riavvolto all'inizio.");


    let mut buffer_again = [0; 5]; // Leggi di nuovo dall'inizio

    let bytes_read_again = cursor.read(&mut buffer_again)?;
    println!("Letti di nuovo {} bytes: {:?}", bytes_read_again, str::from_utf8(&buffer_again[..bytes_read_again]));

    Ok(())


}

// ----------------------------------------------------------------------------------------------------------------------------------------
// slide 56

use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Write;
// Definiamo una struttura dati per i nostri dati JSON.
#[derive(Debug, Serialize, Deserialize)]
struct Persona {
    nome: String,
    cognome: String,
    eta: u32,
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let persona1 = Persona {
        nome: "Mario".to_string(),
        cognome: "Rossi".to_string(),
        eta: 30,
    };
    let persona2 = Persona {
        nome: "Luigi".to_string(),
        cognome: "Bianchi".to_string(),
        eta: 25,
    };
    let persone = vec![persona1, persona2];

    // Serializziamo il vettore in formato JSON.
    let json_data = serde_json::to_string(&persone)?;
    // let json_data = serde_json::to_string_pretty(&persone)?;
    let mut file = File::create("persone.json")?; // Scriviamo il JSON su un file.
    file.write_all(json_data.as_bytes())?;
    Ok(())
}