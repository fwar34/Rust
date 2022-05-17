// https://rustcc.gitbooks.io/rustprimer/content/module/module.html
pub mod a;

fn main() {
    println!("Hello, world!");
    a::b::c::d::print_ddd();
    test_reexport();
    test_use();
}

fn test_reexport() {
    a::d::print_ddd();
}

fn test_use() {
    use a::b::c::d;
    d::print_ddd();
}
