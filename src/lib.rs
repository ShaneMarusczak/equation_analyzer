use crate::actions::{add, divide, multiply, subtract};

mod actions;

pub fn plot(eq: String, x_min: f32, x_max: f32, step_size: f32) -> Result<EquationData, String> {
    let mut x_cur = x_min;

    let mut stack: Vec<String> = vec![];
    let split_eq = eq.split_whitespace().collect::<Vec<&str>>();

    for (i ,term) in split_eq.iter().enumerate() {
        match term.trim() {
            "y" | "=" | "x" => continue,
            "*" => {
                stack.push(split_eq[i - 1].to_string());
                stack.push(split_eq[i + 1].to_string());
                stack.push("m".to_string());
            },
            "/" => {
                stack.push(split_eq[i - 1].to_string());
                stack.push(split_eq[i + 1].to_string());
                stack.push("d".to_string());
            },
            "+" => {
                stack.push(split_eq[i - 1].to_string());
                stack.push(split_eq[i + 1].to_string());
                stack.push("a".to_string());
            },
            "-" => {
                stack.push(split_eq[i - 1].to_string());
                stack.push(split_eq[i + 1].to_string());
                stack.push("s".to_string());
            }
            _ => {
                match  term.parse::<f32>() {
                    Ok(n) => continue,
                    _ => panic!("unknown char")
                }
            }
        }
    }

    let mut test = vec![];

    while x_cur <=x_max {
        let mut current_value = 0_f32;
        let mut stack_clone = stack.clone();
        while !stack_clone.is_empty() {
            match stack_clone.pop().unwrap().as_str() {
                "m" => {
                    let v_1 = stack_clone.pop().unwrap();
                    let v_2 = stack_clone.pop().unwrap();

                    current_value += multiply(x_cur, get_not_x(v_1, v_2));
                },
                "a" => {
                    let v_1 = stack_clone.pop().unwrap();
                    let v_2 = stack_clone.pop().unwrap();

                    current_value = add(x_cur, get_not_x(v_1, v_2));
                },
                "s" => {
                    let v_1 = stack_clone.pop().unwrap();
                    let v_2 = stack_clone.pop().unwrap();

                    if v_2 == "x" {
                        current_value = subtract(x_cur, get_not_x(v_1, v_2));
                    } else {
                        current_value = subtract(get_not_x(v_1, v_2), x_cur);
                    }
                },
                "d" => {
                    let v_1 = stack_clone.pop().unwrap();
                    let v_2 = stack_clone.pop().unwrap();

                    if v_2 == "x" {
                        current_value = divide(x_cur, get_not_x(v_1, v_2));
                    } else {
                        current_value = divide(get_not_x(v_1, v_2), x_cur);
                    }
                },
                _ => {}
            }
        }
        test.push((x_cur, current_value));
        x_cur += step_size;
    }
    Ok(EquationData { points: test})
}

fn get_not_x(v_1: String, v_2: String) -> f32 {
    return if v_1 == "x" {
        v_2.parse::<f32>().unwrap()
    } else {
        v_1.parse::<f32>().unwrap()
    };
}

pub struct EquationData {
    pub points: Vec<(f32, f32)>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_plot_test_1(){
        let expected_points = vec![(-1_f32, 0_f32), (0_f32, 1_f32), (1_f32, 2_f32)];
        let actual = plot("y = x + 1".to_string(), -1_f32, 1_f32, 1_f32).unwrap();
        assert_eq!(expected_points, actual.points);
    }

    #[test]
    fn basic_plot_test_2(){
        let expected_points = vec![(-1_f32, 0_f32), (0_f32, 1_f32), (1_f32, 2_f32)];
        let actual = plot("y = 1 + x".to_string(), -1_f32, 1_f32, 1_f32).unwrap();
        assert_eq!(expected_points, actual.points);
    }

    #[test]
    fn basic_plot_test_3(){
        let expected_points = vec![(-1_f32, 0_f32),(-0.5_f32, 0.5_f32), (0_f32, 1_f32), (0.5_f32, 1.5_f32), (1_f32, 2_f32)];
        let actual = plot("y = 1 + x".to_string(), -1_f32, 1_f32, 0.5_f32).unwrap();
        assert_eq!(expected_points, actual.points);
    }

    #[test]
    fn basic_plot_test_4(){
        let expected_points = vec![(-1_f32, 41_f32),(-0.5_f32, 41.5_f32), (0_f32, 42_f32), (0.5_f32, 42.5_f32), (1_f32, 43_f32)];
        let actual = plot("y = 42 + x".to_string(), -1_f32, 1_f32, 0.5_f32).unwrap();
        assert_eq!(expected_points, actual.points);
    }

    #[test]
    fn basic_plot_test_5(){
        let expected_points = vec![(-1_f32, -2_f32), (0_f32, -1_f32), (1_f32, 0_f32)];
        let actual = plot("y = x - 1".to_string(), -1_f32, 1_f32, 1_f32).unwrap();
        assert_eq!(expected_points, actual.points);
    }

    #[test]
    fn basic_plot_test_6(){
        let expected_points = vec![(-1_f32, 2_f32), (0_f32, 1_f32), (1_f32, 0_f32)];
        let actual = plot("y = 1 - x".to_string(), -1_f32, 1_f32, 1_f32).unwrap();
        assert_eq!(expected_points, actual.points);
    }

    #[test]
    fn basic_plot_test_7(){
        let expected_points = vec![(-1_f32, -2_f32), (0_f32, 0_f32), (1_f32, 2_f32)];
        let actual = plot("y = x * 2".to_string(), -1_f32, 1_f32, 1_f32).unwrap();
        assert_eq!(expected_points, actual.points);
    }

    #[test]
    fn basic_plot_test_8(){
        let expected_points = vec![(-1_f32, -2_f32), (0_f32, 0_f32), (1_f32, 2_f32)];
        let actual = plot("y = 2 * x".to_string(), -1_f32, 1_f32, 1_f32).unwrap();
        assert_eq!(expected_points, actual.points);
    }

    #[test]
    fn basic_plot_test_9(){
        let expected_points = vec![(-1_f32, -0.5_f32), (0_f32, 0_f32), (1_f32, 0.5_f32)];
        let actual = plot("y = x / 2".to_string(), -1_f32, 1_f32, 1_f32).unwrap();
        assert_eq!(expected_points, actual.points);
    }

    #[test]
    fn basic_plot_test_10(){
        let expected_points = vec![(-1_f32, -2_f32), (0_f32, f32::INFINITY), (1_f32, 2_f32)];
        let actual = plot("y = 2 / x".to_string(), -1_f32, 1_f32, 1_f32).unwrap();
        assert_eq!(expected_points, actual.points);
    }
}

