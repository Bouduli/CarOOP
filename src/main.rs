//TODO: Make Create functions quittable.




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

        
        let input = match std_input("Action: ").chars().nth(0){
            Some(x)=>x,
            None=>{
                println!("Action was empty - please enter an action");
                await_continue(); 
                continue
            }
        };

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
            
            _=>println!("Invalid action")
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
    match brand.as_str() {
        "Q"|"q"=>return println!("Return to menu"),
        _=>()
    };

    println!("\nWhat model is your car?");
    
    let model= std_input("Model: ");
    match model.as_str() {
        "Q"|"q"=>return println!("Return to menu"),
        _=>()
    };


    let year = loop{
        println!("\nWhat year was it made? ");
        let input = std_input("Year: ");
        match input.as_str(){
            "Q"|"q"=>return println!("Return to menu"),
            _=>()
        };

        match input.parse::<usize>(){
            Ok(x)=>{break x},
            Err(x)=>{
                println!("error: {x}");
                await_continue();
                
    
            }
        };
    }; 
    

    

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

    let input = std_input("Index: ");
    match input.as_str(){
        "Q"|"q" => return println!("Returning to main menu"),
        _ => ()
    };
    //Match statement over returned Result from parse will make sure that a index is properly chosen.
    let index = match input.parse::<usize>(){
        Ok(x)=>x,
        Err(x)=>return {
            println!("error: {x}");
            await_continue();
            getSpecific(car_vector);

        }
    };
    //Match statement over returned Option from Vec::get() to make sure that the car can be found. 
    match car_vector.get(index){
        Some(x)=>println!("{:?}",x),

        //In the case that no car is found - a message is printed and function will return
        None=>{
            println!("No cars found at index: {index}");
        }
    }

}
fn UpdateCar(car_vector: &mut Vec<Car>){    

    
    let index = loop{
        println!("What car would you like to update? \n");
        ReadAllCars(car_vector);
        let input = std_input("Index: ");
        match input.as_str(){
            "Q"|"q" => return println!("Returning to main menu"),
            _ => ()
        };


        match input.parse::<u32>(){
            Ok(x)=>{break x},
            Err(x)=>{
                println!("error: {x}");
                await_continue();
            }
        };
    }; 





    let car : &mut Car = &mut car_vector[index as usize];
    

    loop{
        println!("\n\nWhat attributes do you wish to edit?\n(Ommitting output will keep original value)\n");
        println!("b: Brand\nm: Model\ny: Year");    

        //Match over attribute to edit
        match std_input("Attribute: ").chars().nth(0){
            //If the option is q/Q then the user probably wants to quit. 
            Some('q'|'Q')=>return println!("Returning to main menu"),

            //All other arms lead to editing an attribute, or doing nothing since invalid input will run the loop again
            Some('b'|'B')=>{
                println!("Edit attribute - brand");
                //If someone inputs 'q' or 'Q' as either brand or model or year - then they most likely want to quit. therefore those cases are handled 
                car.brand = match std_input("Brand: ").as_str(){
                    "q"|"Q"=>return println!("Returning to main menu"),
                    ""=>car.brand.clone(),
                    x=>x.to_string()
                };
                break
            },
            Some('m'|'M')=>{
                println!("Edit attribute - model");
                car.model = match std_input("Model: ").as_str(){
                    "q"|"Q"=>return println!("Returning to main menu"),
                    ""=>car.model.clone(),
                    x=>x.to_string()
                };
                break

               },
            Some('y'|'Y')=>{
                println!("Edit attribute - year (input a number)");
                let input= match std_input("Year: ").as_str(){
                    "q"|"Q"=>return println!("Returning to main menu"),
                    x=>x.to_string()
                };
                car.year = match input.parse::<usize>(){

                    Ok(x)=>x,
                    Err(_)=>car.year

                };
                break
                },
            None | Some(_)=>{
                println!("Invalid Attribute - ");
                }

        };
    };
   
    




}
fn DeleteCar(car_vector: &mut Vec<Car>){

    let index = loop{
        println!("What Car would you like to remove?\n");
        let input = std_input("Index for deletion: ");
        match input.as_str(){
            "Q"|"q" => return println!("Returning to main menu"),
            _ => ()
        };


        match input.parse::<u32>(){
            Ok(x)=>{break x},
            Err(x)=>{
                println!("error: {x}");
                await_continue();
            }
        };
    }; 
    ReadAllCars(car_vector);

    car_vector.remove(index as usize);


}
