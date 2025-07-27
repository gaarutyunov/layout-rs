use yew::prelude::*;

use crate::components::{Header, Layout, Keymap};

#[function_component(App)]
pub fn app() -> Html {
    let current_layer = use_state(|| 0usize);
    let selected_key = use_state(|| None::<(usize, usize)>);
    let keymap = use_state(|| Keymap::new());

    let on_key_click = {
        let selected_key = selected_key.clone();
        Callback::from(move |(row, col): (usize, usize)| {
            selected_key.set(Some((row, col)));
        })
    };

    let on_key_change = {
        let keymap = keymap.clone();
        let selected_key = selected_key.clone();
        Callback::from(move |value: String| {
            if let Some((row, col)) = *selected_key {
                let mut new_keymap = (*keymap).clone();
                new_keymap.update_key(row, col, value);
                keymap.set(new_keymap);
            }
        })
    };

    let on_layer_change = {
        let current_layer = current_layer.clone();
        Callback::from(move |layer: usize| {
            current_layer.set(layer);
        })
    };

    let on_save_layout = {
        let keymap = keymap.clone();
        Callback::from(move |_| {
            let mut new_keymap = (*keymap).clone();
            if let Err(e) = new_keymap.save() {
                web_sys::console::log_1(&format!("Save error: {}", e).into());
            }
            keymap.set(new_keymap);
        })
    };

    let on_load_layout = {
        let keymap = keymap.clone();
        Callback::from(move |_| {
            let mut new_keymap = (*keymap).clone();
            if let Err(e) = new_keymap.load() {
                web_sys::console::log_1(&format!("Load error: {}", e).into());
            }
            keymap.set(new_keymap);
        })
    };

    html! {
        <div class="app">
            <Header 
                current_layer={*current_layer}
                on_layer_change={on_layer_change}
                on_save_layout={on_save_layout}
                on_load_layout={on_load_layout}
                has_unsaved_changes={keymap.has_unsaved_changes()}
            />
            
            <Layout 
                keymap={keymap.get_current().clone()}
                selected_key={*selected_key}
                current_layer={*current_layer}
                on_key_click={on_key_click}
                on_key_change={on_key_change}
            />
        </div>
    }
}

