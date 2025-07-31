/// Implements `std::fmt::Display` for `$t` using its JSON serialization.
///
/// The macro assumes that `$t` implements `serde::Serialize`.
/// If the JSON serialization result is a simple quoted string (e.g. `"abc"`),
/// the surrounding quotes are removed so that `Display` prints `abc`.
#[macro_export]
macro_rules! impl_display_with_serialize {
    ($t:ty) => {
        impl std::fmt::Display for $t
        where
            $t: serde::Serialize,
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match serde_json::to_string(self) {
                    Ok(serialized) => {
                        // Try to interpret the JSON back. If it is a plain string, output the raw value.
                        match serde_json::from_str::<serde_json::Value>(&serialized) {
                            Ok(serde_json::Value::String(inner)) => write!(f, "{}", inner),
                            _ => write!(f, "{}", serialized),
                        }
                    }
                    Err(err) => write!(f, "<serialization error: {}>", err),
                }
            }
        }
    };
}
