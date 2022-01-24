fn main() {
    let price = 3950;
    for i500 in 0..11 {
        for i100 in 0..4 {
            for i50 in 0..11 {
                let sum = 500 * i500 + 100 * i100 + 50 * i50;
                if sum == price {
                    println!("500円{}枚玉100円玉{}枚50円玉{}枚で{}円です。", i500, i100, i50, price);
                }
            }
        }
    }
}
