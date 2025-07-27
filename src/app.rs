use yew::prelude::*;

use crate::components::{Header, Layout, Keymap};

#[function_component(App)]
pub fn app() -> Html {
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

    let on_reset_layout = {
        let keymap = keymap.clone();
        Callback::from(move |_| {
            let mut new_keymap = (*keymap).clone();
            if let Err(e) = new_keymap.reset() {
                web_sys::console::log_1(&format!("Reset error: {}", e).into());
            }
            keymap.set(new_keymap);
        })
    };

    let on_factory_reset_layout = {
        let keymap = keymap.clone();
        Callback::from(move |_| {
            let mut new_keymap = (*keymap).clone();
            if let Err(e) = new_keymap.factory_reset() {
                web_sys::console::log_1(&format!("Factory reset error: {}", e).into());
            }
            keymap.set(new_keymap);
        })
    };

    let on_export_layout = {
        let keymap = keymap.clone();
        Callback::from(move |_| {
            let current_keymap = (*keymap).clone();
            if let Err(e) = current_keymap.download_json() {
                web_sys::console::log_1(&format!("Export error: {}", e).into());
            }
        })
    };

    let on_key_drop = {
        let keymap = keymap.clone();
        let selected_key = selected_key.clone();
        Callback::from(move |((row, col), key): ((usize, usize), String)| {
            // Update the key directly without needing selection
            let mut new_keymap = (*keymap).clone();
            new_keymap.update_key(row, col, key);
            keymap.set(new_keymap);
            
            // Also select the key that was dropped on
            selected_key.set(Some((row, col)));
        })
    };

    html! {
        <div class="app">
            <Header 
                on_save_layout={on_save_layout}
                on_load_layout={on_load_layout}
                on_reset_layout={on_reset_layout}
                on_factory_reset_layout={on_factory_reset_layout}
                on_export_layout={on_export_layout}
                has_unsaved_changes={keymap.has_unsaved_changes()}
            />
            
            <Layout 
                keymap={keymap.current().clone()}
                selected_key={*selected_key}
                on_key_click={on_key_click}
                on_key_change={on_key_change}
                on_key_drop={on_key_drop}
            />
        </div>
    }
}

