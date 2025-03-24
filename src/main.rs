fn main() {
    // if expression
    let number = 0;
    if number != 0 {
        println! ("i am naveed" );
    }
    else {
        println! ("i am not naveed");
    }

    // if else if expression
    let number = 7;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } 
    else if number % 3 == 0 {
        println!("number is divisible by 3");
    } 
    else if number % 2 == 0 {
        println!("number is divisible by 2");
    }
     else {
        println!("number is not divisible by 4, 3, or 2");
        let condition = false;
        let number = if condition { 5 } else { 6 };
    
        println!("The value of number is: {number}");



    }

//     let condition = true;
    
//         let number = if condition { 5 } else { "six" }; 
//         /*
//         let number = if condition { 5 } else { "six" };
//    |                                     -          ^^^^^ expected integer, found `&str`
//    |                                     |
//    |                                     expected because of this
//          */
    
//         println!("The value of number is: {number}");
    
// loop expression:
    let mut rao = 0;

    let rao_naveed = loop {
        rao += 2;
        if rao == 10 {
        break rao * 2;
        } else {
        println!("rao is not 10");
        }

    };

    println!("The value of rao_naveed is: {rao_naveed}");

    // Loop Labels
    let mut count = 0;
    'naveed: loop{
        println!("count is: {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining is: {}", remaining);
            if remaining == 9{
                break;
            }
            if count == 2{
                break 'naveed; 
            }
            remaining -= 1; 
        }

        count += 1;
    }
    println!("End of count = {count}");

    // While loop
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("ye line tab hi print ho gi jab while loop khatam ho jaye ga");

    // Looping Through a array with while
    let a = [10, 20, 30];
    let mut index = 0;

    while index < 3{
        println!("The value is a: {}", {a[index]});
        index += 1;
    }

    /*
    The value is: 10
    The value is: 20
    The value is: 30

    error:
    thread 'main' panicked at src/main.rs:94:39:
    index out of bounds: the len is 3 but the index is 3
    
     */

    // Looping Through a array with for
    let b = [10, 20, 30, 40, 50];
    for element in b{
        println!("The value is b: {}", element);
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

}

