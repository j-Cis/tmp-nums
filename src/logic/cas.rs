#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Var(String),
    Num(i32),
    E,
    Fr(Box<Node>, Box<Node>),
    Pow(Box<Node>, Box<Node>),
    Add(Box<Node>, Box<Node>),
    Mul(Box<Node>, Box<Node>),
    Log(Box<Node>, Box<Node>),
}

impl Node {
    pub fn simplify(&self) -> Node {
        match self {
            Node::Log(base, arg) => {
                // Reguła 1: log_e(e) = 1
                if **base == Node::E && **arg == Node::E {
                    return Node::Num(1);
                }
                // Reguła 2: log_base(k^n) = n * log_base(k)
                if let Node::Pow(k, n) = &**arg {
                    let new_log = Box::new(Node::Log(base.clone(), k.clone()));
                    return Node::Mul(n.clone(), new_log);
                }
                self.clone()
            },
            _ => self.clone(),
        }
    }

    pub fn to_latex(&self) -> String {
        match self {
            Node::Var(v) => v.clone(),
            Node::Num(n) => n.to_string(),
            Node::E => "e".to_string(),
            Node::Fr(top, bottom) => format!("\\frac{{{}}}{{{}}}", top.to_latex(), bottom.to_latex()),
            Node::Pow(base, exp) => format!("{}^{{{}}}", base.to_latex(), exp.to_latex()),
            Node::Add(left, right) => format!("{} + {}", left.to_latex(), right.to_latex()),
            Node::Mul(left, right) => format!("{} \\cdot {}", left.to_latex(), right.to_latex()),
            Node::Log(base, arg) => {
                if **base == Node::E {
                    format!("\\ln({})", arg.to_latex())
                } else {
                    format!("\\log_{{{}}}({})", base.to_latex(), arg.to_latex())
                }
            }
        }
    }
}
