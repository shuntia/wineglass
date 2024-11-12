pub mod ast;
use ast::AstNode::{self, *};
use ast::AST;
use core::error;
use log::{debug, error, info, log, trace, warn, Level};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::*,
    combinator::{map, opt, recognize},
    multi::many0,
    sequence::{delimited, pair, preceded},
    Err, IResult,
};
use nom_locate::LocatedSpan;
use typed_arena::Arena;

pub type Span<'a> = LocatedSpan<&'a str>;

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
pub fn parse<'a>(input: Span<'a>, arena: &'a Arena<AstNode<'a>>) -> IResult<Span<'a>, AST<'a>> {
    let (input, root) = many0(|i| parse_stmt(i, arena))(input)?;
    Ok((
        input,
        AST {
            head: arena.alloc(AstNode::Root { children: root }),
            arena,
        },
    ))
}

fn parse_identifier<'a>(
    input: Span<'a>,
    arena: &'a Arena<AstNode<'a>>,
) -> IResult<Span<'a>, &'a AstNode<'a>> {
    let (input, _) = multispace0(input)?;
    map(
        recognize(pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_")))),
        )),
        |s: Span<'a>| {
            &*arena.alloc(Identifier {
                name: s.fragment().to_string(),
            })
        },
    )(input)
}

fn parse_ret_type<'a>(
    input: Span<'a>,
    arena: &'a Arena<AstNode<'a>>,
) -> IResult<Span<'a>, &'a AstNode<'a>> {
    let (input, _) = delimited(multispace0, tag("->"), multispace0)(input)?;
    parse_identifier(input, arena)
}

fn parse_literalnum<'a>(
    input: Span<'a>,
    arena: &'a Arena<AstNode<'a>>,
) -> IResult<Span<'a>, &'a AstNode<'a>> {
    map(
        pair(
            opt(alt((tag("0x"), tag("0b")))),
            recognize(pair(digit1, opt(pair(tag("."), digit1)))),
        ),
        |(prefix, s): (Option<Span<'a>>, Span<'a>)| {
            &*arena.alloc(IntLiteral {
                value: i64::from_str_radix(
                    s.fragment(),
                    match prefix.map(|p| *p.fragment()) {
                        Some("0x") => 16,
                        Some("0b") => 2,
                        _ => 10,
                    },
                )
                .unwrap(),
            })
        },
    )(input)
}

fn parse_fn<'a>(
    input: Span<'a>,
    arena: &'a Arena<AstNode<'a>>,
) -> IResult<Span<'a>, &'a AstNode<'a>> {
    let (input, _) = delimited(multispace0, tag("fn"), multispace0)(input)?;
    let (input, name) = match parse_identifier(input, arena) {
        Ok((input, n)) => match *n {
            Identifier { ref name } => (input, name.clone()),
            _ => {
                println!("Error parsing identifier");
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
    let (input, _) = delimited(char('('), many0(|i| parse_arg(i, arena)), char(')'))(input)?;
    let (input, return_type) = match parse_ret_type(input, arena) {
        Ok((input, return_type)) => (input, return_type),
        Err(_) => {
            let void_node: &'a AstNode<'a> = &*arena.alloc(Identifier {
                name: "void".to_string(),
            });
            (input, void_node)
        }
    };
    let (input, body) = parse_body(input, arena)?;
    Ok((
        input,
        &*arena.alloc(Function {
            name,
            params: vec![],
            return_type: match *return_type {
                Identifier { ref name } => name.clone(),
                _ => "void".to_string(),
            },
            body,
        }),
    ))
}

fn parse_arg<'a>(
    input: Span<'a>,
    arena: &'a Arena<AstNode<'a>>,
) -> IResult<Span<'a>, &'a AstNode<'a>> {
    let (input, name) = parse_identifier(input, arena)?;
    let (input, _) = char(':')(input)?;
    let (input, ptype) = parse_expr(input, arena)?; // Assuming parse_expr is an appropriate type.
    Ok((input, name))
}

fn parse_expr<'a>(
    input: Span<'a>,
    arena: &'a Arena<AstNode<'a>>,
) -> IResult<Span<'a>, &'a AstNode<'a>> {
    alt((
        |i| parse_literalnum(i, arena),
        |i| parse_identifier(i, arena),
    ))(input)
}

fn parse_body<'a>(
    input: Span<'a>,
    arena: &'a Arena<AstNode<'a>>,
) -> IResult<Span<'a>, Vec<&'a AstNode<'a>>> {
    println!("Parsing body");

    // Match the opening brace and allow for potential whitespace
    let (input, _) = preceded(multispace0, char('{'))(input)?;

    // Parse the body content, which could be a series of statements
    let (input, body) = many0(|i| parse_stmt(i, arena))(input)?;

    // Match the closing brace and ensure it is properly consumed
    let (input, _) = preceded(multispace0, char('}'))(input)?;

    println!("Parsed body: {:#?}", body);

    Ok((input, body))
}

fn parse_stmt<'a>(
    input: Span<'a>,
    arena: &'a Arena<AstNode<'a>>,
) -> IResult<Span<'a>, &'a AstNode<'a>> {
    let (input, _) = multispace0(input)?;
    let (input, ret) = alt((
        |i| call(i, arena),
        |i| parse_kwd(i, arena),
        |i| parse_fn(i, arena),
        |i| parse_expr(i, arena),
    ))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = opt(char(';'))(input)?;
    Ok((input, ret))
}

fn parse_kwd<'a>(
    input: Span<'a>,
    arena: &'a Arena<AstNode<'a>>,
) -> IResult<Span<'a>, &'a AstNode<'a>> {
    let (input, _) = multispace0(input)?;
    return alt((|i| return_stmt(i, arena),))(input);
}

fn call<'a>(input: Span<'a>, arena: &'a Arena<AstNode<'a>>) -> IResult<Span<'a>, &'a AstNode<'a>> {
    let (input, name) = parse_identifier(input, arena)?;
    let (input, bang) = opt(char('!'))(input)?;
    let (input, args) = delimited(char('('), many0(|i| parse_expr(i, arena)), char(')'))(input)?;
    Ok((
        input,
        match bang {
            Some('!') => arena.alloc(BangCall {
                name: match *name {
                    Identifier { ref name } => name.clone(),
                    _ => {
                        return Err(nom::Err::Error(nom::error::Error::new(
                            input,
                            nom::error::ErrorKind::Tag,
                        )))
                    }
                },
                args,
            }),
            _ => arena.alloc(Call {
                name: match *name {
                    Identifier { ref name } => name.clone(),
                    _ => {
                        return Err(nom::Err::Error(nom::error::Error::new(
                            input,
                            nom::error::ErrorKind::Tag,
                        )))
                    }
                },
                args,
            }),
        },
    ))
}

fn return_stmt<'a>(
    input: Span<'a>,
    arena: &'a Arena<AstNode<'a>>,
) -> IResult<Span<'a>, &'a AstNode<'a>> {
    let (input, _) = tag("return")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, expr) = parse_expr(input, arena)?;
    Ok((input, arena.alloc(Return { value: expr })))
}

fn unknown_stmt<'a>(
    input: Span<'a>,
    arena: &'a Arena<AstNode<'a>>,
) -> IResult<Span<'a>, &'a AstNode<'a>> {
    println!("Unknown statement");
    let (input, _) = multispace0(input)?;
    let (input, unknown) = recognize(nom::bytes::streaming::take_until(";"))(input)?;
    let (input, _) = opt(char(';'))(input)?;
    Ok((
        input,
        arena.alloc(Unknown {
            stmt: unknown.fragment().to_string(),
        }),
    ))
}
