fn main() {
    let numbers = [10, 20, 30, 40, 50];

    let slice1 = &numbers[1..3];
    let slice2 = &numbers[..3];
    let slice3 = &numbers[1..];
    let slice4 = &numbers[..];

    println!("{:?}", slice1);
    println!("{:?}", slice2);
    println!("{:?}", slice3);
    println!("{:?}", slice4);
}