
fn first_word(s: &str) -> &str {
    for (i, &v) in s.as_bytes().iter().enumerate() {
        if v == b' ' {
            return &s[..i];
        }
    }
    return &s[..];
}

fn main() {
    let my_string = String::from("sss asdf sdfd");
    let my_str = "sdfsld alksdjflka a";
    let _word = first_word(&my_string[..5]);
    let _word = first_word(&my_string[..]);
    let _word = first_word(&my_string);

    let _word = first_word(&my_str[..5]);
    let _word = first_word(&my_str[..]);
    let _word = first_word(my_str);
}
