fn main() {
    //ifs
    let number = 3;

    if number < 5 { //works like in c, condition MUST be a bool, rust got no type conversion
        println!("condition was true");
    } else { 
        println!("condition was false");
    }

    //multiple conditions, still works like c
    let number = 6;

    if number % 4 == 0 { //btw match is more recommended
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 }; //can be one line, nice, all vals must be of same type
                                                     //(no Python >:))
    println!("The value of number is: {number}");

    loop { //just to remind, works infineleky until ^C or quit or break
        println!("again!");
        break;
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; //this returns value after break
        }
    };
    println!("The result is {result}");

    let mut count = 0;
    'counting_up: loop { //labeling ("namimg") loop
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up; //break can be applied to loop by its name, single quote before name
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;
    while number != 0 { //while as in c
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    for element in a { //while used in this situation could cause panic and would be slower
        println!("the value is: {element}");
    }

    for number in (1..4).rev() { //using for with range
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}