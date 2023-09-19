use std::{string, io::Read};

#[derive(Debug)]
struct Car{
    brand : String, 
    model: String, 
    year: usize
    
}
///Will clear the console by printing control characters to the console. Likely cross platform - however only tested in bash terminal. 
fn clear(){
    //Some black magic shii to clear the console
    print!("\x1B[2J\x1B[1;1H");
}
///A function to take input from stdin.
///A message is provided that will be used to prompt the terminal what to input. 
fn std_input(msg : &str)->String{
    use std::io::{self, Write};
    let mut input:String = String::new();

    print!("{msg}"); 
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut input).expect("Unable to read input"); 

    //Implicitly returns the string from stdin trimmed. 
    input.trim().to_string()
}
///Awaits user input - a means to stay inside a loop iteration. 
fn await_continue(){
    use std::io::{self};
    println!("Press [enter] to continue...\r");
    let mut input:String = String::new();
    io::stdin().read_line(&mut input);

}

fn main() {
    
   

    
    let mut car_vector = vec![
    Car{ brand: String::from("BMW"), model: String::from("530"), year: 2003}, 
    Car{brand : String::from("Audi"), model: String::from("A6 Avant"), year:2022},
    Car{brand: String::from("Volkswagen"), model: String::from("Shirroco R 2.0"), year: 2016}
    ];

    //Main event loop to stay inside the OOP console program. 
    loop{
        clear();
        println!("What do you want to do?");
        println!("C: Create a car\nR: Read all cars in digestable list\nS: Read contents of a specific car\nU: Update a car\nD: Delete a car\nQ: Quit");

        let input = std_input("Action: ").chars().nth(0).unwrap();

        match input{
            'Q'|'q' => {
                println!("The program will quit, Thanks for using my program.");
                return await_continue();
            }
            'C'|'c' => CreateCar(&mut car_vector),
            'R'|'r' => ReadAllCars(&car_vector),
            'S'|'s' => getSpecific(&car_vector),
            'U'|'u' => UpdateCar(&mut car_vector),
            'D'|'d' => DeleteCar( &mut car_vector),
            
            _=>println!("Invalid Input")
        }
        await_continue();
    }


}
//gets input from the console while trimming excess characters (pesky '\r')

//CRUD CRUD CRUD CRUD

fn CreateCar(car_vector: &mut Vec<Car>){
    clear();

    println!("\nWhat brand is your car from?");
    
    let brand = std_input("Brand: ");

    println!("\nWhat model is your car?");
    
    let model = std_input("Model: ");


    println!("\nWhat year was it made? ");

    let year = std_input("Year: ").parse::<usize>().unwrap();
    let new_car = Car{
        brand, 
        model, year
    };

    //Push takes ownership of new_car. 
    car_vector.push(new_car);


    clear();
    println!("Cars in the car vector\n");
    ReadAllCars(car_vector);    

}
fn ReadAllCars(car_vector: &Vec<Car>){
    
    
    for (i, car) in car_vector.iter().enumerate(){
        println!("{i}: {} {}",car.brand, car.model );
    }
}
fn getSpecific(car_vector: &Vec<Car>){
    clear();
    println!("Input the index of the car you want");
    ReadAllCars(&car_vector);

    let index = std_input("Index: ").parse::<usize>().unwrap();
    println!("{:?}", car_vector.get(index).unwrap());

}
fn UpdateCar(car_vector: &mut Vec<Car>){

    clear();
    println!("What car would you like to update? \n");
    ReadAllCars(car_vector);

    
    let index = std_input("Index: ").parse::<u32>().unwrap();

    let car : &mut Car = &mut car_vector[index as usize];
    println!("\n\nWhat attributes do you wish to edit?\n\n");
    println!("b: Brand\nm: Model\ny: Year");

   let attr = std_input("Attribute: ").chars().nth(0).unwrap();

   match attr {
       'b'|'B'=>{
            println!("Edit attribute - brand");
            car.brand = std_input("Brand: ");
       },
       'm'|'M'=>{
            println!("Edit attribute - model");
            car.model = std_input("Model: ");

       },
       'y'|'Y'=>{
            println!("Edit attribute - year (input a number)");
            car.year = std_input("Year: ").parse::<usize>().unwrap();
       },
       _=>{
            println!("Invalid attribute - returning");
       }

   };
   
    




}
fn DeleteCar(car_vector: &mut Vec<Car>){

    clear();
    println!("What Car would you like to remove?\n===============\n");
    ReadAllCars(car_vector);

    let inp = std_input("Index for deletion: ");
    let index = inp.parse::<u32>().unwrap();

    car_vector.remove(index as usize);


}
