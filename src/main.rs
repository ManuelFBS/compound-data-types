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
fn car_quality (miles: u32) -> (Age, u32) {
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
fn car_factory (color: String, motor: Transmission, roof: bool, miles: u32) -> Car {
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