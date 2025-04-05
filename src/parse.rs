//PRE: Receive a string.
//POST: Return an array of the string split by empty space.
pub fn parse(line: String) -> Vec<String> {
    let mut elements = Vec::new();

    for word in line.split(' ') {
        elements.push(word.to_string());
    }

    elements
}
