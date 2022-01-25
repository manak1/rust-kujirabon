fn main() {
    let enc = encrypt("HELLO RUST",3);
    let dec = encrypt(&enc, -3);
    println!("{} => {}", enc, dec);

    let double     = |n: i64| n * 2;
    println!("{}",double(2));


}

fn encrypt(text: &str, shift: i16) -> String {
    let code_a = 'A' as i16;
    let mut result = String::new();

    let is_az = |c| (c >= 'A' && c <= 'Z');
    let enc = |c: i16| (((c -code_a + shift + 26) % 26 + code_a) as u8) as char;
    let cov = |c: char| if is_az(c) { enc(c as i16)} else { c };
    return text.chars().map(|c| cov(c)).collect();
}
fn kakezan(num1: i64, num2: i64) -> i64 {
    return num1 * num2;
}