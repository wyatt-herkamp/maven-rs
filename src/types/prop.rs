use winnow::{
    combinator::{alt, delimited, eof, not, preceded, repeat, rest},
    error::{
        ContextError,
        StrContext::{Expected, Label},
    },
    stream::AsChar,
    token::{literal, take_until, take_while},
    PResult, Parser, Stateful,
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

fn parse_expr(input: &mut Input<'_, '_>) -> PResult<Vec<Property>> {
    repeat(0.., parse_part)
        .context(Label("expr"))
        .parse_next(input)
}

fn parse_part(input: &mut Input<'_, '_>) -> PResult<Property> {
    alt((
        parse_var.map(|s| Property::Variable(s.to_string())),
        parse_unclosed_var.map(|s| Property::UnclosedVariable(s.to_string())),
        parse_literal.map(|s| Property::Literal(s.to_string())),
    ))
    .context(Label("part"))
    .parse_next(input)
}

fn parse_var<'i>(input: &mut Input<'i, '_>) -> PResult<&'i str> {
    delimited(parse_var_prefix, parse_var_value, parse_var_suffix)
        .context(Label("var"))
        .parse_next(input)
}

fn parse_unclosed_var<'i>(input: &mut Input<'i, '_>) -> PResult<&'i str> {
    preceded(parse_var_prefix, parse_rest)
        .context(Label("unclosed_var"))
        .parse_next(input)
}

fn parse_literal<'i>(input: &mut Input<'i, '_>) -> PResult<&'i str> {
    preceded(not(eof), parse_rest)
        .context(Label("literal"))
        .parse_next(input)
}

fn parse_rest<'i>(input: &mut Input<'i, '_>) -> PResult<&'i str> {
    alt((take_until(0.., '$'), rest))
        .context(Label("rest"))
        .parse_next(input)
}

fn parse_var_prefix<'i>(input: &mut Input<'i, '_>) -> PResult<&'i str> {
    literal("${")
        .context(Expected("${".into()))
        .context(Label("var_prefix"))
        .parse_next(input)
}

fn parse_var_value<'i>(input: &mut Input<'i, '_>) -> PResult<&'i str> {
    take_while(0.., |c: char| {
        c.is_space() || c.is_alphanumeric() || c == '.'
    })
    .context(Label("var_value"))
    .parse_next(input)
}

fn parse_var_suffix<'i>(input: &mut Input<'i, '_>) -> PResult<&'i str> {
    literal('}')
        .context(Expected('}'.into()))
        .context(Label("var_suffix"))
        .parse_next(input)
        .map_err(|e| {
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

    #[test]
    fn unclosed_complex() {
        let source = Property::Expression(vec![
            Property::Literal("lit1/".into()),
            Property::UnclosedVariable("var1/lit2/".into()),
            Property::Variable("var2".into()),
            Property::Literal("/lit3".into()),
        ]);
        roundtrip(source, "lit1/${var1/lit2/${var2}/lit3", state(true));
    }

    #[test]
    fn test_complex() {
        let source = Property::Expression(vec![
            Property::Literal("lit1/".into()),
            Property::Variable("var1".into()),
            Property::Literal("/lit2/".into()),
            Property::Variable("var2".into()),
            Property::Literal("/lit3".into()),
        ]);
        roundtrip(source, "lit1/${var1}/lit2/${var2}/lit3", state(false));
    }

    fn state(unclosed: bool) -> ParseState {
        ParseState {
            allow_unclosed_variable: unclosed,
        }
    }

    fn roundtrip(source: Property, sanity: &str, state: ParseState) {
        if !state.allow_unclosed_variable {
            visit(&source, |p| {
                assert!(!matches!(p, Property::UnclosedVariable(_)));
            });
        }

        let string = source.to_string();
        assert_eq!(string, sanity);

        let parsed = state.parse(&string).unwrap();
        assert_eq!(parsed, source);
    }

    fn visit(prop: &Property, mut visitor: impl FnMut(&Property)) {
        visitor(prop);
        if let Property::Expression(vec) = prop {
            vec.iter().for_each(visitor);
        }
    }
}
