mod common;

use common::{read_final_stack, remove_test_file, run_forth_test};

//CMD: cargo test -- --test-threads=1

#[cfg(test)]
mod test_conditionals {
    use super::*;

    #[test]
    fn test_equals_true() {
        let script = "1 1 =";
        let script_file = "tests/test_equals_true.fth";
        let test_stack = "tests/stack_equals_true.fth";

        run_forth_test(script, script_file, test_stack);

        let expected_stack = vec!["-1"];
        let actual_stack = read_final_stack(test_stack).unwrap();

        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_equals_false() {
        let script = "1 2 =";
        let script_file = "tests/test_equals_false.fth";
        let test_stack = "tests/stack_equals_false.fth";

        run_forth_test(script, script_file, test_stack);

        let expected_stack = vec!["0"];
        let actual_stack = read_final_stack(test_stack).unwrap();

        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_less_true() {
        let script = "1 2 <";
        let script_file = "tests/test_less_true.fth";
        let test_stack = "tests/stack_less_true.fth";

        run_forth_test(script, script_file, test_stack);

        let expected_stack = vec!["-1"];
        let actual_stack = read_final_stack(test_stack).unwrap();

        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_less_false() {
        let script = "2 1 <";
        let script_file = "tests/test_less_false.fth";
        let test_stack = "tests/stack_less_false.fth";

        run_forth_test(script, script_file, test_stack);

        let expected_stack = vec!["0"];
        let actual_stack = read_final_stack(test_stack).unwrap();

        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_less_equals() {
        let script = "2 2 <";
        let script_file = "tests/test_less_equals.fth";
        let test_stack = "tests/stack_less_equals.fth";

        run_forth_test(script, script_file, test_stack);

        let expected_stack = vec!["0"];
        let actual_stack = read_final_stack(test_stack).unwrap();

        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_more_true() {
        let script = "2 1 >";
        let script_file = "tests/test_more_true.fth";
        let test_stack = "tests/stack_more_true.fth";

        run_forth_test(script, script_file, test_stack);

        let expected_stack = vec!["-1"];
        let actual_stack = read_final_stack(test_stack).unwrap();

        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_more_false() {
        let script = "1 2 >";
        let script_file = "tests/test_more_false.fth";
        let test_stack = "tests/stack_more_false.fth";

        run_forth_test(script, script_file, test_stack);

        let expected_stack = vec!["0"];
        let actual_stack = read_final_stack(test_stack).unwrap();

        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_more_equals() {
        let script = "2 2 >";
        let script_file = "tests/test_more_equals.fth";
        let test_stack = "tests/stack_more_equals.fth";

        run_forth_test(script, script_file, test_stack);

        let expected_stack = vec!["0"];
        let actual_stack = read_final_stack(test_stack).unwrap();

        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_and_none() {
        let script = "0 0 and";
        let script_file = "tests/test_and_none.fth";
        let test_stack = "tests/stack_and_none.fth";

        run_forth_test(script, script_file, test_stack);

        let expected_stack = vec!["0"];
        let actual_stack = read_final_stack(test_stack).unwrap();

        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_and_one() {
        let script = "-1 0 and";
        let script_file = "tests/test_and_one.fth";
        let test_stack = "tests/stack_and_one.fth";

        run_forth_test(script, script_file, test_stack);

        let expected_stack = vec!["0"];
        let actual_stack = read_final_stack(test_stack).unwrap();

        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_and_both() {
        let script = "-1 -1 and";
        let script_file = "tests/test_and_both.fth";
        let test_stack = "tests/stack_and_both.fth";

        run_forth_test(script, script_file, test_stack);

        let expected_stack = vec!["-1"];
        let actual_stack = read_final_stack(test_stack).unwrap();

        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_or_none() {
        let script = "0 0 or";
        let script_file = "tests/test_or_none.fth";
        let test_stack = "tests/stack_or_none.fth";

        run_forth_test(script, script_file, test_stack);

        let expected_stack = vec!["0"];
        let actual_stack = read_final_stack(test_stack).unwrap();

        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_or_one() {
        let script = "-1 0 or";
        let script_file = "tests/test_or_one.fth";
        let test_stack = "tests/stack_or_one.fth";

        run_forth_test(script, script_file, test_stack);

        let expected_stack = vec!["-1"];
        let actual_stack = read_final_stack(test_stack).unwrap();

        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_or_both() {
        let script = "-1 -1 or";
        let script_file = "tests/test_or_both.fth";
        let test_stack = "tests/stack_or_both.fth";

        run_forth_test(script, script_file, test_stack);

        let expected_stack = vec!["-1"];
        let actual_stack = read_final_stack(test_stack).unwrap();

        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_not_true() {
        let script = "-1 not";
        let script_file = "tests/test_not_true.fth";
        let test_stack = "tests/stack_not_true.fth";

        run_forth_test(script, script_file, test_stack);

        let expected_stack = vec!["0"];
        let actual_stack = read_final_stack(test_stack).unwrap();

        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_not_false() {
        let script = "0 not";
        let script_file = "tests/test_not_false.fth";
        let test_stack = "tests/stack_not_false.fth";

        run_forth_test(script, script_file, test_stack);

        let expected_stack = vec!["-1"];
        let actual_stack = read_final_stack(test_stack).unwrap();

        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_not_not() {
        let script = "10 not not";
        let script_file = "tests/test_not_not.fth";
        let test_stack = "tests/stack_not_not.fth";

        run_forth_test(script, script_file, test_stack);

        let expected_stack = vec!["-1"];
        let actual_stack = read_final_stack(test_stack).unwrap();

        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }
}
