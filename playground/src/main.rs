
fn add_five(num: u32) -> u32 {
    num+5
}
fn main() {
    let mut x: u32 = 50;
    println!("Hello, world!");

    let y: u32 = add_five(x);

    x= 80;
    println!("x is {}", x);


}
