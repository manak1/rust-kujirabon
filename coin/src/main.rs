fn main() {
    let price: i64 = 3950;
    let count500: i64 = 10;
    let count10: i64 = 3;
    let count50: i64 = 10;

    for i500 in 0..(count500+1) {
        for i100 in 0..(count10+1) {
            for i50 in 0..(count50+1) {
                let sum: i64 = 500 * i500 + 100 * i100 + 50 * i50;
                if sum == price {
                    println!("500円{}枚玉100円玉{}枚50円玉{}枚で{}円です。", i500, i100, i50, price);
                }
            }
        }
    }
}
