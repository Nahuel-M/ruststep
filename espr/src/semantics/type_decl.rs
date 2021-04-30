use super::*;
use crate::ast;
use inflector::Inflector;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnderlyingType {
    Simple(TypeRef),
    Reference(TypeRef),
    Enumeration(Vec<String>),
    Select(Vec<TypeRef>),
}

impl Legalize for UnderlyingType {
    type Input = ast::types::UnderlyingType;
    fn legalize(ns: &Namespace, scope: &Scope, input: &Self::Input) -> Result<Self, SemanticError> {
        let underlying_type = match input {
            ast::types::UnderlyingType::Simple(simple) => {
                UnderlyingType::Simple(TypeRef::SimpleType(*simple))
            }
            ast::types::UnderlyingType::Reference(name) => {
                UnderlyingType::Reference(ns.lookup_type(scope, name)?)
            }
            ast::types::UnderlyingType::Enumeration { items, .. } => {
                // FIXME extensibility
                UnderlyingType::Enumeration(items.clone())
            }
            ast::types::UnderlyingType::Select { types, .. } => {
                // FIXME extensibility
                let refs: Result<Vec<TypeRef>, _> =
                    types.iter().map(|ty| ns.lookup_type(scope, ty)).collect();
                UnderlyingType::Select(refs?)
            }
            _ => unimplemented!(),
        };
        Ok(underlying_type)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeDecl {
    type_id: String,
    underlying_type: UnderlyingType,
}

impl Legalize for TypeDecl {
    type Input = ast::types::TypeDecl;
    fn legalize(
        ns: &Namespace,
        scope: &Scope,
        type_decl: &Self::Input,
    ) -> Result<Self, SemanticError> {
        Ok(TypeDecl {
            type_id: type_decl.type_id.clone(),
            underlying_type: UnderlyingType::legalize(ns, scope, &type_decl.underlying_type)?,
        })
    }
}

impl ToTokens for TypeDecl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let id = format_ident!("{}", &self.type_id.to_pascal_case());
        match &self.underlying_type {
            UnderlyingType::Simple(type_ref) | UnderlyingType::Reference(type_ref) => tokens
                .append_all(quote! {
                    pub type #id = #type_ref;
                }),
            UnderlyingType::Enumeration(items) => {
                let items: Vec<_> = items
                    .into_iter()
                    .map(|i| format_ident!("{}", i.to_pascal_case()))
                    .collect();
                tokens.append_all(quote! {
                    #[derive(Debug, Clone, PartialEq)]
                    pub enum #id {
                        #( #items ),*
                    }
                });
            }
            UnderlyingType::Select(types) => tokens.append_all(quote! {
                #[derive(Debug, Clone, PartialEq)]
                pub enum #id {
                    #(#types(Box<#types>)),*
                }
            }),
        }
    }
}