use std::io;

fn main() {
    loop {
        println!("enter a temperature");
        let mut temp = String::new();
        io::stdin()
            .read_line(&mut temp)
            .expect("failed to read line");
        let f: f32 = match temp.trim().parse() {
            Ok(f) => f,
            Err(_) => {
                println!("not a valid temperature");
                continue;
            }
        };
        let c = convert_f_to_c(f);
        println!("{} F = {} C", f, c)
    }
}
fn convert_f_to_c(f: f32) -> f32 {
    return (f - 32.0) * (5.0 / 9.0);
}
