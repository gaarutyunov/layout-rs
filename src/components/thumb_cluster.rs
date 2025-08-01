use yew::prelude::*;
use super::key::Key;
use std::collections::HashMap;
use crate::keycodes::KeyboardUsage;

#[derive(Properties, PartialEq)]
pub struct ThumbClusterProps {
    pub keymap: HashMap<(usize, usize), KeyboardUsage>,
    pub selected_key: Option<(usize, usize)>,
    pub on_key_click: Callback<(usize, usize)>,
    pub is_left: bool,
    #[prop_or_default]
    pub on_key_drop: Option<Callback<((usize, usize), String)>>,
}

#[function_component(ThumbCluster)]
pub fn thumb_cluster(props: &ThumbClusterProps) -> Html {
    let thumb_positions = if props.is_left {
        // Left thumb cluster: rows 5-7
        // Row 5: cols 5-6 (aligned right, empty under col 6)
        // Row 6: cols 5-6 
        // Row 7: cols 5-6
        vec![
            (5, 5), (5, 6),  // Row 1
            (6, 5), (6, 6),  // Row 2
            (7, 5), (7, 6),  // Row 3
        ]
    } else {
        // Right thumb cluster: rows 5-7
        // Row 5: cols 8-9 (aligned left, empty under col 7)
        // Row 6: cols 7-8 (starting from beginning)
        // Row 7: cols 7-8 (starting from beginning)
        vec![
            (5, 8), (5, 9),  // Row 1
            (6, 7), (6, 8),  // Row 2
            (7, 7), (7, 8),  // Row 3
        ]
    };
    
    html! {
        <div class="thumb-cluster">
            {for (0..3).map(|thumb_row| {
                let row_positions: Vec<_> = thumb_positions.iter()
                    .skip(thumb_row * 2)
                    .take(2)
                    .collect();
                
                html! {
                    <div class="thumb-row">
                        {for row_positions.iter().map(|&&(row, col)| {
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
                                    is_thumb={true}
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
