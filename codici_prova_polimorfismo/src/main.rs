// prova tratto summarizable

/* trait Summarizabile {
    fn summary(&self) -> String;
}

impl Summarizabile for f64 {
    fn summary(&self) -> String { // se io scrivo Self, significa che il compilatore si aspetta che il tipo ritornato dal metodo sia un f64, perchè sto implementando il tratto per un f64 nel blocco 7-10. Tuttavia la macro format sta convertendo l'f64 (cioè self) in una stringa, per cui il fatto che ritorno una stringa non è coerente con il fatto che debba ritornare un f64
        format!("{:.2}", self)
    }
}

impl Summarizabile for &str {
    fn summary(&self) -> String {
        if self.len() > 4 {
            format!("{}", &self[..=5])
        }
        else {
            self.to_string()
        }
    }
}


fn main() {
    let n = 5.2/2.4;
    println!("n value without summary : {}", n);
    println!("summarized n : {:?}", n.summary());
    let stringa = "Ciao_mondo!";
    println!("summarized string : {:?}", stringa.summary());
} */

// --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

// slide 17, riprovo a fare il codice 

/* trait Moltiplica {
    fn moltiplica(&self, factor : i32) -> i32;
    fn moltiplica_per_due(&self) -> i32 {
        self.moltiplica(2)
    }
}

struct Numero2 {
    value : i32,
    other : i32,
}
impl Moltiplica for Numero2 {
    fn moltiplica(&self, factor : i32) -> i32 {
        self.value * factor
    }
}

struct Numero {
    value : i32,
}
impl Moltiplica for Numero {
    fn moltiplica(&self, factor : i32) -> i32 {
        self.value * factor
    }
    fn moltiplica_per_due(&self) -> i32 {
        println!("Implementazione specifica della funzione moltiplica_per_due");
        self.moltiplica(2)
    }
}



fn main() {
    let numero = Numero{ value : 5};
    let numero2 = Numero2 { value : 7, other :2};
    println!("risultato : {}.", numero.moltiplica_per_due());
    println!("risultato : {}.", numero2.moltiplica_per_due());
} */

// --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
// slide 25

/* struct Cerchio {
    raggio : f64,
}

struct Rettangolo {
    altezza : f64,
    larghezza : f64,
}

trait Forma {
    fn calcola_area(&self) -> f64;
    fn calcola_perimetro(&self) -> f64;
}

impl Forma for Cerchio {
    fn calcola_area(&self) -> f64 {
        std::f64::consts::PI * self.raggio * self.raggio
    }
    fn calcola_perimetro(&self) -> f64 {
        2.0 *self.raggio * std::f64::consts::PI 
    }
}

impl Forma for Rettangolo {
    fn calcola_area(&self) -> f64 {
        self.altezza * self.larghezza
    }
    fn calcola_perimetro(&self) -> f64 {
        2.0 *(self.altezza + self.larghezza)
    }
}

fn stampa_descrizione(forma : impl Forma) {
    println!("---DESCRIZIONE---");
    println!("Area : {}", format!("{:.2}", forma.calcola_area()) );
    println!("Perimetro : {}", format!("{:.2}", forma.calcola_perimetro()) );
}


fn main() {
    let c = Cerchio { raggio : 5.0 };
    let r = Rettangolo { altezza :2.0 , larghezza : 3.0};
    stampa_descrizione(c);
    stampa_descrizione(r);
} */

// --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
// rifaccio codice slide 15 --> converti un numero intero in un floating point; converti un numero reale in un i32

/* trait Convertibile {
    type Output;
    fn converti(&self) -> Self::Output;
}
 
struct NumeroIntero {
    valore : i32,
}

impl Convertibile for NumeroIntero {
    type Output = f64; // definisco un tipo specifico ad un'istanza di Self
    fn converti(&self) -> Self::Output {
        self.valore as Self::Output
    }
}

struct NumeroReale {
    valore : f64,
}
impl Convertibile for NumeroReale {
    type Output = i32;
    fn converti(&self) -> Self::Output{
        self.valore as Self::Output
    }
}

fn main() {
    let n1 = NumeroIntero { valore : 10 };
    println!("n1 convertito : {}", format!("{:.2}", n1.converti()) );
    let n2 = NumeroReale { valore : 10.34 };
    println!("n2 convertito : {}", n2.converti() );
} */

// esercizio Traduttore di segnali Raw, chat "Rust:analizzo programmi singoli"

/* trait ProcessoreSegnale {
    type Risultato;
    fn elabora(&self) -> Self::Risultato;
}

struct SensorePressione {
    psi : f64,
}

struct SensoreStato {
    attivo : bool,
}

impl ProcessoreSegnale for SensorePressione {
    type Risultato = u32;
    fn elabora(&self) -> Self::Risultato {
        self.psi as Self::Risultato
    }
}

impl ProcessoreSegnale for SensoreStato {
    type Risultato = String;
    fn elabora(&self) -> Self::Risultato {
        if self.attivo {format!("SISTEMA OPERATIVO")}
        else {format!("SISTEMA SPENTO")}
    }
}

fn main() {
    let p = SensorePressione { psi: 32.7 };
    let s = SensoreStato { attivo: true};

    println!("pressione {}", p.elabora());
    println!("Stato {}", s.elabora());
} */

// esercizio per praticare slide 17 --> command f "Sistema di Reset"

/* trait Dispositivo {
    fn reset_totale(&self);
    fn reset_rapido(&self) {
        self.reset_totale()
    }
}

struct Router {
    id : i32,
}


impl Dispositivo for Router {
    fn reset_totale(&self) {
        println!("Router: riavvio hardware completo")
    }
}

struct CacheMemory {
    dimensione : i32
}
impl Dispositivo for CacheMemory {
    fn reset_totale(&self) {
        println!("Cache svuotamento totale e riallocazione");
    }
    fn reset_rapido(&self) {
        println!("Cache: solo invalidazione indici")
    }
}

fn main() {
    let r = Router { id: 192 };
    let c = CacheMemory { dimensione: 1024 };

    println!("--- Test Router ---");
    r.reset_rapido(); // Dovrebbe chiamare il reset totale (default)

    println!("\n--- Test Cache ---");
    c.reset_rapido(); // Dovrebbe eseguire la versione personalizzata
} */

// esercizio per praticare slide 17 e slide 13

/* trait Monitorabile {
    type Valore : PartialOrd; // PartialOrd permette l'uso di operatori come < e >
    fn leggi_sensore(&self) -> Self::Valore;
    fn genera_allarme(&self, soglia: Self::Valore) -> bool {
        if self.leggi_sensore() > soglia {true} else {false}
    }
}

struct CpuSensor {
    carico : u8,
}
impl Monitorabile for CpuSensor {
    type Valore = u8;
    fn leggi_sensore(&self) -> Self::Valore {
        self.carico as Self::Valore
    }
}
struct TempSensor {
    gradi : f32
}
impl Monitorabile for TempSensor {
    type Valore = f32;
    fn leggi_sensore(&self) -> Self::Valore {
        self.gradi as Self::Valore
    }
    fn genera_allarme(&self, soglia: Self::Valore) -> bool {
        if self.gradi > (soglia - 2.0) {true} else {false}
    }
}

fn main() {
    let cpu = CpuSensor { carico: 85 };
    let temp = TempSensor { gradi: 38.5 };

    // SOGLIE
    let soglia_cpu = 80;
    let soglia_temp = 40.0;

    println!("--- MONITORAGGIO SISTEMA ---");
    println!("Allarme CPU (>80%): {}", cpu.genera_allarme(soglia_cpu));
    println!("Allarme TEMP (>40.0°C con margine): {}", temp.genera_allarme(soglia_temp));
} */

// ----------------------------------------------------------------------------------------------------------------------------------------------------------------------
// esercizio per praticare i concetti di slide 31

trait Identificabile {
    type Output;
    fn id(&self) -> Self::Output;
}

// Attuatore eredita Identificabile
trait Attuatore: Identificabile {
    fn esegui_azione(&self);
}

struct Lampadina {
    id_field: u32, // Rinominato per chiarezza, ma non obbligatorio
}

impl Identificabile for Lampadina {
    type Output = u32;
    fn id(&self) -> Self::Output {
        self.id_field
    }
}

impl Attuatore for Lampadina {
    fn esegui_azione(&self) {
        println!("Lampadina {} accesa.", self.id());
    }
}

struct Termostato {
    id_field: u32,
}

impl Identificabile for Termostato {
    type Output = u32;
    fn id(&self) -> Self::Output {
        self.id_field
    }
}

impl Attuatore for Termostato {
    fn esegui_azione(&self) {
        println!("Termostato {} impostato a 20 gradi.", self.id());
    }
}

// NOTA: Qui specifichiamo <Output = u32> per rendere il trait Object Safe
fn attiva_periferica(p: &dyn Attuatore<Output = u32>) {
    println!("Accesso id #{}", p.id());
    p.esegui_azione(); // Chiamata diretta, senza println!
}

fn main() {
    let l = Lampadina { id_field: 1 };
    let t = Termostato { id_field: 2 }; // Cambiato id per distinguerli
    
    attiva_periferica(&l);
    attiva_periferica(&t);
}