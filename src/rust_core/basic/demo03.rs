#![allow(unused)]

// Rust: ownership and scope demo

fn borrow_demo(){
    fn change(s: &mut String) {
        s.push_str(" Updated.");
    }
    let mut s =  String::from("Hello World!");
    change(&mut s);
    println!("{}", s);

    /* 同一时间只能有一个可变引用， 一下代码不合法
        let mut s = String::from("Hello");
        let s1ref = &mut s;
        let s2ref = &mut s;
     */  
}

fn scope_demo() {
    // 同一时间只能有一个可变引用, 但再不同的生命周期内，就不是“同一时间了”因此，
    // 以下代码中的两个可变引用是合法的。
    let mut i = 3;
    {
        let borrow1 = &mut i;
        println!("borrow1 {}", borrow1);
    }
    {
        let borrow2 = &mut i;
        println!("borrow1 {}", borrow2);
    }
}

pub fn test() {

    // borrow_demo();
    scope_demo();

}