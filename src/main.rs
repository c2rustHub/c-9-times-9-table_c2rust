fn main() {
    println!("print 9*9 multiplication table");

    for i in 1..=9 {
        for j in 1..=i {
            print!("{}*{}={}\t", j, i, i * j);
        }
        println!("");
    }
}