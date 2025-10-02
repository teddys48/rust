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

#[test]
fn test_constant() {
    const MINIMUM: i32 = 100;
    println!("{}", MINIMUM)
}

#[test]
fn test_variable_scope() {
    let a = 1;
    {
        println!("{}", a);
        let x = 2;
        println!("{} {}", a, x)
    }

    println!("{}", a)
}

#[test]
fn test_stack_heap() {
    stack_1();
    stack_2();
}

fn stack_1() {
    let a = 10;
    let x = String::from("Test");

    println!("{} {}", a, x)
}
fn stack_2() {
    let a = 10;
    let x = String::from("Test 2");

    println!("{} {}", a, x)
}

#[test]
fn test_string() {
    let a = "  test string dong  ";
    let b = a.trim();
    println!("{}", b)
}

#[test]
fn test_string_type() {
    let mut name = String::from("Name");
    name.push_str(" Test");
    println!("{}", name);

    let a = name.replace("Name", "To");
    println!("{}", a)
}

fn full_name(first_name: &String, last_name: &String) -> String {
    format!("{} {}", first_name, last_name)
}

#[test]
fn test_reference() {
    let first_name = String::from("Teddy");
    let last_name = String::from("Setiawan");

    let name = full_name(&first_name, &last_name);

    println!("{}", name);
    println!("{}", first_name);
    println!("{}", last_name)
}

struct Person {
    name: String,
    age: i32,
}

#[test]
fn test_struct() {
    let name = String::from("Teddy");

    let a: Person = Person { name, age: 20 };

    println!("{} {}", a.name, a.age)
}

#[test]
fn ownership_rules() {
    let a = 10;

    print!("{} cek a", a);

    {
        let b = 20;
        print!("cek b {}", b)
    }

    print!("cek a {} ", a);
}

#[test]
fn data_copy() {
    let a = 10;
    let b = a;

    println!("cek a {} cek b {}", a, b);
}

#[test]
fn data_movement() {
    let a = String::from("test");
    let b = a;

    println!("cek data {}", b)
}

#[test]
fn data_clone() {
    let a = String::from("test");
    let b = a.clone();

    println!("cek data a {} cek data b {}", a, b)
}

// If Expression
#[test]
fn if_expression() {
    let value = 10;
    if value > 11 {
        println!("hello")
    } else {
        println!("world")
    }

    let result = if value >= 8 {
        "great"
    } else if value >= 6 {
        "good"
    } else if value >= 3 {
        "not bad"
    } else {
        "bad"
    };

    println!("result is {}", result)
}

// Loop
#[test]
fn loop_expression() {
    let mut counter = 0;
    loop {
        counter += 1;
        if counter >= 10 {
            break;
        } else {
            continue;
        }
    }

    let mut counter2 = 0;
    let result = loop {
        counter2 += 1;
        if counter2 >= 10 {
            break counter2 * 2
        }
    };

    println!("result is {}", counter);
    println!("result2 is {}", result)
}
