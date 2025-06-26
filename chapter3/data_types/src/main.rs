fn main() {
    let t = ([1; 2], [3; 4]);
    let (a, _b) = t;
    println!("{}", a[0] + t.1[0]);
}
