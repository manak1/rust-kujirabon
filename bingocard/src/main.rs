use rand::seq::SliceRandom;

fn main() {
    let a_vec : Vec<u32> = vec![100,200,300];
    for i in a_vec {
        println!("{}",i);
    }

    let b_vec: Vec<&str> = vec!["A","B","C"];

    for i in b_vec {
        println!("{}",i);
    }

    bingo_list();
}

fn bingo_list() {
    // 1~75までの値を代入
    let mut nums = [0;75];

    for i in 1..=75 {
        nums[i-1] = i;
    }
    let mut rng = rand::thread_rng();
    nums.shuffle(&mut rng);

    for y in 0..5 {
        for x in 0..5 {
            let i = y * 5 + x;
            if i == 12 {
                print!("　*,")
            }else {
                print!("{:3},", nums[i])
            }
        }
        println!("");
    }
}
