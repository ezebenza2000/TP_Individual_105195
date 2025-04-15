mod common;

use common::{read_final_stack, remove_test_file, run_forth_test};

#[cfg(test)]
mod test_heavy {
    use super::*;

    #[test]
    fn test_do_not_clone() {
        let script = ": word1 1 ;
    : word2 word1 ;
    : word4 word2 ;
    : word8 word4 ;
    : word16 word8 ;
    : word32 word16 ;
    : word64 word32 ;
    : word128 word64 ;
    : word256 word128 ;
    : word512 word256 ;
    : word1024 word512 ;
    : word2048 word1024 ;
    : word4096 word2048 ;
    : word8192 word4096 ;
    : word16384 word8192 ;
    : word32768 word16384 ;
    : word65536 word32768 ;
    : word131072 word65536 ;
    : word262144 word131072 ;
    : word524288 word262144 ;
    : word1048576 word524288 ;
    : word2097152 word1048576 ;
    : word4194304 word2097152 ;
    : word8388608 word4194304 ;
    : word16777216 word8388608 ;
    : word33554432 word16777216 ;
    : word67108864 word33554432 ;
    : word134217728 word67108864 ;";

        let script_file = "tests/test_do_not_clone.fth";
        let test_stack = "tests/stack_do_not_clone.fth";

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
}
