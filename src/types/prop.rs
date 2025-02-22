use winnow::{
    combinator::{alt, delimited, eof, not, preceded, repeat},
    error::{
        ContextError, ErrMode,
        StrContext::{Expected, Label},
    },
    stream::AsChar,
    token::{literal, rest, take_until, take_while},
    ModalResult, Parser, Stateful,
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
    use crate::types::{prop::ParseState, Property};

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
}
