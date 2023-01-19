fn main() {
    let string = String::from("hello, world!");
    let first_word = first_word(&string);
    
    println!("{first_word}");
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