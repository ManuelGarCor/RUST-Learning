fn main() {
    println!("Hola Mundo!");
    println!("\t Se llama a la primera función.");
    funcion_primera(5);
    println!("\t Se llama a la segunda función.");
    funcion_segunda();
}

fn funcion_primera(x: i32) {
    println!("\t\tThe value of x is: {x}");
}

fn funcion_segunda () {
    println!("\t\tDentro de la segunda funcion.");
}