use yew::prelude::*;
use wasm_bindgen::JsCast;
use crate::keycodes::KeyboardUsage;

#[derive(Properties, PartialEq)]
pub struct KeyProps {
    pub key_config: KeyboardUsage,
    pub is_selected: bool,
    pub onclick: Callback<()>,
    #[prop_or_default]
    pub is_thumb: bool,
    #[prop_or_default]
    pub on_drop: Option<Callback<String>>,
}

#[function_component(Key)]
pub fn key(props: &KeyProps) -> Html {
    let is_drag_over = use_state(|| false);
    
    let onclick = {
        let onclick = props.onclick.clone();
        move |_| onclick.emit(())
    };

    let on_drag_over = {
        let is_drag_over = is_drag_over.clone();
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            is_drag_over.set(true);
        })
    };

    let on_drag_leave = {
        let is_drag_over = is_drag_over.clone();
        Callback::from(move |_: DragEvent| {
            is_drag_over.set(false);
        })
    };

    let on_drop = {
        let is_drag_over = is_drag_over.clone();
        let on_drop_callback = props.on_drop.clone();
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            is_drag_over.set(false);
            
            if let Some(callback) = &on_drop_callback {
                // Try to get the dragged key from the data attribute fallback
                if let Some(window) = web_sys::window() {
                    if let Some(document) = window.document() {
                        if let Ok(Some(dragged)) = document.query_selector("[data-drag-key]") {
                            if let Ok(element) = dragged.dyn_into::<web_sys::HtmlElement>() {
                                if let Some(key) = element.get_attribute("data-drag-key") {
                                    callback.emit(key);
                                    let _ = element.remove_attribute("data-drag-key");
                                }
                            }
                        }
                    }
                }
            }
        })
    };

    html! {
        <button 
            class={classes!(
                "key", 
                props.is_thumb.then(|| "thumb-key"),
                props.is_selected.then(|| "selected"),
                (*is_drag_over && props.on_drop.is_some()).then(|| "drag-over")
            )}
            onclick={onclick}
            ondragover={on_drag_over}
            ondragleave={on_drag_leave}
            ondrop={on_drop}
        >
            {Into::<&'static str>::into(props.key_config)}
        </button>
    }
}
