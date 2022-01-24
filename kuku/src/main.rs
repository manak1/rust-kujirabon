fn main() {
    kuku();
    println!("--------------------------");
    kuku_formated();
    println!("--------------------------");
    gengou_list();
}

fn kuku() {
    for y in 1..10 {
        for x in 1..10 {
            print!("{:3}", x * y)
        }
        println!("");
    }
}

fn kuku_formated() {
    for y in 1..10 {
        let s = (1..10).map(|x| format!("{:3}", x * y)).collect::<Vec<String>>().join(",");
        println!("{}", s);
    }
}

fn gengou_list() {
    for i in 1926..2027 {
        print!("西暦{}年 = ",i);
        if i <= 1988 {
            if i == 1926 { println!("昭和元年"); }
            else {
                println!("昭和{}年", i-1926+1);
            }
        }
        else if i <= 2018 {
            if i == 1989 { println!("平成元年"); }
            else {
                println!("平成{}年", i-1989+1);
            }
        }
        else {
            if i == 2019 { println!("令和元年"); }
            else {
                println!("令和{}年", i-2019+1);
            }
        }
    }
}
