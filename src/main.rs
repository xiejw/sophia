pub mod alg {
    use std::collections::HashSet;
    use std::rc::Rc;

    #[derive(Debug, Hash, PartialEq)]
    pub enum VType {
        Param,
        Input,
    }

    #[derive(Debug, Hash)]
    pub struct Value {
        name: String,
        vtype: VType,
    }

    impl PartialEq for Value {
        fn eq(&self, other: &Self) -> bool {
            self as *const _ == other as *const _
        }
    }
    impl Eq for Value {}

    #[derive(Debug, Hash, Eq, PartialEq)]
    pub enum BinOpType {
        Add,
        Minus,
    }

    #[derive(Debug, Hash, Eq, PartialEq)]
    pub enum Op {
        Inp(Rc<Value>),
        Read(Rc<Value>),
        Binary(BinOpType, Rc<Op>, Rc<Op>),
    }

    impl Op {
        pub fn inp(v: Rc<Value>) -> Rc<Op> {
            assert!(v.vtype == VType::Input);
            Rc::new(Op::Inp(v))
        }

        pub fn read(v: Rc<Value>) -> Rc<Op> {
            assert!(v.vtype == VType::Param);
            Rc::new(Op::Read(v))
        }

        pub fn add(arg1: Rc<Op>, arg2: Rc<Op>) -> Rc<Op> {
            Rc::new(Op::Binary(BinOpType::Add, arg1, arg2))
        }

        pub fn minus(arg1: Rc<Op>, arg2: Rc<Op>) -> Rc<Op> {
            Rc::new(Op::Binary(BinOpType::Minus, arg1, arg2))
        }
    }

    impl std::fmt::Display for Op {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            self.fmt_with_indent(f, 0)
        }
    }

    impl Op {
        fn fmt_with_indent(&self, f: &mut std::fmt::Formatter, indent: usize) -> std::fmt::Result {
            match self {
                Op::Inp(_) => {
                    write!(f, "{:?}", self)
                }
                Op::Read(_) => {
                    write!(f, "{:?}", self)
                }
                Op::Binary(btype, ref op1, ref op2) => {
                    let space = format!("{:1$}", " ", indent);
                    let new_indext = indent + 4;
                    write!(
                        f,
                        "Binary(\n{}  +- {:?},\n{}  +- ", //{}\n {} +- {}\n{})",
                        space, btype, space
                    )
                    .unwrap();
                    op1.fmt_with_indent(f, new_indext).unwrap();
                    write!(f, "\n{}  +- ", space).unwrap();
                    op2.fmt_with_indent(f, new_indext).unwrap();
                    write!(f, "\n{})", space)
                }
            }
        }
    }

    pub struct Ns {}

    impl Ns {
        pub fn new() -> Ns {
            Ns {}
        }

        pub fn new_fn(self: &mut Self, name: &str) -> Fn {
            Fn {
                name: name.to_owned(),
                inps: Vec::new(),
                outs: Vec::new(),
            }
        }

        pub fn new_raw_param(self: &mut Self, name: &str) -> Rc<Value> {
            Rc::new(Value {
                name: name.to_owned(),
                vtype: VType::Param,
            })
        }
    }

    pub struct Fn {
        name: String,
        inps: Vec<Rc<Value>>,
        outs: Vec<Rc<Op>>,
    }

    impl Fn {
        pub fn inps(self: &Self) -> &Vec<Rc<Value>> {
            &self.inps
        }

        pub fn outs(self: &Self) -> &Vec<Rc<Op>> {
            &self.outs
        }

        pub fn new_inp(self: &mut Self, name: &str) -> Rc<Value> {
            let v = Rc::new(Value {
                name: format!("{}/{}", &self.name, name),
                vtype: VType::Input,
            });
            self.inps.push(v.clone());
            v
        }

        pub fn add_out(self: &mut Self, v: Rc<Op>) {
            self.outs.push(v);
        }

        pub fn dump(self: &Self) {
            // maintain a input sets
            let mut inps = HashSet::new();
            for i in &self.inps {
                inps.insert(i);
            }

            // maintain a stack to loop of all values
            let mut ops = Vec::new();
            for o in &self.outs {
                ops.push(o);
            }

            loop {
                if ops.len() == 0 {
                    break;
                }

                let o = ops.pop().unwrap();

                match **o {
                    Op::Inp(ref inp) => {
                        if inps.contains(inp) {
                            println!("found input {:?}", inp.name);
                        } else {
                            panic!("unknown input {:?}", inp);
                        }
                    }
                    Op::Read(ref param) => {
                        println!("found param {:?}", param.name);
                    }
                    Op::Binary(_, ref op1, ref op2) => {
                        ops.push(op1);
                        ops.push(op2);
                    }
                }
            }
        }
    }
}

pub mod cc {
    use std::collections::HashMap;
    use std::collections::HashSet;
    use std::rc::Rc;

    use crate::alg::Fn;
    use crate::alg::Op;

    pub fn compile(f: &Fn) {
        // maintain a input sets
        let mut tensors = HashMap::new();
        let mut index = 0;
        for i in f.inps() {
            println!("assigning {} for {:?}", index, i);
            tensors.insert(i, index);
            index += 1;
        }

        // topology sort
        let mut ops = Vec::new();
        for o in f.outs() {
            ops.push(o);
        }

        let mut outs = Vec::new();
        {
            let mut states = HashSet::new();
            top_sort(&mut ops, &mut outs, &mut states);
        }

        for o in &outs {
            println!("op after sort {}", o);
        }
    }

    fn top_sort<'a>(
        ops: &mut Vec<&'a Rc<Op>>,
        outs: &mut Vec<&'a Rc<Op>>,
        states: &mut HashSet<&'a Rc<Op>>,
    ) {
        if ops.len() == 0 {
            return;
        }

        let o = ops.pop().unwrap(); // should always be safe

        if states.contains(o) {
            return;
        }

        match **o {
            Op::Inp(_) => {
                outs.push(o);
                states.insert(o);
            }
            Op::Read(_) => {
                outs.push(o);
                states.insert(o);
            }
            Op::Binary(_, ref arg1, ref arg2) => {
                ops.push(arg1);
                top_sort(ops, outs, states);
                ops.push(arg2);
                top_sort(ops, outs, states);
                outs.push(o);
                states.insert(o);
            }
        };
    }
}

use alg::Ns;
use alg::Op;

fn main() {
    let mut root = Ns::new();
    let w = root.new_raw_param("w");

    let mut f = root.new_fn("foo");
    let x = f.new_inp("x");
    let y = f.new_inp("y");

    let o = Op::minus(Op::minus(Op::inp(x.clone()), Op::inp(x)), Op::inp(y));
    let o = Op::add(o, Op::read(w));

    println!("Op {}", &o);
    f.add_out(o);

    f.dump();

    cc::compile(&f);
}
