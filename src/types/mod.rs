use std::{fmt::Display, str::FromStr};

use thiserror::Error;

use crate::{
    editor::{InvalidValueError, PomValue},
    utils::serde_utils::serde_via_string_types,
};

#[derive(Debug, Clone, Error)]
pub enum InvalidMavenVariable {
    #[error("The Maven variable is missing a closing bracket")]
    MissingClosingBracket,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StringOrVariable {
    Variable(String),
    String(String),
}
impl Default for StringOrVariable {
    fn default() -> Self {
        StringOrVariable::String(Default::default())
    }
}
impl StringOrVariable {
    pub fn is_variable(&self) -> bool {
        matches!(self, StringOrVariable::Variable(_))
    }
    pub fn is_maven_variable(&self) -> bool {
        let StringOrVariable::Variable(name) = self else {
            return false;
        };
        name.starts_with("maven.")
    }
    pub fn is_project_variable(&self) -> bool {
        let StringOrVariable::Variable(name) = self else {
            return false;
        };
        name.starts_with("project.")
    }
}

impl TryFrom<String> for StringOrVariable {
    type Error = InvalidMavenVariable;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.starts_with("${") {
            if !value.ends_with("}") {
                return Err(InvalidMavenVariable::MissingClosingBracket);
            }
            let variable_name = value[2..value.len() - 1].to_string();
            Ok(StringOrVariable::Variable(variable_name))
        } else {
            Ok(StringOrVariable::String(value))
        }
    }
}
impl TryFrom<&str> for StringOrVariable {
    type Error = InvalidMavenVariable;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_string())
    }
}
impl FromStr for StringOrVariable {
    type Err = InvalidMavenVariable;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s.to_string())
    }
}
impl Display for StringOrVariable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StringOrVariable::Variable(name) => write!(f, "${{{}}}", name),
            StringOrVariable::String(value) => write!(f, "{}", value),
        }
    }
}
impl PomValue for StringOrVariable {
    fn from_str_for_editor(value: &str) -> Result<Self, crate::editor::InvalidValueError> {
        let value =
            Self::from_str(value).map_err(|err| InvalidValueError::InvalidFormattedValue {
                error: err.to_string(),
            })?;
        Ok(value)
    }

    fn to_string_for_editor(&self) -> String {
        self.to_string()
    }
}

serde_via_string_types!(StringOrVariable);

#[cfg(test)]
mod tests {
    use crate::types::StringOrVariable;
    #[test]
    fn test_regular_string_parse() {
        let value = StringOrVariable::try_from("test").unwrap();
        assert_eq!(value, StringOrVariable::String("test".to_string()));
    }
    #[test]
    fn test_variable_parse() {
        let value = "${test}".parse::<StringOrVariable>().unwrap();
        assert_eq!(value, StringOrVariable::Variable("test".to_string()));
    }

    #[test]
    fn test_maven_variable() {
        let value: StringOrVariable = "${maven.version}".parse::<StringOrVariable>().unwrap();
        assert!(value.is_maven_variable());
    }
}
