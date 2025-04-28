pub fn parse(line: String) -> Vec<String> {
    let mut elements = Vec::new();
    let mut auxiliar_string = String::new();
    let mut chars = line.chars().peekable();

    while let Some(char) = chars.next() {
        if char == ' ' {
            if !auxiliar_string.is_empty() {
                elements.push(auxiliar_string.to_lowercase());
                auxiliar_string = String::new();
            }
        }
        // Cuando encuentra el punto seguido de comilla (.")
        else if char == '.' && chars.peek() == Some(&'"') {
            auxiliar_string.push(char); // Se guarda el .
            chars.next(); // Consume la comilla doble
            auxiliar_string.push('"'); // Guarda la comilla doble
            elements.push(auxiliar_string.to_lowercase());
            auxiliar_string = String::new();
            chars.next();
            // Ahora iteramos hasta encontrar el cierre de la comilla

            for c in chars.by_ref() {
                if c == '"' {
                    break;
                }
                auxiliar_string.push(c);
            }

            // Al terminar con el texto entre comillas, guardamos en elements
            elements.push(auxiliar_string.to_lowercase());
            auxiliar_string = String::new(); // Reinicio el string auxiliar
        }
        // En caso contrario, seguimos acumulando el carácter en auxiliar_string
        else {
            auxiliar_string.push(char);
        }
    }

    // Si al final hay algo en auxiliar_string, lo agregamos
    if !auxiliar_string.is_empty() {
        elements.push(auxiliar_string.to_lowercase());
    }

    elements
}

pub fn parse_stack_len(argument: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let mut chain = String::new();
    let mut inside = false;

    for element in argument.chars() {
        if element == '{' {
            inside = true;
            continue;
        }
        if element == '}' {
            break;
        }
        if inside {
            chain.push(element);
        }
    }

    let value = chain.parse::<usize>()?;
    Ok(value)
}

///FUNCTION: parse_conditional_blocks( iter: &mut impl Iterator<Item = String> ) -> (Vec<String>, Option<Vec<String>>)
///
///This function iterates over a sequence of Strings tokens and extracts the token belonging to
/// the first level 'IF' block and if it exist the 'ELSE' block.
///
/// # Parameters
///
/// - `iter`: A mutable iterator over a sequence of `String` tokens.
///
/// # Returns
///
/// A tuple containing:
/// - `Vec<String>`: The tokens corresponding to the `IF` block.
/// - `Option<Vec<String>>`: The tokens corresponding to the `ELSE` block, if it exists.
///   If no `ELSE` block is found, returns `None`.
///
/// # Behavior
///
/// - Detects `IF` and `THEN` tokens to track nested conditional levels.
/// - Switches to the `ELSE` block when an `ELSE` token is encountered at the appropriate nesting level.
/// - Stops parsing when the outermost `IF` block is closed with a `THEN`(at the right level).
///
///
/// # Notes
///
/// - Token comparison is case-insensitive (using `to_uppercase()`).
/// - Nested `IF` blocks are handled by incrementing `nested_ifs`.
/// - When calling the function it is assumed that the last string in the iterator was if type
///     so we begin iterating the first string in 'IF' block.
pub fn parse_conditional_blocks(
    iter: &mut impl Iterator<Item = String>,
) -> (Vec<String>, Option<Vec<String>>) {
    let mut if_block = Vec::new();
    let mut else_block = Vec::new();

    let mut current_block = &mut if_block;

    // will tell the level we are
    let mut nested_ifs = 1;

    // will tell if we are in else block (if there is)
    let mut in_else = false;

    for token in iter.by_ref() {
        let upper_token = token.to_uppercase();

        // when 'IF' is found means new nested level.
        // so we increment nested_ifs
        // and keep pushing to current_block
        if upper_token == "IF" {
            nested_ifs += 1;
            current_block.push(token);
        }
        // When 'ELSE' is found it could be from another nested if.
        // So we must validate the level we actually are
        // If we are in the fist level and we are not in 'ELSE' block
        // means we have just encounter the begining of else_block
        // and we change the current_block
        else if upper_token == "ELSE" {
            if nested_ifs == 1 && !in_else {
                current_block = &mut else_block;
                in_else = true;
            } else {
                current_block.push(token);
            }
        }
        // When 'THEN' is encountered
        // It might mean the end of this function because all
        // conditionals evaluations were made
        else if upper_token == "THEN" {
            nested_ifs -= 1;
            if nested_ifs == 0 {
                break;
            }
            current_block.push(token);
        } else {
            current_block.push(token);
        }
    }

    if else_block.is_empty() {
        (if_block, None)
    } else {
        (if_block, Some(else_block))
    }
}
