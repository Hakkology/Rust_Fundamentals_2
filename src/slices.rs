fn main(){

    let numbers = [1, 2, 3, 4, 5];

    // Create a slice containing elements from index 1 to 3 (inclusive start, exclusive end)
    let slice = &numbers[1..4];

    println!("Slice: {:?}", slice);

    let vec = vec![10, 20, 30, 40, 50];

    // Borrow a slice from the vector
    let part_of_vec = &vec[1..3];

    println!("Part of the vector: {:?}", part_of_vec);
}


fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}