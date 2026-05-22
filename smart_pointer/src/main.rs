
/* fn produce ( odd:bool) -> Box<i32> {
    let mut b = Box::new(0);
    if odd { *b = 5; }
    b
}


fn main() {
    let b1 = produce(false);
    println!("b1 : {:?}", b1);
    let b2 = produce(true);
    println!("b2 : {:?}", b2);
} */

// ------------------------------------------------------------------------------------------------------------------------------------------------------------------
// slide 24




// ------------------------------------------------------------------------------------------------------------------------------------------------------------------
// slide 26
// Il box serve per implementare oggetti tratto

/* trait Animal {
    fn make_sound(&self) -> String;
}

struct Dog;
impl Animal for Dog {
    fn make_sound(&self) -> String 
    {"Bau!".to_string()}
}

struct Cat;
impl Animal for Cat {
    fn make_sound(&self) -> String {"Miao!".to_string()}
}

fn main() {
    let animals : Vec<Box<dyn Animal>> = vec![Box::new(Dog), Box::new(Cat)]
} */

// ------------------------------------------------------------------------------------------------------------------------------------------------------------------
// slide 27

/* use std::io;
fn main() {
    println!("Inserisci la dimensione (in byte) del dato puntato dal Box:");
    let mut input_line = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Errore nella lettura dell'input.");

    let num_bytes: usize = input_line.trim().parse().unwrap();
    // .trim() cancella tutti gli spazi bianchi e rimuove il carattere di "a capo"
    // .parse() tenta di convertire un testo in un numero in cui tipo deve essere specificato come tipo. Per questo motivo, se l'utente scrive una stringa del tipo "ciao", il parse inserisce dentro un Err la stringa, perchè si apsetta un input tipo "10" da poter convertire in un numero
    // .unwrap() estrae il risultato, e in caso positivo assegna il valore numerico alla variabile num_bytes

    // Alloca un Vec<u8> con la dimensione specificata.
    let buffer: Vec<u8> = vec![0; num_bytes];
    // Converti il Vec in un Box<[u8]> per avere un puntatore a un blocco di memoria di dimensione fissa sull'heap
    let boxed_slice: Box<[u8]> = buffer.into_boxed_slice();
    println!("Box<[u8]> allocato con successo. Dimensione: {} byte.", boxed_slice.len());
} */

// ------------------------------------------------------------------------------------------------------------------------------------------------------------------
// slide 33 Albero Binario

/* struct Node {
    value : i32,
    left : Option<Box<Node>>,
    right : Option<Box<Node>>,
}

impl Node {
    fn new(value:i32) -> Self {
        Node { valure, left : None, right : None}
    }
}

struct BinarySearchTree {
    root : Option<Box<Node>>,
}

impl BinarySearchTree {
    fn new() -> Self {
        BinarySearchTree { root : None}
    }

    fn insert(&mut self, value : i32) {
        let new_node = Box::new(Node::new(value));
        match &mut self.root {
            None => self.root = Some(new_node),
            Some(root) => root.insert_node(value),
        }
    }
    // la funzione contains verifica se un certo valore è presente nell'albero
    fn contains(&self, value: i32) -> bool {
        let mut current = &self.root;
        while let Some(node) = current { // spacchetto il contenitore di some
            match value.cmp(&node.value) {
                std::cmp::Ordering::Equal => return true,
                std::cmp::Ordering::Less => current = &node.left, // se il valore passato in input è minore, il nuovo nodo corrente da analizzare diventa il sinistro 
                std::cmp::Ordering::Greater => current = &node.right,
            }
        }
    }
    false
    }

impl Node {
    fn insert_node(&mut self, value:i32) {
        if value < self.value {
            match &mut self.left {
                None => self.left = Some(Box::new(Node::new(value))),
                Some(left) => left.insert_node(value),
            }
        } else {
            match &mut self.right {
                None => self.right = Some(Box::new(Node::new(value))),
                Some(right) => right.insert_node(value),
            }
        }
    }
}

fn inoder(node: &Option<Box<Node>>) {
    if let Some(n) =node {
        inorder(&n.left);
        print!("{} ", n.value);
        inorder(&n.right);
    }
}

fn main() {
    let mut bst = BinarySearchTree::new();

    for val in [5,3,7,1,4] {
        bst.insert(val);
    }

    inorder(&bst.root)

    println!("\nContiene 4? {}", bst.contains(4));
    println!("Contiene 9? {}", bst. contains(9));
} */

// ------------------------------------------------------------------------------------------------------------------------------------------------------------------
// slide 32
/* use crate::List::Cons;
use crate::List::Nil;

#[derive(Debug)]

enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    let list = Cons(0, Box::new(Cons(1,Box::new(Cons(2,Box::new(Cons(3, Box::new(Nil))))))));
    println!("{:?}", list);

    let a = Cons(3, Box::new(Nil));
    let b = Cons(2, Box::new(a));
    let c = Cons(2, Box::new(b));
    let head = Cons(0, Box::new(c));

    let mut current_node = &head;
    while let Cons(value, next) = current_node {
        println!("Value : {}", value);
        current_node = next;
    }
} */
// ------------------------------------------------------------------------------------------------------------------------------------------------------------------
// per la parte prima, da slide  32 a slide 44 guarda il quaderno deli appunti e le slide

// ------------------------------------------------------------------------------------------------------------------------------------------------------------------
// slide 45
/* use std::rc::Rc;
use crate::List::Nil;
use crate::List::Cons;

enum List {
    Nil,
    Cons(i32, Rc<List>),
    }

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Rc::new(Cons(3, Rc::clone(&a)));
    let c = Rc::new(Cons(4, Rc::clone(&a)));
} */

// ------------------------------------------------------------------------------------------------------------------------------------------------------------------
// slide 52 tipo Weak
/* use std::rc::Rc;

fn main() {
    let rc1 = Rc::new(5);

    let _rc2 = Rc::clone(&rc1);

    println!("strong counter : {}", Rc::strong_count(&rc1)); // 2

    let weak_rc1 = Rc::downgrade(&rc1);

    // strong counter remains 2, because a weak rc variable doesn't affect the strong counter
    println!("strong counter : {}", Rc::strong_count(&rc1)); // 

    // Provo a fare l'upgrade di weak_rc1
    let strong_rc1 = weak_rc1.upgrade(); // ottengo Option<Rc<T>>
    println!("Rc1 box to be unwrapped using match : {:?}", strong_rc1); // Some(5)
    match strong_rc1 {
        Some(value) => println!("strong_rc1 value : {}", value),
        None => println!("Data is not accesible"),
    }

    // the match clause consumes strong_rc1, therefore the strong counter remains the same
    println!("New strong counter value : {}", Rc::strong_count(&rc1));
} */

// ------------------------------------------------------------------------------------------------------------------------------------------------------------------
// slide 53 tipo Weak
/* use std::rc::Rc;

fn main() {
    let rc1 = Rc::new(5);

    let _rc2 = Rc::clone(&rc1);

    println!("strong counter : {}", Rc::strong_count(&rc1)); // 2

    let weak_rc1 = Rc::downgrade(&rc1);

    // strong counter remains 2, because a weak rc variable doesn't affect the strong counter
    println!("strong counter : {}", Rc::strong_count(&rc1)); // 

    // Provo a fare l'upgrade di weak_rc1
    let strong_rc1 = weak_rc1.upgrade(); // ottengo Option<Rc<T>>
    println!("Rc1 box to be unwrapped using match : {:?}", strong_rc1); // Some(5)
    match &strong_rc1 {
        Some(value) => println!("strong_rc1 value : {}", value),
        None => println!("Data is not accesible"),
    }

    // the reference doesn't consume the original value. In conclusion, the strong counter value reamins 3
    println!("New strong counter value : {}", Rc::strong_count(&rc1));
} */

// ------------------------------------------------------------------------------------------------------------------------------------------------------------------
// slide 55
// I will try to print the strong counter after drop(five) instruction

/* use std::rc::Rc;
fn main() {
    let rc1 = Rc::new(5);

    let weak_rc1 = Rc::downgrade(&rc1);

    drop(rc1);
    // it is not possible to print the strong value if the original rc value is dropped
    // println!("Attempt to print the strong counter value : {:?}", Rc::strong_count(&weak_rc1));

    // However it is possible to upgrade the existing weak variable, despite the strong one has been dropped
    let strong_five = weak_rc1.upgrade();

    match strong_five {
        Some(rc_strong_five) => println!("{:?}", rc_strong_five),
        None => println!("Il dato non esiste più"),

        }
    }
 */

// ------------------------------------------------------------------------------------------------------------------------------------------------------------------
// slide 55
// ricopia il codice della slide

// ------------------------------------------------------------------------------------------------------------------------------------------------------------------
// slide 61
/* use std::cell::Cell;

struct Counter {
    c1 : Cell<i32>,
}

impl Counter {
    fn new() -> Counter {
        Counter {c1 : Cell::new(0)}
    }

    // IMPORTANTE : Ricorda che in questi metodi scrivi &self perchè agiscono su un istanza della struct Counter
    fn increment(&self)  {
        let current_counter = self.c1.get(); // this instruction returns a copy of the attribute
        self.c1.set(current_counter + 1);
    }

    fn decrement(&self) {
        let  current_counter = self.c1.get();
        self.c1.set(current_counter - 1);
    }

    fn read(&self) -> i32 {
        self.c1.get()
    }
}

fn main() {
    let c1 = Counter::new();
    c1.increment();
    c1.increment();
    println!("Conteggio : {}", c1.read());
    c1.decrement();
    println!("Conteggio : {}", c1.read());
} */

// ------------------------------------------------------------------------------------------------------------------------------------------------------------------
// slide 62

/* use std::cell::Cell;

fn main() {
    let c1 = Cell::new(42);

    // I want to obtain the value inside the cell variable and substitute it with the default value -> I use the .take() method
    // IMPROTANTE : in questo caso il valore di default è lo zero, perchè la cell inizialmente contiene un i32. 
    let taken_value1 = c1.take();
    println!("The first previous value is : {}", taken_value);

    // I want to obtain the value inside the cell variable and substitute it with a value that I personally decide, e.g 32.
    let taken_value2 = c1.replace(32);
    println!("The second previous value is : {}", taken_value2);

    // I want to obtain the value inside the cell and consume the cell variable
    let retrieve_value_from_cell_consumingit = c1.into_inner();
    println!("Extracted value : {}", retrieve_value_from_cell_consumingit);

    // the cell variable has been consumed using the .into_inner() method
} */

// ------------------------------------------------------------------------------------------------------------------------------------------------------------------
// slide 63 --> scrivi il codice in un secondo momento, ma è importante


// ------------------------------------------------------------------------------------------------------------------------------------------------------------------
// slide 67
/* use std::cell::RefCell;

fn main() {
    let mut ref_c1 = RefCell::new(10);
    // I want to obtain an immutable reference to the RefCell variable
    let immutable_reference = ref_c1.borrow();
    println!("The value is : {}", immutable_reference);

    drop(immutable_reference);

    // I attempt to obtain a mutable reference from the original RefCell value
    let mutable_reference = ref_c1.get_mut();
    *mutable_reference += 1;
    println!("New value : {}", mutable_reference);
} */

// ------------------------------------------------------------------------------------------------------------------------------------------------------------------
// slide 68

/* use std::cell::RefCell;

// 2. I have to specify both the parameter type and the value contained in the parameter to have a correct compiling
fn print_refcell(rc : &RefCell<i32>) {
    // 3. I extract the value within the parameter so that I can print it.
    println!("Value : {:?}",  *rc.borrow())
}

fn main() {
    let mut rc1 = RefCell::new(5);
    // 1. I want to pass a reference to keep alive the original variable in the main function
    print_refcell(&rc1);
    {
        // 4. I don't have any other active reference, so I can call the .get_mut() method to obtain a reference OF THE VALUE WITHIN the RefCell variable; then I modify the value within the RefCell using this reference. This pointer directly address the value within the RefCell

        // IMPORTANTE : se vuoi modificare il valore dentro RefCell devi deferenziare con l'asterisco
        *rc1.get_mut() += 5;
 
        print_refcell(&rc1);

        let mut mutable_reference = rc1.borrow_mut();

        if rc1.try_borrow().is_err() {
            println!("Impossible to create another reference");
        }

        // 5. I change the value of the mutable_reference; this obviously affects the actual value within the RefCell variable
        *mutable_reference = 6;

        // 6. I drop the reference before attempting to print the original RefCell variable    
        drop(mutable_reference);

        print_refcell(&rc1); //6
    }
    if rc1.try_borrow().is_ok() {
        println!("It is possible to creare an immutable refernence");
        let immutable_reference = rc1.borrow();
        print_refcell(&rc1);
        if rc1.try_borrow_mut().is_ok() {
            println!("It is possible to make another borrow");
            let mut mutable_reference = rc1.borrow_mut();
            *mutable_reference = 12;
        }
        // 7. It is mandatory to drop every active reference to obtain the value within the refcell withouth ending up with an error
        drop(immutable_reference);
        let value = rc1.into_inner();
        println!("The value within the refcell is : {}", value);
    }
}
*/

// ------------------------------------------------------------------------------------------------------------------------------------------------------------------
// slide 69

/* use std::cell::RefCell;

fn main() {
    let numero = RefCell::new(5);

    // Tentiamo di ottenere un riferimento mutabile alla RefCell
    match numero.try_borrow_mut() {
        Ok(mut riferimento_mutabile) => {
            println!("Riferimento mutabile ottenuto con successo : {}", riferimento_mutabile);
            // Ora possiamo modificare il valore all'interno della RefCell
            *riferimento_mutabile = 10;
            // Il riferimento mutabile ottenuto non è stato mosso in una variabile con un
            // binding per assegnare il valore ad una variabile memorizzata
            println!("Riferimento mutabile modificato con successo : {}", riferimento_mutabile);
        }
        Err(_) => {
            println!("Impossibile ottenere un riferimento mutabile perché è già in uso.");
        }
    }
    
    // Numero di riferimenti in essere: 0
    // Prendiamo un riferimento in lettura e leggiamo il valore
    println!("Il valore all'interno della RefCell è ora: {}", *numero.borrow());
    // Anche qua non c'è binding, numero di riferimenti in essere: 0

    // Tentiamo di ottenere un altro riferimento mutabile
    match numero.try_borrow_mut() {
        Ok(mut riferimento_mutabile) => {
            println!("Si può fare!");
            *riferimento_mutabile = 100;
        }
        Err(_) => {
            println!("Impossibile ottenere un secondo riferimento mutabile.");
        }
    }

    println!("Il valore finale della RefCell è: {:?}", *numero.borrow());
} */

// ------------------------------------------------------------------------------------------------------------------------------------------------------------------
// slide 72-74
/* use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct Node {
    value : i32, 
    neighbor : RefCell<Weak<Node>>,
}

impl Node {
    // vogliamo che il nodo sia contenuto in un Rc perchè vogliamo che l'accesso al nodo possa essere fatto da più entità
    fn nuovo(value : i32) -> Rc<Node> {
        Rc::new(Node {
            value, 
            neighbor : RefCell::new(Weak::new()),
        })
    }

    fn imposta_vicino(nodo: &Rc<Node>, vicino: &Rc<Node>) {
        // accedo al campo neighbor, di tipo RefCell tramite riferimento mutabile, cioè utilizzo un metodo del tipo RefCell; poichè si tratta di un tipo Weak devo applicare il metodo associato downgrade al parametro "vicino", perchè vicino è di tipo Rc che deve essere convertita in Weak. 
        *nodo.neighbor.borrow_mut() = Rc::downgrade(vicino);
    }

    fn ottieni_vicino(&self) -> Option<Rc<Node>> {
        // se voglio restituire l'istanza del vicino, la promuovo ad Rc. Da Weak passo ad Rc con .upgrade()
        self.neighbor.borrow().upgrade()
    }

    fn ottieni_vicino_ver2(&self) -> Option<i32> {
        match self.neighbor.borrow().upgrade() {
            Some(nodo) => Some(nodo.value),
            None => None,
        }
    }
}

fn main() {
    let nodo_a = Node::nuovo(50);
    let nodo_b = Node::nuovo(100);

    // con il metodo .map() posso estrarre il contenuto di Option<Rc<Node>>, ovvero Node, e su questo richiamo l'attributo value
    println!("Vicino di nodo_a (iniziale): {:?}", nodo_a.ottieni_vicino().map(|n| n.value));
    println!("Vicino di nodo_b (iniziale): {:?}", nodo_b.ottieni_vicino().map(|n| n.value));

    Node::imposta_vicino(&nodo_a, &nodo_b);
    Node::imposta_vicino(&nodo_b, &nodo_a);

    println!("Vicino di nodo_a: {:?}", nodo_a.ottieni_vicino().unwrap().value);
    println!("Vicino di nodo_a: {:?}", nodo_a.ottieni_vicino().unwrap().value);
    println!("Vicino di nodo_b: {:?}", nodo_b.ottieni_vicino().unwrap().value);
} */

// ------------------------------------------------------------------------------------------------------------------------------------------------------------------



// ------------------------------------------------------------------------------------------------------------------------------------------------------------------
// slide 80-85
// guarda le slide, se hai tempo scrivi i codici




// ------------------------------------------------------------------------------------------------------------------------------------------------------------------
// slide 89
// IMPORTANTE : se voglio passare come parametro di input un Box, a differenza delle variabili normali, devo specificare la notazione self: Box<Self>. La stessa cosa vale per gli Rc, sostituendo Box con Rc

/* struct Task {
    name : String,
}
impl Task {
    // in questo caso la scritta Self dentro il Box fa riferimento a ciò che stai implementando, in questo caso la struct Task
    fn complete(self: Box<Self>) {
        println!("Task '{}' completed and dropped.", self.name);
        // self viene consumato
    }
}

fn main() {
    let task = Box::new(Task{name : String::from("Studio Rust") });
    task.complete();
} */


// ------------------------------------------------------------------------------------------------------------------------------------------------------------------
// slide 91 
// stmpa di una catena di nodi di una lista concatenata
use std::rc::Rc;

struct Node {
    value : i32,
    next : Option<Rc<Node>>,
}
        
impl Node{
    fn print_chain(self: Rc<Self>) {
        let mut current = Some(self);
        while let Some(node) = current {
            println!("{}", node.value);
            current = node.next.clone();
        }
    }
}

fn main() {
    let third = Rc::new(Node {value: 3, next: None });
    let second = Rc::new(Node {value : 2, next : Some(thid) });
    let first = Rc::new(Node {value : 1, next : Some(second) });

    first.print_chain();
}


