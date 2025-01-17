use dialoguer::Input;

pub fn prompt_for_title() -> String {
    Input::new()
        .with_prompt("Enter the todo title")
        .interact_text()
        .unwrap()
}

pub fn prompt_for_id() -> u32 {
    Input::new()
        .with_prompt("Enter the todo id")
        .interact_text()
        .unwrap()
}