pub trait ST {
    fn lit(num: isize) -> Self;
    fn neg(num: Self) -> Self;
    fn add(left: Self, right: Self) -> Self;
}

pub trait Sub {
    fn sub(left: Self, right: Self) -> Self;
}

impl<T: ST> Sub for T {
    fn sub(left: Self, right: Self) -> Self {
        return ST::add(left, ST::neg(right));
    }
}

#[derive(Debug)]
pub enum Code {
    Load(isize),
    Neg,
    Add,
}

impl ST for Vec<Code> {
    fn lit(num: isize) -> Self {
        vec![Code::Load(num)]
    }

    fn neg(mut num: Self) -> Self {
        num.push(Code::Neg);
        return num;
    }

    fn add(mut left: Self, mut right: Self) -> Self {
        left.append(&mut right);
        left.push(Code::Add);
        return left;
    }
}

pub struct VM {
    code: Vec<Code>,
    place: usize,
    stack: Vec<isize>,
}

impl VM {
    pub fn new(code: Vec<Code>) -> VM {
        VM { code, place: 0, stack: vec![] }
    }

    pub fn run(&mut self) -> isize {
        while self.place < self.code.len() {
            match self.code[self.place] {
                Code::Load(num) => self.stack.push(num),
                Code::Neg => {
                    let num = self.stack.pop().unwrap();
                    self.stack.push(-num);
                }
                Code::Add => {
                    let right = self.stack.pop().unwrap();
                    let left  = self.stack.pop().unwrap();
                    self.stack.push(left + right);
                }
            }
            self.place += 1;
        }
        return self.stack.pop().unwrap();
    }
}

fn main() {
    let code: Vec<Code> = ST::add(ST::lit(8), ST::neg(Sub::sub(ST::lit(1), ST::lit(2))));
    println!("{:#?}", code);
    let mut vm = VM::new(code);
    let result = vm.run();
    println!("{}", result);
}
