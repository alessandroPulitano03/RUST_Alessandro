// in questo primo esempio abbiamo visto che il valore di ritorno della funzione dipende solo dall'istanza di Example, per cui non serve annotare il tempo di vita
// il compilatore applica la regola: se c'è un &self, si asusme che il riferimento del valore di ritorno sia legato al parametro &self. Poichè questo fatto è rispettato in questo codice, non ci sono errori
/* struct Example {
    value1:i32,
    value2:i32,
}

impl Example {
    fn get_data_ref(&self, other:&i32) -> &i32 {
        if self.value1 > *other {
            &self.value1
        } else{
            &self.value2
        }
    }
}


fn main() {
    let ex = Example { value1:45, value2:20};
    let other_data = 30;
    let data_ref = ex.get_data_ref(&other_data);
    println!("Valore restituito : {}", data_ref);
}  */


// --------------------------------------------------------------------------------------------------------------------------------------------------------------------------

// in questo secondo esempio vediamo che la funzione restiuisce un output che non dipende solo da &self

/* struct Example {
    value1:i32,
    value2:i32,
} 

impl Example {
    fn get_data_ref<'a>(&'a self, other:&'a i32) -> &'a i32 {
        if self.value1 > *other {
            &self.value1
        } else{
            &other
        }
    }
}


fn main() {
    let ex = Example { value1:45, value2:20};
    let other_data = 30;
    let data_ref = ex.get_data_ref(&other_data);
    println!("Valore restituito : {}", data_ref);
}  */


// --------------------------------------------------------------------------------------------------------------------------------------------------------------------------

// le annotazioni del tempo di vita vanno scritte subito dopo i due punti
/* fn confronta<'a>(s1 : &'a str, s2 : &'a str) -> & 'a str {
    if s1.len() > s2.len() {
        &s1
    } else {
        &s2
    }
}

fn main() {
    let stringa1 = String::from("hello");
    let stringa2 = String::from("ciao");
    let risultato = confronta(&stringa1, &stringa2);
    println!("Stringa vincente : {}", risultato);
} */

// --------------------------------------------------------------------------------------------------------------------------------------------------------------------------

/* fn merge_slices<'a,T>(slice1 : 'a [T], slice2 : 'a [T]) -> Vec<'a T> 
    where T : Copy // IMPORTANTE : Il vincolo Copy è fondamentale quando voglio ottenere un valore partendo da un riferimento, senza rubare l'originale. Io non potrei spostare un dato fuori da un riferimento condiviso, perchè se si tratta di un riferimento si fa la promessa di non distruggere l'oggetto originale mediante una move. IN QUESTO CODICE NON È NECESSARIO COPY, PERCHÈ NON STO RESTITUENDO I VALORI ORIGINALI DELLE SLICE, NE STO PRENDENDO UN RIFERIMENTO.
    {
        let mut merged = Vec::new();
        for item in slice1.iter().chain(slice2.iter()) { // il metodo chain concatena due iteratori
            merged.push(item);
        }
    }

fn main() {
    let first_slice = &[1,2,3,4];
    let second_slice = &[5,6,7,8,9];
    merge_slices(first_slice,second_slice);

    println!("Merged slices: {:?}", merged_result);
} */

// --------------------------------------------------------------------------------------------------------------------------------------------------------------------------
// questo codice sotto richiede di implementare il tratto copy per il tipo T. La funzione generica prende in input una slice e cerca di stampare il primo elemento

/* fn get_first<T>(v1 : &[T]) -> T 
    where T: Copy 
    {
        v1[0]
    }

fn main() {
    let vettore = vec![1,2,3,4];
    let primo_elemento = get_first(&vettore); // passo una slice
    println!("primo elemento : {}", primo_elemento);

} */

// --------------------------------------------------------------------------------------------------------------------------------------------------------------------------

// guarda le slide dal mac per le slide 18-22

// --------------------------------------------------------------------------------------------------------------------------------------------------------------------------
// slide 24-27
/* 
fn insert(vet : &mut Vec<&str>, stringa : &str) { // IMPORTANTE : qui occorre specificare il tempo di vita sia del vettore, sia della stringa in quanto il vettore contiene dei riferimenti. Vediamo dal codice che il valore di ritorno è il vettore, quindi sicuramente serve l'annotazione del tempo di vita sul vettore; in più è richiesta anche sul riferimento della stringa, in quanto tale riferimento è dentro il vettore --> il succo del dicorso è che dobbiamo dire al compilatore che il tempo di vita del contenuto deve durare almeno quanto il tempo di vita del contenitore
    vet.push(stringa);
}

fn main() {
    let mut v = Vec::<&str>::new(); // questo è un vettore contenente dei riferimenti a stringhe
    {
        let s = "Ciao".to_string();
        insert(&mut v, &s) 
    }
    println!("{:?}",v); // IMPORTANTE questo codice non compila in quanto il vettore contiene un riferimento ad un dato, in questo caso una String, che è morta nello scope sintattico interno. 
} */

// CODICE CORRETTO

fn insert<'a>(v1 : &mut Vec<& 'a str>, s1 : & 'a str) { // Di conseguenza, il compilatore "assegna" automaticamente al vettore v un tempo di vita compatibile con quello di s
    v1.push(s1);
}


fn main() {
    let mut v = Vec::<&str>::new(); 
   
    let s = "Ciao".to_string();
    insert(&mut v, &s);
   
    println!("{:?}",v); 
}

// 50'06'' 31-03-2026




// ------------------------------------------------------------------------------------------------
// slide 27 (per le slide precedenti guarda il desktop pc)



/* fn insert<'a>(vet: &mut Vec<&'a str>, s:&str) {
    vet.push("1");
    println!("{}", s);
}

fn main() {
    let mut v = Vec::<&str>::new();
    insert(&mut v,&"Inserisco una stringa".to_string());
    println!("Vettore : {:?}", v);
} */


// ------------------------------------------------------------------------------------------------
// slide 28
/* fn insert<'a>(v1 : &mut Vec<&'a str>, s : &'a str) {
    v1.push(s);
}

fn main(){
    let v = Vec::<&str>::new();
    insert(&mut v, &"ciao".to_string());
    println!("{:?}",v);
} */
// ------------------------------------------------------------------------------------------------

struct TextWindow<'a> {
    content: &'a str,
    }

impl TextWindow<'_> {
    fn new(content: &str) -> TextWindow {
        TextWindow { content }
    }

    fn display(&self) {
        println!("Text window content: {}", self.content);
    }
}





fn main() {
    let my_text = "Hello, world!".to_string();
    let text_window = TextWindow::new(&my_text);
    text_window.display();
}