use env_logger::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::*,
    combinator::{map, opt, recognize},
    multi::many0,
    sequence::{delimited, pair, preceded},
    IResult,
};
pub mod ast;
use ast::AstNode::{self, *};
use ast::AST;

/// Parses the input string into an Abstract Syntax Tree (AST).
///
/// This function takes a string slice as input and attempts to parse it into an AST.
/// The AST is represented by the `AST` struct, which contains a `Root` node with
/// a list of child nodes.
///
/// # Arguments
///
/// * `input` - A string slice that holds the code to be parsed.
///
/// # Returns
///
/// * `Ok(AST)` - If the parsing is successful, it returns an `AST` struct.
/// * `Err(String)` - If the parsing fails, it returns an error message as a `String`.
///
/// # Example
///
/// ```rust
/// use parser::parse;
/// let code = r#"fn main()->u32{
///    return 42;
/// }
/// "#;
/// match parse(code) {
///     Ok(ast) => println!("Parsed successfully: {:?}", ast),
///     Err(e) => println!("Error parsing: {}", e),
/// }
/// ```
pub fn parse(input: &str) -> Result<AST, String> {
    match many0(parse_stmt)(input) {
        Ok((_, stmts)) => Ok(AST {
            head: Root { children: stmts },
        }),
        Err(e) => Err(format!("Error parsing: {}", e)),
    }
}

fn parse_identifier(input: &str) -> IResult<&str, AstNode> {
    let (input, _) = multispace0(input)?;
    map(
        recognize(pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_")))),
        )),
        |s: &str| Identifier {
            name: s.to_string(),
        },
    )(input)
}
fn parse_ret_type(input: &str) -> IResult<&str, AstNode> {
    let (input, _) = delimited(multispace0, tag("->"), multispace0)(input)?;
    let (input, rtype) = parse_identifier(input)?;
    Ok((input, rtype))
}
fn parse_literalnum(input: &str) -> IResult<&str, AstNode> {
    map(
        pair(
            opt(alt((tag("0x"), tag("0b")))),
            recognize(pair(digit1, opt(pair(tag("."), digit1)))),
        ),
        |(prefix, s): (Option<&str>, &str)| IntLiteral {
            value: i64::from_str_radix(
                s,
                match prefix {
                    Some("0x") => 16,
                    Some("0b") => 2,
                    _ => 10,
                },
            )
            .unwrap(),
        },
    )(input)
}
fn parse_fn(input: &str) -> IResult<&str, AstNode> {
    let (input, _) = delimited(multispace0, tag("fn"), multispace0)(input)?;
    let (input, name) = match parse_identifier(input) {
        Ok((input, n)) => match n {
            Identifier { name } => (input, name),
            _ => {
                println!("Error parsing identifier: {}", "Invalid identifier");
                return Err(nom::Err::Error(nom::error::Error::new(
                    input,
                    nom::error::ErrorKind::Tag,
                )));
            }
        },
        Err(e) => {
            println!("Error parsing identifier: {}", e);
            return Err(e);
        }
    };
    let (input, _) = delimited(char('('), many0(parse_arg), char(')'))(input)?;
    let (input, return_type) = match parse_ret_type(input) {
        Ok((input, return_type)) => (input, return_type),
        Err(_) => (
            input,
            Identifier {
                name: "void".to_string(),
            },
        ),
    };
    let (input, body) = parse_body(input)?;
    Ok((
        input,
        Function {
            name: name.to_string(),
            params: vec![],
            return_type: match return_type {
                Identifier { name } => name,
                _ => "void".to_string(),
            },
            body,
        },
    ))
}
fn parse_arg(input: &str) -> IResult<&str, AstNode> {
    let (input, name) = parse_identifier(input)?;
    let (input, _) = char(':')(input)?;
    let (input, ptype) = parse_expr(input)?;
    Ok((input, name))
}
fn parse_expr(input: &str) -> IResult<&str, AstNode> {
    alt((parse_literalnum, parse_identifier))(input)
}
fn parse_body(input: &str) -> IResult<&str, Vec<AstNode>> {
    return delimited(
        preceded(multispace0, char('{')),
        many0(parse_stmt),
        preceded(multispace0, char('}')),
    )(input);
}
fn parse_stmt(input: &str) -> IResult<&str, AstNode> {
    let (input, _) = multispace0(input)?;
    let (input, ret) = alt((call, parse_kwd, parse_fn, parse_expr))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = opt(char(';'))(input)?;
    Ok((input, ret))
}

fn parse_kwd(input: &str) -> IResult<&str, AstNode> {
    let (input, _) = multispace0(input)?;
    return alt((return_stmt,))(input);
}

fn call(input: &str) -> IResult<&str, AstNode> {
    let (input, name) = parse_identifier(input)?;
    let (input, bang) = opt(char('!'))(input)?;
    let (input, args) = delimited(char('('), many0(parse_expr), char(')'))(input)?;
    Ok((
        input,
        match bang {
            Some('!') => BangCall {
                name: match name {
                    Identifier { name } => name,
                    _ => {
                        return Err(nom::Err::Error(nom::error::Error::new(
                            input,
                            nom::error::ErrorKind::Tag,
                        )))
                    }
                },
                args,
            },
            _ => Call {
                name: match name {
                    Identifier { name } => name,
                    _ => {
                        return Err(nom::Err::Error(nom::error::Error::new(
                            input,
                            nom::error::ErrorKind::Tag,
                        )))
                    }
                },
                args,
            },
        },
    ))
}

fn return_stmt(input: &str) -> IResult<&str, AstNode> {
    let (input, _) = tag("return")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, expr) = parse_expr(input)?;
    Ok((
        input,
        Return {
            value: Box::new(expr),
        },
    ))
}
