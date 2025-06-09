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
fn convert_visibility(_vis: &syn::Visibility) -> Visibility {
    Visibility::Private // TODO: Implement proper conversion
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

fn convert_block(_block: &syn::Block) -> Vec<Statement> {
    Vec::new() // TODO: Implement proper conversion
}

fn convert_attributes(_attrs: &[syn::Attribute]) -> Vec<Attribute> {
    Vec::new() // TODO: Implement proper conversion
}

fn convert_struct_fields(_fields: &syn::Fields) -> Vec<Field> {
    Vec::new() // TODO: Implement proper conversion
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

fn convert_expression(_expr: &syn::Expr) -> Expression {
    Expression::Literal(Literal::Integer(0)) // TODO: Implement proper conversion
}
