mod atoms;

fn main() {
    println!("{:?}", atoms::Atom::from_p_weight(String::from("Lv"), 57, 138.9).electron_shell().shell_to_string());
}
