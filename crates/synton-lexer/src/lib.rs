//! # Synton Lexer
//!
//! Lexical analysis for Synton using Logos.

#![warn(missing_docs, unused_crate_dependencies)]

use logos::Logos;

pub mod error;

pub use error::{LexError, LexResult};

/// Token in Synton source code
#[derive(Logos, Debug, Clone, PartialEq, Eq)]
pub enum Token {
    // Whitespace (skipped via skip attributes on the enum)
    #[regex(r"[ \t\r\n]+", logos::skip)]
    Whitespace,

    // Comments (skipped)
    #[regex(r"//[^\n]*", logos::skip)]
    LineComment,

    #[regex(r"/\*[^*]*\*+(?:[^/*][^*]*\*+)*/", logos::skip)]
    BlockComment,

    // Delimiters
    #[token("(")] LParen,
    #[token(")")] RParen,
    #[token("[")] LBracket,
    #[token("]")] RBracket,
    #[token("{")] LBrace,
    #[token("}")] RBrace,
    #[token(";")] Semi,
    #[token(",")] Comma,
    #[token(":")] Colon,
    #[token("::")] ColonColon,
    #[token("->")] Arrow,

    // Operators
    #[token("+")] Plus,
    #[token("-")] Minus,
    #[token("*")] Star,
    #[token("/")] Slash,
    #[token("%")] Percent,
    #[token("^")] Caret,
    #[token("&")] Amp,
    #[token("|")] Pipe,
    #[token("!")] Bang,
    #[token("=")] Eq,
    #[token("<")] Lt,
    #[token(">")] Gt,
    #[token("~")] Tilde,
    #[token(".")] Dot,

    // Multi-character operators
    #[token("==")] EqEq,
    #[token("!=")] NotEq,
    #[token("<=")] LtEq,
    #[token(">=")] GtEq,
    #[token("&&")] AndAnd,
    #[token("||")] OrOr,
    #[token("<<")] Shl,
    #[token(">>")] Shr,
    #[token("**")] StarStar,

    // Short operators for AI efficiency
    #[token("?")] Question,    // branch (if)
    #[token("$")] Dollar,      // call
    #[token("@")] At,          // contract

    // Keywords
    #[token("fn")] KwFn,
    #[token("let")] KwLet,
    #[token("set")] KwSet,
    #[token("if")] KwIf,
    #[token("else")] KwElse,
    #[token("branch")] KwBranch,
    #[token("match")] KwMatch,
    #[token("loop")] KwLoop,
    #[token("while")] KwWhile,
    #[token("for")] KwFor,
    #[token("in")] KwIn,
    #[token("break")] KwBreak,
    #[token("continue")] KwContinue,
    #[token("return")] KwReturn,
    #[token("struct")] KwStruct,
    #[token("enum")] KwEnum,
    #[token("type")] KwType,
    #[token("const")] KwConst,
    #[token("import")] KwImport,
    #[token("export")] KwExport,
    #[token("as")] KwAs,
    #[token("mod")] KwMod,

    // Type keywords
    #[token("i32")] KwI32,
    #[token("i64")] KwI64,
    #[token("u32")] KwU32,
    #[token("u64")] KwU64,
    #[token("f32")] KwF32,
    #[token("f64")] KwF64,
    #[token("bool")] KwBool,
    #[token("string")] KwString,
    #[token("str")] KwStr,
    #[token("char")] KwChar,
    #[token("list")] KwList,
    #[token("map")] KwMap,
    #[token("maybe")] KwMaybe,
    #[token("result")] KwResult,

    // Literals
    #[regex(r"[0-9]+", |lex| lex.slice().parse::<i64>().ok())]
    Integer(i64),

    #[regex(r"[0-9]+\.[0-9]+([eE][+-]?[0-9]+)?")]
    Float,

    #[regex(r#"'[^']'"#, |lex| lex.slice().chars().nth(1))]
    Char(char),

    #[regex(r#""[^"]*""#, |lex| Some(lex.slice().to_string()))]
    String(String),

    #[regex(r#"b"[^"]*""#, |lex| Some(lex.slice().to_string()))]
    ByteString(String),

    // Boolean literals
    #[token("true")] True,
    #[token("false")] False,

    // Special tokens
    #[token("=>")] FatArrow,

    // Identifiers (must come after underscore)
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_-]*", |lex| Some(lex.slice().to_string()))]
    Identifier(String),
}

impl Token {
    /// Check if this is a keyword
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            Self::KwFn
                | Self::KwLet
                | Self::KwSet
                | Self::KwIf
                | Self::KwElse
                | Self::KwBranch
                | Self::KwMatch
                | Self::KwLoop
                | Self::KwWhile
                | Self::KwFor
                | Self::KwIn
                | Self::KwBreak
                | Self::KwContinue
                | Self::KwReturn
                | Self::KwStruct
                | Self::KwEnum
                | Self::KwType
                | Self::KwConst
                | Self::KwImport
                | Self::KwExport
                | Self::KwAs
                | Self::KwMod
        )
    }

    /// Check if this is a type keyword
    pub fn is_type_keyword(&self) -> bool {
        matches!(
            self,
            Self::KwI32
                | Self::KwI64
                | Self::KwU32
                | Self::KwU64
                | Self::KwF32
                | Self::KwF64
                | Self::KwBool
                | Self::KwString
                | Self::KwStr
                | Self::KwChar
                | Self::KwList
                | Self::KwMap
                | Self::KwMaybe
                | Self::KwResult
        )
    }

    /// Check if this is a literal
    pub fn is_literal(&self) -> bool {
        matches!(
            self,
            Self::Integer(_) | Self::Float | Self::Char(_) | Self::String(_) | Self::True | Self::False
        )
    }

    /// Check if token can start an expression
    pub fn can_start_expr(&self) -> bool {
        matches!(
            self,
            Self::Integer(_)
                | Self::Float
                | Self::String(_)
                | Self::True
                | Self::False
                | Self::Identifier(_)
                | Self::LParen
                | Self::LBracket
                | Self::LBrace
                | Self::Minus
                | Self::Bang
                | Self::Tilde
                | Self::Star
                | Self::Amp
                | Self::Question
                | Self::Dollar
        )
    }
}

/// A token with location information
#[derive(Debug, Clone)]
pub struct TokenKind {
    pub token: Token,
    pub span: std::ops::Range<usize>,
}

/// Lexer for Synton source code
pub struct Lexer<'a> {
    source: &'a str,
    inner: logos::Lexer<'a, Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            inner: Token::lexer(source),
        }
    }

    pub fn source(&self) -> &'a str {
        self.source
    }

    pub fn remainder(&self) -> &'a str {
        self.inner.remainder()
    }

    pub fn tokenize_all(&mut self) -> LexResult<Vec<TokenKind>> {
        let mut tokens = Vec::new();
        while let Some(result) = self.inner.next() {
            let token = result.map_err(|_| LexError::Other("Lexer error".to_string()))?;
            let span = self.inner.span();
            tokens.push(TokenKind {
                token,
                span,
            });
        }
        Ok(tokens)
    }
}

/// Convenience function to tokenize a string
pub fn tokenize(source: &str) -> LexResult<Vec<TokenKind>> {
    Lexer::new(source).tokenize_all()
}
