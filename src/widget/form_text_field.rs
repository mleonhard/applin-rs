use crate::internal::{FormTextfieldAllow, FormTextfieldAutoCapitalize, Widget};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FormTextfield {
    allow: FormTextfieldAllow,
    auto_capitalize: FormTextfieldAutoCapitalize,
    check_rpc: Option<String>,
    initial: String,
    label: String,
    max_chars: u32,
    max_lines: u32,
    min_chars: u32,
    var: String,
}
impl FormTextfield {
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
            allow: FormTextfieldAllow::All,
            auto_capitalize: FormTextfieldAutoCapitalize::Sentences,
            check_rpc: None,
            initial: String::new(),
            label: label.into(),
            max_chars: u32::MAX,
            max_lines: u32::MAX,
            min_chars: 0,
            var,
        }
    }

    #[must_use]
    pub fn with_check_rpc(mut self, check_rpc: impl Into<String>) -> Self {
        self.check_rpc = Some(check_rpc.into());
        self
    }

    #[must_use]
    pub fn with_initial(mut self, initial: impl Into<String>) -> Self {
        self.initial = initial.into();
        self
    }

    #[must_use]
    pub fn with_allow_all(mut self) -> Self {
        self.allow = FormTextfieldAllow::All;
        self
    }

    #[must_use]
    pub fn with_allow_ascii(mut self) -> Self {
        self.allow = FormTextfieldAllow::Ascii;
        self
    }

    #[must_use]
    pub fn with_allow_email(mut self) -> Self {
        self.allow = FormTextfieldAllow::Email;
        self
    }

    #[must_use]
    pub fn with_allow_numbers(mut self) -> Self {
        self.allow = FormTextfieldAllow::Numbers;
        self
    }

    #[must_use]
    pub fn with_allow_tel(mut self) -> Self {
        self.allow = FormTextfieldAllow::Tel;
        self
    }

    #[must_use]
    pub fn with_autocap_names(mut self) -> Self {
        self.auto_capitalize = FormTextfieldAutoCapitalize::Names;
        self
    }

    /// This is the default.
    #[must_use]
    pub fn with_autocap_sentences(mut self) -> Self {
        self.auto_capitalize = FormTextfieldAutoCapitalize::Sentences;
        self
    }

    /// Ask the client to prevent the user from entering to many characters.
    /// Use `u32::MAX` for no limit.
    #[must_use]
    pub fn with_max_chars(mut self, max_chars: u32) -> Self {
        if max_chars == 0 {
            println!("WARN FormTextfield::with_max_chars called with 0");
            self.max_chars = u32::MAX;
        } else {
            self.max_chars = max_chars
        }
        self
    }

    /// Ask the client to prevent the user from entering to many newline characters.
    /// Use `u32::MAX` for no limit.
    #[must_use]
    pub fn with_max_lines(mut self, max_lines: u32) -> Self {
        if max_lines == 0 {
            println!("WARN FormTextfield::with_max_lines called with 0");
            self.max_lines = u32::MAX;
        } else {
            self.max_lines = max_lines
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
    pub fn to_widget(self) -> Widget {
        Widget::FormTextfieldVariant {
            allow: self.allow,
            auto_capitalize: self.auto_capitalize,
            check_rpc: self.check_rpc,
            initial_string: self.initial,
            label: self.label,
            max_chars: self.max_chars,
            max_lines: self.max_lines,
            min_chars: self.min_chars,
            var: self.var,
        }
    }
}
impl From<FormTextfield> for Widget {
    fn from(src: FormTextfield) -> Self {
        src.to_widget()
    }
}