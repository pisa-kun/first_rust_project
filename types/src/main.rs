// オーバーフローを起こすようなキャストによる警告を無視する。
#![allow(overflowing_literals)]
// 警告を抑えるアトリビュートを使用。
#[allow(non_camel_case_types)]

fn main() {
    let decimal = 65.4321_f32;

    // 暗黙的な型変換はできない
    // let integer: u8 = decimal;
    let integer: u8 = decimal as u8;
    let character = integer as char;
    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // 型推論
    let elem = 5u8;

    // 空のベクトルを生成
    let mut vec = Vec::new();
    // vecの型がu8になる。コメントアウトするとエラーになる。
    vec.push(elem);
    println!("{:?}",vec);

    type Inch = u64;
    let inches: Inch = 2;
    println!("{:?}",inches);

    #[derive(Debug)]
    struct Number {
        value: i32,
    }

    // FromとInto
    impl From<i32> for Number {
        fn from(item: i32) -> Self{
            Number {value: item}
        }
    }
    let num = Number::from(30);
    println!("My Number is {:?}", num);

    let int = 5;
    // 型指定がないとエラーになる
    let num_2: Number = int.into();
    println!("My Number is {:?}", num_2);
}
