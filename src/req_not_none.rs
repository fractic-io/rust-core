#[macro_export]
macro_rules! req_not_none {
    ($var:ident, $err_type:ident, $extra_dbg:expr) => {
        let Some($var) = $var else {
            return Err($err_type::with_debug(
                &concat!(stringify!($var), " cannot be empty."),
                &$extra_dbg(),
            ));
        };
    };
    ($var:ident, $err_type:ident) => {
        let Some($var) = $var else {
            return Err($err_type::new(&concat!(
                stringify!($var),
                " cannot be empty."
            )));
        };
    };
}

#[cfg(test)]
mod tests {
    use fractic_server_error::{define_internal_error, ServerError};

    define_internal_error!(InvalidConfig, "Invalid configuration: {details}.", { details: &str });

    #[test]
    fn test_require_not_none_ok() {
        let important_var = Some("String");
        let result: Result<&str, ServerError> = (|| {
            req_not_none!(important_var, InvalidConfig);
            Ok(important_var)
        })();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "String");
    }

    #[test]
    fn test_require_not_none_err() {
        let _important_var: Option<String> = None;
        let result: Result<(), ServerError> = (|| {
            req_not_none!(_important_var, InvalidConfig);
            Ok(())
        })();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("_important_var cannot be empty."),);
    }
}
