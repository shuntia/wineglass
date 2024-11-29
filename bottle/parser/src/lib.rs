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

pub struct Parser<'a> {
    input: Span<'a>,
    arena: &'a Arena<AstNode<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(input: Span<'a>, arena: &'a Arena<AstNode<'a>>) -> Self {
        Self { input, arena }
    }

    pub fn parse(&mut self) -> IResult<Span<'a>, AST<'a>> {
        let input = self.input;
        let (input, root) = many0(|i| self.parse_stmt(i))(input)?;
        self.input = input;
        Ok((
            self.input,
            AST {
                head: self.arena.alloc(AstNode::Root { children: root }),
                arena: self.arena,
            },
        ))
    }

    fn parse_identifier(&mut self, input: Span<'a>) -> IResult<Span<'a>, &'a AstNode<'a>> {
        let (input, _) = multispace0(input)?;
        map(
            recognize(pair(
                alt((alpha1, tag("_"))),
                many0(alt((alphanumeric1, tag("_")))),
            )),
            |s: Span<'a>| {
                &*self.arena.alloc(Identifier {
                    name: s.fragment().to_string(),
                })
            },
        )(input)
    }

    fn parse_ret_type(&mut self, input: Span<'a>) -> IResult<Span<'a>, &'a AstNode<'a>> {
        let (input, _) = delimited(multispace0, tag("->"), multispace0)(input)?;
        self.parse_identifier(input)
    }

    fn parse_literalnum(&mut self, input: Span<'a>) -> IResult<Span<'a>, &'a AstNode<'a>> {
        map(
            pair(
                opt(alt((tag("0x"), tag("0b")))),
                recognize(pair(digit1, opt(pair(tag("."), digit1)))),
            ),
            |(prefix, s): (Option<Span<'a>>, Span<'a>)| {
                &*self.arena.alloc(IntLiteral {
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

    fn parse_fn(&mut self, input: Span<'a>) -> IResult<Span<'a>, &'a AstNode<'a>> {
        let (input, _) = delimited(multispace0, tag("fn"), multispace0)(input)?;
        let (input, name_node) = self.parse_identifier(input)?;
        let name = if let Identifier { ref name } = *name_node {
            name.to_owned()
        } else {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Tag,
            )));
        };
        let (input, _) = delimited(char('('), many0(|i| self.parse_arg(i)), char(')'))(input)?;
        let (input, return_type_node) = match self.parse_ret_type(input) {
            Ok((input, return_type_node)) => (input, return_type_node),
            Err(_) => {
                let void_node = &*self.arena.alloc(Identifier {
                    name: "void".to_string(),
                });
                (input, void_node)
            }
        };
        let return_type = if let Identifier { ref name } = *return_type_node {
            name.clone()
        } else {
            "void".to_owned()
        };
        let (input, body) = self.parse_body(input)?;
        Ok((
            input,
            &*self.arena.alloc(Function {
                name,
                params: vec![],
                return_type,
                body,
            }),
        ))
    }

    fn parse_arg(&mut self, input: Span<'a>) -> IResult<Span<'a>, &'a AstNode<'a>> {
        let (input, name) = self.parse_identifier(input)?;
        let (input, _) = char(':')(input)?;
        let (input, _ptype) = self.parse_expr(input)?;
        Ok((input, name))
    }

    fn parse_expr(&mut self, input: Span<'a>) -> IResult<Span<'a>, &'a AstNode<'a>> {
        self.parse_literalnum(input)
            .or(self.parse_identifier(input))
    }

    fn parse_body(&mut self, input: Span<'a>) -> IResult<Span<'a>, Vec<&'a AstNode<'a>>> {
        println!("Parsing body");
        let (input, _) = preceded(multispace0, char('{'))(input)?;
        let (input, body) = many0(|i| self.parse_stmt(i))(input)?;
        let (input, _) = preceded(multispace0, char('}'))(input)?;
        println!("Parsed body: {:#?}", body);
        Ok((input, body))
    }

    fn parse_stmt(&mut self, input: Span<'a>) -> IResult<Span<'a>, &'a AstNode<'a>> {
        let (input, _) = multispace0(input)?;
        let (input, ret) = self
            .call(input)
            .or_else(|_| self.parse_kwd(input))
            .or_else(|_| self.parse_fn(input))
            .or_else(|_| self.parse_expr(input))?;
        let (input, _) = multispace0(input)?;
        let (input, _) = opt(char(';'))(input)?;
        Ok((input, ret))
    }

    fn parse_kwd(&mut self, input: Span<'a>) -> IResult<Span<'a>, &'a AstNode<'a>> {
        self.return_stmt(input)
        //alt((|i| self.return_stmt(i)))(input)
    }

    fn call(&mut self, input: Span<'a>) -> IResult<Span<'a>, &'a AstNode<'a>> {
        let (input, name_node) = self.parse_identifier(input)?;
        let name = if let Identifier { ref name } = *name_node {
            name.clone()
        } else {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Tag,
            )));
        };
        let (input, bang) = opt(char('!'))(input)?;
        let (input, args) = delimited(char('('), many0(|i| self.parse_expr(i)), char(')'))(input)?;
        let node = if bang.is_some() {
            self.arena.alloc(BangCall { name, args })
        } else {
            self.arena.alloc(Call { name, args })
        };
        Ok((input, node))
    }

    fn return_stmt(&mut self, input: Span<'a>) -> IResult<Span<'a>, &'a AstNode<'a>> {
        let (input, _) = tag("return")(input)?;
        let (input, _) = multispace1(input)?;
        let (input, expr) = self.parse_expr(input)?;
        Ok((input, self.arena.alloc(Return { value: expr })))
    }

    fn unknown_stmt(&mut self, input: Span<'a>) -> IResult<Span<'a>, &'a AstNode<'a>> {
        println!("Unknown statement");
        let (input, _) = multispace0(input)?;
        let (input, unknown) = recognize(nom::bytes::streaming::take_until(";"))(input)?;
        let (input, _) = opt(char(';'))(input)?;
        Ok((
            input,
            self.arena.alloc(Unknown {
                stmt: unknown.fragment().to_string(),
            }),
        ))
    }
}
