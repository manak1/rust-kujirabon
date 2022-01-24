fn main() {
    kuku();
}

fn kuku() {
    for y in 1..10 {
        for x in 1..10 {
            print!("{:3}", x * y)
        }
        println!("");
    }
}
