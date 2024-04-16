fn main() {
    chceck_number_pos_or_neg(1);
    chceck_number_pos_or_neg(-1);
    chceck_number_pos_or_neg(0); 

    let n = 5;
    let big_n = 
    if n < 10 && n > -10 {
        println!("{:?} is small number, increase ten-fold", n);
        //return 10 * n
        10 * n
    } else {
        println!("{:?} and is big number, halve the number", n);
        n / 2
    }; // セミコロン必要
    println!("{} -> {}", n, big_n);

    // 変数countはimmutable(不変)
    let count = 0i32;
    // error
    //count = 1;
    infloop(count);

    fizzBuzz();

    // https://doc.rust-jp.rs/rust-by-example-ja/flow_control/match.html
}

fn chceck_number_pos_or_neg(n: i32){
    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else {
        println!("{} is zero", n);
    }
}

// 関数infloopが受け取る変数countは、値のコピーを受け取る
fn infloop(mut count: i32){
    println!("Let's count until infinity!");
    loop {
        count += 1;
        if count == 3{
            println!("three");
            continue;
        }
        println!("{:?}", count);
        if count == 5{
            println!("OK, thats enough");
            break;
        }
    }
}

fn fizzBuzz(){
    println!("FizzBuzz Start");
    // let mut counter = 1;
    // while counter < 101 {
    //     if counter % 15 == 0 {
    //         println!("{:?} is FizzBuzz", counter)
    //     } else if counter % 3 == 0 {
    //         println!("{:?} is Fizz", counter)
    //     } else if counter % 5 == 0 {
    //         println!("{:?} is Buzz", counter)
    //     } else {
    //         println!("{:?}", counter)
    //     }
    //     counter += 1;
    // }
    for counter in 1..101 {
        if counter % 15 == 0 {
            println!("{:?} is FizzBuzz", counter)
        } else if counter % 3 == 0 {
            println!("{:?} is Fizz", counter)
        } else if counter % 5 == 0 {
            println!("{:?} is Buzz", counter)
        } else {
            println!("{:?}", counter)
        }
    }
    println!("FizzBuzz Finish");
}