fn main() {
    let logical: bool = true;

    let a_float: f64 = 1.0; // 通常の型指定
    let an_integer = 5i32; // サフィックスによる型指定

    let default_float = 3.0; // f64
    let default_integer = 7; // i32

    // 型を文脈から推定することも可能
    let mut inferred_type = 12; // 次の行でint64
    inferred_type = 429496729616i64;

    let mut mutable = 12; // Mutable i32
    mutable = 21;

    // 型は不変 Error!
    //mutalbe = true;

    // 変数はシャドーイングによって上書きできる
    let mutable = true;
    println!("{}", mutable);

    // A tuple with a bunch of different types.
    // 様々な型を値に持つタプル
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
        -1i8, -2i16, -3i32, -4i64,
        0.1f32, 0.2f64,
        'a', true);

    // Values can be extracted from the tuple using tuple indexing.
    // インデックスを用いて、タプル内の要素を参照できる。
    println!("Long tuple first value: {}", long_tuple.0);
    println!("Long tuple first value: {}", long_tuple.5);

    // Tuples are printable.
    // タプルはプリント可能である。
    println!("tuple of tuples: {:?}", long_tuple);

    let pair = (1, true);
    println!("Pair is {:?}", pair);
    println!("The reversed pair is {:?}", reverse(pair));

    // 固定長の配列
    let xs: [i32;5] = [1,2,3,4,5];
    let ys: [i32;100] = [0;100];
    // インデックスは０から
    println!("First element of the array: {}", xs[0]);
    println!("Second element of the array: {}", xs[1]);

    println!("Number of elements in array: {}", ys.len());

    // for loop
    for i in 0..xs.len() + 1{
        match xs.get(i){
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far !",i),
        }
    }
}

// タプルを関数の引数及び返り値として使用する
fn reverse(pair: (i32, bool)) -> (bool, i32){
    let (int_param, bool_param) = pair;
    return (bool_param, int_param)
}