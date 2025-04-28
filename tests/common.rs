use std::fs;
use std::process::Command;
use tp_1_individual::file;
use tp_1_individual::parse;

///Given a file_name this function remove it
pub fn remove_test_file(file_name: &str) {
    if fs::metadata(file_name).is_ok() {
        if let Err(e) = fs::remove_file(file_name) {
            eprintln!("Couldnt remove file '{}': {}", file_name, e);
        }
    } else {
        println!("No such file: '{}'", file_name);
    }
}

///Given a script, a test_file to to write the script and a test_stack to store output.
/// The function runs the main program
pub fn run_forth_test(script: &str, test_file: &str, test_stack_size: &str, test_stack: &str) {
    fs::write(test_file, script).expect("Failed to write test file");

    Command::new("cargo")
        .args(["run", "--", test_file, test_stack_size, test_stack])
        .output()
        .expect("Failed to execute process");
}

///Same as 'run_forth_test' but catches stout and return it as string
pub fn run_forth_catch_output_test(
    script: &str,
    test_file: &str,
    test_stack_size: &str,
    test_stack: &str,
) -> String {
    fs::write(test_file, script).expect("Failed to write test file");

    let output = Command::new("cargo")
        .args(["run", "--", test_file, test_stack_size, test_stack])
        .output()
        .expect("Failed to execute process");

    String::from_utf8_lossy(&output.stdout).to_string()
}

///Read the content of a given file
pub fn read_final_stack(stack_file: &str) -> Option<Vec<String>> {
    match file::read_file(stack_file) {
        Ok(lines) => {
            let parsed = parse::parse(lines);

            if parsed.is_empty() {
                None
            } else {
                Some(parsed)
            }
        }
        _ => None,
    }
}
