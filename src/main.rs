#[derive(PartialEq, Debug)]
// Se declara una struct Car que describe un vehículo con 4 campos...
struct Car {
    color: String,
    motor: Transmission,
    roof: bool,
    age: (Age, u32)
}

#[derive(PartialEq, Debug)]
// Se declara un enum para el tipo de transmisión del Car...
enum Transmission {
    Manual,
    SemiAuto,
    Automatic
}

#[derive(PartialEq, Debug)]
// Se declara un enum Age para Car age...
enum Age {
    New,
    Used
}

// Obtenemos la calidad del auto probando el valor del 
// argumento de entrada...
// - millas (u32)...
// Se crea una tupla para la calidad del auto con la Edad
// ("Nuevo" o "Usado") y el kilometraje...
// Devuelve una tupla con la sintaxis de flecha `->`...
fn car_quality(miles: u32) -> (Age, u32) {
  // Declarar e inicializar el valor de tupla de retorno
  // Para un auto nuevo, establezca las millas en 0
  // - Establece el valor a un auto "New"
  // - Establece el mileage usando el argumento de entrada "miles"
    let quality = (Age::New, miles); 

  // Devuelve tupla, no es necesaria la palabra clave "retorno"
  // ni punto y coma
    quality
}

// Construye un nuevo "Car" usando los valores de cuatro argumentos 
// de entrada:
// - color (String)
// - motor (enum Transmisión)
// - roof (bool, verdadero si el auto tiene techo rígido)
// - miles (u32)
// Llame a la función car_quality(millas) para obtener la antigüedad 
// del automóvil...
// Devuelve una instancia de una estructura "Car" con la sintaxis de
// flecha `->`
fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {
  // Crea una nueva instancia de "Car" según lo solicitado...
  // - Vincula los primeros tres campos a los valores de los 
  // argumentos de entrada...
  // - el campo "age" llama a la función "car_quality" con el
  // argumento de entrada "mileage"..
    Car {
        color: color,
        motor: motor,
        roof: roof,
        age: car_quality(miles)
    }
}

fn main() {
  // Crea una matriz de colores para el auto
  // Código corregido: 0 = Blue, 1 = Green, 2 = Red, 3 = Silver
    let colors = ["Blue", "Green", "Red", "Silver"];

  // Declarar el tipo de auto y los valores iniciales...
  // Código corregido: Declarar "car" como estructura mutable "Car"...
  // Código corregido: Declarar "engine" como enumeración mutable 
  // de "Transmission", inicializar en "Manual"...
    let mut car: Car;
    let mut engine = Transmission::Manual;

  ///////////////////////////////////////////////////////////////////////////
        
  // Pida 3 autos, un auto para cada tipo de transmisión
  // Código corregido: indexa en la matriz `colors` y varía el color 
  // de los pedidos...
    
  // Orden de auto #1: New, Manual, Hard Top...
    car = car_factory(String::from(colors[0]), engine, true, 0);
    println!("Car order 1: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.roof, car.motor, car.color, car.age.1);
  
  // Orden de auto #2: Used, SemiAuto, Convertible...
    engine = Transmission::SemiAuto;
    car = car_factory(String::from(colors[1]), engine, false, 100);
    println!("Car order 2: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.roof, car.motor, car.color, car.age.1);
  
  // Orden de auto #3: Used, Automatic, Hard Top...
    engine = Transmission::Automatic;
    car = car_factory(String::from(colors[2]), engine, true, 200);
    println!("Car order 3: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.roof, car.motor, car.color, car.age.1);
}