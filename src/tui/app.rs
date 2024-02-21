pub(crate) type Pairs = std::collections::HashMap<String, String>;

#[derive(Clone, Copy, Eq, PartialEq)]
pub(crate) enum Screen {
    Key,
    Main,
    Value,
}

pub(crate) struct App {
    pub(crate) key: String,
    pub(crate) value: String,
    pub(crate) pairs: Pairs,
    pub(crate) screen: Screen,
}

impl App {
    pub(crate) fn new() -> Self {
        Self {
            key: String::new(),
            value: String::new(),
            pairs: std::collections::HashMap::new(),
            screen: Screen::Main,
        }
    }

    pub(crate) fn save_pair(&mut self) {
        let key = self.key.clone();
        let value = self.value.clone();
        self.pairs.insert(key, value);

        self.key.clear();
        self.value.clear();
    }
}

