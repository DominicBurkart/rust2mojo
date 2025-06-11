//! Rust source code parsing functionality
//!
//! This module handles parsing Rust source code into our intermediate AST representation.

use crate::ast::*;
use crate::error::Result;
use syn::{visit::Visit, File, Item as SynItem};

/// Parse Rust source code into our intermediate representation
pub fn parse_rust_code(source: &str) -> Result<CompilationUnit> {
    let syntax_tree: File = syn::parse_str(source)?;

    let mut converter = AstConverter::new();
    converter.visit_file(&syntax_tree);

    Ok(CompilationUnit {
        items: converter.items,
        metadata: CompilationMetadata {
            source_file: None,
            rust_edition: "2021".to_string(),
            target_mojo_version: "24.5".to_string(),
        },
    })
}

/// Convert syn AST to our intermediate representation
struct AstConverter {
    items: Vec<Item>,
}

impl AstConverter {
    fn new() -> Self {
        Self { items: Vec::new() }
    }
}

impl<'ast> Visit<'ast> for AstConverter {
    fn visit_item(&mut self, item: &'ast SynItem) {
        match item {
            SynItem::Fn(item_fn) => {
                let function = convert_function(item_fn);
                self.items.push(Item::Function(function));
            }
            SynItem::Struct(item_struct) => {
                let struct_item = convert_struct(item_struct);
                self.items.push(Item::Struct(struct_item));
            }
            SynItem::Enum(item_enum) => {
                let enum_item = convert_enum(item_enum);
                self.items.push(Item::Enum(enum_item));
            }
            SynItem::Impl(item_impl) => {
                let impl_item = convert_impl(item_impl);
                self.items.push(Item::Impl(impl_item));
            }
            SynItem::Use(item_use) => {
                let use_item = convert_use(item_use);
                self.items.push(Item::Use(use_item));
            }
            SynItem::Mod(item_mod) => {
                let mod_item = convert_module(item_mod);
                self.items.push(Item::Mod(mod_item));
            }
            SynItem::Const(item_const) => {
                let const_item = convert_const(item_const);
                self.items.push(Item::Const(const_item));
            }
            SynItem::Static(item_static) => {
                let static_item = convert_static(item_static);
                self.items.push(Item::Static(static_item));
            }
            SynItem::Type(item_type) => {
                let type_item = convert_type_alias(item_type);
                self.items.push(Item::Type(type_item));
            }
            _ => {
                // TODO: Handle other item types or emit warnings
            }
        }

        syn::visit::visit_item(self, item);
    }
}

fn convert_function(item_fn: &syn::ItemFn) -> Function {
    Function {
        name: item_fn.sig.ident.to_string(),
        visibility: convert_visibility(&item_fn.vis),
        generics: convert_generics(&item_fn.sig.generics),
        parameters: convert_parameters(&item_fn.sig.inputs),
        return_type: convert_return_type(&item_fn.sig.output),
        body: convert_block(&item_fn.block),
        attributes: convert_attributes(&item_fn.attrs),
    }
}

fn convert_struct(item_struct: &syn::ItemStruct) -> Struct {
    Struct {
        name: item_struct.ident.to_string(),
        visibility: convert_visibility(&item_struct.vis),
        generics: convert_generics(&item_struct.generics),
        fields: convert_struct_fields(&item_struct.fields),
        attributes: convert_attributes(&item_struct.attrs),
    }
}

fn convert_enum(item_enum: &syn::ItemEnum) -> Enum {
    Enum {
        name: item_enum.ident.to_string(),
        visibility: convert_visibility(&item_enum.vis),
        generics: convert_generics(&item_enum.generics),
        variants: item_enum.variants.iter().map(convert_variant).collect(),
        attributes: convert_attributes(&item_enum.attrs),
    }
}

fn convert_impl(item_impl: &syn::ItemImpl) -> Impl {
    Impl {
        target_type: convert_type(&item_impl.self_ty),
        trait_: item_impl
            .trait_
            .as_ref()
            .map(|(_, path, _)| convert_path_type(path)),
        generics: convert_generics(&item_impl.generics),
        items: item_impl.items.iter().map(convert_impl_item).collect(),
    }
}

fn convert_use(item_use: &syn::ItemUse) -> Use {
    Use {
        path: quote::quote!(#item_use).to_string(),
        visibility: convert_visibility(&item_use.vis),
    }
}

fn convert_module(item_mod: &syn::ItemMod) -> Module {
    Module {
        name: item_mod.ident.to_string(),
        visibility: convert_visibility(&item_mod.vis),
        items: Vec::new(), // TODO: Convert module items
    }
}

fn convert_const(item_const: &syn::ItemConst) -> Const {
    Const {
        name: item_const.ident.to_string(),
        visibility: convert_visibility(&item_const.vis),
        type_: convert_type(&item_const.ty),
        value: convert_expression(&item_const.expr),
    }
}

fn convert_static(item_static: &syn::ItemStatic) -> Static {
    Static {
        name: item_static.ident.to_string(),
        visibility: convert_visibility(&item_static.vis),
        mutable: matches!(item_static.mutability, syn::StaticMutability::Mut(_)),
        type_: convert_type(&item_static.ty),
        value: convert_expression(&item_static.expr),
    }
}

fn convert_type_alias(item_type: &syn::ItemType) -> TypeAlias {
    TypeAlias {
        name: item_type.ident.to_string(),
        visibility: convert_visibility(&item_type.vis),
        generics: convert_generics(&item_type.generics),
        type_: convert_type(&item_type.ty),
    }
}

// Helper conversion functions (stubs for now)
fn convert_visibility(vis: &syn::Visibility) -> Visibility {
    match vis {
        syn::Visibility::Public(_) => Visibility::Public,
        syn::Visibility::Restricted(_) => Visibility::Private, // pub(crate), pub(super), etc. treated as private
        syn::Visibility::Inherited => Visibility::Private,
    }
}

fn convert_generics(_generics: &syn::Generics) -> Vec<Generic> {
    Vec::new() // TODO: Implement proper conversion
}

fn convert_parameters(
    inputs: &syn::punctuated::Punctuated<syn::FnArg, syn::Token![,]>,
) -> Vec<Parameter> {
    inputs
        .iter()
        .filter_map(|arg| {
            match arg {
                syn::FnArg::Typed(pat_type) => {
                    if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                        Some(Parameter {
                            name: pat_ident.ident.to_string(),
                            type_: convert_type(&pat_type.ty),
                            mutable: pat_ident.mutability.is_some(),
                        })
                    } else {
                        None // Skip complex patterns for now
                    }
                }
                syn::FnArg::Receiver(_) => None, // Skip self parameters for now
            }
        })
        .collect()
}

fn convert_return_type(output: &syn::ReturnType) -> Option<Type> {
    match output {
        syn::ReturnType::Default => None,
        syn::ReturnType::Type(_, ty) => Some(convert_type(ty)),
    }
}

fn convert_block(block: &syn::Block) -> Vec<Statement> {
    block.stmts.iter().filter_map(convert_statement).collect()
}

fn convert_attributes(_attrs: &[syn::Attribute]) -> Vec<Attribute> {
    Vec::new() // TODO: Implement proper conversion
}

fn convert_struct_fields(fields: &syn::Fields) -> Vec<Field> {
    match fields {
        syn::Fields::Named(fields_named) => fields_named
            .named
            .iter()
            .filter_map(|field| {
                field.ident.as_ref().map(|ident| Field {
                    name: ident.to_string(),
                    type_: convert_type(&field.ty),
                    visibility: convert_visibility(&field.vis),
                })
            })
            .collect(),
        syn::Fields::Unnamed(fields_unnamed) => {
            // Tuple struct fields - create numbered field names
            fields_unnamed
                .unnamed
                .iter()
                .enumerate()
                .map(|(i, field)| Field {
                    name: format!("field_{}", i),
                    type_: convert_type(&field.ty),
                    visibility: convert_visibility(&field.vis),
                })
                .collect()
        }
        syn::Fields::Unit => {
            // Unit struct has no fields
            Vec::new()
        }
    }
}

fn convert_variant(_variant: &syn::Variant) -> Variant {
    Variant {
        name: _variant.ident.to_string(),
        data: VariantData::Unit, // TODO: Implement proper conversion
    }
}

fn convert_impl_item(_item: &syn::ImplItem) -> ImplItem {
    match _item {
        syn::ImplItem::Fn(method) => ImplItem::Function(convert_function(&syn::ItemFn {
            attrs: method.attrs.clone(),
            vis: method.vis.clone(),
            sig: method.sig.clone(),
            block: Box::new(method.block.clone()),
        })),
        _ => {
            // TODO: Handle other impl item types
            ImplItem::Function(Function {
                name: "placeholder".to_string(),
                visibility: Visibility::Private,
                generics: Vec::new(),
                parameters: Vec::new(),
                return_type: None,
                body: Vec::new(),
                attributes: Vec::new(),
            })
        }
    }
}

fn convert_type(ty: &syn::Type) -> Type {
    match ty {
        syn::Type::Path(type_path) => {
            if let Some(ident) = type_path.path.get_ident() {
                let type_str = ident.to_string();
                Type::Path(type_str)
            } else {
                // Handle complex paths
                let path_str = type_path
                    .path
                    .segments
                    .iter()
                    .map(|s| s.ident.to_string())
                    .collect::<Vec<_>>()
                    .join("::");
                Type::Path(path_str)
            }
        }
        syn::Type::Reference(type_ref) => Type::Reference {
            mutable: type_ref.mutability.is_some(),
            inner: Box::new(convert_type(&type_ref.elem)),
        },
        syn::Type::Ptr(type_ptr) => Type::Pointer {
            mutable: type_ptr.mutability.is_some(),
            inner: Box::new(convert_type(&type_ptr.elem)),
        },
        syn::Type::Array(type_array) => {
            let inner = Box::new(convert_type(&type_array.elem));
            let size = if let syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Int(lit_int),
                ..
            }) = &type_array.len
            {
                lit_int.base10_parse().ok()
            } else {
                None
            };
            Type::Array { inner, size }
        }
        syn::Type::Slice(type_slice) => Type::Slice(Box::new(convert_type(&type_slice.elem))),
        syn::Type::Tuple(type_tuple) => {
            if type_tuple.elems.is_empty() {
                Type::Unit
            } else {
                Type::Tuple(type_tuple.elems.iter().map(convert_type).collect())
            }
        }
        _ => {
            // For unsupported types, default to Unit for now
            Type::Unit
        }
    }
}

fn convert_path_type(_path: &syn::Path) -> Type {
    Type::Path(
        _path
            .segments
            .iter()
            .map(|s| s.ident.to_string())
            .collect::<Vec<_>>()
            .join("::"),
    )
}

fn convert_statement(stmt: &syn::Stmt) -> Option<Statement> {
    match stmt {
        syn::Stmt::Local(local) => convert_local_statement(local),
        syn::Stmt::Item(_item) => {
            // Items in function bodies are rare, skip for now
            None
        }
        syn::Stmt::Expr(expr, _semicolon) => {
            // Check if this is a control flow statement
            match expr {
                syn::Expr::If(expr_if) => convert_if_statement(expr_if),
                syn::Expr::While(expr_while) => convert_while_statement(expr_while),
                syn::Expr::ForLoop(expr_for) => convert_for_statement(expr_for),
                syn::Expr::Return(expr_return) => {
                    let value = expr_return.expr.as_ref().map(|e| convert_expression(e));
                    Some(Statement::Return(value))
                }
                _ => {
                    // Regular expression statement
                    Some(Statement::Expression(convert_expression(expr)))
                }
            }
        }
        syn::Stmt::Macro(_) => {
            // Macro calls, skip for now
            None
        }
    }
}

fn convert_local_statement(local: &syn::Local) -> Option<Statement> {
    // Extract variable name from pattern
    if let syn::Pat::Ident(pat_ident) = &local.pat {
        let name = pat_ident.ident.to_string();
        let mutable = pat_ident.mutability.is_some();

        // Extract type annotation if present - syn::Local doesn't have ty field directly
        let type_ = None; // TODO: Extract from pattern type annotations when present

        // Extract initializer if present
        let value = local
            .init
            .as_ref()
            .map(|init| convert_expression(&init.expr));

        Some(Statement::Let {
            name,
            mutable,
            type_,
            value,
        })
    } else {
        // Complex patterns not supported yet
        None
    }
}

fn convert_expression(expr: &syn::Expr) -> Expression {
    match expr {
        syn::Expr::Lit(expr_lit) => convert_literal_expression(expr_lit),
        syn::Expr::Path(expr_path) => {
            if let Some(ident) = expr_path.path.get_ident() {
                Expression::Identifier(ident.to_string())
            } else {
                // Complex paths
                let path = expr_path
                    .path
                    .segments
                    .iter()
                    .map(|s| s.ident.to_string())
                    .collect::<Vec<_>>()
                    .join("::");
                Expression::Path(path)
            }
        }
        syn::Expr::Binary(expr_binary) => {
            let left = Box::new(convert_expression(&expr_binary.left));
            let right = Box::new(convert_expression(&expr_binary.right));
            let op = convert_binary_operator(&expr_binary.op);
            Expression::Binary { left, op, right }
        }
        syn::Expr::Call(expr_call) => {
            let function = Box::new(convert_expression(&expr_call.func));
            let args = expr_call.args.iter().map(convert_expression).collect();
            Expression::Call { function, args }
        }
        syn::Expr::Return(expr_return) => {
            // Return expressions are handled as statements in our AST
            match &expr_return.expr {
                Some(return_expr) => convert_expression(return_expr),
                None => Expression::Literal(Literal::String("()".to_string())), // Unit return
            }
        }
        syn::Expr::If(_expr_if) => {
            // If expressions are complex, for now return a placeholder
            Expression::Literal(Literal::String("if_expr_placeholder".to_string()))
        }
        syn::Expr::Block(_expr_block) => {
            // Block expressions can contain statements
            Expression::Literal(Literal::String("block_expr_placeholder".to_string()))
        }
        syn::Expr::Assign(expr_assign) => {
            let left = Box::new(convert_expression(&expr_assign.left));
            let right = Box::new(convert_expression(&expr_assign.right));
            Expression::Binary {
                left,
                op: BinaryOp::Assign,
                right,
            }
        }
        syn::Expr::Field(expr_field) => {
            // Field access like obj.field
            let base = convert_expression(&expr_field.base);
            if let syn::Member::Named(field_name) = &expr_field.member {
                Expression::Path(format!(
                    "{}.{}",
                    match base {
                        Expression::Identifier(id) => id,
                        Expression::Path(path) => path,
                        _ => "unknown".to_string(),
                    },
                    field_name
                ))
            } else {
                Expression::Literal(Literal::String("field_access_placeholder".to_string()))
            }
        }
        syn::Expr::Index(_expr_index) => {
            // Array/slice indexing
            Expression::Literal(Literal::String("index_placeholder".to_string()))
        }
        syn::Expr::Unary(_expr_unary) => {
            // Unary operations like !x, -x, *x, &x
            Expression::Literal(Literal::String("unary_placeholder".to_string()))
        }
        _ => {
            // Fallback for unsupported expressions
            Expression::Literal(Literal::String("unsupported_expr".to_string()))
        }
    }
}

fn convert_literal_expression(expr_lit: &syn::ExprLit) -> Expression {
    match &expr_lit.lit {
        syn::Lit::Str(lit_str) => Expression::Literal(Literal::String(lit_str.value())),
        syn::Lit::Int(lit_int) => {
            if let Ok(value) = lit_int.base10_parse::<i64>() {
                Expression::Literal(Literal::Integer(value))
            } else {
                Expression::Literal(Literal::Integer(0))
            }
        }
        syn::Lit::Float(lit_float) => {
            if let Ok(value) = lit_float.base10_parse::<f64>() {
                Expression::Literal(Literal::Float(value))
            } else {
                Expression::Literal(Literal::Float(0.0))
            }
        }
        syn::Lit::Bool(lit_bool) => Expression::Literal(Literal::Boolean(lit_bool.value)),
        syn::Lit::Char(lit_char) => Expression::Literal(Literal::Char(lit_char.value())),
        _ => Expression::Literal(Literal::String("unsupported_literal".to_string())),
    }
}

fn convert_if_statement(expr_if: &syn::ExprIf) -> Option<Statement> {
    let condition = convert_expression(&expr_if.cond);
    let then_branch = convert_block(&expr_if.then_branch);

    let else_branch = if let Some((_, else_expr)) = &expr_if.else_branch {
        match &**else_expr {
            syn::Expr::Block(expr_block) => Some(convert_block(&expr_block.block)),
            syn::Expr::If(nested_if) => {
                // Handle else if by converting to nested if statement
                convert_if_statement(nested_if).map(|nested_if_stmt| vec![nested_if_stmt])
            }
            _ => None,
        }
    } else {
        None
    };

    Some(Statement::If {
        condition,
        then_branch,
        else_branch,
    })
}

fn convert_while_statement(expr_while: &syn::ExprWhile) -> Option<Statement> {
    let condition = convert_expression(&expr_while.cond);
    let body = convert_block(&expr_while.body);

    Some(Statement::While { condition, body })
}

fn convert_for_statement(expr_for: &syn::ExprForLoop) -> Option<Statement> {
    // Extract pattern (usually just an identifier)
    let pattern = match &*expr_for.pat {
        syn::Pat::Ident(pat_ident) => pat_ident.ident.to_string(),
        _ => "item".to_string(), // Fallback for complex patterns
    };

    let iterator = convert_expression(&expr_for.expr);
    let body = convert_block(&expr_for.body);

    Some(Statement::For {
        pattern,
        iterator,
        body,
    })
}

fn convert_binary_operator(op: &syn::BinOp) -> BinaryOp {
    match op {
        syn::BinOp::Add(_) => BinaryOp::Add,
        syn::BinOp::Sub(_) => BinaryOp::Sub,
        syn::BinOp::Mul(_) => BinaryOp::Mul,
        syn::BinOp::Div(_) => BinaryOp::Div,
        syn::BinOp::Rem(_) => BinaryOp::Mod,
        syn::BinOp::And(_) => BinaryOp::And,
        syn::BinOp::Or(_) => BinaryOp::Or,
        syn::BinOp::BitXor(_) => BinaryOp::BitXor,
        syn::BinOp::BitAnd(_) => BinaryOp::BitAnd,
        syn::BinOp::BitOr(_) => BinaryOp::BitOr,
        syn::BinOp::Shl(_) => BinaryOp::Shl,
        syn::BinOp::Shr(_) => BinaryOp::Shr,
        syn::BinOp::Eq(_) => BinaryOp::Eq,
        syn::BinOp::Lt(_) => BinaryOp::Lt,
        syn::BinOp::Le(_) => BinaryOp::Le,
        syn::BinOp::Ne(_) => BinaryOp::Ne,
        syn::BinOp::Ge(_) => BinaryOp::Ge,
        syn::BinOp::Gt(_) => BinaryOp::Gt,
        _ => BinaryOp::Add, // Fallback
    }
}
