use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Action {
    ChoosePhoto(String),
    TakePhoto(String),
    CopyToClipboard(String),
    LaunchUrl(String),
    Logout,
    Nothing,
    Pop,
    Push(String),
    Rpc(String),
}
impl Action {
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn to_value(&self) -> Value {
        serde_json::to_value(self).unwrap()
    }
}
impl From<Action> for Value {
    fn from(src: Action) -> Self {
        src.to_value()
    }
}
impl Serialize for Action {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Action::ChoosePhoto(s) => serializer.serialize_str(&format!("choose-photo:{s}")),
            Action::CopyToClipboard(s) => {
                serializer.serialize_str(&format!("copy-to-clipboard:{s}"))
            }
            Action::LaunchUrl(s) => serializer.serialize_str(&format!("launch-url:{s}")),
            Action::Logout => serializer.serialize_str("logout"),
            Action::Nothing => serializer.serialize_str("nothing"),
            Action::Pop => serializer.serialize_str("pop"),
            Action::Push(s) => serializer.serialize_str(&format!("push:{s}")),
            Action::Rpc(s) => serializer.serialize_str(&format!("rpc:{s}")),
            Action::TakePhoto(s) => serializer.serialize_str(&format!("take-photo:{s}")),
        }
    }
}
impl<'de> Deserialize<'de> for Action {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ActionVisitor;
        impl<'de> Visitor<'de> for ActionVisitor {
            type Value = Action;

            fn expecting(
                &self,
                formatter: &mut core::fmt::Formatter,
            ) -> Result<(), core::fmt::Error> {
                formatter.write_str("a string matching the action string format")
            }

            fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
                let mut parts = value.splitn(2, ':');
                match (parts.next(), parts.next()) {
                    (Some("choose-photo"), Some(s)) if !s.is_empty() => {
                        Ok(Action::ChoosePhoto(s.to_string()))
                    }
                    (Some("copy-to-clipboard"), Some(s)) => {
                        Ok(Action::CopyToClipboard(s.to_string()))
                    }
                    (Some("launch-url"), Some(s)) if !s.is_empty() => {
                        Ok(Action::LaunchUrl(s.to_string()))
                    }
                    (Some("logout"), None) => Ok(Action::Logout),
                    (Some("nothing"), None) => Ok(Action::Nothing),
                    (Some("pop"), None) => Ok(Action::Pop),
                    (Some("push"), Some(s)) if !s.is_empty() => Ok(Action::Push(s.to_string())),
                    (Some("rpc"), Some(s)) if !s.is_empty() => Ok(Action::Rpc(s.to_string())),
                    (Some("take-photo"), Some(s)) if !s.is_empty() => {
                        Ok(Action::TakePhoto(s.to_string()))
                    }
                    _ => Err(E::custom(format!("invalid action: {value:?}"))),
                }
            }
        }
        deserializer.deserialize_str(ActionVisitor {})
    }
}
