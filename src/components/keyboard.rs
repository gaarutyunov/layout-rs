use yew::prelude::*;
use super::hand::Hand;
use super::thumb_cluster::ThumbCluster;
use std::collections::HashMap;
use crate::keycodes::KeyboardUsage;

#[derive(Properties, PartialEq)]
pub struct KeyboardProps {
    pub keymap: HashMap<(usize, usize), KeyboardUsage>,
    pub selected_key: Option<(usize, usize)>,
    pub on_key_click: Callback<(usize, usize)>,
    pub current_layer: usize,
    #[prop_or_default]
    pub on_key_drop: Option<Callback<((usize, usize), String)>>,
}

#[function_component(Keyboard)]
pub fn keyboard(props: &KeyboardProps) -> Html {
    html! {
        <div class="keyboard-container">
            <div class="keyboard">
                <div class="left-hand">
                    <Hand 
                        keymap={props.keymap.clone()}
                        selected_key={props.selected_key}
                        on_key_click={props.on_key_click.clone()}
                        current_layer={props.current_layer}
                        is_left={true}
                        on_key_drop={props.on_key_drop.clone()}
                    />
                </div>
                <div class="right-hand">
                    <Hand 
                        keymap={props.keymap.clone()}
                        selected_key={props.selected_key}
                        on_key_click={props.on_key_click.clone()}
                        current_layer={props.current_layer}
                        is_left={false}
                        on_key_drop={props.on_key_drop.clone()}
                    />
                </div>
            </div>
            
            <div class="thumb-clusters">
                <div class="left-thumb">
                    <ThumbCluster 
                        keymap={props.keymap.clone()}
                        selected_key={props.selected_key}
                        on_key_click={props.on_key_click.clone()}
                        current_layer={props.current_layer}
                        is_left={true}
                        on_key_drop={props.on_key_drop.clone()}
                    />
                </div>
                <div class="right-thumb">
                    <ThumbCluster 
                        keymap={props.keymap.clone()}
                        selected_key={props.selected_key}
                        on_key_click={props.on_key_click.clone()}
                        current_layer={props.current_layer}
                        is_left={false}
                        on_key_drop={props.on_key_drop.clone()}
                    />
                </div>
            </div>
        </div>
    }
}
