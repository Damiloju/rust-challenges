use std::collections::HashMap; // import the hashmap from the standard library
use std::io; // import the io library from the standard library

fn main() {
    println!("Welcome to employee list. Type a sentence in the format 'Add [employer] to [department]' to add employee");
    println!();

    let mut employee_registry: HashMap<String, String> = HashMap::new();

    loop {
        let mut user_word = String::new(); // define a mutable variable using the mut keyword
        io::stdin()
            .read_line(&mut user_word) // pass a reference to the guess variable using & and make it mutable because referenc are immutable by default
            .expect("Failed to read line");
        let user_word: String = match user_word.trim().parse() {
            Ok(word) => word,
            Err(_) => break,
        };

        let mut v: Vec<&str> = user_word.split(' ').collect();

        if v.len() == 4 {
            let employee: String = String::from(v.remove(1).to_lowercase());
            let department: String = String::from(v.remove(2).to_lowercase());
            employee_registry.entry(employee).or_insert(department);
            println!("{:?}", employee_registry);
        } else {
            println!(
                "Sentence should be in the format 'Add [employer] to [department]' to add employee"
            );
            println!();
        }
    }

    let mut v: Vec<_> = employee_registry.into_iter().collect();
    v.sort_by(|x, y| y.1.cmp(&x.1));
    v.reverse();

    println!("Department ----- Employee");
    for string in &v {
        println!("{} -----  {}", string.1, string.0);
    }
}
