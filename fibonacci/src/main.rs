fn main() {
   let pc = 98000.0;
   let a_rate = 0.8;
   let b_rate = 0.9;
   let ship_price = 1200.0;

   let a_price = pc * a_rate + ship_price;
   let b_price = pc * b_rate;

    println!("A社の値段は{}円です。", a_price);
    println!("B社の値段は{}円です。", b_price);

}

/* fn fibonacci() {
    let mut a = 1;
    let mut b = 1;
    println!("{}", a);
    println!("{}", b);
    for _ in 0..30 {
        println!("{}", a + b);
        let tmp = a;
        a = b;
        b = tmp+a;
    }
} */
