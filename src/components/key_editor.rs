use yew::prelude::*;
use web_sys::HtmlInputElement;
use crate::keycodes::KeyboardUsage;

#[derive(Properties, PartialEq)]
pub struct KeyEditorProps {
    pub selected_key: Option<(usize, usize)>,
    pub key_config: Option<KeyboardUsage>,
    pub on_key_change: Callback<String>,
}

#[function_component(KeyEditor)]
pub fn key_editor(props: &KeyEditorProps) -> Html {
    let on_change = {
        let on_key_change = props.on_key_change.clone();
        Callback::from(move |e: Event| {
            let input = e.target_dyn_into::<HtmlInputElement>().unwrap();
            let value = input.value();
            on_key_change.emit(value);
        })
    };

    if let Some((row, col)) = props.selected_key {
        let key_config = props.key_config.clone().unwrap_or(KeyboardUsage::KeyboardErrorRollOver);
        html! {
            <div class="key-editor">
                <h3>{format!("Editing Key [{}, {}]", row, col)}</h3>
                <input 
                    type="text" 
                    value={Into::<&'static str>::into(key_config)} 
                    placeholder="Key label"
                    onchange={on_change}
                />
            </div>
        }
    } else {
        html! {
            <div class="key-editor">
                <h3>{"Select a key to edit"}</h3>
                <p class="editor-instructions">
                    {"Click a key on the keyboard to edit its label, or drag keys from the library directly onto the keyboard."}
                </p>
            </div>
        }
    }
}
