// option1.rs
// Make me compile! Execute `rustlings hint option1` for hints



// you can modify anything EXCEPT for this function's sig
fn print_number(maybe_number: Option<u16>) {
    println!("printing: {}", maybe_number.unwrap());
}

fn main() {
    print_number(Some(13));
    print_number(Some(99));

    let mut numbers: Vec<Option<u16>>=Vec::new();
    for iter in 0..5 {
        let number_to_add: Option<u16> = {
            Some(((iter * 5) + 2) / (4 * 16))
        };

        numbers.push(number_to_add);
    }
}
