pub mod roman_converter;
pub mod arabic_converter;
pub mod dictionary;

#[cfg(test)]
mod tests {
    use crate::roman_converter::convert as roman_convert;
    use crate::arabic_converter::convert as arabic_convert;

    #[test]
    fn test_conversion_to_roman_1939() {
        let converted = roman_convert(1939);

        assert_eq!(converted, "MCMXXXIX");
    }

    #[test]
    fn test_conversion_to_arabic_1939() {
        let converted = arabic_convert("MCMXXXIX".to_string());

        assert_eq!(converted, 1939);
    }
}
