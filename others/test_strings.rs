fn main() {
    let mut s = String::from("olá");

    s.push_str(", mundo!"); // push_str() adiciona um literal à String

    println!("{}", s); // Isso vai exibir `olá, mundo!`
}
