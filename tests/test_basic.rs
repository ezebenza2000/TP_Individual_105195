mod common;

use common::{read_final_stack, remove_test_file, run_forth_test};

//CMD: cargo test -- --test-threads=1

#[cfg(test)]
mod test_basic {
    use super::*;

    #[test]
    fn test_positive_numbers() {
        let script = "1 2 3 4 5";
        let script_file = "tests/test_positive_numers.fth";
        let test_stack = "tests/stack_positive_numers.fth";
        run_forth_test(script, script_file, test_stack);

        let expected_stack = vec!["1", "2", "3", "4", "5"];
        let actual_stack = read_final_stack(test_stack).unwrap();

        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_negative_numbers() {
        let script = "-1 -2 -3 -4 -5";
        let script_file = "tests/test_negative_numbers.fth";
        let test_stack = "tests/stack_negative_numbers.fth";

        run_forth_test(script, script_file, test_stack);

        let expected_stack = vec!["-1", "-2", "-3", "-4", "-5"];
        let actual_stack = read_final_stack(test_stack).unwrap();

        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_add_1() {
        let script = "1 2 +";
        let script_file = "tests/test_add_1.fth";
        let test_stack = "tests/stack_add_1.fth";

        run_forth_test(script, script_file, test_stack);

        let expected_stack = vec!["3"];
        let actual_stack = read_final_stack(test_stack).unwrap();

        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_add_2() {
        let script = "1 2 3 +";
        let script_file = "tests/test_add_2.fth";
        let test_stack = "tests/stack_add_2.fth";

        run_forth_test(script, script_file, test_stack);

        let expected_stack = vec!["1", "5"];
        let actual_stack = read_final_stack(test_stack).unwrap();

        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_sub_1() {
        let script = "3 4 -";
        let script_file = "tests/test_sub_1.fth";
        let test_stack = "tests/stack_sub_1.fth";

        run_forth_test(script, script_file, test_stack);

        let expected_stack = vec!["-1"];
        let actual_stack = read_final_stack(test_stack).unwrap();

        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_sub_2() {
        let script = "1 12 3 -";
        let script_file = "tests/test_sub_2.fth";
        let test_stack = "tests/stack_sub_2.fth";

        run_forth_test(script, script_file, test_stack);

        let expected_stack = vec!["1", "9"];
        let actual_stack = read_final_stack(test_stack).unwrap();

        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_mul_1() {
        let script = "2 4 *";
        let script_file = "tests/test_mul_1.fth";
        let test_stack = "tests/stack_mul_1.fth";

        run_forth_test(script, script_file, test_stack);

        let expected_stack = vec!["8"];
        let actual_stack = read_final_stack(test_stack).unwrap();

        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_mul_2() {
        let script = "1 2 3 *";
        let script_file = "tests/test_mul_2.fth";
        let test_stack = "tests/stack_mul_2.fth";

        run_forth_test(script, script_file, test_stack);

        let expected_stack = vec!["1", "6"];
        let actual_stack = read_final_stack(test_stack).unwrap();

        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_div_1() {
        let script = "12 3 /";
        let script_file = "tests/test_div_1.fth";
        let test_stack = "tests/stack_div_1.fth";

        run_forth_test(script, script_file, test_stack);

        let expected_stack = vec!["4"];
        let actual_stack = read_final_stack(test_stack).unwrap();

        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_div_2() {
        let script = "8 3 /";
        let script_file = "tests/test_div_2.fth";
        let test_stack = "tests/stack_div_2.fth";

        run_forth_test(script, script_file, test_stack);

        let expected_stack = vec!["2"];
        let actual_stack = read_final_stack(test_stack).unwrap();

        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_div_3() {
        let script = "1 12 3 /";
        let script_file = "tests/test_div_3.fth";
        let test_stack = "tests/stack_div_3.fth";

        run_forth_test(script, script_file, test_stack);

        let expected_stack = vec!["1", "4"];
        let actual_stack = read_final_stack(test_stack).unwrap();

        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_add_sub() {
        let script = "1 2 + 4 -";
        let script_file = "tests/test_add_sub.fth";
        let test_stack = "tests/stack_add_sub.fth";

        run_forth_test(script, script_file, test_stack);

        let expected_stack = vec!["-1"];
        let actual_stack = read_final_stack(test_stack).unwrap();

        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_mul_div() {
        let script = "2 4 * 3 /";
        let script_file = "tests/test_mul_div.fth";
        let test_stack = "tests/stack_mul_div.fth";

        run_forth_test(script, script_file, test_stack);

        let expected_stack = vec!["2"];
        let actual_stack = read_final_stack(test_stack).unwrap();

        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_mul_add() {
        let script = "1 3 4 * +";
        let script_file = "tests/test_mul_add.fth";
        let test_stack = "tests/stack_mul_add.fth";

        run_forth_test(script, script_file, test_stack);

        let expected_stack = vec!["13"];
        let actual_stack = read_final_stack(test_stack).unwrap();

        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_add_mul() {
        let script = "1 3 4 + *";
        let script_file = "tests/test_add_mul.fth";
        let test_stack = "tests/stack_add_mul.fth";

        run_forth_test(script, script_file, test_stack);

        let expected_stack = vec!["7"];
        let actual_stack = read_final_stack(test_stack).unwrap();

        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_dup_1() {
        let script = "1 dup";
        let script_file = "tests/test_dup_1.fth";
        let test_stack = "tests/stack_dup_1.fth";

        run_forth_test(script, script_file, test_stack);

        let expected_stack = vec!["1", "1"];
        let actual_stack = read_final_stack(test_stack).unwrap();

        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_dup_2() {
        let script = "1 2 dup";
        let script_file = "tests/test_dup_2.fth";
        let test_stack = "tests/stack_dup_2.fth";

        run_forth_test(script, script_file, test_stack);

        let expected_stack = vec!["1", "2", "2"];
        let actual_stack = read_final_stack(test_stack).unwrap();

        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_drop_1() {
        let script = "1 drop";
        let script_file = "tests/test_drop_1.fth";
        let test_stack = "tests/stack_drop_1.fth";

        run_forth_test(script, script_file, test_stack);

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
    fn test_drop_2() {
        let script = "1 2 drop";
        let script_file = "tests/test_drop_2.fth";
        let test_stack = "tests/stack_drop_2.fth";

        run_forth_test(script, script_file, test_stack);

        let expected_stack = vec!["1"];
        let actual_stack = read_final_stack(test_stack).unwrap();
        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_swap_1() {
        let script = "1 2 swap";
        let script_file = "tests/test_swap_1.fth";
        let test_stack = "tests/stack_swap_1.fth";

        run_forth_test(script, script_file, test_stack);

        let expected_stack = vec!["2", "1"];
        let actual_stack = read_final_stack(test_stack).unwrap();
        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_swap_2() {
        let script = "1 2 3 swap";
        let script_file = "tests/test_swap_2.fth";
        let test_stack = "tests/stack_swap_2.fth";

        run_forth_test(script, script_file, test_stack);

        let expected_stack = vec!["1", "3", "2"];
        let actual_stack = read_final_stack(test_stack).unwrap();
        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_over_1() {
        let script = "1 2 over";
        let script_file = "tests/test_over_1.fth";
        let test_stack = "tests/stack_over_1.fth";

        run_forth_test(script, script_file, test_stack);

        let expected_stack = vec!["1", "2", "1"];
        let actual_stack = read_final_stack(test_stack).unwrap();
        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_over_2() {
        let script = "1 2 3 over";
        let script_file = "tests/test_over_2.fth";
        let test_stack = "tests/stack_over_2.fth";

        run_forth_test(script, script_file, test_stack);

        let expected_stack = vec!["1", "2", "3", "2"];
        let actual_stack = read_final_stack(test_stack).unwrap();
        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_rot_1() {
        let script = "1 2 3 rot";
        let script_file = "tests/test_rot_1.fth";
        let test_stack = "tests/stack_rot_1.fth";

        run_forth_test(script, script_file, test_stack);

        let expected_stack = vec!["2", "3", "1"];
        let actual_stack = read_final_stack(test_stack).unwrap();
        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_rot_2() {
        let script = "1 2 3 rot rot rot";
        let script_file = "tests/test_rot_2.fth";
        let test_stack = "tests/stack_rot_2.fth";

        run_forth_test(script, script_file, test_stack);

        let expected_stack = vec!["1", "2", "3"];
        let actual_stack = read_final_stack(test_stack).unwrap();
        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_definition_1() {
        let script = ": dup-twice dup dup ; 1 dup-twice";
        let script_file = "tests/test_definition_1.fth";
        let test_stack = "tests/stack_definition_1.fth";

        run_forth_test(script, script_file, test_stack);

        let expected_stack = vec!["1", "1", "1"];
        let actual_stack = read_final_stack(test_stack).unwrap();
        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_definition_2() {
        let script = ": countup 1 2 3 ; countup";
        let script_file = "tests/test_definition_2.fth";
        let test_stack = "tests/stack_definition_2.fth";

        run_forth_test(script, script_file, test_stack);

        let expected_stack = vec!["1", "2", "3"];
        let actual_stack = read_final_stack(test_stack).unwrap();
        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_redefinition() {
        let script = ": foo dup ; : foo dup dup ; 1 foo";
        let script_file = "tests/test_redefinition.fth";
        let test_stack = "tests/stack_redefinition.fth";

        run_forth_test(script, script_file, test_stack);

        let expected_stack = vec!["1", "1", "1"];
        let actual_stack = read_final_stack(test_stack).unwrap();
        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_shadowing() {
        let script = ": swap dup ; 1 swap";
        let script_file = "tests/test_shadowing.fth";
        let test_stack = "tests/stack_shadowing.fth";

        run_forth_test(script, script_file, test_stack);

        let expected_stack = vec!["1", "1"];
        let actual_stack = read_final_stack(test_stack).unwrap();
        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_shadowing_symbol_1() {
        let script = ": + * ; 3 4 +";
        let script_file = "tests/test_shadowing_symbol_1.fth";
        let test_stack = "tests/stack_shadowing_symbol_1.fth";

        run_forth_test(script, script_file, test_stack);

        let expected_stack = vec!["12"];
        let actual_stack = read_final_stack(test_stack).unwrap();
        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_non_transitive() {
        let script = ": foo 5 ; : bar foo ; : foo 6 ; bar foo";
        let script_file = "tests/test_non_transitive.fth";
        let test_stack = "tests/stack_non_transitive.fth";

        run_forth_test(script, script_file, test_stack);

        let expected_stack = vec!["5", "6"];
        let actual_stack = read_final_stack(test_stack).unwrap();
        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_self_definition() {
        let script = ": foo 10 ; : foo foo 1 + ; foo";
        let script_file = "tests/test_self_definition.fth";
        let test_stack = "tests/stack_self_definition.fth";

        run_forth_test(script, script_file, test_stack);

        let expected_stack = vec!["11"];
        let actual_stack = read_final_stack(test_stack).unwrap();
        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_case_insensitive_1() {
        let script = "1 DUP Dup dup";
        let script_file = "tests/test_case_insensitive_1.fth";
        let test_stack = "tests/stack_case_insensitive_1.fth";

        run_forth_test(script, script_file, test_stack);

        let expected_stack = vec!["1", "1", "1", "1"];
        let actual_stack = read_final_stack(test_stack).unwrap();
        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_case_insensitive_2() {
        let script = "1 2 3 4 DROP Drop drop";
        let script_file = "tests/test_case_insensitive_2.fth";
        let test_stack = "tests/stack_case_insensitive_2.fth";

        run_forth_test(script, script_file, test_stack);

        let expected_stack = vec!["1"];
        let actual_stack = read_final_stack(test_stack).unwrap();
        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_case_insensitive_3() {
        let script = "1 2 SWAP 3 Swap 4 swap";
        let script_file = "tests/test_case_insensitive_3.fth";
        let test_stack = "tests/stack_case_insensitive_3.fth";

        run_forth_test(script, script_file, test_stack);

        let expected_stack = vec!["2", "3", "4", "1"];
        let actual_stack = read_final_stack(test_stack).unwrap();
        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_case_insensitive_4() {
        let script = "1 2 OVER Over over";
        let script_file = "tests/test_case_insensitive_4.fth";
        let test_stack = "tests/stack_case_insensitive_4.fth";

        run_forth_test(script, script_file, test_stack);

        let expected_stack = vec!["1", "2", "1", "2", "1"];
        let actual_stack = read_final_stack(test_stack).unwrap();
        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_case_insensitive_5() {
        let script = ": foo dup ; 1 FOO Foo foo";
        let script_file = "tests/test_case_insensitive_5.fth";
        let test_stack = "tests/stack_case_insensitive_5.fth";

        run_forth_test(script, script_file, test_stack);

        let expected_stack = vec!["1", "1", "1", "1"];
        let actual_stack = read_final_stack(test_stack).unwrap();
        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_case_insensitive_6() {
        let script = ": SWAP DUP Dup dup ; 1 swap";
        let script_file = "tests/test_case_insensitive_6.fth";
        let test_stack = "tests/stack_case_insensitive_6.fth";

        run_forth_test(script, script_file, test_stack);

        let expected_stack = vec!["1", "1", "1", "1"];
        let actual_stack = read_final_stack(test_stack).unwrap();
        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }
}
