use super::dictionary::CONVERTIBLE;

pub fn convert(number_to_convert: &str) -> Result<u32, String> {
    let mut arabic_result = 0;

    let normalized_array = get_normalized_array(&number_to_convert);

    let mut curr_idx = 0;
    let max_idx = normalized_array.len() - 1;

    while curr_idx <= max_idx {
        if curr_idx + 1 <= max_idx {
            let key_candidate = normalized_array.get(curr_idx).unwrap().to_owned() + normalized_array.get(curr_idx + 1).unwrap();
            let symbol = get_symbol(&key_candidate);

            if symbol.is_some() {
                arabic_result = arabic_result + symbol.unwrap().1;
                curr_idx = curr_idx + 2;

                continue;
            }
        }

        let key_candidate = normalized_array.get(curr_idx).unwrap();
        let symbol = get_symbol(&key_candidate);

        if symbol.is_some() {
            arabic_result = arabic_result + symbol.unwrap().1;
            curr_idx = curr_idx + 1;

            continue;
        }

        return Err(format!("Symbol '{}' not found", key_candidate.to_string()));
    }

    Ok(arabic_result)
}

fn get_normalized_array(number_to_convert: &str) -> Vec<String>
{
    let normalized_string = number_to_convert.to_ascii_uppercase();
    let normalized_string = normalized_string.trim();

    if normalized_string == "" {
        return Vec::new();
    }

    normalized_string.split("")
        .filter(|s| s.to_string() != "")
        .map(|s| s.to_string())
        .collect()
}

fn get_symbol(symbol: &str) -> Option<(String, u32)>
{
    for s in CONVERTIBLE.iter() {
        if symbol == s.0 {
            return Some((s.0.to_string(), s.1));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_normalize_empty_array() {
        let result = get_normalized_array("");

        assert_eq!(result.is_empty(), true);
    }

    #[test]
    fn test_can_normalize_one_element_array() {
        let result = get_normalized_array("X");

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], "X");
    }

    #[test]
    fn test_can_normalize_multiple_elements_array() {
        let result = get_normalized_array("XL");

        assert_eq!(result.len(), 2);
        assert_eq!(result[0], "X");
        assert_eq!(result[1], "L");
    }

    #[test]
    fn test_positive_get_symbol() {
        let result = get_symbol("XL");

        assert_eq!(result.is_some(), true);

        let unwrapped = result.unwrap();

        assert_eq!(unwrapped.0, String::from("XL"));
        assert_eq!(unwrapped.1, 40);
    }

    #[test]
    fn test_negative_get_symbol() {
        let result = get_symbol("XXX");

        assert_eq!(result.is_none(), true);
    }

    #[test]
    fn test_can_convert_roman_numeral_to_integer() {
        let result = convert("X");

        assert_eq!(result, Ok(10));
    }

    #[test]
    fn test_invalid_symbol_will_return_error() {
        let result = convert("U");

        assert_eq!(result, Err("Symbol 'U' not found".to_string()));
    }
}
