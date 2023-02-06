use crate::internal::{TextfieldAllow, TextfieldAutoCapitalize, Widget};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Textfield {
    allow: TextfieldAllow,
    auto_capitalize: TextfieldAutoCapitalize,
    initial: String,
    label: String,
    max_chars: u32,
    max_lines: u32,
    min_chars: u32,
    rpc: Option<String>,
    var: String,
}
impl Textfield {
    /// Creates a text field.
    ///
    /// Clients use these defaults:
    /// - "sentences" auto-capitalize mode
    /// - allow all characters
    /// - max 100 chars
    /// - max 5 lines
    ///
    /// # Panics
    /// Panics when `id` is empty.
    #[must_use]
    pub fn new(var: impl Into<String>, label: impl Into<String>) -> Self {
        let var = var.into();
        assert!(!var.is_empty());
        Self {
            allow: TextfieldAllow::All,
            auto_capitalize: TextfieldAutoCapitalize::Sentences,
            initial: String::new(),
            label: label.into(),
            max_chars: u32::MAX,
            max_lines: u32::MAX,
            min_chars: 0,
            rpc: None,
            var,
        }
    }

    #[must_use]
    pub fn with_allow_all(mut self) -> Self {
        self.allow = TextfieldAllow::All;
        self
    }

    #[must_use]
    pub fn with_allow_ascii(mut self) -> Self {
        self.allow = TextfieldAllow::Ascii;
        self
    }

    #[must_use]
    pub fn with_allow_email(mut self) -> Self {
        self.allow = TextfieldAllow::Email;
        self
    }

    #[must_use]
    pub fn with_allow_numbers(mut self) -> Self {
        self.allow = TextfieldAllow::Numbers;
        self
    }

    #[must_use]
    pub fn with_allow_tel(mut self) -> Self {
        self.allow = TextfieldAllow::Tel;
        self
    }

    #[must_use]
    pub fn with_autocap_names(mut self) -> Self {
        self.auto_capitalize = TextfieldAutoCapitalize::Names;
        self
    }

    /// This is the default.
    #[must_use]
    pub fn with_autocap_sentences(mut self) -> Self {
        self.auto_capitalize = TextfieldAutoCapitalize::Sentences;
        self
    }

    #[must_use]
    pub fn with_initial(mut self, initial: impl Into<String>) -> Self {
        self.initial = initial.into();
        self
    }

    /// Ask the client to prevent the user from entering to many characters.
    /// Use `u32::MAX` for no limit.
    #[must_use]
    pub fn with_max_chars(mut self, max_chars: u32) -> Self {
        if max_chars == 0 {
            println!("WARN Textfield::with_max_chars called with 0");
            self.max_chars = u32::MAX;
        } else {
            self.max_chars = max_chars;
        }
        self
    }

    /// Ask the client to prevent the user from entering to many newline characters.
    /// Use `u32::MAX` for no limit.
    #[must_use]
    pub fn with_max_lines(mut self, max_lines: u32) -> Self {
        if max_lines == 0 {
            println!("WARN Textfield::with_max_lines called with 0");
            self.max_lines = u32::MAX;
        } else {
            self.max_lines = max_lines;
        }
        self
    }

    /// Show a warning when the user has not entered enough characters.
    /// Use 0 for no minimum.
    #[must_use]
    pub fn with_min_chars(mut self, min_chars: u32) -> Self {
        self.min_chars = min_chars;
        self
    }

    #[must_use]
    pub fn with_rpc(mut self, rpc: impl Into<String>) -> Self {
        self.rpc = Some(rpc.into());
        self
    }

    #[must_use]
    pub fn to_widget(self) -> Widget {
        Widget::TextfieldVariant {
            allow: self.allow,
            auto_capitalize: self.auto_capitalize,
            initial_string: self.initial,
            label: self.label,
            max_chars: self.max_chars,
            max_lines: self.max_lines,
            min_chars: self.min_chars,
            rpc: self.rpc,
            var: self.var,
        }
    }
}
impl From<Textfield> for Widget {
    fn from(src: Textfield) -> Self {
        src.to_widget()
    }
}
