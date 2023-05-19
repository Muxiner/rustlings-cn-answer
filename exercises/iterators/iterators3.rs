// iterators3.rs
// 这个练习比大多数别的都大！你可以做到的！
// 如果你接受的话，这是你的任务：
// 1. 完成 divide 函数使前四个测试通过。
// 2. 通过完成 result_with_list 和 list_of_results 函数使其余的测试通过。
// 执行 `rustlings hint iterators3` 或在观察模式下使用 `hint` 子命令来获取提示。

// // I AM NOT DONE
use std::result::Result;
use std::error::Error;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum DivisionError {
    NotDivisible(NotDivisibleError),
    DivideByZero,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct NotDivisibleError {
    dividend: i32,
    divisor: i32,
}

// 如果 'a' 可被 'b' 整除，计算 'a' 除以 'b'。
// 不然，返回适当的错误。
pub fn divide(a: i32, b: i32) -> Result<i32, DivisionError> {
    // todo!();
    match b == 0 {
        true => Err(DivisionError::DivideByZero),
        false => match a % b == 0 {
            true => Ok(a / b),
            false => Err(DivisionError::NotDivisible(NotDivisibleError {
                dividend: a,
                divisor: b,
            })),
        },
    }
}

// 完成函数并返回正确类型的值让测试通过。
// 期待的输出： Ok([1, 11, 1426, 3])
fn result_with_list() -> Result<Vec<i32>, Box<dyn Error>> {
    let numbers = vec![27, 297, 38502, 81];
    let division_results: Vec<_> = numbers.into_iter().map(|n| divide(n, 27)).collect::<_>();
    let mut vec_return = vec![];
    for result in division_results.iter() {
        if let Some(num) = result.as_ref().ok() {
            vec_return.push(*num);
        };
    }

    Ok(vec_return)
}

// 完成函数并返回正确类型的值让测试通过。
// 期待的输出： [Ok(1), Ok(11), Ok(1426), Ok(3)]
fn list_of_results() -> Vec<Result<i32, DivisionError>> {
    let numbers = vec![27, 297, 38502, 81];
    let division_results: Vec<_> = numbers.into_iter().map(|n| divide(n, 27)).collect::<_>();
    let mut vec_return = Vec::new();
    
    // division_results.iter().for_each(|x| vec_return.push(*x)); // 方法一
    // 方法二
    for result in division_results.iter() {
        vec_return.push(*result);
    }

    vec_return
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(divide(81, 9), Ok(9));
    }

    #[test]
    fn test_not_divisible() {
        assert_eq!(
            divide(81, 6),
            Err(DivisionError::NotDivisible(NotDivisibleError {
                dividend: 81,
                divisor: 6
            }))
        );
    }

    #[test]
    fn test_divide_by_0() {
        assert_eq!(divide(81, 0), Err(DivisionError::DivideByZero));
    }

    #[test]
    fn test_divide_0_by_something() {
        assert_eq!(divide(0, 81), Ok(0));
    }

    #[test]
    fn test_result_with_list() {
        assert_eq!(format!("{:?}", result_with_list()), "Ok([1, 11, 1426, 3])");
    }

    #[test]
    fn test_list_of_results() {
        assert_eq!(
            format!("{:?}", list_of_results()),
            "[Ok(1), Ok(11), Ok(1426), Ok(3)]"
        );
    }
}
