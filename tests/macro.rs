#[macro_use]
extern crate validators;
#[cfg(feature = "rocketly")]
extern crate rocket;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validated_customized_string() {
        validated_customized_string!(S1,
            from_string input {
                Ok(input)
            },
            from_str input {
                Ok(input.to_string())
            }
        );

        validated_customized_string!(pub S2,
            from_string input {
                Ok(input)
            },
            from_str input {
                Ok(input.to_string())
            }
        );
    }

    #[test]
    fn test_validated_customized_regex_string() {
        validated_customized_regex_string!(S1, "^(Hi|Hello)$");
        validated_customized_regex_string!(pub S2, r"^[\S\s]+$");
    }

    #[test]
    fn test_validated_customized_number() {
        validated_customized_number!(N1, u8,
            from_string _input {
                Ok(5)
            },
            from_str _input {
                Ok(6)
            },
            from_number input {
                Ok(input)
            }
        );

        validated_customized_number!(pub N2, u16,
            from_string _input {
                Ok(5)
            },
            from_str _input {
                Ok(6)
            },
            from_number input {
                Ok(input)
            }
        );
    }

    #[test]
    fn test_validated_customized_regex_number() {
        validated_customized_regex_number!(N1, u8, r"^[1-8][0-9]$");
        validated_customized_regex_number!(pub N2, u16, r"^[0-1]?[1-8][0-9]$");
        validated_customized_regex_number!(N3, f32, r"^[0-1]?[1-8][0-9]\.[0-9]$");
    }

    #[test]
    fn test_validated_customized_ranged_number() {
        validated_customized_ranged_number!(N1, u8, 0, 100);
        validated_customized_ranged_number!(pub N2, u16, 3, 46);
        validated_customized_ranged_number!(N3, f32, -45.5, 80.0);
    }

    #[test]
    fn test_validated_customized_primitive_number() {
        validated_customized_primitive_number!(N1, u8);
        validated_customized_primitive_number!(pub N2, u8);
        validated_customized_primitive_number!(N3, f32);
    }
}