fn main() {
    let s = String::from("Ankit Singh Lingwal");
    let res = calculat_length(&s);

    // s.clear() this line will cause a compile-time error because the res is a reference to the original string s and it is still being used after the function call. Therefore, we cannot clear the original string while the res is still in use.

    println!("the first space is at index: {}", res.len());
}

fn calculat_length(s: &String) -> &str {
    let input = s.as_bytes();

    for (i, &item) in input.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    return &s[..];
}

// here the res borrows the value of the first space index from the original string s. The function calculat_length takes a reference to the string s and returns a slice of the string up to the first space index. This allows us to access a portion of the original string without taking ownership of it, and it ensures that the original string remains valid and accessible after the function call.

// also i just can do s.clear after the function call even if the s is mutable because the res is a reference to the original string and not a copy of it. Therefore, even if the original string is cleared, the res will still hold the value of the first space index because it is a reference to the original string.

// res and s are related to one another
