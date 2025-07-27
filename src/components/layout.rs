use yew::prelude::*;
use super::keyboard::Keyboard;
use super::key_editor::KeyEditor;
use super::key_library::KeyLibrary;
use super::key::KeyConfig;
use std::collections::HashMap;

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    pub keymap: HashMap<(usize, usize), KeyConfig>,
    pub selected_key: Option<(usize, usize)>,
    pub current_layer: usize,
    pub on_key_click: Callback<(usize, usize)>,
    pub on_key_change: Callback<String>,
    pub on_key_drop: Callback<((usize, usize), String)>,
}

#[function_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
    let key_config = props.selected_key
        .and_then(|(row, col)| props.keymap.get(&(row, col)).cloned());

    let on_carousel_key_select = {
        let on_key_change = props.on_key_change.clone();
        Callback::from(move |key: String| {
            on_key_change.emit(key);
        })
    };

    html! {
        <main class="main">
            <div class="layout-container">
                <div class="keyboard-section">
                    <Keyboard 
                        keymap={props.keymap.clone()}
                        selected_key={props.selected_key}
                        on_key_click={props.on_key_click.clone()}
                        current_layer={props.current_layer}
                        on_key_drop={Some(props.on_key_drop.clone())}
                    />
                    
                    <KeyEditor 
                        selected_key={props.selected_key}
                        key_config={key_config}
                        on_key_change={props.on_key_change.clone()}
                    />
                </div>
                
                <div class="library-section">
                    <KeyLibrary 
                        on_key_select={on_carousel_key_select}
                    />
                </div>
            </div>
        </main>
    }
}
