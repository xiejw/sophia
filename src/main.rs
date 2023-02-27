use std::collections::HashSet;
use std::rc::Rc;

#[derive(Debug, Hash, PartialEq)]
enum VType {
    Param,
    Input,
}

#[derive(Debug, Hash)]
struct Value {
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
enum Op {
    Inp(Rc<Value>),
    Read(Rc<Value>),
    Add(Rc<Op>, Rc<Op>),
    Minus(Rc<Op>, Rc<Op>),
}

impl Op {
    fn inp(v: Rc<Value>) -> Rc<Op> {
        assert!(v.vtype == VType::Input);
        Rc::new(Op::Inp(v))
    }

    fn read(v: Rc<Value>) -> Rc<Op> {
        assert!(v.vtype == VType::Param);
        Rc::new(Op::Read(v))
    }

    fn add(arg1: Rc<Op>, arg2: Rc<Op>) -> Rc<Op> {
        Rc::new(Op::Add(arg1, arg2))
    }

    fn minus(arg1: Rc<Op>, arg2: Rc<Op>) -> Rc<Op> {
        Rc::new(Op::Minus(arg1, arg2))
    }
}

struct Fn {
    inps: Vec<Rc<Value>>,
    outs: Vec<Rc<Op>>,
    //ops: Vec<Op>,
}

impl Fn {
    fn new() -> Fn {
        Fn {
            inps: Vec::new(),
            outs: Vec::new(),
            //ops: Vec::new(),
        }
    }

    fn new_inp(self: &mut Self, name: &str) -> Rc<Value> {
        let v = Rc::new(Value {
            name: name.to_owned(),
            vtype: VType::Input,
        });
        self.inps.push(v.clone());
        v
    }

    fn add_out(self: &mut Self, v: Rc<Op>) {
        self.outs.push(v);
    }

    fn print(self: &Self) {
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
                Op::Add(ref arg1, ref arg2) => {
                    ops.push(arg2);
                    ops.push(arg1);
                }
                Op::Minus(ref arg1, ref arg2) => {
                    ops.push(arg2);
                    ops.push(arg1);
                }
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
    //println!("{:?}", Op::Add);

    // let mut g = Fn::new();
    // let gx = g.new_inp("x");

    let w = Rc::new(Value {
        name: "w".to_owned(),
        vtype: VType::Param,
    });

    let mut f = Fn::new();
    let x = f.new_inp("x");
    let y = f.new_inp("y");
    let o = Op::minus(Op::minus(Op::inp(x.clone()), Op::inp(x)), Op::inp(y));
    let o = Op::add(o, Op::read(w));

    println!("{:?}", &o);
    //println!("{:?}", &f.inps[0].name);
    f.add_out(o);
    f.print();
}
