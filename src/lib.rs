use crate::operands::{get_operator, Operand};

mod operands;

pub fn get_eq_data(eq: String, x_min: f32, x_max: f32, step_size: f32) -> Result<EquationData, String> {

    let rpn = get_rpn(eq);

    let mut test = vec![];

    let mut x_cur = x_min;
    while x_cur <=x_max {
        test.push((x_cur, eval_rpn(&rpn, x_cur)));
        x_cur += step_size;
    }
    Ok(EquationData { points: test})
}

fn eval_rpn(tokens: &Vec<String>, x: f32) -> f32 {
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
                _ => {}
            }
        }
    }
    stack[0]
}

fn get_rpn(eq: String) -> Vec<String> {
    let mut operator_stack: Vec<Operand> = vec![];
    let mut output = vec![];

    for term in eq.split_whitespace() {
        match term.trim() {
            "y" | "=" => continue,
            "*" | "/" | "+" | "-" | "^" => {
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
                match  term.parse::<f32>() {
                    Ok(_) => output.push(term.to_string()),
                    _ => {
                        let s = format!("unknown term: {}", term);
                        panic!("{}",s);
                    }
                }
            }
        }
    }

    while !operator_stack.is_empty() {
        let op = operator_stack.pop().unwrap();
        assert_ne!(op, get_operator("("));
        output.push(op.token);
    }
    output
}

pub struct EquationData {
    pub points: Vec<(f32, f32)>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_rpn_test_1(){
        let test = "3 + 4 * 2 / ( 1 - 5 ) ^ 2 ^ 3".to_string();
        let ans = vec!["3", "4", "2", "*", "1", "5", "-", "2", "3", "^", "^", "/", "+"];
        assert_eq!(get_rpn(test),ans);
    }

    #[test]
    fn get_rpn_test_2(){
        let test = "3 + 4 * ( 2 - 1 )".to_string();
        let ans = vec!["3", "4", "2", "1", "-", "*", "+"];
        assert_eq!(get_rpn(test),ans);
    }

    #[test]
    fn eval_rpn_test_3(){
        let test = "3 + 4 * ( 2 - 1 )".to_string();
        let rpn = get_rpn(test);
        let ans = eval_rpn(&rpn, f32::NAN);
        assert_eq!(ans, 7_f32);
    }

    #[test]
    fn eval_rpn_test_4(){
        let test = "3 + 4 * 2 - 1".to_string();
        let rpn = get_rpn(test);
        let ans = eval_rpn(&rpn, f32::NAN);
        assert_eq!(ans, 10_f32);
    }

    #[test]
    fn eval_rpn_test_5(){
        let test = "y = 3 + 4 * ( 2 - x )".to_string();
        let rpn = get_rpn(test);
        let ans = eval_rpn(&rpn, 1_f32);
        assert_eq!(ans, 7_f32);
    }

    #[test]
    fn eval_rpn_test_6(){
        let test = "y = x ^ 2 + x + 3".to_string();
        let rpn = get_rpn(test);
        let ans = eval_rpn(&rpn, 2_f32);
        assert_eq!(ans, 9_f32);
    }

}

