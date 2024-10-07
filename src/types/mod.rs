use std::{fmt::Display, str::FromStr};

use prop::ParseState;

use crate::{
    editor::{InvalidValueError, PomValue},
    utils::{parse::ParseErrorExt, serde_utils::serde_via_string_types},
};

pub(crate) mod prop;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Property {
    Variable(String),
    UnclosedVariable(String),
    Literal(String),
    Expression(Vec<Property>),
}
impl Default for Property {
    fn default() -> Self {
        Property::Literal(Default::default())
    }
}
impl Property {
    pub fn is_variable(&self) -> bool {
        matches!(self, Property::Variable(_))
    }
    pub fn is_maven_variable(&self) -> bool {
        let Property::Variable(name) = self else {
            return false;
        };
        name.starts_with("maven.")
    }
    pub fn is_project_variable(&self) -> bool {
        let Property::Variable(name) = self else {
            return false;
        };
        name.starts_with("project.")
    }
}

impl TryFrom<String> for Property {
    type Error = ParseErrorExt<String, winnow::error::ContextError>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match ParseState::default().parse(&value) {
            Ok(o) => Ok(o),
            Err(e) => {
                let offset = e.offset;
                let inner = e.inner;
                Err(ParseErrorExt::new(value, offset, inner))
            }
        }
    }
}
impl<'s> TryFrom<&'s str> for Property {
    type Error = ParseErrorExt<&'s str, winnow::error::ContextError>;

    fn try_from(value: &'s str) -> Result<Self, Self::Error> {
        ParseState::default()
            .parse(&value)
            .map_err(|e| e.map(|s| s.input))
    }
}
impl FromStr for Property {
    type Err = ParseErrorExt<(), winnow::error::ContextError>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ParseState::default().parse(s).map_err(|e| e.map(|_| ()))
    }
}
impl Display for Property {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Property::Variable(name) => write!(f, "${{{}}}", name),
            Property::UnclosedVariable(value) => write!(f, "${{{}", value),
            Property::Literal(value) => write!(f, "{}", value),
            Property::Expression(vec) => {
                for part in vec {
                    part.fmt(f)?;
                }
                Ok(())
            }
        }
    }
}
impl PomValue for Property {
    fn from_str_for_editor(value: &str) -> Result<Self, crate::editor::InvalidValueError> {
        Self::from_str(value).map_err(|err| InvalidValueError::InvalidFormattedValue {
            error: err.to_string(),
        })
    }

    fn to_string_for_editor(&self) -> String {
        self.to_string()
    }
}

serde_via_string_types!(Property);

#[cfg(test)]
mod tests {
    use crate::types::Property;
    #[test]
    fn test_regular_string_parse() {
        let value = Property::try_from("test").unwrap();
        assert_eq!(value, Property::Literal("test".to_string()));
    }
    #[test]
    fn test_variable_parse() {
        let value = "${test}".parse::<Property>().unwrap();
        assert_eq!(value, Property::Variable("test".to_string()));
    }

    #[test]
    fn test_maven_variable() {
        let value = "${maven.version}".parse::<Property>().unwrap();
        assert!(value.is_maven_variable());
    }

    #[test]
    fn test_unclosed_var() {
        let result = "${var".parse::<Property>();
        assert!(result.is_err())
    }
}
