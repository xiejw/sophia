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

    #[derive(Debug)]
    pub enum BinOpType {
        Add,
        Minus,
    }

    #[derive(Debug)]
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

    pub struct Ns {}

    impl Ns {
        pub fn new() -> Ns {
            Ns {}
        }

        pub fn new_fn(self: &Self, name: &str) -> Fn {
            Fn {
                name: name.to_owned(),
                inps: Vec::new(),
                outs: Vec::new(),
            }
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

    pub fn new_raw_param(name: &str) -> Rc<Value> {
        Rc::new(Value {
            name: name.to_owned(),
            vtype: VType::Param,
        })
    }
}

pub mod cc {
    use std::collections::HashMap;

    use crate::alg::Fn;

    pub fn compile(f: &Fn) {
        // maintain a input sets
        let mut tensors = HashMap::new();
        let mut index = 0;
        for i in f.inps() {
            println!("assigning {} for {:?}", index, i);
            tensors.insert(i, index);
            index += 1;
        }

        // needs a top sort

        // // maintain a stack to loop of all ops
        // let mut ops = Vec::new();
        // for o in f.outs() {
        //     ops.push(o);
        // }

        // loop {
        //     if ops.len() == 0 {
        //         println!("cc end");
        //         break;
        //     }
        //     let o = ops.pop().unwrap();
        // }

        //     match **o {
        //         Op::Inp(ref inp) => {
        //             if inps.contains(inp) {
        //                 println!("found input {:?}", inp.name);
        //             } else {
        //                 panic!("unknown input {:?}", inp);
        //             }
        //         }
        //         Op::Read(ref param) => {
        //             println!("found param {:?}", param.name);
        //         }
        //         Op::Binary(_, ref arg1, ref arg2) => {
        //             ops.push(arg2);
        //             ops.push(arg1);
        //         }
        //     }
        // }
        //}
    }
}

use alg::Ns;
use alg::Op;

fn main() {
    let root = Ns::new();
    let w = alg::new_raw_param("w");

    let mut f = root.new_fn("foo");
    let x = f.new_inp("x");
    let y = f.new_inp("y");

    let o = Op::minus(Op::minus(Op::inp(x.clone()), Op::inp(x)), Op::inp(y));
    let o = Op::add(o, Op::read(w));

    println!("Op {:?}", &o);
    f.add_out(o);

    f.dump();

    cc::compile(&f);
}
