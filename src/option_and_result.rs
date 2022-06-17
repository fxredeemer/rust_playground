use std::fs;

pub fn option_and_result() {
    let _value = divide(7.0, 0.0).unwrap_or(0.0);
    println!("The value is {}", _value);

    let _data_or_default = match read_file_if_existing("asdf.txt") {
        Some(data) => data,
        None => "<div></div>".to_owned(),
    };
}

fn divide(value: f64, divisor: f64) -> Result<f64, String> {
    if divisor.abs() < 1e-10 {
        return Err("Unable to divide by 0".to_owned());
    }

    return Ok(value / divisor);
}

fn read_file_if_existing(path: &str) -> Option<String> {
    let data = fs::read_to_string(path);
    data.ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide() {
        assert_eq!(divide(1.0, 2.0), Ok(0.5));
    }

    #[test]
    fn test_divide_by_0() {
        assert_eq!(divide(1.0, 0.0), Err("Unable to divide by 0".to_owned()));
    }
}
