use yew::prelude::*;
use super::keyboard::Keyboard;
use super::key_editor::KeyEditor;
use super::key::KeyConfig;
use std::collections::HashMap;

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    pub keymap: HashMap<(usize, usize), KeyConfig>,
    pub selected_key: Option<(usize, usize)>,
    pub current_layer: usize,
    pub on_key_click: Callback<(usize, usize)>,
    pub on_key_change: Callback<String>,
}

#[function_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
    let key_config = props.selected_key
        .and_then(|(row, col)| props.keymap.get(&(row, col)).cloned());

    html! {
        <main class="main">
            <Keyboard 
                keymap={props.keymap.clone()}
                selected_key={props.selected_key}
                on_key_click={props.on_key_click.clone()}
                current_layer={props.current_layer}
            />
            
            <KeyEditor 
                selected_key={props.selected_key}
                key_config={key_config}
                on_key_change={props.on_key_change.clone()}
            />
        </main>
    }
}
