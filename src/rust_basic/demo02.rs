// Demo code for Rust Generic

fn larger(a: u32, b: u32) -> u32 {
    if a > b {
        a
    } else {
        b
    }
}


// 范型参数T + 范型特征绑定
fn larger_generic<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}


fn generic_for_struct() {
    struct Point<T> {
        x: T,
        y: T
    }

    let integer: Point<u32> = Point { x: 5, y: 7};
    let float = Point {x: 1.2, y: 2.3};
    
}

fn generic_impl_demo() {
    struct Point<T> {
        x: T,
        y: T,
    }

    impl<T: Clone + std::cmp::PartialOrd> Point<T>  {
        fn max(&self) -> T {
            if self.x > self.y {
                self.x.clone()
            } else {
                self.y.clone()
            }
        }
    }

    let p1: Point<u64> = Point {
        x: 10, y: 20
    };

    println!("{}", p1.max());

    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            return (self.x.powi(2) + self.y.powi(2)).sqrt();
        }
    }

    let p2: Point<f32> = Point {
        x: 1.2, y: 2.4
    };

    println!("{}", p2.max());
    println!("{}", p2.distance_from_origin());

}


fn struct_traits_demo() {
    struct Point<T> {
        x: T, y: T,
    }

    impl<T: Clone> Point<T> {
        fn clone(&self) -> Point<T> {
            return Point {
                x: self.x.clone(),
                y: self.y.clone(),
            }
        }
    }

    impl<T: std::fmt::Display> std::fmt::Display for Point<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}, {}", self.x, self.y)
        }
    }

    let p: Point<u32> = Point {x: 10, y: 20};

    println!("{}", p);

    // traits_as_arg_demo
    fn show<T: std::fmt::Display>(a: T) {
        println!("{}", a);
    }

    fn show2(a: impl std::fmt::Display) {
        println!("{}", a);
    }

    let p2: Point<u32> = p.clone();
    
    show(p);

    show2(p2);

    
}





// Gneric for struct

pub fn demo() {
    // println!("{}", larger_generic(10, 4));
    // println!("{}", larger_generic(1.2, 2.6));
    // generic_impl_demo();
    struct_traits_demo();
}