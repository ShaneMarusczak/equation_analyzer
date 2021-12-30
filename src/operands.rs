#[derive(Debug, PartialEq)]
pub struct Operand {
    pub token: String,
    pub prec: usize,
    pub assoc: String,
}

pub fn get_operator(token: &str) -> Operand {
    match token {
        "+" => add_op(),
        "-" => sub_op(),
        "*" => mul_op(),
        "/" => div_op(),
        "^" => pow_op(),
        "(" => left_par_op(),
        _ => panic!("unknown operator")
    }
}

fn pow_op() -> Operand {
    Operand {
        token: String::from("^"),
        prec: 4,
        assoc: String::from("r"),
    }
}

fn div_op() -> Operand {
    Operand {
        token: String::from("/"),
        prec: 3,
        assoc: String::from("l"),
    }
}

fn mul_op() -> Operand {
    Operand {
        token: String::from("*"),
        prec: 3,
        assoc: String::from("l"),
    }
}

fn add_op() -> Operand {
    Operand {
        token: String::from("+"),
        prec: 2,
        assoc: String::from("l"),
    }
}

fn sub_op() -> Operand {
    Operand {
        token: String::from("-"),
        prec: 2,
        assoc: String::from("l"),
    }
}

fn left_par_op() -> Operand {
    Operand {
        token: String::from("("),
        prec: 0,
        assoc: String::from("l"),
    }
}