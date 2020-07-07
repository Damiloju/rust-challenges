fn main() {
    let number = fibonacci(30);
    println!("Hello, world! {}", number);
    println!();
    twelve_days_of_xmas();
}

fn fibonacci(number: i32) -> i64 {
    if number == 0 {
        return 0;
    } else if number == 1 {
        return 1;
    }

    return fibonacci(number - 1) + fibonacci(number - 2);
}

fn twelve_days_of_xmas() {
    const LYRICS: [&str; 12] = [
        "A partridge in a pear tree",
        "Two turtle doves, and",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "10 lords a-leaping",
        "11 pipers piping",
        "12 drummers drumming",
    ];

    let mut number = 1;

    while number < 13 {
        println!("On the {} day of Christmas my true love sent to me", number);
        for number in (1..(number + 1)).rev() {
            println!("{}", LYRICS[number - 1]);
        }
        println!();
        number += 1;
    }
}
