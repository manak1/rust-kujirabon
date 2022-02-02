fn main() {
    let mut height;
    loop {
        println!("身長は?");
        height = input_f(0.0);
        if height > 0.0 { break; }
        println!("正しい数値を入力してください")
    }
    let weight = 22.0 * (height/100.0).powf(2.0);
    println!("標準体重は{:.1}", weight)
}

fn bmi() {
    let height_cm = input("身長(cm)は？");
    let weight = input("体重は(kg)は？");
    let height = height_cm / 100.0;
    let bmi = weight / height.powf(2.0);
    println!("BMI={:.1}", bmi);

    if bmi < 18.5 {println!("低体重")}
    else if bmi < 25.0 {println!("普通体重")}
    else if bmi < 30.0 {println!("肥満1度")}
    else if bmi < 35.0 {println!("肥満2度")}
    else if bmi <40.0 {println!("肥満3度")}
    else {println!("肥満4度")}
}

fn input(prompt: &str) -> f64 {
    println!("{}",prompt);
    let mut s  = String::new();
    std::io::stdin().read_line(&mut s).expect("入力エラー");
    return s.trim().parse().expect("数字変換エラー");
}

fn input_str()-> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("入力エラー");
    s.trim_end().to_string()
}

fn input_f(def: f64) -> f64 {
    let s = input_str();
    match s.trim().parse() {
        Ok(v) => v,
        Err(_)=> def,
    }
}