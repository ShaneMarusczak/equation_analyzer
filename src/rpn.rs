use std::f32::consts::{E, PI};
use crate::operands::{get_operator, Operand};

pub fn eval_rpn(tokens: &Vec<String>, x: f32) -> Result<f32, String> {
    let mut stack: Vec<f32> = Vec::new();
    for token in tokens.iter() {
        if let Ok(n) = token.parse() {
            stack.push(n);
        }
        else if *token == "π".to_string() {
            stack.push(PI);
        }
        else if *token == "e".to_string() {
            stack.push(E);
        }
        else if *token == "x".to_string(){
            stack.push(x);
        }
        else if *token == "sin".to_string() {
            let temp = stack.pop().unwrap();
            stack.push(temp.sin());
        }
        else if *token == "cos".to_string() {
            let temp = stack.pop().unwrap();
            stack.push(temp.cos());
        }
        else if *token == "tan".to_string() {
            let temp = stack.pop().unwrap();
            stack.push(temp.tan());
        }
        else if *token == "abs".to_string() {
            let temp = stack.pop().unwrap();
            stack.push(temp.abs());
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
                "max" => stack.push(lhs.max(rhs)),
                _ => return Err(format!("Unknown token: {}", token))

            }
        }
    }
    Ok(stack[0])
}

pub fn get_rpn(eq: String) -> Result<Vec<String>,String> {
    let mut operator_stack: Vec<Operand> = vec![];
    let mut output = vec![];

    for (i , term) in eq.split_whitespace().enumerate() {
        match term.to_lowercase().trim() {
            "y" | "=" | "," => continue,
            "π" | "e" => output.push(term.to_string()),
            "sin" | "cos" | "tan" | "(" | "max" | "abs" => {
                operator_stack.push(get_operator(term));
            },
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
            ")" => {
                while operator_stack.last().unwrap().token != "(" {
                    assert!(!operator_stack.is_empty());
                    let op = operator_stack.pop().unwrap();
                    output.push(op.token);
                }

                operator_stack.pop();
                if operator_stack.is_empty() {
                    let e = format!("Invalid Closing Parenthesis at character {}", i + 1);
                    return Err(e);

                }

                if operator_stack.last().unwrap().is_func {
                    let func = operator_stack.pop().unwrap();
                    output.push(func.token);
                }
            }
            //TODO: What about 2x? (even if it would work as 2 * x)
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
        if op.token == "(" {
            return Err("Invalid Opening Parenthesis".to_string());
        }
        output.push(op.token);
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_close(x1: f32, x2: f32) -> bool {
        (x1 - x2).abs() < 0.00001
    }

    #[test]
    fn get_rpn_invalid_parens_1(){
        let test = "( 3 + 4 ) )".to_string();
        assert_eq!(get_rpn(test).unwrap_err(), String::from("Invalid Closing Parenthesis at character 5"));
    }

    #[test]
    fn get_rpn_invalid_parens_2(){
        let test = "( ( 3 + 4 )".to_string();
        assert_eq!(get_rpn(test).unwrap_err(), String::from("Invalid Opening Parenthesis"));
    }

    #[test]
    fn get_rpn_invalid_parens_3(){
        let test = "( 3 + 4 ( ( )".to_string();
        assert_eq!(get_rpn(test).unwrap_err(), String::from("Invalid Opening Parenthesis"));
    }

    #[test]
    fn get_rpn_test_1(){
        let test = "3 + 4 * 2 / ( 1 - 5 ) ^ 2 ^ 3".to_string();
        let ans = vec!["3", "4", "2", "*", "1", "5", "-", "2", "3", "^", "^", "/", "+"];
        assert_eq!(get_rpn(test).unwrap(),ans);
    }

    #[test]
    fn get_rpn_test_2(){
        let test = "3 + 4 * ( 2 - 1 )".to_string();
        let ans = vec!["3", "4", "2", "1", "-", "*", "+"];
        assert_eq!(get_rpn(test).unwrap(),ans);
    }

    #[test]
    fn get_rpn_test_3(){
        let test = "sin ( max ( 2 , 3 ) / 3 * π )".to_string();
        let ans = vec!["2", "3", "max", "3", "/", "π", "*", "sin"];
        assert_eq!(get_rpn(test).unwrap(),ans);
    }

    #[test]
    fn eval_rpn_test_1(){
        let test = "3 + 4 * ( 2 - 1 )".to_string();
        let rpn = get_rpn(test).unwrap();
        let ans = eval_rpn(&rpn, f32::NAN).unwrap();
        assert_eq!(ans, 7_f32);
    }

    #[test]
    fn eval_rpn_test_2(){
        let test = "3 + 4 * 2 - 1".to_string();
        let rpn = get_rpn(test).unwrap();
        let ans = eval_rpn(&rpn, f32::NAN).unwrap();
        assert_eq!(ans, 10_f32);
    }

    #[test]
    fn eval_rpn_test_3(){
        let test = "y = 3 + 4 * ( 2 - x )".to_string();
        let rpn = get_rpn(test).unwrap();
        let ans = eval_rpn(&rpn, 1_f32).unwrap();
        assert_eq!(ans, 7_f32);
    }

    #[test]
    fn eval_rpn_test_4(){
        let test = "y = x ^ 2 + x + 3".to_string();
        let rpn = get_rpn(test).unwrap();
        let ans = eval_rpn(&rpn, 2_f32).unwrap();
        assert_eq!(ans, 9_f32);
    }

    #[test]
    fn eval_rpn_test_5(){
        let test = "sin π".to_string();
        let rpn = get_rpn(test).unwrap();
        let ans = eval_rpn(&rpn, f32::NAN).unwrap();
        assert!(is_close(ans, 0_f32));
    }

    #[test]
    fn eval_rpn_test_6(){
        let test = " sin ( π ) / 2".to_string();
        let rpn = get_rpn(test).unwrap();
        let ans = eval_rpn(&rpn, f32::NAN).unwrap();
        assert!(is_close(ans, 0_f32));
    }

    #[test]
    fn eval_rpn_test_7(){
        let test = "sin ( π / 2 )".to_string();
        let rpn = get_rpn(test).unwrap();
        let ans = eval_rpn(&rpn, f32::NAN).unwrap();
        assert!(is_close(ans, 1_f32));
    }

    #[test]
    fn eval_rpn_test_8(){
        let test = "cos ( π ) / 2".to_string();
        let rpn = get_rpn(test).unwrap();
        let ans = eval_rpn(&rpn, f32::NAN).unwrap();
        assert!(is_close(ans, -0.5f32));
    }

    #[test]
    fn eval_rpn_test_9(){
        let test = "tan ( π ) + cos ( π + π ) + sin ( 2 * π )".to_string();
        let rpn = get_rpn(test).unwrap();
        let ans = eval_rpn(&rpn, f32::NAN).unwrap();
        assert!(is_close(ans, 1_f32));
    }

    #[test]
    fn eval_rpn_test_10(){
        let test = "tan ( π ) + max ( 0 , π ) + sin ( 2 * π )".to_string();
        let rpn = get_rpn(test).unwrap();
        let ans = eval_rpn(&rpn, f32::NAN).unwrap();
        assert!(is_close(ans, PI));
    }

    #[test]
    fn eval_rpn_test_11(){
        let test = "max ( sin ( π ) , max ( ( 2 ^ 3 ) , 6 ) )".to_string();
        let rpn = get_rpn(test).unwrap();
        let ans = eval_rpn(&rpn, f32::NAN).unwrap();
        assert!(is_close(ans, 8_f32));
    }

    #[test]
    fn eval_rpn_test_12(){
        let test = "abs ( 2 - 3 ^ 2 )".to_string();
        let rpn = get_rpn(test).unwrap();
        let ans = eval_rpn(&rpn, f32::NAN).unwrap();
        assert_eq!(ans, 7_f32);
    }

    #[test]
    fn eval_rpn_test_13(){
        let test = "abs ( 2 * 3 - 3 ^ 2 )".to_string();
        let rpn = get_rpn(test).unwrap();
        let ans = eval_rpn(&rpn, f32::NAN).unwrap();
        assert_eq!(ans, 3_f32);
    }
}