use crate::app::{App, PlayStyle};

pub fn process_command(app: &mut App, command_string: String) {
    // split command buffer
    let mut splist_command: Vec<String> = command_string
        .trim_start_matches(":")
        .split_ascii_whitespace()
        .map(String::from)
        .collect();

    if splist_command.len() == 0 {
        splist_command.append(&mut vec![String::new()]);
    }

    match splist_command[0].to_ascii_uppercase().as_ref() {
        "REMOVE" | "RM" => remove_command(app,splist_command),
        _ => app.error = Some(String::from("Not a command")),
    }
}


fn remove_command(app: &mut App, splist_command: Vec<String>) {
    let mut remove_index: Vec<usize> = Vec::new();
    for index_string in &splist_command[1..] {
        match index_string.parse::<usize>() {
            Ok(index) => {
                if index > 0 {
                    remove_index.push(index - 1)
                }
            }
            Err(_) => app.error = Some(String::from("It must be a positive integer")),
        };
    }
    if remove_index.len() > 0 {
        // todo:app.remove_play_list_by_id()
    }
}