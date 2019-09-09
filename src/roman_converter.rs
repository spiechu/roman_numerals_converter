use super::dictionary::CONVERTIBLE;

pub fn convert(number_to_convert: u32) -> Result<String, String> {
    if number_to_convert > 3999 {
        return Err("Invalid conversion above maximum limit of 3999".to_string());
    }

    let mut roman_number = String::new();
    let mut to_convert = number_to_convert;

    while to_convert > 0 {
        'inner: for s in CONVERTIBLE.iter() {
            if s.1 > to_convert {
                continue 'inner;
            }

            roman_number = roman_number + s.0;
            to_convert = to_convert - s.1;

            break 'inner;
        }
    }

    Ok(roman_number)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_convert_zeros() {
        assert_eq!(convert(0), Ok("".to_string()));
    }

    #[test]
    fn test_can_convert_one() {
        assert_eq!(convert(1), Ok("I".to_string()));
    }

    #[test]
    fn test_can_convert_2019() {
        assert_eq!(convert(2019), Ok("MMXIX".to_string()));
    }

    #[test]
    fn test_can_convert_3999() {
        assert_eq!(convert(3999), Ok("MMMCMXCIX".to_string()));
    }

    #[test]
    fn test_should_fail_over_3999() {
        assert_eq!(convert(4000), Err("Invalid conversion above maximum limit of 3999".to_string()));
    }
}
