#[derive(Debug, Clone, Copy)]
enum Bool {
    True,
    False,
}

impl Bool {
    fn new() -> Self {
        Bool::False
    }

    fn flipped() -> Bool {
        Bool::False
    }

    fn is_true(&self) -> bool {
        match self {
            Bool::True => true,
            Bool::False => false,
        }
    }

    fn not(&self) -> Bool {
        match self {
            Bool::True => Bool::False,
            Bool::False => Bool::True,
        }
    }

    fn and(&self, other: &Bool) -> Bool {
        match (self, other) {
            (Bool::True, Bool::True) => Bool::True,
            _ => Bool::False,
        }
    }

    fn or(&self, other: &Bool) -> Bool {
        match (self, other) {
            (Bool::False, Bool::False) => Bool::False,
            _ => Bool::True,
        }
    }

    fn xor(&self, other: &Bool) -> Bool {
        match (self, other) {
            (Bool::True, Bool::False) => Bool::True,
            (Bool::False, Bool::True) => Bool::True,
            _ => Bool::False,
        }
    }
}

fn main() {
    let a = Bool::True;
    let b = Bool::False;

    let _default = Bool::new();
    let _flipped = Bool::flipped();

    println!("a            = {:?}", a);
    println!("b            = {:?}", b);
    println!("a.is_true()  = {}",   a.is_true());
    println!("not a        = {:?}", a.not());
    println!("a AND b      = {:?}", a.and(&b));
    println!("a OR  b      = {:?}", a.or(&b));
    println!("a XOR b      = {:?}", a.xor(&b));
}
