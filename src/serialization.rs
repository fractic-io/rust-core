// Implements fmt::Display for the given type, outputing a kind of 'ID' string
// built deterministically from the type's serde serialization.
//
// Some examples of what these IDs look like:
// - ProductCode("example") -> "example"
// - SimpleEnum::VariantA -> "VariantA"
// - SimpleEnum::VariantB("with spaces") -> "VariantB_with-spaces"
// - ComplexEnum::VariantB { id: 42, name: "test".to_string() } -> "VariantB_{id_42,name_test}"
#[macro_export]
macro_rules! impl_deterministic_display_from_serde {
    ($type:ty) => {
        impl std::fmt::Display for $type {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                serde_json::to_string(self)
                    // If string, remove surrounding quotes.
                    .map(|s| {
                        if s.starts_with('"') && s.ends_with('"') {
                            s[1..s.len() - 1].to_string()
                        } else {
                            s
                        }
                    })
                    // If map, remove surrounding braces.
                    .map(|s| {
                        if s.starts_with('{') && s.ends_with('}') {
                            s[1..s.len() - 1].to_string()
                        } else {
                            s
                        }
                    })
                    // Replace special characters.
                    .map(|s| {
                        s.replace('\\', "~")
                            .replace('"', "")
                            .replace(':', "_")
                            .replace(' ', "-")
                            .to_string()
                    })
                    .map_err(|e| {
                        eprintln!(
                            "UNHANDLED SERIALIZATION ERROR\n{}.to_string() failed.\n{:?}",
                            stringify!($type),
                            e
                        );
                        std::fmt::Error
                    })?
                    .fmt(f)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    struct ProductCode(String);

    impl_deterministic_display_from_serde!(ProductCode);

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "lowercase")]
    enum SimpleEnum {
        ValueOne,
        #[serde(rename = "renamed_value")]
        ValueTwo,
        ValueThree,
    }

    impl_deterministic_display_from_serde!(SimpleEnum);

    #[derive(Serialize, Deserialize, Debug)]
    enum ComplexEnum {
        VariantA,
        VariantB { id: i32, name: String },
        VariantC(Vec<String>),
    }

    impl_deterministic_display_from_serde!(ComplexEnum);

    #[test]
    fn test_display_removes_surrounding_quotes_struct() {
        let code = ProductCode("example".to_string());
        assert_eq!(code.to_string(), "example");
    }

    #[test]
    fn test_display_preserves_internal_escaped_quotes_struct() {
        let code = ProductCode("ex\"ample".to_string());
        assert_eq!(code.to_string(), "ex~ample");
    }

    #[test]
    fn test_display_handles_empty_string_struct() {
        let code = ProductCode("".to_string());
        assert_eq!(code.to_string(), "");
    }

    #[test]
    fn test_display_simple_enum() {
        assert_eq!(SimpleEnum::ValueOne.to_string(), "valueone");
        assert_eq!(SimpleEnum::ValueTwo.to_string(), "renamed_value");
        assert_eq!(SimpleEnum::ValueThree.to_string(), "valuethree");
    }

    #[test]
    fn test_display_complex_enum() {
        let variant_a = ComplexEnum::VariantA;
        assert_eq!(variant_a.to_string(), "VariantA");

        let variant_b = ComplexEnum::VariantB {
            id: 42,
            name: "test".to_string(),
        };
        assert_eq!(variant_b.to_string(), "VariantB_{id_42,name_test}");

        let variant_c = ComplexEnum::VariantC(vec!["one".to_string(), "two".to_string()]);
        assert_eq!(variant_c.to_string(), "VariantC_[one,two]");
    }

    #[test]
    fn test_display_handles_special_characters_struct() {
        let code = ProductCode("text with \n newlines and \t tabs".to_string());
        assert_eq!(code.to_string(), "text-with-~n-newlines-and-~t-tabs");
    }
}
