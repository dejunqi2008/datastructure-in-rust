#![allow(unused)]
use std::rc::Rc;

/*
Rc<T>: 引用计数

通过记录一个数据被引用的次数来确定该数据是否正在被使用。当引述次数归零时就带比哦啊该数据不再被使用，因此可以被清理释放

当我们希望在堆上分配一个对象供程序的多个部分使用且无法确定那个部分最后一个结束时，就可以使用Rc作为数据值的持有者。

*/

fn rc_demo_error() {
    /*
    let s = String::from("hello world");
    let a = Box::new(s);
    let b = Box::new(s); 报错， s 所有权已转移到a, 不能被再次转移。
    */
}

fn rc_demo_ok() {
    let s = String::from("hello world");
    let a = Rc::new(s);
    let b = Rc::clone(&a);
    assert_eq!(2, Rc::strong_count(&a));
    assert_eq!(Rc::strong_count(&a), Rc::strong_count(&b))

}

fn case1() {

}

fn test() {

}