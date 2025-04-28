mod common;

use common::{read_final_stack, remove_test_file, run_forth_catch_output_test, run_forth_test};

#[cfg(test)]
mod test_varied {
    use super::*;

    #[test]
    fn test_unit_computation_1() {
        let script = ": meter 100 * ; 
        : decimeter 10 * ; 
        : centimeter 1 * ; 
        1 meter 5 decimeter 2 centimeter + +";
        let script_file = "tests/test_unit_computation_1.fth";
        let test_stack = "tests/stack_unit_computation_1.fth";
        let test_stack_size = "stack-size={128000}";

        run_forth_test(script, script_file, test_stack_size, test_stack);

        let expected_stack = vec!["152"];
        let actual_stack = read_final_stack(test_stack).unwrap();

        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_unit_computation_2() {
        let script = ": seconds 1 * ; 
        : minutes 60 * seconds ; 
        : hours 60 * minutes ; 
        2 hours 13 minutes 5 seconds + +";
        let script_file = "tests/test_unit_computation_2.fth";
        let test_stack = "tests/stack_unit_computation_2.fth";
        let test_stack_size = "stack-size={128000}";

        run_forth_test(script, script_file, test_stack_size, test_stack);

        let expected_stack = vec!["7985"];
        let actual_stack = read_final_stack(test_stack).unwrap();

        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_constant_summation() {
        let script = ": one1 1 ; 
        : one2  one1 one1 ; 
        : one4  one2 one2 ; 
        : one8  one4 one4 ; 
        : one16 one8 one8 ; 
        : add1 + ; 
        : add2  add1 add1 ; 
        : add4  add2 add2 ; 
        : add8  add4 add4 ; 
        : add16 add8 add8 ; 
        0 one16 add16";
        let script_file = "tests/test_constant_summation.fth";
        let test_stack = "tests/stack_constant_summation.fth";
        let test_stack_size = "stack-size={128000}";

        run_forth_test(script, script_file, test_stack_size, test_stack);

        let expected_stack = vec!["16"];
        let actual_stack = read_final_stack(test_stack).unwrap();

        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_linear_summation() {
        let script = ": next1 dup 1 + ; 
        : next2  next1 next1 ; 
        : next4  next2 next2 ; 
        : next8  next4 next4 ; 
        : next16 next8 next8 ; 
        : add1 + ; 
        : add2  add1 add1 ; 
        : add4  add2 add2 ; 
        : add8  add4 add4 ; 
        : add16 add8 add8 ; 
        0 next16 add16";
        let script_file = "tests/test_linear_summation.fth";
        let test_stack = "tests/stack_linear_summation.fth";
        let test_stack_size = "stack-size={128000}";

        run_forth_test(script, script_file, test_stack_size, test_stack);

        let expected_stack = vec!["136"];
        let actual_stack = read_final_stack(test_stack).unwrap();

        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_geometric_summation() {
        let script = ": next1 dup 2 * ; 
        : next2  next1 next1 ; 
        : next4  next2 next2 ; 
        : next8  next4 next4 ; 
        : add1 + ; 
        : add2  add1 add1 ; 
        : add4  add2 add2 ; 
        : add8  add4 add4 ; 
        1 next8 add8";
        let script_file = "tests/test_geometric_summation.fth";
        let test_stack = "tests/stack_geometric_summation.fth";
        let test_stack_size = "stack-size={128000}";

        run_forth_test(script, script_file, test_stack_size, test_stack);

        let expected_stack = vec!["511"];
        let actual_stack = read_final_stack(test_stack).unwrap();

        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_power_of_2() {
        let script = ": next1 dup 2 * ; 
        : next2  next1 next1 ; 
        : next4  next2 next2 ; 
        : mul1 * ; 
        : mul2  mul1 mul1 ; 
        : mul4  mul2 mul2 ; 
        1 next4 mul4";
        let script_file = "tests/test_power_of_2.fth";
        let test_stack = "tests/stack_power_of_2.fth";
        let test_stack_size = "stack-size={128000}";

        run_forth_test(script, script_file, test_stack_size, test_stack);

        let expected_stack = vec!["1024"];
        let actual_stack = read_final_stack(test_stack).unwrap();

        assert_eq!(actual_stack, expected_stack);

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }

    #[test]
    fn test_digit_to_string() {
        let script = ": f dup 0 = if drop .\" zero\" else dup 1 = if drop .\" one\" else dup 2 = if drop .\" two\" then then then ; 0 f cr 1 f cr 2 f cr";
        let script_file = "tests/test_digit_to_string.fth";
        let test_stack = "tests/stack_digit_to_string.fth";
        let test_stack_size = "stack-size={128000}";

        run_forth_test(script, script_file, test_stack_size, test_stack);

        let actual_stack = read_final_stack(test_stack);
        assert!(
            actual_stack.is_none(),
            "Expected empty stack, got: {:?}",
            actual_stack
        );

        let output = run_forth_catch_output_test(script, script_file, test_stack_size, test_stack);

        let expected_output = "zero\none\ntwo\n";
        assert_eq!(
            output, expected_output,
            "Expected output to be 'zero\none\ntwo\n', but got:\n{}",
            output
        );

        remove_test_file(script_file);
        remove_test_file(test_stack);
    }
}
