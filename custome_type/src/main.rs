// An attribute to hide warnings for unused code.
// 使用されていないコードよる警告を隠すアトリビュート
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

struct Pair(i32, f32);

struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    let name = String::from("haruki");
    let age = 16;
    let haruki = Person{name, age};
    println!("{:?}", haruki);

    let point: Point = Point{x: 10.3, y:0.4};
    println!("point coordinates: ({}, {})", point.x, point.y);
    
    let bottom_right = Point {x: 5.2, ..point};
    let _rectangle = Rectangle{
        top_left: point,
        bottom_right: bottom_right,
    };
    println!("rectangle area is {}", rect_area(_rectangle));

    let n = 16;
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) {"big"} else {"small"});
    let n = 8;
    println!("{} is {}", n, if is_big(n) {"big"} else {"small"});
}

// グローバルスコープを含む任意のスコープで宣言
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool{
    return n > THRESHOLD;
}

fn rect_area(rect: Rectangle) -> f32{
    let width = rect.bottom_right.x - rect.top_left.x;
    let height = rect.top_left.y - rect.bottom_right.y;
    return width * height;
}
