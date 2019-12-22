use tcod::colors::Color;

pub struct Messages {
    messages: Vec<(String, Color)>,
}

impl Messages {
    pub fn new() -> Self {
        Messages { messages: vec![] }
    }

    /// Add the new message as a tuple, with the text and color
    pub fn add<T: Into<String>>(&mut self, message: T, color: Color) {
        self.messages.push((message.into(), color));
    }

    /// Create a `DoubleEndedIterator` over the messages
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &(String, Color)> {
        self.messages.iter()
    }
}
