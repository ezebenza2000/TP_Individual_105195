mod common;

use common::{read_final_stack, remove_test_file, run_forth_catch_output_test};

#[cfg(test)]
mod test_print {
    use super::*;

    #[test]
    fn test_dot_without_leftover() {
        let script = "1 2 . .";
        let script_file = "tests/test_dot_without_leftover.fth";
        let test_stack = "tests/stack_dot_without_leftover.fth";

        let output = run_forth_catch_output_test(script, script_file, test_stack);

        assert_eq!(
            output, " 2 1",
            "Expected output to be '2 1', but got:\n{}",
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
    fn test_dot_with_leftover() {
        let script = "1 2 3 4 5 . . .";
        let script_file = "tests/test_dot_with_leftover.fth";
        let test_stack = "tests/stack_dot_with_leftover.fth";

        let output = run_forth_catch_output_test(script, script_file, test_stack);

        assert_eq!(
            output, " 5 4 3",
            "Expected output to be ' 5 4 3', but got:\n{}",
            output
        );

        let expected_stack = vec!["1", "2"];
        let actual_stack = read_final_stack(test_stack).unwrap();

        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_cr_1() {
        let script = "cr";
        let script_file = "tests/test_cr_1.fth";
        let test_stack = "tests/stack_cr_1.fth";

        let output = run_forth_catch_output_test(script, script_file, test_stack);

        assert_eq!(
            output, "\n",
            "Expected output to be '\n', but got:\n{}",
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
    fn test_cr_2() {
        let script = "cr cr";
        let script_file = "tests/test_cr_2.fth";
        let test_stack = "tests/stack_cr_2.fth";

        let output = run_forth_catch_output_test(script, script_file, test_stack);

        assert_eq!(
            output, "\n\n",
            "Expected output to be '\n\n', but got:\n{}",
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
    fn test_dot_and_cr() {
        let script = "1 . cr cr 2 .";
        let script_file = "tests/test_dot_and_cr.fth";
        let test_stack = "tests/stack_dot_and_cr.fth";

        let output = run_forth_catch_output_test(script, script_file, test_stack);

        assert_eq!(
            output, " 1\n\n 2",
            "Expected output to be ' 1\n\n 2', but got:\n{}",
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
    fn test_emit_uppercase() {
        let script = "65 emit";
        let script_file = "tests/test_emit_uppercase.fth";
        let test_stack = "tests/stack_emit_uppercase.fth";

        let output = run_forth_catch_output_test(script, script_file, test_stack);

        assert_eq!(
            output, " A",
            "Expected output to be 'A', but got:\n{}",
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
    fn test_emit_lowercase() {
        let script = "97 emit";
        let script_file = "tests/test_emit_lowercase.fth";
        let test_stack = "tests/stack_emit_lowercase.fth";

        let output = run_forth_catch_output_test(script, script_file, test_stack);

        assert_eq!(
            output, " a",
            "Expected output to be 'a', but got:\n{}",
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
    fn test_emit_multiple() {
        let script = "68 67 66 65 emit emit emit emit";
        let script_file = "tests/test_emit_multiple.fth";
        let test_stack = "tests/stack_emit_multiple.fth";

        let output = run_forth_catch_output_test(script, script_file, test_stack);

        assert_eq!(
            output, " A B C D",
            "Expected output to be ' A B C D', but got:\n{}",
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
    fn test_do_quote_hello_world() {
        let script = ".\" hello world\"";
        let script_file = "tests/test_do_quote_hello_world.fth";
        let test_stack = "tests/stack_do_quote_hello_world.fth";

        let output = run_forth_catch_output_test(script, script_file, test_stack);

        assert_eq!(
            output, "hello world",
            "Expected output to be 'hello world', but got:\n{}",
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
}
