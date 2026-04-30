pub fn add(arr: &Vec<i32>) -> Option<i32> {

    if arr.len() == 0 {
        println!("You did not select any numbers!");
        return None;
    }

    let mut total = 0;
    for num in arr {
        total += *num // add the actual number not the pointer and a number
    }
    Some(total)
}

pub fn subtract(arr: &Vec<i32>) -> Option<i32> {

    if arr.len() == 0 {
        println!("You did not select any numbers!");
        return None;
    }

    let mut first_val = arr[0];
    for i in 1..arr.len() {
        first_val -= arr[i]
    }
    Some(first_val)
}


pub fn multiply(arr: &Vec<i32>) -> Option<i32>  {

    if arr.len() == 0 {
        println!("You did not select any numbers!");
        return None;
    }
    
    let mut total = 1;
    for num in arr {
        total *= *num // add the actual number not the pointer and a number
    }
    Some(total)
}

pub fn divide(arr: &Vec<i32>) -> Option<i32> {

    if arr.len() == 0 {
        println!("You did not select any numbers!");
        return None;
    }

    let mut first_val = arr[0];
    for i in 1..arr.len() {

        if check_zero(arr[i]) {
            return None;
        } else {
        first_val /= arr[i];
        }
    }
    Some(first_val) 
}

fn check_zero(num: i32) -> bool {
    if num == 0 {
        true
    } else {
        false
    }
}