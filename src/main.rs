fn main() {
    println!("Hello, world!");
    println!("Hello teddy!")
}

#[test]
fn hello_world() {
    println!("test hello world ")
}

#[test]
fn test_variable() {
    let name = "teddy";
    println!("hello {}", name)
}

#[test]
fn test_mutable() {
    let mut name = "teddy";
    println!("hello {}", name);

    name = "setiawan";
    println!("hello {}", name)
}

#[test]
fn test_static_typing() {
    let name = "teddy";
    println!("hello {}", name);

    // name = 10;
    println!("hello {}", name)
}

#[test]
fn test_shadowing() {
    let name = "teddy";
    println!("hello {}", name);

    let name = 10;
    println!("hello {}", name)
}

#[test]
fn test_comment() {
    // let name = "teddy";
    // println!("hello {}", name);

    /*
       test comment
       test comment
       test comment
    */

    let name = 10;
    println!("hello {}", name)
}

#[test]
fn test_explicit() {
    let age: i32 = 10;
    println!("hello {}", age)
}

#[test]
fn test_number() {
    let age: i32 = 10;
    println!("hello {}", age);

    let float = 10.5;
    println!("hello {}", float)
}

#[test]
fn test_number_convertion() {
    let a: i8 = 10;
    println!("hello {}", a);

    let b: i16 = a as i16;
    println!("hello {}", b);

    let c = b as i32;
    println!("hello {}", c);

    let d: i64 = 10000000000;
    let e: i8 = d as i8; // integer overflow
    println!("hello {}", e)
}

#[test]
fn test_numeric_operator() {
    let a = 10;
    let b = 10;
    let c = a * b;
    println!("hello {}", c)
}

#[test]
fn test_augmented_assignment() {
    let mut a = 10;
    println!("{}", a);

    a += 10;
    println!("{}", a);

    a -= 10;
    println!("{}", a)
}

#[test]
fn test_boolean() {
    let a = true;
    let b: bool = false;
    println!("{} {}", a, b)
}

#[test]
fn test_comparison() {
    let a = 10 > 20;
    let b = 20 > 10;
    println!("{} {}", a, b)
}

#[test]
fn test_boolean_operator() {
    let a = 80;
    let b = 700;

    let c = a >= 80;
    let d = b >= 800;

    let e = c || d;

    println!("{}", e)
}

#[test]
fn test_char_type() {
    let a = 'a';

    println!("{}", a)
}

#[test]
fn test_tuple() {
    let a: (char, i32, &str) = ('a', 10, "test");

    // destructure tuple
    let (x, y, _) = a;

    println!("{:?} {} {}", a, x, y)
}

fn unit() {
    println!("hello")
}

#[test]
fn test_unit() {
    let a = unit();

    println!("{:?}", a)
}

#[test]
fn test_array() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // akses array
    let b = a[0];

    // get jumlah data array
    let c = a.len();

    println!("{:?} {} {}", a, b, c)
}

#[test]
fn test_two_dimentional_array() {
    let a: [[i32; 2]; 2] = [[1, 2], [1, 2]];

    println!("{:?}", a)
}
