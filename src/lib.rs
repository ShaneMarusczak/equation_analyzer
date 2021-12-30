use crate::rpn::{eval_rpn, get_rpn};

mod rpn;
mod operands;

pub fn get_eq_data(eq: String, x_min: f32, x_max: f32, step_size: f32) -> Result<EquationData, String> {

    let rpn = get_rpn(eq)?;

    let mut test = vec![];

    let mut x_cur = x_min;
    while x_cur <=x_max {
        test.push((x_cur, eval_rpn(&rpn, x_cur)?));
        x_cur += step_size;
    }
    Ok(EquationData { points: test})
}



#[derive(Debug, PartialEq)]
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
        assert_eq!(get_rpn(test).unwrap(),ans);
    }

    #[test]
    fn get_rpn_test_2(){
        let test = "3 + 4 * ( 2 - 1 )".to_string();
        let ans = vec!["3", "4", "2", "1", "-", "*", "+"];
        assert_eq!(get_rpn(test).unwrap(),ans);
    }

    #[test]
    fn eval_rpn_test_3(){
        let test = "3 + 4 * ( 2 - 1 )".to_string();
        let rpn = get_rpn(test).unwrap();
        let ans = eval_rpn(&rpn, f32::NAN).unwrap();
        assert_eq!(ans, 7_f32);
    }

    #[test]
    fn eval_rpn_test_4(){
        let test = "3 + 4 * 2 - 1".to_string();
        let rpn = get_rpn(test).unwrap();
        let ans = eval_rpn(&rpn, f32::NAN).unwrap();
        assert_eq!(ans, 10_f32);
    }

    #[test]
    fn eval_rpn_test_5(){
        let test = "y = 3 + 4 * ( 2 - x )".to_string();
        let rpn = get_rpn(test).unwrap();
        let ans = eval_rpn(&rpn, 1_f32).unwrap();
        assert_eq!(ans, 7_f32);
    }

    #[test]
    fn eval_rpn_test_6(){
        let test = "y = x ^ 2 + x + 3".to_string();
        let rpn = get_rpn(test).unwrap();
        let ans = eval_rpn(&rpn, 2_f32).unwrap();
        assert_eq!(ans, 9_f32);
    }

    #[test]
    fn get_eq_data_test_1() {
        let test_eq = "y = x ^ 2 + 2 * x + 1".to_string();
        let ans = vec![(-1_f32, 0_f32), (0_f32, 1_f32), (1_f32, 4_f32)];

        assert_eq!(get_eq_data(test_eq, -1f32, 1_f32, 1_f32).unwrap().points, ans);
    }

}

