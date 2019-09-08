use super::dictionary::CONVERTIBLE;

pub fn convert(number_to_convert: u32) -> String {
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

    roman_number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_convert_zeros() {
        assert_eq!(convert(0), "");
    }

    #[test]
    fn test_can_convert_one() {
        assert_eq!(convert(1), "I");
    }

    #[test]
    fn test_can_convert_ww2_start_date() {
        assert_eq!(convert(1939), "MCMXXXIX");
    }

    #[test]
    fn test_can_convert_2019() {
        assert_eq!(convert(2019), "MMXIX");
    }
}
