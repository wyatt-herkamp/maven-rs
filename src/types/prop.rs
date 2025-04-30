use winnow::{
    ModalResult, Parser, Stateful,
    combinator::{alt, delimited, eof, not, preceded, repeat},
    error::{
        ContextError, ErrMode,
        StrContext::{Expected, Label},
    },
    stream::AsChar,
    token::{literal, rest, take_until, take_while},
};

use crate::utils::parse::{ParseErrorExt, ParserExt};

use super::Property;

type Input<'i, 's> = Stateful<&'i str, &'s ParseState>;

#[derive(Debug, Default)]
pub struct ParseState {
    pub allow_unclosed_variable: bool,
}

impl<'s> ParseState {
    pub fn parse<'i>(
        &'s self,
        value: &'i str,
    ) -> Result<Property, ParseErrorExt<Input<'i, 's>, ContextError>> {
        parse_expr
            .parse_ext(Input {
                input: value,
                state: self,
            })
            .map(|mut vec| {
                if vec.len() == 1 {
                    vec.remove(0)
                } else {
                    Property::Expression(vec)
                }
            })
    }
}

fn parse_expr(input: &mut Input<'_, '_>) -> ModalResult<Vec<Property>> {
    repeat(0.., parse_part)
        .context(Label("expr"))
        .parse_next(input)
}

fn parse_part(input: &mut Input<'_, '_>) -> ModalResult<Property> {
    alt((
        parse_var.map(|s| Property::Variable(s.to_string())),
        parse_unclosed_var.map(|s| Property::UnclosedVariable(s.to_string())),
        parse_literal.map(|s| Property::Literal(s.to_string())),
    ))
    .context(Label("part"))
    .parse_next(input)
}

fn parse_var<'i>(input: &mut Input<'i, '_>) -> ModalResult<&'i str> {
    delimited(parse_var_prefix, parse_var_value, parse_var_suffix)
        .context(Label("var"))
        .parse_next(input)
}

fn parse_unclosed_var<'i>(input: &mut Input<'i, '_>) -> ModalResult<&'i str> {
    preceded(parse_var_prefix, parse_rest)
        .context(Label("unclosed_var"))
        .parse_next(input)
}

fn parse_literal<'i>(input: &mut Input<'i, '_>) -> ModalResult<&'i str> {
    preceded(not(eof), parse_rest)
        .context(Label("literal"))
        .parse_next(input)
}

fn parse_rest<'i>(input: &mut Input<'i, '_>) -> ModalResult<&'i str> {
    alt((take_until(0.., '$'), rest))
        .context(Label("rest"))
        .parse_next(input)
}

fn parse_var_prefix<'i>(input: &mut Input<'i, '_>) -> ModalResult<&'i str> {
    literal("${")
        .context(Expected("${".into()))
        .context(Label("var_prefix"))
        .parse_next(input)
}

fn parse_var_value<'i>(input: &mut Input<'i, '_>) -> ModalResult<&'i str> {
    take_while(0.., |c: char| {
        c.is_space() || c.is_alphanumeric() || c == '.' || c == '-'
    })
    .context(Label("var_value"))
    .parse_next(input)
}

fn parse_var_suffix<'i>(input: &mut Input<'i, '_>) -> ModalResult<&'i str> {
    literal('}')
        .context(Expected('}'.into()))
        .context(Label("var_suffix"))
        .parse_next(input)
        .map_err(|e: ErrMode<_>| {
            if input.state.allow_unclosed_variable {
                e
            } else {
                e.cut()
            }
        })
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use crate::types::{Property, prop::ParseState};

    fn get_unclosed_complex() -> (Property, &'static str) {
        let source = Property::Expression(vec![
            Property::Literal("lit1/".into()),
            Property::UnclosedVariable("var1/lit2/".into()),
            Property::Variable("var2".into()),
            Property::Literal("/lit3".into()),
        ]);
        (source, "lit1/${var1/lit2/${var2}/lit3")
    }

    #[test]
    fn test_unclosed_complex() {
        roundtrip(&get_unclosed_complex(), state(true));
    }

    #[test]
    #[should_panic]
    fn panic_unclosed_complex() {
        roundtrip(&get_unclosed_complex(), state(false));
    }

    fn get_complex() -> (Property, &'static str) {
        let source = Property::Expression(vec![
            Property::Literal("lit1/".into()),
            Property::Variable("var1".into()),
            Property::Literal("/lit2/".into()),
            Property::Variable("var2".into()),
            Property::Literal("/lit3".into()),
        ]);
        (source, "lit1/${var1}/lit2/${var2}/lit3")
    }

    #[test]
    fn test_complex() {
        let input = verify_closed(get_complex());
        roundtrip(&input, state(false));
        roundtrip(&input, state(true));
    }

    fn state(allow_unclosed: bool) -> ParseState {
        ParseState {
            allow_unclosed_variable: allow_unclosed,
        }
    }

    fn verify_closed<'a>(input: (Property, &'a str)) -> (Property, &'a str) {
        visit(&input.0, |p| {
            assert!(!matches!(p, Property::UnclosedVariable(_)));
        });
        input
    }

    fn roundtrip(input: &(Property, &str), state: ParseState) {
        let string = input.0.to_string();
        assert_eq!(string, input.1);

        let parsed = state.parse(&string).unwrap();
        assert_eq!(parsed, input.0);
    }

    fn visit(prop: &Property, mut visitor: impl FnMut(&Property)) {
        visitor(prop);
        if let Property::Expression(vec) = prop {
            vec.iter().for_each(visitor);
        }
    }
    #[test]
    fn enum_is_testing() {
        {
            let variable = Property::Variable("var".to_string());
            assert!(variable.is_variable());
        }

        {
            let maven_variable = Property::Variable("maven.var".to_string());
            assert!(maven_variable.is_variable());
            assert!(maven_variable.is_maven_variable());

            let none_variable = Property::Literal("literal".to_string());

            assert!(!none_variable.is_variable());
            assert!(!none_variable.is_maven_variable());
        }

        {
            let project_variable = Property::Variable("project.var".to_string());
            assert!(project_variable.is_variable());
            assert!(project_variable.is_project_variable());

            let none_variable = Property::Literal("literal".to_string());

            assert!(!none_variable.is_variable());
            assert!(!none_variable.is_project_variable());
        }
    }

    #[test]
    fn try_from_string() {
        let variable = Property::try_from("${project.version}".to_string()).unwrap();
        assert!(variable.is_variable());
        assert!(variable.is_project_variable());

        let literal = Property::try_from("1.0.0".to_string()).unwrap();
        assert!(!literal.is_variable());
        assert!(!literal.is_project_variable());

        let expression =
            Property::try_from("${project.version}-${maven.buildNumber}".to_string()).unwrap();
        if let Property::Expression(vec) = expression {
            assert_eq!(vec.len(), 3);
            assert!(matches!(vec[0], Property::Variable(_)));
            assert!(matches!(vec[1], Property::Literal(_)));
            assert!(matches!(vec[2], Property::Variable(_)));
        } else {
            panic!("Expected expression");
        }
    }

    #[test]
    fn fuzz() {
        let rand = &mut rand::rngs::ThreadRng::default();

        for _ in 0..100 {
            let variable_or_literal = rand.random_bool(0.5f64);
            let value = if variable_or_literal {
                let number_of_parts = rand.random_range(1..=5);

                let mut parts: Vec<String> = Vec::new();

                for _ in 0..number_of_parts {
                    let length = rand.random_range(3..=10);
                    parts.push(
                        rand.sample_iter(&rand::distr::Alphanumeric)
                            .take(length)
                            .map(char::from)
                            .collect(),
                    )
                }

                format!("${{{}}}", parts.join("."))
            } else {
                let length = rand.random_range(3..=10);

                rand.sample_iter(&rand::distr::Alphanumeric)
                    .take(length)
                    .map(char::from)
                    .collect()
            };

            let parsed = ParseState::default().parse(&value).unwrap();
            if variable_or_literal {
                assert!(parsed.is_variable());
            } else {
                assert!(!parsed.is_variable());
            }
            assert_eq!(parsed.to_string(), value);
        }
    }
}
