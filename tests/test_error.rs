mod common;

use common::{read_final_stack, remove_test_file, run_forth_catch_output_test};

#[cfg(test)]
mod test_error {
    use super::*;

    #[test]
    fn test_underflow_1() {
        let script = "+";
        let script_file = "tests/test_underflow_1.fth";
        let test_stack = "tests/stack_underflow_1.fth";
        let test_stack_size = "stack-size={128000}";

        let output = run_forth_catch_output_test(script, script_file, test_stack_size, test_stack);

        assert!(
            output.contains("stack-underflow"),
            "Expected output to contain 'stack-underflow', but got:\n{}",
            output
        );

        let actual_stack = read_final_stack(test_stack);
        assert!(
            actual_stack.is_none(),
            "Expected empty stack, got: {:?}",
            actual_stack
        );

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_underflow_2() {
        let script = "1 +";
        let script_file = "tests/test_underflow_2.fth";
        let test_stack = "tests/stack_underflow_2.fth";
        let test_stack_size = "stack-size={128000}";

        let output = run_forth_catch_output_test(script, script_file, test_stack_size, test_stack);

        assert!(
            output.contains("stack-underflow"),
            "Expected output to contain 'stack-underflow', but got:\n{}",
            output
        );

        let actual_stack = read_final_stack(test_stack);
        assert!(
            actual_stack.is_none(),
            "Expected empty stack, got: {:?}",
            actual_stack
        );

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_underflow_3() {
        let script = "-";
        let script_file = "tests/test_underflow_3.fth";
        let test_stack = "tests/stack_underflow_3.fth";
        let test_stack_size = "stack-size={128000}";

        let output = run_forth_catch_output_test(script, script_file, test_stack_size, test_stack);

        assert!(
            output.contains("stack-underflow"),
            "Expected output to contain 'stack-underflow', but got:\n{}",
            output
        );

        let actual_stack = read_final_stack(test_stack);
        assert!(
            actual_stack.is_none(),
            "Expected empty stack, got: {:?}",
            actual_stack
        );

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_underflow_4() {
        let script = "1 -";
        let script_file = "tests/test_underflow_4.fth";
        let test_stack = "tests/stack_underflow_4.fth";
        let test_stack_size = "stack-size={128000}";

        let output = run_forth_catch_output_test(script, script_file, test_stack_size, test_stack);

        assert!(
            output.contains("stack-underflow"),
            "Expected output to contain 'stack-underflow', but got:\n{}",
            output
        );

        let actual_stack = read_final_stack(test_stack);
        assert!(
            actual_stack.is_none(),
            "Expected empty stack, got: {:?}",
            actual_stack
        );

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_underflow_5() {
        let script = "*";
        let script_file = "tests/test_underflow_5.fth";
        let test_stack = "tests/stack_underflow_5.fth";
        let test_stack_size = "stack-size={128000}";

        let output = run_forth_catch_output_test(script, script_file, test_stack_size, test_stack);

        assert!(
            output.contains("stack-underflow"),
            "Expected output to contain 'stack-underflow', but got:\n{}",
            output
        );

        let actual_stack = read_final_stack(test_stack);
        assert!(
            actual_stack.is_none(),
            "Expected empty stack, got: {:?}",
            actual_stack
        );

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_underflow_6() {
        let script = "1 *";
        let script_file = "tests/test_underflow_6.fth";
        let test_stack = "tests/stack_underflow_6.fth";
        let test_stack_size = "stack-size={128000}";

        let output = run_forth_catch_output_test(script, script_file, test_stack_size, test_stack);

        assert!(
            output.contains("stack-underflow"),
            "Expected output to contain 'stack-underflow', but got:\n{}",
            output
        );

        let actual_stack = read_final_stack(test_stack);
        assert!(
            actual_stack.is_none(),
            "Expected empty stack, got: {:?}",
            actual_stack
        );

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_underflow_7() {
        let script = "/";
        let script_file = "tests/test_underflow_7.fth";
        let test_stack = "tests/stack_underflow_7.fth";
        let test_stack_size = "stack-size={128000}";

        let output = run_forth_catch_output_test(script, script_file, test_stack_size, test_stack);

        assert!(
            output.contains("stack-underflow"),
            "Expected output to contain 'stack-underflow', but got:\n{}",
            output
        );

        let actual_stack = read_final_stack(test_stack);
        assert!(
            actual_stack.is_none(),
            "Expected empty stack, got: {:?}",
            actual_stack
        );

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_underflow_8() {
        let script = "1 /";
        let script_file = "tests/test_underflow_8.fth";
        let test_stack = "tests/stack_underflow_8.fth";
        let test_stack_size = "stack-size={128000}";

        let output = run_forth_catch_output_test(script, script_file, test_stack_size, test_stack);

        assert!(
            output.contains("stack-underflow"),
            "Expected output to contain 'stack-underflow', but got:\n{}",
            output
        );

        let actual_stack = read_final_stack(test_stack);
        assert!(
            actual_stack.is_none(),
            "Expected empty stack, got: {:?}",
            actual_stack
        );

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_underflow_9() {
        let script = "dup";
        let script_file = "tests/test_underflow_9.fth";
        let test_stack = "tests/stack_underflow_9.fth";
        let test_stack_size = "stack-size={128000}";

        let output = run_forth_catch_output_test(script, script_file, test_stack_size, test_stack);

        assert!(
            output.contains("stack-underflow"),
            "Expected output to contain 'stack-underflow', but got:\n{}",
            output
        );

        let actual_stack = read_final_stack(test_stack);
        assert!(
            actual_stack.is_none(),
            "Expected empty stack, got: {:?}",
            actual_stack
        );

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_underflow_10() {
        let script = "drop";
        let script_file = "tests/test_underflow_10.fth";
        let test_stack = "tests/stack_underflow_10.fth";
        let test_stack_size = "stack-size={128000}";

        let output = run_forth_catch_output_test(script, script_file, test_stack_size, test_stack);

        assert!(
            output.contains("stack-underflow"),
            "Expected output to contain 'stack-underflow', but got:\n{}",
            output
        );

        let actual_stack = read_final_stack(test_stack);
        assert!(
            actual_stack.is_none(),
            "Expected empty stack, got: {:?}",
            actual_stack
        );

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_underflow_11() {
        let script = "swap";
        let script_file = "tests/test_underflow_11.fth";
        let test_stack = "tests/stack_underflow_11.fth";
        let test_stack_size = "stack-size={128000}";

        let output = run_forth_catch_output_test(script, script_file, test_stack_size, test_stack);

        assert!(
            output.contains("stack-underflow"),
            "Expected output to contain 'stack-underflow', but got:\n{}",
            output
        );

        let actual_stack = read_final_stack(test_stack);
        assert!(
            actual_stack.is_none(),
            "Expected empty stack, got: {:?}",
            actual_stack
        );

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_underflow_12() {
        let script = "1 swap";
        let script_file = "tests/test_underflow_12.fth";
        let test_stack = "tests/stack_underflow_12.fth";
        let test_stack_size = "stack-size={128000}";

        let output = run_forth_catch_output_test(script, script_file, test_stack_size, test_stack);

        assert!(
            output.contains("stack-underflow"),
            "Expected output to contain 'stack-underflow', but got:\n{}",
            output
        );

        let actual_stack = read_final_stack(test_stack);
        assert!(
            actual_stack.is_none(),
            "Expected empty stack, got: {:?}",
            actual_stack
        );

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_underflow_13() {
        let script = "over";
        let script_file = "tests/test_underflow_13.fth";
        let test_stack = "tests/stack_underflow_13.fth";
        let test_stack_size = "stack-size={128000}";

        let output = run_forth_catch_output_test(script, script_file, test_stack_size, test_stack);

        assert!(
            output.contains("stack-underflow"),
            "Expected output to contain 'stack-underflow', but got:\n{}",
            output
        );

        let actual_stack = read_final_stack(test_stack);
        assert!(
            actual_stack.is_none(),
            "Expected empty stack, got: {:?}",
            actual_stack
        );

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_underflow_14() {
        let script = "1 over";
        let script_file = "tests/test_underflow_14.fth";
        let test_stack = "tests/stack_underflow_14.fth";
        let test_stack_size = "stack-size={128000}";

        let output = run_forth_catch_output_test(script, script_file, test_stack_size, test_stack);

        assert!(
            output.contains("stack-underflow"),
            "Expected output to contain 'stack-underflow', but got:\n{}",
            output
        );

        let actual_stack = read_final_stack(test_stack);
        assert!(
            actual_stack.is_none(),
            "Expected empty stack, got: {:?}",
            actual_stack
        );

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_division_by_zero() {
        let script = "4 0 /";
        let script_file = "tests/test_division_by_zero.fth";
        let test_stack = "tests/stack_division_by_zero.fth";
        let test_stack_size = "stack-size={128000}";

        let output = run_forth_catch_output_test(script, script_file, test_stack_size, test_stack);

        assert!(
            output.contains("division-by-zero"),
            "Expected output to contain 'stack-underflow', but got:\n{}",
            output
        );

        let actual_stack = read_final_stack(test_stack);
        assert!(
            actual_stack.is_none(),
            "Expected empty stack, got: {:?}",
            actual_stack
        );

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_invalid_word_1() {
        let script = ": 1 2 ;";
        let script_file = "tests/test_invalid_word_1.fth";
        let test_stack = "tests/stack_invalid_word_1.fth";
        let test_stack_size = "stack-size={128000}";

        let output = run_forth_catch_output_test(script, script_file, test_stack_size, test_stack);

        assert!(
            output.contains("invalid-word"),
            "Expected output to contain 'invalid-word', but got:\n{}",
            output
        );

        let actual_stack = read_final_stack(test_stack);
        assert!(
            actual_stack.is_none(),
            "Expected empty stack, got: {:?}",
            actual_stack
        );

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_invalid_word_2() {
        let script = ": -1 2 ;";
        let script_file = "tests/test_invalid_word_2.fth";
        let test_stack = "tests/stack_invalid_word_2.fth";
        let test_stack_size = "stack-size={128000}";

        let output = run_forth_catch_output_test(script, script_file, test_stack_size, test_stack);

        assert!(
            output.contains("invalid-word"),
            "Expected output to contain 'invalid-word', but got:\n{}",
            output
        );

        let actual_stack = read_final_stack(test_stack);
        assert!(
            actual_stack.is_none(),
            "Expected empty stack, got: {:?}",
            actual_stack
        );

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_unknown_word() {
        let script = "foo";
        let script_file = "tests/test_unknown_word.fth";
        let test_stack = "tests/stack_unknown_word.fth";
        let test_stack_size = "stack-size={128000}";

        let output = run_forth_catch_output_test(script, script_file, test_stack_size, test_stack);

        assert!(
            output.contains("?"),
            "Expected output to contain '?', but got:\n{}",
            output
        );

        let actual_stack = read_final_stack(test_stack);
        assert!(
            actual_stack.is_none(),
            "Expected empty stack, got: {:?}",
            actual_stack
        );

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_limited_stack() {
        let script = "1 2 3 4 5 . cr 5 6";
        let script_file = "tests/test_limited_stack.fth";
        let test_stack = "tests/stack_limited_stack.fth";
        let test_stack_size = "stack-size={10}";

        let output = run_forth_catch_output_test(script, script_file, test_stack_size, test_stack);
        assert!(
            output.contains("5\nstack-overflow\n"),
            "Expected output to contain 'stack-overflow', but got:\n{}",
            output
        );

        let expected_stack = vec!["1", "2", "3", "4", "5"];
        let actual_stack = read_final_stack(test_stack).unwrap();
        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }
}
