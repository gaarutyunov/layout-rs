use yew::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyConfig {
    pub label: String,
    pub keycode: String,
    pub layer: usize,
}

impl Default for KeyConfig {
    fn default() -> Self {
        Self {
            label: String::new(),
            keycode: String::new(),
            layer: 0,
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct KeyProps {
    pub key_config: KeyConfig,
    pub is_selected: bool,
    pub onclick: Callback<()>,
    #[prop_or_default]
    pub is_thumb: bool,
}

#[function_component(Key)]
pub fn key(props: &KeyProps) -> Html {
    let onclick = {
        let onclick = props.onclick.clone();
        move |_| onclick.emit(())
    };

    html! {
        <button 
            class={classes!(
                "key", 
                props.is_thumb.then(|| "thumb-key"),
                props.is_selected.then(|| "selected")
            )}
            onclick={onclick}
        >
            {&props.key_config.label}
        </button>
    }
}
