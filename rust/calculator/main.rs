use std::{io::{self, Write}, vec};
mod math;

enum Choice {
    Add,
    Subtract,
    Multiply,
    Divide,
    Other,
}

impl Choice {
    fn calculate(&self, arr: &Vec<i32>) -> Option<i32> {
        match self {
            Choice::Add => math::add(arr),
            Choice::Subtract => math::subtract(arr),
            Choice::Multiply => math::multiply(arr),
            Choice::Divide => math::divide(arr),
            Choice::Other => {
                println!("Please pick a valid option!");
                None  // still returns Option<i32>
            }
        }   
    }
}

fn main() {
    loop {
        println!("1 - Add, 2 - Subtract, 3 - Multiply, 4 - Divide");
        print!("Type an answer: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        println!("you typed {}", choice);
        
        let choice = match choice.trim() {
            "1" => Choice::Add,
            "2" => Choice::Subtract,
            "3" => Choice::Multiply,
            "4" => Choice::Divide,
            _   => Choice::Other,
        };

        let operation = match choice {
        Choice::Add => "add",
        Choice::Subtract => "subtract",
        Choice::Multiply => "multiply",
        Choice::Divide => "divide",
        Choice::Other => "unknown",
        };

        let mut arr = vec![];
        loop {
            println!("1 - Include numbers, 2 - Calculate");
            print!("Type an answer: ");
            io::stdout().flush().unwrap();
            let mut answer = String::new();
            io::stdin().read_line(&mut answer).unwrap();
            
            if answer.trim() == String::from("1") {
                print!("Pick numbers to {}: ", operation);
                io::stdout().flush().unwrap();
                let mut num = String::new();
                io::stdin().read_line(&mut num).unwrap();
                let num: i32 = num.trim().parse().unwrap();
                arr.push(num)   
            } else if answer.trim() == String::from("2") {
                let result = choice.calculate(&arr);
                println!("\nHere is the result of your calculation: {:?}", result);
                break
            } else {
                println!("Please select a valid option")
            }
        }

    }
}