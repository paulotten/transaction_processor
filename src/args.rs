use std::env;

/*
Processes command line args.
Returns input filename or an error
*/
pub fn process_args() -> Result<String, &'static str> {
    process_args_impl(env::args().collect())
}

fn process_args_impl(mut args: Vec<String>) -> Result<String, &'static str> {
    if args.len() == 2 {
        // filename
        Ok(args.remove(1))
    } else {
        Err("Usage: cargo run -- INPUT_FILENAME")
    }
}

#[cfg(test)]
mod tests {
    use super::process_args_impl;

    #[test]
    fn too_few_args() {
        // 0 args
        assert!(process_args_impl(vec![]).is_err());

        // 1 arg
        assert!(process_args_impl(vec!["1".to_string()]).is_err());
    }

    #[test]
    fn correct_number_of_args() {
        // 2 args
        assert_eq!(
            process_args_impl(vec!["program".to_string(), "filename".to_string()]),
            Ok("filename".to_string()),
        );
    }

    #[test]
    fn too_many_args() {
        // 3 args
        assert!(
            process_args_impl(vec!["1".to_string(), "2".to_string(), "3".to_string(),]).is_err()
        );
    }
}
