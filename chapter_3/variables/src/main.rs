fn main() {

    //variables and mutability
    //mutability
    let mut x = 5;
    println!("value of x is: {x}");
    x = 6;
    println!("value of x is: {x}");

    //shadowing
    let x = x + 1;
    {
        let x = x * 2;
        println!("value of x is: {x}");
    }
    println!("value of x is: {x}");


    //data types
    //tup
    let tup: (u32, i32) = (13, 1);

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
    
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

        another_function(5);
        let a = five(5);
        println!("{a}");

    if x < 10 {
    println!("It's smaller");
    } else {
    println!("It has somehow gotten bigger..."); //the tup setting x to 500 made it bigger
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");


    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

}

fn another_function(x: i32) {
    println!("Another function. {x}");
}

fn five(x: i32) -> i32 {
    x + 1
}
