use std::io;
fn main() {
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    println!("🟡 Select a month: ");

    let mut index = String::new();

    io::stdin().read_line(&mut index).expect("🔴 Failed to Read Line");

    let index: usize = index.trim().parse().expect("🔴 Index is NaN");

    let month = months[index - 1];

    println!("🟢 {index} month = {month}");
}
