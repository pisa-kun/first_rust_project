fn main() {
    // コメントアウト、コメント文
    // println!("Hello, world!");

    /*
    ブロックコメント
     */
    println!("I'm a Rustacean!");

    let x = 5 + /* 90+ */ 5;
    println!("Is 'x' 10 or 100? x = {}", x);

    // {} はどんな引数であろうと自動的に置き換えられる
    println!("{} days", 31);

    // 名前での指定
    println!("{subject} {verb} {object}",
            object = "the lazy dog",
            subject = "the quick brown fox",
            verb = "jumps over");

    
}
