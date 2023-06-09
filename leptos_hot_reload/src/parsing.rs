use syn_rsx::{NodeElement, NodeValueExpr};

pub fn value_to_string(value: &NodeValueExpr) -> Option<String> {
    match &value.as_ref() {
        syn::Expr::Lit(lit) => match &lit.lit {
            syn::Lit::Str(s) => Some(s.value()),
            syn::Lit::Char(c) => Some(c.value().to_string()),
            syn::Lit::Int(i) => Some(i.base10_digits().to_string()),
            syn::Lit::Float(f) => Some(f.base10_digits().to_string()),
            _ => None,
        },
        _ => None,
    }
}

pub fn is_component_node(node: &NodeElement) -> bool {
    node.name
        .to_string()
        .starts_with(|c: char| c.is_ascii_uppercase())
}
