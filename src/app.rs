use std::collections::HashMap;

pub enum CurrentScreen {
    Main,
    Editing,
    Exiting,
}

pub enum CurrentlyEditing {
    Key,
    Value,
}

pub struct App {
    pub key_input: String,   // the currently being edited json key.
    pub value_input: String, // the currently being edited json value.
    // The representation of our key and value pairs with serde Serialize support
    pub pairs: HashMap<String, String>,
    // the current screen the user is looking at, and will later determine what is rendered.
    pub current_screen: CurrentScreen,
    // the optional state containing which of the key or value pair the user is editing. It is an
    // option, because when the user is not directly editing a key-value pair,
    // this will be set to `None`.
    pub currently_editing: Option<CurrentlyEditing>,
}

impl App {
    pub fn new() -> Self {
        App {
            key_input: String::new(),
            value_input: String::new(),
            pairs: HashMap::new(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
        }
    }

    pub fn save_key_value(&mut self) {
        self.pairs
            .insert(self.key_input.clone(), self.value_input.clone());

        self.key_input = String::new();
        self.value_input = String::new();
        self.currently_editing = None;
    }

    pub fn toggle_editing(&mut self) {
        match &self.currently_editing {
            Some(edit_mode) => match edit_mode {
                CurrentlyEditing::Key => self.currently_editing = Some(CurrentlyEditing::Value),
                CurrentlyEditing::Value => self.currently_editing = Some(CurrentlyEditing::Key),
            },
            // default to editing the key if no editing mode is set.
            None => self.currently_editing = Some(CurrentlyEditing::Key),
        }
    }

    // if let Some(edit_mode) = &self.currently_editing {
    //     match edit_mode {
    //         CurrentlyEditing::Key => self.currently_editing = Some(CurrentlyEditing::Value),
    //         CurrentlyEditing::Value => self.currently_editing = Some(CurrentlyEditing::Key),
    //     };
    // } else {
    //     self.currently_editing = Some(Curren    pub fn print_json(&self) -> serde_json::Result<()> {
    pub fn print_json(&self) -> serde_json::Result<()> {
        let output = serde_json::to_string(&self.pairs)?;
        println!("{}", output);
        Ok(())
    }
}
