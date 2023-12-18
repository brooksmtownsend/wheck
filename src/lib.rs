cargo_component_bindings::generate!();

use bindings::{Case, Guest};
use heck::*;

struct Component;

impl Guest for Component {
    fn to_case(input: String, to: Case) -> String {
        match to {
            Case::Kebab => AsKebabCase(input).to_string(),
            Case::LowerCamel => AsLowerCamelCase(input).to_string(),
            Case::Pascal => AsPascalCase(input).to_string(),
            Case::ShoutyKebab => AsShoutyKebabCase(input).to_string(),
            Case::ShoutySnake => AsShoutySnakeCase(input).to_string(),
            Case::ShoutySnek => AsShoutySnekCase(input).to_string(),
            Case::Snake => AsSnakeCase(input).to_string(),
            Case::Snek => AsSnekCase(input).to_string(),
            Case::Title => AsTitleCase(input).to_string(),
            Case::Train => AsTrainCase(input).to_string(),
            Case::UpperCamel => AsUpperCamelCase(input).to_string(),
        }
    }
}
