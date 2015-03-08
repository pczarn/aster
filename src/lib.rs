#![feature(rustc_private)]

extern crate syntax;

pub mod attr;
pub mod block;
pub mod ctx;
pub mod expr;
pub mod fn_decl;
pub mod generics;
pub mod ident;
pub mod invoke;
pub mod item;
pub mod lifetime;
pub mod lit;
pub mod method;
pub mod name;
pub mod pat;
pub mod path;
pub mod stmt;
pub mod str;
pub mod ty;
pub mod ty_param;

use syntax::ast;
use syntax::codemap::{DUMMY_SP, Span};

pub use ctx::Ctx;

//////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct AstBuilder<'a> {
    ctx: &'a Ctx,
    span: Span,
}

impl<'a> AstBuilder<'a> {
    pub fn new(ctx: &Ctx) -> AstBuilder {
        AstBuilder {
            ctx: ctx,
            span: DUMMY_SP,
        }
    }

    pub fn span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }

    pub fn id<I>(&self, id: I) -> ast::Ident
        where I: ident::ToIdent
    {
        id.to_ident(self.ctx)
    }

    pub fn name<N>(&self, name: N) -> ast::Name
        where N: name::ToName
    {
        name.to_name(self.ctx)
    }

    pub fn path(&self) -> path::PathBuilder {
        path::PathBuilder::new(self.ctx)
    }

    pub fn ty(&self) -> ty::TyBuilder {
        ty::TyBuilder::new(self.ctx).span(self.span)
    }

    pub fn ty_param<I>(&self, id: I) -> ty_param::TyParamBuilder
        where I: ident::ToIdent,
    {
        ty_param::TyParamBuilder::new(self.ctx, id).span(self.span)
    }

    pub fn from_ty_param(&self, ty_param: ast::TyParam) -> ty_param::TyParamBuilder {
        ty_param::TyParamBuilder::from_ty_param(self.ctx, ty_param)
    }

    pub fn generics(&self) -> generics::GenericsBuilder {
        generics::GenericsBuilder::new(self.ctx).span(self.span)
    }

    pub fn from_generics(&self, generics: ast::Generics) -> generics::GenericsBuilder {
        generics::GenericsBuilder::from_generics(self.ctx, generics).span(self.span)
    }

    pub fn expr(&self) -> expr::ExprBuilder {
        expr::ExprBuilder::new(self.ctx).span(self.span)
    }

    pub fn stmt(&self) -> stmt::StmtBuilder {
        stmt::StmtBuilder::new(self.ctx).span(self.span)
    }

    pub fn block(&self) -> block::BlockBuilder {
        block::BlockBuilder::new(self.ctx).span(self.span)
    }

    pub fn fn_decl(&self) -> fn_decl::FnDeclBuilder {
        fn_decl::FnDeclBuilder::new(self.ctx).span(self.span)
    }

    pub fn method<I>(&self, id: I) -> method::MethodBuilder
        where I: ident::ToIdent,
    {
        method::MethodBuilder::new(self.ctx, id).span(self.span)
    }

    pub fn arg<I>(&self, id: I) -> fn_decl::ArgBuilder
        where I: ident::ToIdent,
    {
        fn_decl::ArgBuilder::new(self.ctx, id).span(self.span)
    }

    pub fn item(&self) -> item::ItemBuilder {
        item::ItemBuilder::new(self.ctx).span(self.span)
    }
}
