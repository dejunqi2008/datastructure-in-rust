use std::thread;
use rand::Rng;
use std::io;

// demos code for Rust Basics

fn arry_demo() {
    let index = "4".parse::<usize>().unwrap();
    let myarray: [u32; 5] = [1, 2, 3, 4, 5];
    println!("{}, {}", myarray[2], myarray[index]);

    let mybuffer: [u32; 32 * 1024] = [1; 32 * 1024];
    println!("{}", mybuffer[0]);
}

fn slice_demo() {
    let arr: [u32; 5] = [1, 2, 3, 4, 5];
    let len = arr.len();
    let slice = &arr[len -2 .. len];
    println!("{} {} {}", slice[0], slice[1], slice.len());
}

fn struct_demo() {
    // 元组结构
    struct Pair(i32, f32);

    let pair = Pair(10, 4.2);
    println!("{}", pair.0);

    // C 结构
    struct Person {
        name: String,
        age: u32,
    }

    let jack = Person {
        name: String::from("Jack"),
        age: 6
    };

    println!("{} {}", jack.name, jack.age);
}

fn enum_demo() {
    enum IPAddr {
        IPv4(u8, u8, u8, u8),
        IPv6(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)
    }

    let localhost: IPAddr = IPAddr::IPv4(127, 0, 0, 1);

    match localhost {
        IPAddr::IPv4(a, b, c, d) => {
            println!("{}, {}, {}, {}", a, b, c, d);
        }
        _ => {}
    }
}

fn loop_demo() {
    // 1 + 2 + ... + 100
    let mut sum = 0;
    let mut n = 1;
    loop {
        sum += n;
        n += 1;
        if n > 100 {
            break
        }
    }

    println!("{:?}", sum);

}

fn loop_demo2() {
    let mut counter = 0;

    let res = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("{}, {}", counter, res);
}

fn while_demo() {
    let mut n = 1;

    while n < 101 {
        if n % 15 == 0 {
            println!("FizzBuzz");
        } else if n % 3 == 0 {
            println!("Fizz")
        } else if n % 5 == 0 {
            println!("Buzz")
        } else {
            println!("{}", n)
        }

        n += 1;
    }
}

fn forrange_demo() {
    for i in 0..5 {
        print!("{}, ", i);
    }
    println!("\n");

    for i in 0..=5 {
        print!("{}, ", i);
    }
    println!("\n");

    let arr = ["a", "b", "c"];
    for i in arr.iter() {
        println!("{}", i);
    }

    // to alter element in array
    let mut arr2 = [1, 2, 3];
    for i in arr2.iter_mut() {
        *i *= 2;
    }

    for i in arr2.iter() {
        println!("{}", i)
    }
    
}


fn iflet_demo() {
    enum Alphabet {
        A, B, C
    }

    let letter = Alphabet::A;

    // version 1
    match letter {
        Alphabet::A => {
            println!("It is A")
        }
        _ => {
            println!("It is not A")
        }
    }

    // version 2
    if let Alphabet::A = letter {
        println!("it is A")
    }

}


fn fibonacci(n: u64) -> u64 {
    if n < 2 {
        return n;
    }

    let mut a: u64 = 0;
    let mut b: u64 = 1;
    let mut k = n - 2;
    let mut c = 0;
    while k > 0 {
        c = a + b;
        a = b;
        b = c;
        k -= 1;
    }

    println!("fibonacci({}) is {}", n, b);

    return b;
}


/** methods of struct demo */
struct Point {
    x: u64,
    y: u64
}

impl Point {
    fn new(x: u64, y: u64) -> Point {
        Point { x, y }
    }

    fn getX(&self) -> u64 {
        return self.x
    }

    fn setX(&mut self, x: u64) {
        self.x = x;
    }
}

fn method_demo() {
    let mut p = Point::new(10, 20);
    println!("{}, {}", p.x, p.y);

    p.setX(20);
    println!("{}, {}", p.x, p.y);
}

/** method demo --- end */


fn closure_demo() {
    let myclosure = |n: u64| -> u64 { return n * 2 };
    println!("{}", myclosure(10));

    // move
    let hello_message = "Hello World!";
    thread::spawn(move || { println!("{}", hello_message) }).join();
}

fn higher_order_func_demo() {

    type MethodType = fn(u32, u32) -> u32;

    fn calc(func: MethodType, a: u32, b: u32) -> u32 {
        return func(a, b);
    }

    fn add(a: u32, b: u32) -> u32 {
        return a + b;
    }

    fn sub(a: u32, b: u32) -> u32 {
        return a - b;
    }

    println!("{}", calc(add, 1, 2));
    println!("{}", calc(sub, 3, 2));
}

fn higher_order_func_demo2() {
    type MethodType = fn(u32, u32) -> u32;

    fn calc(func_name: &str) -> MethodType {
        match func_name {
            "add" => add,
            "sub" => sub,
            _ => unimplemented!(),
        }
    }

    fn add(a: u32, b: u32) -> u32 {
        return a + b;
    }

    fn sub(a: u32, b: u32) -> u32 {
        return a - b;
    }

    println!("{}", calc("add")(1, 2));
    println!("{}", calc("sub")(3, 2));
}


// programming exercise
fn guess_random_number() {

    let select_number: u32 = rand::thread_rng().gen_range(1..11);
    loop {
        println!("Input ?");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();

        let guess_number: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => continue,
        };

        println!("Input is {}", guess_number);

        if guess_number > select_number {
            println!("Too big");
        } else if guess_number < select_number {
            println!("Too small");
        } else {
            println!("It is correct!");
            break;
        }
    }

    return;
}

pub const MESSAGE: &str = "Hello World";

pub fn show_demo() {
    slice_demo();
    struct_demo();
    enum_demo();
    loop_demo2();
    while_demo();
    forrange_demo();
    iflet_demo();
    fibonacci(6);
    method_demo();
    closure_demo();
    guess_random_number();
    higher_order_func_demo2();
}