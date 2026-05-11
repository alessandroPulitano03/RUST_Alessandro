/* enum Shape {
    Rectangle {width, height},
    Circle { radius },
    Square { side },
}

fn calculate_area(shape : Shape) {
    match shape {
        Shape::Rectangle {width,health} => width * health,
        Shape::Circle {radius} => radius*radius * 3.14,
        Shape::Square {side} => side*side,
    }
}

fn main() {
    // definisco le istanze
    let r1 = Shape::Rectangle {width : 5, height : 2};
    let c1 = Shape::Circle {raiuds : 2.5};
    let s1 = Shape::Square {side : 10 };

    //calcolo le aree e le stampo
    println!("Area rettangolo : " {}, calculate_area(r1));
    println!("Area cerchio : {}", calculate_area(c1));
    println!("Area quadrato : {}", calculate_area(s1));
    

} */

// questo codice stampa il risultato della divisione controllando che il divisore sia diverso da zero. Si utilizza il generic enum Result<T,E>

fn divide(numerator:f64,denominator:f64) -> Result<f64,& 'static str> {
    if denominator == 0.0 {
        Err("il denominatore vale 0. Impossibile dividere per 0") // la stringa che scrivi dentro la variante Err sono stringhe scritte nella sezione dati del file eseguibile del programma, pertanto non si trovano nè nello stack nè nell'heap. Quindi dobbiamo dire al compilatore che il tempo di vita della stringa è di tutta la durata del programma. IMPORTANTE --> static serve a specificare al compilatore l'origine di quella stringa, e non rischiamo che il puntatore diventa deadling
    } else {
        Ok(numerator/denominator)
    }
}

fn main(){
    let d1 = divide(400.9, 3.5);
    let d2 = divide(400.3, 0.0);

    match d1 {
        Ok(result) => println!("Il risultato della divisione è {}", result),
        Err(err) => println!("Error : {}", err),
    }
    
}