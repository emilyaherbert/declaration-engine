use std::fmt;
use std::fmt::Write;

use colored::Colorize;

use crate::{
    collection_context::{
        collection_context::CollectionContext, collection_index::CollectionIndex,
    },
    type_system::type_mapping::TypeMapping,
    types::{copy_types::CopyTypes, pretty_print::PrettyPrint},
};

use self::{typed_declaration::TyDeclaration, typed_expression::TyExpression};

pub(crate) mod typed_declaration;
pub(crate) mod typed_expression;

#[derive(Clone)]
pub(crate) struct TyApplication {
    pub files: Vec<CollectionIndex>,
}

impl PrettyPrint for TyApplication {
    #[allow(clippy::useless_format)]
    fn pretty_print(&self, cc: &CollectionContext) -> String {
        let mut builder = String::new();
        write!(
            builder,
            "{}{}\n{}",
            format!("\n++++++++ RESOLVED").blue(),
            self.files
                .iter()
                .map(|file| file.pretty_print(cc))
                .collect::<Vec<_>>()
                .join("\n"),
            format!("++++++++").blue(),
        )
        .unwrap();
        builder
    }
}

impl CopyTypes for TyApplication {
    fn copy_types(&mut self, cc: &mut CollectionContext, type_mapping: &TypeMapping) {
        self.files
            .iter_mut()
            .for_each(|node_index| node_index.copy_types(cc, type_mapping));
    }
}

#[derive(Clone)]
pub(crate) struct TyFile {
    pub(crate) name: String,
    pub(crate) nodes: Vec<CollectionIndex>,
}

impl PrettyPrint for TyFile {
    #[allow(clippy::useless_format)]
    fn pretty_print(&self, cc: &CollectionContext) -> String {
        let mut builder = String::new();
        let mut nodes_str = self
            .nodes
            .iter()
            .map(|node| node.pretty_print(cc))
            .collect::<Vec<_>>()
            .join(";\n");
        nodes_str.insert(0, '\n');
        nodes_str.push(';');
        write!(
            builder,
            "{}{}{}",
            format!("\n>>> {}", self.name).green(),
            nodes_str,
            format!("\n<<<").green(),
        )
        .unwrap();
        builder
    }
}

impl CopyTypes for TyFile {
    fn copy_types(&mut self, cc: &mut CollectionContext, type_mapping: &TypeMapping) {
        self.nodes
            .iter_mut()
            .for_each(|node_index| node_index.copy_types(cc, type_mapping));
    }
}

#[derive(Clone, PartialEq)]
pub(crate) enum TyNode {
    Declaration(TyDeclaration),
    Expression(TyExpression),
    ReturnStatement(TyExpression),
}

impl fmt::Display for TyNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TyNode::Declaration(declaration) => write!(f, "{}", declaration),
            TyNode::Expression(expression) => write!(f, "{}", expression),
            TyNode::ReturnStatement(expression) => write!(f, "return {}", expression),
        }
    }
}

impl CopyTypes for TyNode {
    fn copy_types(&mut self, cc: &mut CollectionContext, type_mapping: &TypeMapping) {
        match self {
            TyNode::Declaration(declaration) => declaration.copy_types(cc, type_mapping),
            TyNode::Expression(expression) => expression.copy_types(cc, type_mapping),
            TyNode::ReturnStatement(expression) => expression.copy_types(cc, type_mapping),
        }
    }
}
