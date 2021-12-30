use crate::operands::{get_operator, Operand};

pub fn eval_rpn(tokens: &Vec<String>, x: f32) -> Result<f32, String> {
    let mut stack: Vec<f32> = Vec::new();
    for token in tokens.iter() {
        if let Ok(n) = token.parse() {
            stack.push(n);
        } else if *token == "x".to_string(){
            stack.push(x);
        }
        else {
            let rhs = stack.pop().unwrap();
            let lhs = stack.pop().unwrap();
            match token.as_str() {
                "+" => stack.push(lhs + rhs),
                "-" => stack.push(lhs - rhs),
                "*" => stack.push(lhs * rhs),
                "/" => stack.push(lhs / rhs),
                "^" => stack.push(lhs.powf(rhs)),
                _ => return Err(format!("Unknown token: {}", token))

            }
        }
    }
    Ok(stack[0])
}

pub fn get_rpn(eq: String) -> Result<Vec<String>,String> {
    let mut operator_stack: Vec<Operand> = vec![];
    let mut output = vec![];

    for term in eq.split_whitespace() {
        match term.trim() {
            "y" | "=" => continue,
            "*" | "/" | "+" | "-" | "^" => {
                //TODO: Write this cleaner
                let o_1 = get_operator(term);
                while !operator_stack.is_empty() && operator_stack.last().unwrap().token != "(".to_string() &&
                    ( operator_stack.last().unwrap().prec > o_1.prec || (operator_stack.last().unwrap().prec == o_1.prec && o_1.assoc == "l".to_string())) {
                    let o_2_new = operator_stack.pop().unwrap();
                    output.push(o_2_new.token);
                }

                operator_stack.push(o_1);
            },
            "(" => operator_stack.push(get_operator("(")),
            ")" => {
                while operator_stack.last().unwrap().token != "(" {
                    assert!(!operator_stack.is_empty());
                    let op = operator_stack.pop().unwrap();
                    output.push(op.token);
                }
                assert_eq!(operator_stack.last().unwrap().token, "(");
                operator_stack.pop();
            }
            "x" => output.push(term.to_string()),
            _ => {
                if let Ok(_) = term.parse::<f32>() {
                    output.push(term.to_string());
                } else{
                    return Err(format!("Unknown term: {}", term));
                }
            }
        }
    }

    while !operator_stack.is_empty() {
        let op = operator_stack.pop().unwrap();
        assert_ne!(op, get_operator("("));
        output.push(op.token);
    }
    Ok(output)
}