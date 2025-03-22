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

   





}

