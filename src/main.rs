use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];
    
    let mut index: String = String::new();

    println!("Enter a number to see the content in that index");
    io::stdin()
        .read_line(&mut index)
        .expect("Didn't read that. Sorry.");

    let index: usize = index
                        .trim()
                        .parse()
                        .expect("That's not a number");

    println!("The number in index {index} is: {}", a[index])
}

// fn main() {
//     let _x: i32 = "42".parse().expect("Error");
// }

// fn main() {
//     let x: i32 = 5;
//     println!("The value of x is: {x}");

//     {
//         let y: i32 = 20 + x;
//         println!("The value of y is: {y}");
//     }

//     let x: i32 = x + 6;
//     println!("The value of x is: {x}");
// }