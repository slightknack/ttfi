use std::collections::HashMap;

pub trait ST {
    fn lit(num: isize) -> Self;
    fn neg(num: Self) -> Self;
    fn add(left: Self, right: Self) -> Self;
    fn get(var: String) -> Self;
    fn set(var: String, val: Self) -> Self;
    fn block(exprs: Vec<Self>) -> Self where Self: Sized;
}

pub trait Sub {
    fn sub(left: Self, right: Self) -> Self;
}

impl<T: ST + Auto> Sub for T {
    fn sub(left: Self, right: Self) -> Self {
        return ST::add(left, ST::neg(right));
    }
}

pub trait Auto {}
pub trait ParseST: ST + Sub {}

pub struct Source(String);

impl std::fmt::Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Sub for Source {
    fn sub(left: Self, right: Self) -> Self {
        Source(format!("({} - {})", left, right))
    }
}

impl ST for Source {
    fn lit(num: isize)              -> Self { Source(format!("{}", num)) }
    fn neg(num: Self)               -> Self { Source(format!("-{}", num)) }
    fn add(left: Self, right: Self) -> Self { Source(format!("({} + {})", left, right)) }
    fn get(var: String)             -> Self { Source(var) }
    fn set(var: String, val: Self)  -> Self { Source(format!("({} = {})", var, val)) }

    fn block(exprs: Vec<Self>) -> Self where Self: Sized {
        let inner = exprs.iter()
            .map(|expr| {
                expr.0.split("\n")
                    .map(|line| { format!("    {}\n", line) })
                    .collect::<Vec<String>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .join("");

        let formatted = format!("{{\n{}}}", inner);
        return Source(formatted);
    }
}

impl ParseST for Source {}

#[derive(Debug)]
pub enum Code {
    Get(String),
    Set(String),
    Drop,
    Load(isize),
    Neg,
    Add,
}

impl Auto for Vec<Code> {}

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

    fn get(var: String) -> Self {
        return vec![Code::Get(var)];
    }

    fn set(var: String, mut val: Self) -> Self {
        val.push(Code::Set(var));
        return val;
    }

    fn block(mut exprs: Vec<Self>) -> Self where Self: Sized {
        let mut combined = vec![];
        let last = exprs.len()-1;

        for expr in exprs[0..last].iter_mut() {
            combined.append(expr);
            combined.push(Code::Drop);
        }

        combined.append(&mut exprs[last]);
        return combined;
    }
}

pub struct VM {
    code:  Vec<Code>,
    place: usize,
    stack: Vec<isize>,
    vars:  HashMap<String, isize>,
}

impl VM {
    pub fn new(code: Vec<Code>) -> VM {
        VM { code, place: 0, stack: vec![], vars: HashMap::new() }
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
                Code::Get(ref var) => {
                    let val = self.vars.get(var)
                        .expect(&format!("{} reffed before assign", var));
                    self.stack.push(*val);
                }
                Code::Set(ref var) => {
                    let val = self.stack.pop().unwrap();
                    let old = self.vars.insert(var.to_owned(), val);
                    let old = if let Some(val) = old { val } else { 0 };
                    self.stack.push(old);
                }
                Code::Drop => { self.stack.pop(); },
            }
            self.place += 1;
        }
        return self.stack.pop().unwrap();
    }
}

fn main() {
    let code: Vec<Code> = ST::block(vec![
        Sub::sub(ST::lit(8), ST::add(ST::lit(1), ST::lit(2))),
        ST::set("x".to_string(), ST::block(vec![
            ST::lit(5),
        ])),
        ST::add(ST::get("x".to_string()), ST::lit(7)),
    ]);
    println!("{:#?}", code);
    let mut vm = VM::new(code);
    let result = vm.run();
    println!("{}", result);
}
