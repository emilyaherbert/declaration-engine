use crate::{
    declaration_engine::declaration_engine::DeclarationEngine,
    language::literal::Literal,
    type_system::{type_id::TypeId, type_mapping::TypeMapping},
    types::{copy_types::CopyTypes, pretty_print::PrettyPrint},
};

#[derive(Clone, PartialEq, Debug)]
pub(crate) struct TypedExpression {
    pub(crate) variant: TypedExpressionVariant,
    pub(crate) type_id: TypeId,
}

impl CopyTypes for TypedExpression {
    fn copy_types(&mut self, type_mapping: &TypeMapping) {
        self.variant.copy_types(type_mapping);
        self.type_id.copy_types(type_mapping);
    }
}

impl PrettyPrint for TypedExpression {
    fn pretty_print(&self, declaration_engine: &DeclarationEngine) -> String {
        self.variant.pretty_print(declaration_engine)
    }
}

#[derive(Clone, PartialEq, Debug)]
pub(crate) enum TypedExpressionVariant {
    Literal {
        value: Literal,
    },
    Variable {
        name: String,
    },
    FunctionApplication {
        name: String,
        arguments: Vec<TypedExpression>,
    },
    // a no-op variant used to indicate that a variable is in scope
    // as a result of a function parameter
    FunctionParameter,
    // Struct {
    //     struct_name: String,
    //     fields: Vec<TypedStructExpressionField>,
    // },
    // Enum {
    //     enum_name: String,
    //     variant_name: String,
    //     value: Box<TypedExpression>,
    // },
}

impl CopyTypes for TypedExpressionVariant {
    fn copy_types(&mut self, type_mapping: &TypeMapping) {
        match self {
            TypedExpressionVariant::FunctionApplication { arguments, .. } => {
                arguments
                    .iter_mut()
                    .for_each(|argument| argument.copy_types(type_mapping));
            }
            TypedExpressionVariant::Literal { .. }
            | TypedExpressionVariant::Variable { .. }
            | TypedExpressionVariant::FunctionParameter => {}
        }
    }
}

impl PrettyPrint for TypedExpressionVariant {
    fn pretty_print(&self, declaration_engine: &DeclarationEngine) -> String {
        match self {
            TypedExpressionVariant::Literal { value } => format!("{}", value),
            TypedExpressionVariant::Variable { name } => name.to_string(),
            TypedExpressionVariant::FunctionApplication { name, arguments } => {
                format!(
                    "{}({})",
                    name,
                    &arguments
                        .iter()
                        .map(|argument| argument.pretty_print(declaration_engine))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            TypedExpressionVariant::FunctionParameter => "function param".to_string(),
        }
    }
}

#[derive(Clone, PartialEq)]
pub(crate) struct TypedStructExpressionField {
    pub(crate) name: String,
    pub(crate) value: TypedExpression,
}
