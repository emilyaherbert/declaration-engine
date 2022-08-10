//use crate::type_system::*;

// #[derive(Debug, Clone, PartialEq)]
// pub(crate) struct TypedExpression {
//     variant: TypedExpressionVariant,
//     type_id: TypeId,
// }

use crate::language::Literal;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum TypedExpression {
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
    Struct {
        struct_name: String,
        fields: Vec<TypedStructExpressionField>,
    },
    Enum {
        enum_name: String,
        variant_name: String,
        value: Box<TypedExpression>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TypedStructExpressionField {
    pub(crate) name: String,
    pub(crate) value: TypedExpression,
}
