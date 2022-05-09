fn add(i:i32,j:i32) -> i32 {
    i+j
}
fn demo() {
    let a = 10;
    let b: i32 = 20;
    let mut c = 30i32;
    c += 40_i32;
    let d = 30_i32;
    let e = add(add(a, b),add(c, d));
    println!("(a+b) + (c+d)={}",e)
}

fn main() {
    demo();
    let penguin_data = "\
    common name length (cm)
    Little penguin,33
    Yellow-eyed penguin,65
    Fiordland penguin,60
    Invalid,data
    ";
    let records = penguin_data.lines();
    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        let fields: Vec<_> = record
        .split(',')
        .map(|field| field.trim())
        .collect();

        if cfg!(debug_assertions) {
            eprintln!("debug: {:?} -> {:?}",record,fields);
        }
        let name = fields[0];
        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{}, {}cm", name, length);
        }
    }
}