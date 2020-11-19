#![allow(dead_code)]
#![allow(unused_variables)]
//mod sh;
use std::mem;

struct Point {
    x: f64,
    y: f64
}

struct Line {
    start: Point,
    end: Point
}

fn structures() {
    let p = Point { x: 3.0, y: 4.0 };
    println!("point p is at ({}, {})", p.x, p.y);

    let p2 = Point { x: 5.0, y: 10.0 };
    let myline = Line { start: p, end: p2 };
}

enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8,u8,u8), // tuple
    CmykColor{cyan:u8, magenta:u8, yellow: u8, black: u8}, // struct
}

fn enums() {
    let c:Color = Color::CmykColor{cyan: 0, magenta: 128, yellow: 0, black: 255};

    match c {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
        Color::RgbColor(0,0,0)
        | Color::CmykColor{cyan:_, magenta:_, yellow:_, black:255} => println!("black"),
        Color::RgbColor(r,g,b) => println!("rgb({},{},{})", r,g,b),
        _ => ()
    }
}

union IntOrFloat {
    i: i32,
    f: f32
}

fn process_value(iof: IntOrFloat)
{
    unsafe {
        match iof {
            IntOrFloat { i: 42 } => {
                println!("meaning of life value");
            }

            IntOrFloat { f } => {
                println!("value = {}", f);
            }
        }
    }
}

fn unions() {
    let mut iof = IntOrFloat { i: 123 };
    iof.i = 234;

    let value = unsafe { iof.i };
    println!("iof.i = {}", value);

    process_value(IntOrFloat{i:5});
}

fn arrays() {
    let mut a:[i32;5] = [1,2,3,4,5];

    println!("a has {} elements, first is {}",
        a.len(), a[0]);
    a[0] = 321;
    println!("a[0] = {}", a[0]);

    println!("{:?}", a);

    if a != [1,2,3,4,5] {
        println!("does not match");
    }

    let b = [1; 10];
    for i in 0..b.len() {
        println!("{}", b[i]);
    }

    println!("b took up {} bytes", mem::size_of_val(&b));

    let mtx:[[f32;3];2] =
    [
        [1.0, 0.0, 0.0],
        [0.0, 2.0, 0.0]
    ];
    println!("{:?}", mtx);

    for i in 0..mtx.len() {
        for j in 0..mtx[i].len() {
            if i == j {
                println!("mtx[{}][{}] = {}", i, j, mtx[i][j]);
            }
        }
    }
}

fn main() {
    //structures();
    //enums();
    //unions();
    arrays();
}
