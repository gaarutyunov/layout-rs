use yew::prelude::*;
use super::key::KeyConfig;
use super::hand::Hand;
use super::thumb_cluster::ThumbCluster;
use std::collections::HashMap;

#[derive(Properties, PartialEq)]
pub struct KeyboardProps {
    pub keymap: HashMap<(usize, usize), KeyConfig>,
    pub selected_key: Option<(usize, usize)>,
    pub on_key_click: Callback<(usize, usize)>,
    pub current_layer: usize,
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
                    />
                </div>
                <div class="right-hand">
                    <Hand 
                        keymap={props.keymap.clone()}
                        selected_key={props.selected_key}
                        on_key_click={props.on_key_click.clone()}
                        current_layer={props.current_layer}
                        is_left={false}
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
                    />
                </div>
                <div class="right-thumb">
                    <ThumbCluster 
                        keymap={props.keymap.clone()}
                        selected_key={props.selected_key}
                        on_key_click={props.on_key_click.clone()}
                        current_layer={props.current_layer}
                        is_left={false}
                    />
                </div>
            </div>
        </div>
    }
}
