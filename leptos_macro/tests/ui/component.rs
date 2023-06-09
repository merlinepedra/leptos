use leptos::*;

#[component]
fn missing_scope() {}

#[component]
fn missing_return_type(cx: Scope) {}

#[component]
fn unknown_prop_option(cx: Scope, #[prop(hello)] test: bool) -> impl IntoView {}

#[component]
fn optional_and_optional_no_strip(
    cx: Scope,
    #[prop(optional, optional_no_strip)] conflicting: bool,
) -> impl IntoView {
}

#[component]
fn optional_and_strip_option(
    cx: Scope,
    #[prop(optional, strip_option)] conflicting: bool,
) -> impl IntoView {
}

#[component]
fn optional_no_strip_and_strip_option(
    cx: Scope,
    #[prop(optional_no_strip, strip_option)] conflicting: bool,
) -> impl IntoView {
}

#[component]
fn default_without_value(
    cx: Scope,
    #[prop(default)] default: bool,
) -> impl IntoView {
}

#[component]
fn default_with_invalid_value(
    cx: Scope,
    #[prop(default= |)] default: bool,
) -> impl IntoView {
}

fn main() {}
