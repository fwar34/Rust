//http://rustdoc.saigao.fun/ch04-03-slices.html
fn main() {
    let str = String::from("hello world");
    let slice = first_word(&str);
    println!("string = {}", slice);

    // first_word works on slices of `String`s
    let _word = first_word2(&str[..]);

    let str_literal = "hello world";
    // first_word works on slices of string literals
    let _word = first_word2(&str_literal[..]);

    // since string literals *are* string slices already,
    // this works too, without the slice syntax!
    let _word = first_word2(str_literal);

    let a = [1, 2, 3, 4];
    let slice = &a[1..3];
    println!("slice = {:?}", slice);
}

// only work on reference of String
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn first_word2(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
