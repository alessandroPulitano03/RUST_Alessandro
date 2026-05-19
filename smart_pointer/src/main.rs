
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
use std::rc::Rc;
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
}