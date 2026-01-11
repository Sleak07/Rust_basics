pub fn tups() {
    // tuples are used to group variety of types into one compound type
    // they have a fixed size and cannot grow or shrink
    let data = (10, 5.7, "Hello");
    println!("{:?}", data)
}

pub fn array() {
    // Arrays have a fixed length in rust
    // They should contain the same element type
    let numbers: [u8; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", numbers[4])
}
