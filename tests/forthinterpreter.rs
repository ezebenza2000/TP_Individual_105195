use std::fs;
use std::process::Command;

//CMD: cargo test -- --test-threads=1

/// Ejecuta un script Forth y devuelve su salida estándar como String
fn run_forth_test(script: &str) -> String {
    let test_file = "tests/temp_test.fth";

    // Escribir el script en un archivo temporal
    fs::write(test_file, script).expect("Failed to write test file");

    // Ejecutar el intérprete
    let output = Command::new("cargo")
        .args(["run", "--", test_file])
        .output()
        .expect("Failed to execute process");

    // Retornar la salida estándar como String
    String::from_utf8_lossy(&output.stdout).to_string()
}
#[cfg(test)]
mod test_basic {
    use super::*;
    #[test]
    fn test_forth_addition() {
        let output = run_forth_test("10 20 + .");
        assert!(
            output.contains("30"),
            "Expected output to contain 30, but got:\n{}",
            output
        );
    }

    #[test]
    fn test_forth_multiplication() {
        let output = run_forth_test("5 6 * .");
        assert!(
            output.contains("30"),
            "Expected output to contain 30, but got:\n{}",
            output
        );
    }

    #[test]
    fn test_forth_subtraction() {
        let output = run_forth_test("50 20 - .");
        assert!(
            output.contains("30"),
            "Expected output to contain 30, but got:\n{}",
            output
        );
    }

    #[test]
    fn test_forth_division() {
        let output = run_forth_test("50 25 / .");
        assert!(
            output.contains("2"),
            "Expected output to contain 2, but got:\n{}",
            output
        );
    }

    #[test]
    fn test_forth_division_by_cero() {
        let output = run_forth_test("10 0 /");
        assert!(
            output.contains("Error: Divide by CERO"),
            "Expected: 'Error: Divide by CERO', but got:\n{}",
            output
        );
    }

    #[test]
    fn test_positive_number() {
        let output = run_forth_test("1 2 3 4 5 .");
        assert!(
            output.contains("5"),
            "Expected output to contain 5 , but got:\n{}",
            output
        );
    }

    #[test]
    fn test_negative_number() {
        let output = run_forth_test("-1 -2 -3 -4 -5 .");
        assert!(
            output.contains("-5"),
            "Expected output to contain -5 , but got:\n{}",
            output
        );
    }

    #[test]
    fn test_add_1() {
        let output = run_forth_test("1 2 + .");
        assert!(
            output.contains("3"),
            "Expected output to contain 3 , but got:\n{}",
            output
        );
    }

    #[test]
    fn test_add_2() {
        let output = run_forth_test("1 2 3 + .");
        assert!(
            output.contains("5"),
            "Expected output to contain 5 , but got:\n{}",
            output
        );
    }

    #[test]
    fn test_sub_1() {
        let output = run_forth_test("3 4 - .");
        assert!(
            output.contains("-1"),
            "Expected output to contain -1 , but got:\n{}",
            output
        );
    }

    #[test]
    fn test_sub_2() {
        let output = run_forth_test("1 12 3 - .");
        assert!(
            output.contains("9"),
            "Expected output to contain 9 , but got:\n{}",
            output
        );
    }

    #[test]
    fn test_mul_1() {
        let output = run_forth_test("2 4 * .");
        assert!(
            output.contains("8"),
            "Expected output to contain 8 , but got:\n{}",
            output
        );
    }

    #[test]
    fn test_mul_2() {
        let output = run_forth_test("1 2 3 * .");
        assert!(
            output.contains("6"),
            "Expected output to contain 6 , but got:\n{}",
            output
        );
    }

    #[test]
    fn test_div_1() {
        let output = run_forth_test("12 3 / .");
        assert!(
            output.contains("4"),
            "Expected output to contain 4 , but got:\n{}",
            output
        );
    }

    #[test]
    fn test_div_2() {
        let output = run_forth_test("8 3 / .");
        assert!(
            output.contains("2"),
            "Expected output to contain 2 , but got:\n{}",
            output
        );
    }

    #[test]
    fn test_div_3() {
        let output = run_forth_test("1 12 3 / .");
        assert!(
            output.contains("4"),
            "Expected output to contain 4 , but got:\n{}",
            output
        );
    }

    #[test]
    fn test_add_sub() {
        let output = run_forth_test("1 2 + 4 - .");
        assert!(
            output.contains("-1"),
            "Expected output to contain -1 , but got:\n{}",
            output
        );
    }

    #[test]
    fn test_mul_div() {
        let output = run_forth_test("2 4 * 3 / .");
        assert!(
            output.contains("2"),
            "Expected output to contain 2 , but got:\n{}",
            output
        );
    }

    #[test]
    fn test_mul_add() {
        let output = run_forth_test("1 3 4 * + .");
        assert!(
            output.contains("13"),
            "Expected output to contain 13 , but got:\n{}",
            output
        );
    }

    #[test]
    fn test_add_mul() {
        let output = run_forth_test("1 3 4 + * .");
        assert!(
            output.contains("7"),
            "Expected output to contain 7 , but got:\n{}",
            output
        );
    }

    #[test]
    fn test_dup_1() {
        let output = run_forth_test("1 dup .");
        assert!(
            output.contains("1"),
            "Expected output to contain 1 , but got:\n{}",
            output
        );
    }

    #[test]
    fn test_dup_2() {
        let output = run_forth_test("1 2 dup .");
        assert!(
            output.contains("2"),
            "Expected output to contain2 , but got:\n{}",
            output
        );
    }

    #[test]
    fn test_drop_1() {
        let output = run_forth_test("1 DROP .");
        assert!(
            output.contains("Error: Couldnt print valid value"),
            "Expected output to contain , but got:\n{}",
            output
        );
    }

    #[test]
    fn test_drop_2() {
        let output = run_forth_test("1 2 DROP .");
        assert!(
            output.contains("1"),
            "Expected output to contain 1 , but got:\n{}",
            output
        );
    }

    #[test]
    fn test_swap_1() {
        let output = run_forth_test("1 2 SWAP .");
        assert!(
            output.contains("1"),
            "Expected output to contain 1 , but got:\n{}",
            output
        );
    }
}

#[cfg(test)]
mod test_conditionals {
    use super::*;

    #[test]
    fn test_equals_true() {
        let output = run_forth_test("1 1 = .");
        assert!(
            output.contains("1"),
            "Expected output to contain 1 , but got:\n{}",
            output
        );
    }

    #[test]
    fn test_equals_false() {
        let output = run_forth_test("1 2 = .");
        assert!(
            output.contains("0"),
            "Expected output to contain 0 , but got:\n{}",
            output
        );
    }

    #[test]
    fn test_less_true() {
        let output = run_forth_test("1 2 < .");
        assert!(
            output.contains("1"),
            "Expected output to contain 1 , but got:\n{}",
            output
        );
    }

    #[test]
    fn test_less_false() {
        let output = run_forth_test("2 1 < .");
        assert!(
            output.contains("0"),
            "Expected output to contain 0 , but got:\n{}",
            output
        );
    }

    #[test]
    fn test_less_equals() {
        let output = run_forth_test("2 2 < .");
        assert!(
            output.contains("0"),
            "Expected output to contain 0 , but got:\n{}",
            output
        );
    }

    #[test]
    fn test_more_true() {
        let output = run_forth_test("2 1 > .");
        assert!(
            output.contains("1"),
            "Expected output to contain 1 , but got:\n{}",
            output
        );
    }

    #[test]
    fn test_more_false() {
        let output = run_forth_test("1 2 > .");
        assert!(
            output.contains("0"),
            "Expected output to contain 0 , but got:\n{}",
            output
        );
    }

    #[test]
    fn test_more_equals() {
        let output = run_forth_test("2 2 > .");
        assert!(
            output.contains("0"),
            "Expected output to contain 0 , but got:\n{}",
            output
        );
    }

    #[test]
    fn test_and_none() {
        let output = run_forth_test("0 0 and .");
        assert!(
            output.contains("0"),
            "Expected output to contain 0 , but got:\n{}",
            output
        );
    }

    #[test]
    fn test_and_one() {
        let output = run_forth_test("-1 0 and .");
        assert!(
            output.contains("0"),
            "Expected output to contain 0 , but got:\n{}",
            output
        );
    }

    #[test]
    fn test_and_both() {
        let output = run_forth_test("-1 -1 and .");
        assert!(
            output.contains("1"),
            "Expected output to contain 1 , but got:\n{}",
            output
        );
    }

    #[test]
    fn test_or_none() {
        let output = run_forth_test("0 0 or .");
        assert!(
            output.contains("0"),
            "Expected output to contain 0 , but got:\n{}",
            output
        );
    }

    #[test]
    fn test_or_one() {
        let output = run_forth_test("-1 0 OR .");
        assert!(
            output.contains("1"),
            "Expected output to contain 1 , but got:\n{}",
            output
        );
    }

    #[test]
    fn test_or_both() {
        let output = run_forth_test("-1 -1 or .");
        assert!(
            output.contains("1"),
            "Expected output to contain 1 , but got:\n{}",
            output
        );
    }

    #[test]
    fn test_not_true() {
        let output = run_forth_test("-1 NOT .");
        assert!(
            output.contains("0"),
            "Expected output to contain 0 , but got:\n{}",
            output
        );
    }

    #[test]
    fn test_not_false() {
        let output = run_forth_test("0 NOT .");
        assert!(
            output.contains("1"),
            "Expected output to contain 1 , but got:\n{}",
            output
        );
    }

    #[test]
    fn test_not_not() {
        let output = run_forth_test("10 NOT NOT .");
        assert!(
            output.contains("1"),
            "Expected output to contain 1 , but got:\n{}",
            output
        );
    }
}

#[cfg(test)]
mod test_ifs {
    use super::*;

    #[test]
    fn test_if_simple() {
        let output = run_forth_test(": f if 2 then ; -1 f .");
        assert!(
            output.contains("2"),
            "Expected output to contain 2, but got:\n{}",
            output
        );
    }

    #[test]
    fn test_if_else() {
        let output = run_forth_test(": f if 2 else 3 then ; -1 f 0 f .");
        assert!(
            output.contains("3"),
            "Expected output to contain 3, but got:\n{}",
            output
        );
    }

    #[test]
    fn test_if_non_canonical() {
        let output = run_forth_test(": f if 10 then ; 5 f .");
        assert!(
            output.contains("10"),
            "Expected output to contain 10, but got:\n{}",
            output
        );
    }
}

#[cfg(test)]
mod test_print {
    use super::*;
    #[test]
    fn test_dot_without_leftover() {
        let output = run_forth_test("1 2 . .");
        assert!(
            output.contains("1"),
            "Expected output to be '1', but got:\n{}",
            output
        );
    }

    #[test]
    fn test_dot_with_leftover() {
        let output = run_forth_test("1 2 3 4 5 . . .");
        assert!(
            output.contains("3"),
            "Expected output to be '5 4 3', but got:\n{}",
            output
        );
    }

    #[test]
    fn test_cr_1() {
        let output = run_forth_test("CR");
        assert!(
            output.contains(""),
            "Expected output to be '\\n', but got:\n{}",
            output
        );
    }

    #[test]
    fn test_cr_2() {
        let output = run_forth_test("CR CR");
        assert!(
            output.contains(""),
            "Expected output to be '\\n\\n', but got:\n{}",
            output
        );
    }

    #[test]
    fn test_dot_and_cr() {
        let output = run_forth_test("1 . cr cr 2 .");
        assert!(
            output.contains("2"),
            "Expected output to be '2', but got:\n{}",
            output
        );
    }

    #[test]
    fn test_emit_uppercase() {
        let output = run_forth_test("65 EMIT");
        assert!(
            output.contains("A"),
            "Expected output to be 'A', but got:\n{}",
            output
        );
    }

    #[test]
    fn test_emit_lowercase() {
        let output = run_forth_test("97 EMIT");
        assert!(
            output.contains("a"),
            "Expected output to be 'a', but got:\n{}",
            output
        );
    }

    #[test]
    fn test_emit_multiple() {
        let output = run_forth_test("68 67 66 65 EMIT EMIT EMIT EMIT");
        assert!(
            output.contains("D"),
            "Expected output to be 'D', but got:\n{}",
            output
        );
    }
}
