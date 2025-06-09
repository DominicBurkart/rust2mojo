//! AST representations and transformations
//!
//! This module contains intermediate representations used during compilation from Rust to Mojo.

use serde::{Deserialize, Serialize};

/// Intermediate representation for the compiler
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilationUnit {
    pub items: Vec<Item>,
    pub metadata: CompilationMetadata,
}

/// Metadata about the compilation unit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilationMetadata {
    pub source_file: Option<String>,
    pub rust_edition: String,
    pub target_mojo_version: String,
}

/// Top-level items in the compilation unit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Item {
    Function(Function),
    Struct(Struct),
    Enum(Enum),
    Impl(Impl),
    Use(Use),
    Mod(Module),
    Const(Const),
    Static(Static),
    Type(TypeAlias),
}

/// Function representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    pub name: String,
    pub visibility: Visibility,
    pub generics: Vec<Generic>,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<Type>,
    pub body: Vec<Statement>,
    pub attributes: Vec<Attribute>,
}

/// Struct representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Struct {
    pub name: String,
    pub visibility: Visibility,
    pub generics: Vec<Generic>,
    pub fields: Vec<Field>,
    pub attributes: Vec<Attribute>,
}

/// Enum representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enum {
    pub name: String,
    pub visibility: Visibility,
    pub generics: Vec<Generic>,
    pub variants: Vec<Variant>,
    pub attributes: Vec<Attribute>,
}

/// Implementation block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Impl {
    pub target_type: Type,
    pub trait_: Option<Type>,
    pub generics: Vec<Generic>,
    pub items: Vec<ImplItem>,
}

/// Use statement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Use {
    pub path: String,
    pub visibility: Visibility,
}

/// Module
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub name: String,
    pub visibility: Visibility,
    pub items: Vec<Item>,
}

/// Constant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Const {
    pub name: String,
    pub visibility: Visibility,
    pub type_: Type,
    pub value: Expression,
}

/// Static variable
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Static {
    pub name: String,
    pub visibility: Visibility,
    pub mutable: bool,
    pub type_: Type,
    pub value: Expression,
}

/// Type alias
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeAlias {
    pub name: String,
    pub visibility: Visibility,
    pub generics: Vec<Generic>,
    pub type_: Type,
}

/// Visibility modifiers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Visibility {
    Public,
    Private,
    Crate,
    Super,
    InPath(String),
}

/// Generic parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Generic {
    pub name: String,
    pub bounds: Vec<Type>,
}

/// Function parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub type_: Type,
    pub mutable: bool,
}

/// Struct field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub visibility: Visibility,
    pub type_: Type,
}

/// Enum variant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Variant {
    pub name: String,
    pub data: VariantData,
}

/// Variant data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VariantData {
    Unit,
    Tuple(Vec<Type>),
    Struct(Vec<Field>),
}

/// Implementation item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImplItem {
    Function(Function),
    Const(Const),
    Type(TypeAlias),
}

/// Type representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Type {
    Path(String),
    Reference {
        mutable: bool,
        inner: Box<Type>,
    },
    Pointer {
        mutable: bool,
        inner: Box<Type>,
    },
    Array {
        inner: Box<Type>,
        size: Option<usize>,
    },
    Slice(Box<Type>),
    Tuple(Vec<Type>),
    Function {
        params: Vec<Type>,
        return_: Box<Type>,
    },
    Generic(String),
    Unit,
}

/// Statement representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Statement {
    Expression(Expression),
    Let {
        name: String,
        mutable: bool,
        type_: Option<Type>,
        value: Option<Expression>,
    },
    Return(Option<Expression>),
    If {
        condition: Expression,
        then_branch: Vec<Statement>,
        else_branch: Option<Vec<Statement>>,
    },
    While {
        condition: Expression,
        body: Vec<Statement>,
    },
    For {
        pattern: String,
        iterator: Expression,
        body: Vec<Statement>,
    },
    Match {
        expr: Expression,
        arms: Vec<MatchArm>,
    },
    Block(Vec<Statement>),
}

/// Expression representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Expression {
    Literal(Literal),
    Identifier(String),
    Path(String),
    Call {
        function: Box<Expression>,
        args: Vec<Expression>,
    },
    MethodCall {
        receiver: Box<Expression>,
        method: String,
        args: Vec<Expression>,
    },
    FieldAccess {
        object: Box<Expression>,
        field: String,
    },
    Index {
        object: Box<Expression>,
        index: Box<Expression>,
    },
    Binary {
        left: Box<Expression>,
        op: BinaryOp,
        right: Box<Expression>,
    },
    Unary {
        op: UnaryOp,
        operand: Box<Expression>,
    },
    Cast {
        expr: Box<Expression>,
        type_: Type,
    },
    Reference {
        mutable: bool,
        expr: Box<Expression>,
    },
    Dereference(Box<Expression>),
    Block(Vec<Statement>),
    Array(Vec<Expression>),
    Tuple(Vec<Expression>),
    Struct {
        name: String,
        fields: Vec<(String, Expression)>,
    },
}

/// Literal values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Literal {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Char(char),
}

/// Binary operators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
    BitAnd,
    BitOr,
    BitXor,
    Shl,
    Shr,
    Assign,
}

/// Unary operators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnaryOp {
    Not,
    Neg,
    Deref,
}

/// Match arm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub guard: Option<Expression>,
    pub body: Vec<Statement>,
}

/// Pattern matching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Pattern {
    Wildcard,
    Identifier(String),
    Literal(Literal),
    Tuple(Vec<Pattern>),
    Struct {
        name: String,
        fields: Vec<(String, Pattern)>,
    },
    Enum {
        path: String,
        variant: String,
        fields: Vec<Pattern>,
    },
}

/// Attribute
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attribute {
    pub path: String,
    pub tokens: String,
}
