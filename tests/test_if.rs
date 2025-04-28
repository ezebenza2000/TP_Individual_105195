mod common;

use common::{read_final_stack, remove_test_file, run_forth_test};

#[cfg(test)]
mod test_varied {
    use super::*;

    #[test]
    fn test_if_simple() {
        let script = ": f if 2 then ; -1 f";
        let script_file = "tests/test_if_simple.fth";
        let test_stack = "tests/stack_if_simple.fth";
        let test_stack_size = "stack-size={128000}";

        run_forth_test(script, script_file, test_stack_size, test_stack);

        let expected_stack = vec!["2"];
        let actual_stack = read_final_stack(test_stack).unwrap();

        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_if_else() {
        let script = ": f if 2 else 3 then ;
    -1 f
    0 f";
        let script_file = "tests/test_if_else.fth";
        let test_stack = "tests/stack_if_else.fth";
        let test_stack_size = "stack-size={128000}";

        run_forth_test(script, script_file, test_stack_size, test_stack);

        let expected_stack = vec!["2", "3"];
        let actual_stack = read_final_stack(test_stack).unwrap();

        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_nested_if() {
        let script = ": f if if 1 else 2 then else drop 3 then ; -1 -1 f 0 -1 f 0 0 f";
        let script_file = "tests/test_nested_if.fth";
        let test_stack = "tests/stack_nested_if.fth";
        let test_stack_size = "stack-size={128000}";

        run_forth_test(script, script_file, test_stack_size, test_stack);

        let expected_stack = vec!["1", "2", "3"];
        let actual_stack = read_final_stack(test_stack).unwrap();

        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_nested_if_else() {
        let script =
            ": f dup 0 = if drop 2 else dup 1 = if drop 3 else drop 4 then then ; 0 f 1 f 2 f";
        let script_file = "tests/test_nested_if_else.fth";
        let test_stack = "tests/stack_nested_if_else.fth";
        let test_stack_size = "stack-size={128000}";

        run_forth_test(script, script_file, test_stack_size, test_stack);

        let expected_stack = vec!["2", "3", "4"];
        let actual_stack = read_final_stack(test_stack).unwrap();

        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_if_non_cannonical() {
        let script = ": f if 10 then ; 5 f";
        let script_file = "tests/test_if_non_cannonical.fth";
        let test_stack = "tests/stack_if_non_cannonical.fth";
        let test_stack_size = "stack-size={128000}";

        run_forth_test(script, script_file, test_stack_size, test_stack);

        let expected_stack = vec!["10"];
        let actual_stack = read_final_stack(test_stack).unwrap();

        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }
}
