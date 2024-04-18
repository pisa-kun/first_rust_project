struct Point {
    x: f64,
    y: f64,
}

// Point への関連関数の定義
impl Point {
    // コンストラクタのような使われ方をする
    // 特定の型(Point)に関連した関数なので関連関数
    fn origin() -> Point {
        Point {x: 0.0, y: 0.0}
    }

    // 引数を2つ受け取る
    fn new(x: f64, y: f64) -> Point {
        Point {x: x, y: y}
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // こちらはメソッド
    // &selfは self: &selfのシンタックスシュガー
    fn area(&self) -> f64 {
        let Point {x: x1, y: y1 } = self.p1;
        let Point {x: x2, y: y2 } = self.p2;

        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // 呼び出し元のインスタンスがミュータブルであること
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

fn main() {
    let rectangle = Rectangle {
        // 関連関数はコロン2つ挟んで呼び出し
        p1: Point::origin(),
        p2: Point::new(-1.0, -1.0),
    };

    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square: Rectangle = rectangle;
    square.translate(2.0, 1.0);
    println!("Rectangle p1.x is {}, p1.y is {}", square.p1.x, square.p1.y);
    
    let color = String::from("green");
    let print = || println!("`color`: {}", color);

    // Call the closure using the borrow.
    // 借用を行ったクロージャをコールする。
    print();

    let _reborrow = &color;
    print();
}