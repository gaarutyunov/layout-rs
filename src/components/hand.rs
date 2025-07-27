use yew::prelude::*;
use super::key::Key;
use std::collections::HashMap;
use crate::keycodes::KeyboardUsage;

#[derive(Properties, PartialEq)]
pub struct HandProps {
    pub keymap: HashMap<(usize, usize), KeyboardUsage>,
    pub selected_key: Option<(usize, usize)>,
    pub on_key_click: Callback<(usize, usize)>,
    pub is_left: bool,
    #[prop_or_default]
    pub on_key_drop: Option<Callback<((usize, usize), String)>>,
}

#[function_component(Hand)]
pub fn hand(props: &HandProps) -> Html {
    html! {
        <div class="hand">
            {for (0..5).map(|row| {
                let (start_col, end_col) = if props.is_left {
                    match row {
                        0..=2 => (0, 7),       // Rows 0-2: 7 keys
                        3 => (0, 6),           // Row 3: 6 keys
                        4 => (0, 4),           // Row 4: 4 keys
                        _ => (0, 0),
                    }
                } else {
                    match row {
                        0..=2 => (7, 14),      // Rows 0-2: 7 keys
                        3 => (8, 14),          // Row 3: 6 keys (cols 8-13)
                        4 => (10, 14),         // Row 4: 4 keys (cols 10-13)
                        _ => (0, 0),
                    }
                };
                
                html! {
                    <div class="row">
                        {for (start_col..end_col).map(|col| {
                            let key_config = props.keymap.get(&(row, col)).cloned().unwrap_or(KeyboardUsage::KeyboardErrorRollOver);
                            let is_selected = props.selected_key == Some((row, col));
                            let onclick = {
                                let on_key_click = props.on_key_click.clone();
                                move |_| on_key_click.emit((row, col))
                            };
                            
                            let on_drop = props.on_key_drop.as_ref().map(|callback| {
                                let callback = callback.clone();
                                Callback::from(move |key: String| {
                                    callback.emit(((row, col), key));
                                })
                            });
                            
                            html! {
                                <Key 
                                    key_config={key_config}
                                    is_selected={is_selected}
                                    onclick={onclick}
                                    on_drop={on_drop}
                                />
                            }
                        })}
                    </div>
                }
            })}
        </div>
    }
}
