fn main() {
    let _guess: i32 = "42".parse().expect("Not a number!");
    let twenty = 20;
    let twenty_one: i32 = 21;
    let twenty_two = 22i32;

    let addtion = twenty + twenty_one + twenty_two;
    println!("{}+{}+{}={}", twenty, twenty_one, twenty_two, addtion);

    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2));

    let forty_twos = [42.0, 42f32, 42.0_f32];
    println!("{:.2}", forty_twos[0]);

    for i in 1..=5 {
        println!("{}", i);
    }

    for i in 'a'..='z' {
        println!("{:}", i);
    }

    let x = '中';
    println!("字符'中'占用了{}个字节的内存",std::mem::size_of_val(&x));

    let v: u16 = 38_u8 as u16;
}
