// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand for a hint.

fn main() {
    let a = [3,3,4,5,6,7,7,5,4,3,3,2,2,2,2,2,2,2,2,2,2,2,2,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,1, 3, 4, 5, 6, 7,4,2,5,6,2,5,5,2,1,1,4,7,6,4,2,6,2,2,6,6,7,8,8,2,2,4,4,5,6,7,7,8,8,6,6,45,4,4,4,2,3,4,5,6,7,8,6,4,4,4,4,4,4,4,4,4,4,4,4,4];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
