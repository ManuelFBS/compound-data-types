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

