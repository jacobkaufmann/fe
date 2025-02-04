use crate::names;
use fe_analyzer::namespace::types::Struct;
use yultsur::*;

pub fn new(struct_type: &Struct, params: Vec<yul::Expression>) -> yul::Expression {
    let function_name = names::struct_new_call(&struct_type.name);
    expression! { [function_name]([params...]) }
}

pub fn get_attribute(
    struct_type: &Struct,
    field_name: &str,
    val: yul::Expression,
) -> yul::Expression {
    let function_name = names::struct_getter_call(&struct_type.name, field_name);
    expression! { [function_name]([val]) }
}

#[cfg(test)]
mod tests {
    use crate::operations::structs;
    use fe_analyzer::namespace::items::StructId;
    use fe_analyzer::namespace::types::Struct;
    use yultsur::*;

    #[test]
    fn test_new() {
        let val = Struct {
            name: "Foo".to_string(),
            id: StructId::default(),
            field_count: 2,
        };
        let params = vec![
            identifier_expression! { (1) },
            identifier_expression! { (2) },
        ];
        assert_eq!(
            structs::new(&val, params).to_string(),
            "struct_Foo_new(1, 2)"
        )
    }
}
