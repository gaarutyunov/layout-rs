use yew::prelude::*;
use web_sys::window;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::JsCast;
use crate::keycodes::KeyboardUsage;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeyLibraryData {
    custom_keys: HashMap<String, Vec<KeyboardUsage>>,
}

impl KeyLibraryData {
    pub fn new() -> Self {
        Self::load_from_storage()
    }

    pub fn get_default_categories() -> Vec<(&'static str, Vec<KeyboardUsage>)> {
        use KeyboardUsage::*;
        
        vec![
            ("Letters", vec![
                KeyboardAa, KeyboardBb, KeyboardCc, KeyboardDd, KeyboardEe, KeyboardFf, KeyboardGg, 
                KeyboardHh, KeyboardIi, KeyboardJj, KeyboardKk, KeyboardLl, KeyboardMm, KeyboardNn, 
                KeyboardOo, KeyboardPp, KeyboardQq, KeyboardRr, KeyboardSs, KeyboardTt, KeyboardUu, 
                KeyboardVv, KeyboardWw, KeyboardXx, KeyboardYy, KeyboardZz
            ]),
            ("Numbers", vec![
                Keyboard0CloseParens, Keyboard1Exclamation, Keyboard2At, Keyboard3Hash, Keyboard4Dollar, 
                Keyboard5Percent, Keyboard6Caret, Keyboard7Ampersand, Keyboard8Asterisk, Keyboard9OpenParens
            ]),
            ("Function Keys", vec![
                KeyboardF1, KeyboardF2, KeyboardF3, KeyboardF4, KeyboardF5, KeyboardF6, 
                KeyboardF7, KeyboardF8, KeyboardF9, KeyboardF10, KeyboardF11, KeyboardF12
            ]),
            ("Modifiers", vec![
                KeyboardLeftShift, KeyboardRightShift, KeyboardLeftControl, KeyboardRightControl, 
                KeyboardLeftAlt, KeyboardRightAlt, KeyboardLeftGUI, KeyboardRightGUI
            ]),
            ("Navigation", vec![
                KeyboardUpArrow, KeyboardDownArrow, KeyboardLeftArrow, KeyboardRightArrow, 
                KeyboardHome, KeyboardEnd, KeyboardPageUp, KeyboardPageDown, KeyboardInsert, 
                KeyboardDelete, KeyboardBackspace, KeyboardTab, KeyboardEscape, KeyboardPrintScreen, 
                KeyboardPause, KeyboardScrollLock
            ]),
            ("Symbols", vec![
                KeyboardSpacebar, KeyboardEnter, KeyboardDashUnderscore, KeyboardEqualPlus, 
                KeyboardOpenBracketBrace, KeyboardCloseBracketBrace, KeyboardBackslashBar, 
                KeyboardSemiColon, KeyboardSingleDoubleQuote, KeyboardCommaLess, KeyboardPeriodGreater, 
                KeyboardSlashQuestion, KeyboardBacktickTilde
            ]),
            ("Special", vec![
                KeyboardCapsLock, KeypadNumLock, KeyboardScrollLock, KeyboardPause, KeyboardPower, 
                KeyboardMute, KeyboardVolumeUp, KeyboardVolumeDown
            ]),
            ("Numpad", vec![
                Keypad0Insert, Keypad1End, Keypad2DownArrow, Keypad3PageDown, Keypad4LeftArrow, 
                Keypad5, Keypad6RightArrow, Keypad7Home, Keypad8UpArrow, Keypad9PageUp, 
                KeypadPeriodDelete, KeypadEnter, KeypadPlus, KeypadMinus, KeypadMultiply, 
                KeypadDivide, KeypadEqual
            ])
        ]
    }

    pub fn get_all_categories(&self) -> Vec<(String, Vec<KeyboardUsage>)> {
        let mut categories = Vec::new();
        
        // Add default categories
        for (name, keys) in Self::get_default_categories() {
            categories.push((name.to_string(), keys));
        }
        
        // Add custom categories
        for (name, keys) in &self.custom_keys {
            categories.push((format!("Custom: {}", name), keys.clone()));
        }
        
        categories
    }

    pub fn add_key(&mut self, category: String, key: String) -> Result<(), String> {
        if key.trim().is_empty() {
            return Err("Key cannot be empty".to_string());
        }
        
        let category = if category.trim().is_empty() {
            "Custom".to_string()
        } else {
            category.trim().to_string()
        };
        
        // Convert string key to KeyboardUsage
        let keycode: KeyboardUsage = key.trim().into();
        
        // Check if key already exists in this category
        if let Some(existing_keys) = self.custom_keys.get(&category) {
            if existing_keys.contains(&keycode) {
                return Err(format!("Key '{}' already exists in category '{}'", key, category));
            }
        }
        
        self.custom_keys
            .entry(category)
            .or_insert_with(Vec::new)
            .push(keycode);
        
        self.save_to_storage()
    }

    pub fn remove_key(&mut self, category: &str, keycode: KeyboardUsage) -> Result<(), String> {
        if let Some(keys) = self.custom_keys.get_mut(category) {
            if let Some(pos) = keys.iter().position(|k| *k == keycode) {
                keys.remove(pos);
                if keys.is_empty() {
                    self.custom_keys.remove(category);
                }
                self.save_to_storage()?;
                return Ok(());
            }
        }
        Err(format!("Key '{:?}' not found in category '{}'", keycode, category))
    }

    fn save_to_storage(&self) -> Result<(), String> {
        let window = window().ok_or("Window not available")?;
        let storage = window.local_storage()
            .map_err(|_| "Failed to access localStorage")?
            .ok_or("localStorage not available")?;

        let json = serde_json::to_string(self)
            .map_err(|e| format!("Failed to serialize key library: {}", e))?;

        storage.set_item("key_library", &json)
            .map_err(|_| "Failed to save key library to localStorage")?;

        web_sys::console::log_1(&"Key library saved to localStorage".into());
        Ok(())
    }

    fn load_from_storage() -> Self {
        Self::load_from_storage_result().unwrap_or_else(|_| Self {
            custom_keys: HashMap::new(),
        })
    }

    fn load_from_storage_result() -> Result<Self, String> {
        let window = window().ok_or("Window not available")?;
        let storage = window.local_storage()
            .map_err(|_| "Failed to access localStorage")?
            .ok_or("localStorage not available")?;

        let saved_data = storage.get_item("key_library")
            .map_err(|_| "Failed to read from localStorage")?
            .ok_or("No saved key library found")?;

        let data: Self = serde_json::from_str(&saved_data)
            .map_err(|e| format!("Failed to parse key library data: {}", e))?;

        web_sys::console::log_1(&format!("Key library loaded with {} custom categories", data.custom_keys.len()).into());
        Ok(data)
    }
}

#[derive(Properties, PartialEq)]
pub struct KeyLibraryProps {
    pub on_key_select: Callback<KeyboardUsage>,
}

#[function_component(KeyLibrary)]
pub fn key_library(props: &KeyLibraryProps) -> Html {
    let library_data = use_state(|| KeyLibraryData::new());
    let new_key_input = use_state(|| String::new());
    let new_category_input = use_state(|| String::new());
    let show_add_form = use_state(|| false);

    let on_toggle_form = {
        let show_add_form = show_add_form.clone();
        Callback::from(move |_| {
            show_add_form.set(!*show_add_form);
        })
    };

    let on_new_key_change = {
        let new_key_input = new_key_input.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            new_key_input.set(input.value());
        })
    };

    let on_new_category_change = {
        let new_category_input = new_category_input.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            new_category_input.set(input.value());
        })
    };

    let on_key_input_keydown = {
        let on_add_key = {
            let library_data = library_data.clone();
            let new_key_input = new_key_input.clone();
            let new_category_input = new_category_input.clone();
            let show_add_form = show_add_form.clone();
            move || {
                let mut data = (*library_data).clone();
                let category = (*new_category_input).clone();
                let key = (*new_key_input).clone();
                
                match data.add_key(category.clone(), key.clone()) {
                    Ok(_) => {
                        library_data.set(data);
                        new_key_input.set(String::new());
                        new_category_input.set(String::new());
                        show_add_form.set(false);
                        web_sys::console::log_1(&format!("Key '{}' added successfully to category '{}'", key, if category.is_empty() { "Custom" } else { &category }).into());
                    }
                    Err(e) => {
                        web_sys::console::log_1(&format!("Error adding key: {}", e).into());
                    }
                }
            }
        };
        
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                on_add_key();
            }
        })
    };

    let on_add_key = {
        let library_data = library_data.clone();
        let new_key_input = new_key_input.clone();
        let new_category_input = new_category_input.clone();
        let show_add_form = show_add_form.clone();
        Callback::from(move |_| {
            let mut data = (*library_data).clone();
            let category = (*new_category_input).clone();
            let key = (*new_key_input).clone();
            
            match data.add_key(category.clone(), key.clone()) {
                Ok(_) => {
                    library_data.set(data);
                    new_key_input.set(String::new());
                    new_category_input.set(String::new());
                    show_add_form.set(false);
                    web_sys::console::log_1(&format!("Key '{}' added successfully to category '{}'", key, if category.is_empty() { "Custom" } else { &category }).into());
                }
                Err(e) => {
                    web_sys::console::log_1(&format!("Error adding key: {}", e).into());
                }
            }
        })
    };

    let on_remove_key = {
        let library_data = library_data.clone();
        Callback::from(move |key_info: (String, KeyboardUsage)| {
            let mut data = (*library_data).clone();
            let (category, keycode) = key_info;
            
            match data.remove_key(&category, keycode) {
                Ok(_) => {
                    library_data.set(data);
                    web_sys::console::log_1(&format!("Key '{:?}' removed successfully from category '{}'", keycode, category).into());
                }
                Err(e) => {
                    web_sys::console::log_1(&format!("Error removing key: {}", e).into());
                }
            }
        })
    };

    let categories = library_data.get_all_categories();

    html! {
        <div class="key-library">
            <div class="library-header">
                <h3>{"Key Library"}</h3>
                <button 
                    class="add-key-btn" 
                    onclick={on_toggle_form}
                    title="Add custom key"
                >
                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <line x1="12" y1="5" x2="12" y2="19"/>
                        <line x1="5" y1="12" x2="19" y2="12"/>
                    </svg>
                </button>
            </div>
            
            {if *show_add_form {
                html! {
                    <div class="add-key-form">
                        <div class="form-row">
                            <input
                                type="text"
                                placeholder="Category (optional)"
                                value={(*new_category_input).clone()}
                                onchange={on_new_category_change}
                                class="category-input"
                            />
                        </div>
                        <div class="form-row">
                            <input
                                type="text"
                                placeholder="Key name (e.g., CUSTOM_KEY)"
                                value={(*new_key_input).clone()}
                                onchange={on_new_key_change}
                                onkeydown={on_key_input_keydown}
                                class="key-input"
                            />
                            <button class="add-btn" onclick={on_add_key}>
                                {"Add"}
                            </button>
                        </div>
                    </div>
                }
            } else {
                html! {}
            }}
            
            <div class="library-content">
                {
                    categories.into_iter().map(|(category, keys)| {
                        let is_custom = category.starts_with("Custom:");
                        html! {
                            <div class="key-category" key={category.clone()}>
                                <h4 class="category-title">{&category}</h4>
                                <div class="key-grid">
                                    {
                                        keys.into_iter().map(|keycode| {
                                            let label: &str = keycode.into();
                                            let on_select = {
                                                let keycode = keycode.clone();
                                                let on_key_select = props.on_key_select.clone();
                                                Callback::from(move |_| {
                                                    on_key_select.emit(keycode.clone());
                                                })
                                            };
                                            
                                            let on_remove = {
                                                let category = category.clone();
                                                let keycode = keycode.clone();
                                                let on_remove_key = on_remove_key.clone();
                                                Callback::from(move |e: web_sys::MouseEvent| {
                                                    e.stop_propagation();
                                                    // Extract the actual category name from the display name
                                                    let actual_category = if category.starts_with("Custom: ") {
                                                        category.strip_prefix("Custom: ").unwrap_or(&category).to_string()
                                                    } else {
                                                        category.clone()
                                                    };
                                                    on_remove_key.emit((actual_category, keycode.clone()));
                                                })
                                            };
                                            
                                            let on_drag_start = {
                                                let label = label.to_string();
                                                Callback::from(move |e: DragEvent| {
                                                    // Store key label in a data attribute for the drop handler to access
                                                    if let Some(target) = e.target() {
                                                        if let Ok(element) = target.dyn_into::<web_sys::HtmlElement>() {
                                                            let _ = element.set_attribute("data-drag-key", &label);
                                                        }
                                                    }
                                                })
                                            };
                                            
                                            html! {
                                                <div class="library-key-container">
                                                    <button 
                                                        class="library-key"
                                                        onclick={on_select}
                                                        key={label}
                                                        title={format!("Click to use '{}' or drag to keyboard", label)}
                                                        draggable="true"
                                                        ondragstart={on_drag_start}
                                                    >
                                                        {label}
                                                    </button>
                                                    {if is_custom {
                                                        html! {
                                                            <button 
                                                                class="remove-key-btn"
                                                                onclick={on_remove}
                                                                title="Remove custom key"
                                                            >
                                                                {"×"}
                                                            </button>
                                                        }
                                                    } else {
                                                        html! {}
                                                    }}
                                                </div>
                                            }
                                        }).collect::<Html>()
                                    }
                                </div>
                            </div>
                        }
                    }).collect::<Html>()
                }
            </div>
        </div>
    }
}
