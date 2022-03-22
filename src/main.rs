use std::io;

fn main() {
    loop {
        println!("ENTER temperature and scale (format 212F or -100C)!");
        println!("Type 'quit' to exit!");

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed");

        let temp_scale: &str = input.trim();

        if temp_scale == "quit" {
            break;
        }
        let (temp, scale) = temp_scale.split_at(temp_scale.len() - 1);

        let temp: f64 = match temp.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        fn ftoc(temp: f64) -> f64 {
            (temp - 32.) * (5. / 9.)
        }
        fn ctof(temp: f64) -> f64 {
            (temp * 9. / 5.) + 32.
        }
        match scale.trim() {
            "c" => println!("{} \u{00B0}C = {} \u{00B0}F", temp, ctof(temp)),
            "f" => println!("{} \u{00B0}F = {} \u{00B0}C", temp, ftoc(temp)),
            _ => continue,
        }
    }
}
