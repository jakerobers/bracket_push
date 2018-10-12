use std::collections::LinkedList;

pub fn get_inverse(symbol: char) -> char {
    match symbol {
        ']' => '[',
        ')' => '(',
        '}' => '{',
        _ => 'N' // This cannot be reached.
    }
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: LinkedList<char> = LinkedList::new();

    for c in string.chars() {
        match c {
            '[' | '(' | '{' => stack.push_front(c),
            ']' | ')' | '}' => {
                let peek =
                    match stack.front() {
                        Some(peek) => peek.clone(),
                        None => return false
                    };

                if peek == get_inverse(c) {
                    stack.pop_front();
                } else {
                    return false;
                }
            }
            _ => ()
        }
    }
    if stack.len() > 0 {
        return false
    }

    return true;
}
