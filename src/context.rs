pub struct Message {
    speaker: String,
    content: String,
}

impl Message {
    fn new(speaker: &str, content: &str) -> Self {
        Self {
            speaker: speaker.into(),
            content: content.into(),
        }
    }
}

pub struct Context {
    background: String,
    messages: Vec<Message>,
}

impl Context {
    const BACKGROUND: &str = "
The following is a chat conversation with an AI friend and some people.
The AI friend is very friendly.";

    pub fn new() -> Self {
        Self {
            background: Self::BACKGROUND.into(),
            messages: vec![],
        }
    }

    pub fn speak(&mut self, speaker: &str, content: &str) {
        self.messages.push(Message::new(speaker, content));
    }

    pub fn as_prompt(&self) -> String {
        let msgs: Vec<_> = self
            .messages
            .iter()
            .map(|m| format!("{}: {}", m.speaker, m.content))
            .collect();

        format!(
            "{}\n\n{}",
            self.background,
            msgs.join("\n\n") + &format!("\n\nAI: ")
        )
    }
}
