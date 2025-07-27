use yew::prelude::*;
use web_sys::window;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::JsCast;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeyLibraryData {
    custom_keys: HashMap<String, Vec<String>>,
}

impl KeyLibraryData {
    pub fn new() -> Self {
        Self::load_from_storage()
    }

    pub fn get_default_categories() -> Vec<(&'static str, Vec<&'static str>)> {
        vec![
            ("Letters", vec![
                "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M",
                "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"
            ]),
            ("Numbers", vec![
                "0", "1", "2", "3", "4", "5", "6", "7", "8", "9"
            ]),
            ("Function Keys", vec![
                "F1", "F2", "F3", "F4", "F5", "F6", "F7", "F8", "F9", "F10", "F11", "F12"
            ]),
            ("Modifiers", vec![
                "SHIFT", "CTRL", "ALT", "GUI", "CMD", "WIN", "META", "SUPER",
                "LSHIFT", "RSHIFT", "LCTRL", "RCTRL", "LALT", "RALT", "LGUI", "RGUI"
            ]),
            ("Navigation", vec![
                "UP", "DOWN", "LEFT", "RIGHT", "HOME", "END", "PGUP", "PGDN",
                "INS", "DEL", "BKSP", "TAB", "ESC", "MENU", "PRTSC", "PAUSE", "SCROLL"
            ]),
            ("Symbols", vec![
                "SPACE", "SPC", "ENT", "ENTER", "-", "=", "[", "]", "\\", ";", "'",
                ",", ".", "/", "`", "~", "!", "@", "#", "$", "%", "^", "&", "*",
                "(", ")", "_", "+", "{", "}", "|", ":", "\"", "<", ">", "?"
            ]),
            ("Special", vec![
                "CAPS", "CAPSLOCK", "NUMLOCK", "SCROLL", "BREAK", "SYSREQ", "POWER",
                "SLEEP", "WAKE", "MUTE", "VOLUP", "VOLDN", "PREV", "NEXT", "PLAY", "STOP"
            ]),
            ("Numpad", vec![
                "KP0", "KP1", "KP2", "KP3", "KP4", "KP5", "KP6", "KP7", "KP8", "KP9",
                "KPDOT", "KPENTER", "KPPLUS", "KPMINUS", "KPMULT", "KPDIV", "KPEQL"
            ]),
            ("Layers", vec![
                "LAYER0", "LAYER1", "LAYER2", "LAYER3", "LAYER4", "LAYER5",
                "LOWER", "RAISE", "ADJUST", "SYMBOL", "NUMERIC", "FUNCTION", "GAMING"
            ]),
            ("Macros", vec![
                "MACRO1", "MACRO2", "MACRO3", "MACRO4", "MACRO5",
                "COPY", "PASTE", "CUT", "UNDO", "REDO", "SAVE", "FIND", "REPLACE"
            ]),
            ("Mouse", vec![
                "MS_L", "MS_R", "MS_M", "MS_U", "MS_D", "MS_WH_U", "MS_WH_D",
                "MS_BTN1", "MS_BTN2", "MS_BTN3", "MS_BTN4", "MS_BTN5"
            ]),
            ("Empty", vec![
                "NONE", "TRANS", "___", "XXX", "NO"
            ])
        ]
    }

    pub fn get_all_categories(&self) -> Vec<(String, Vec<String>)> {
        let mut categories = Vec::new();
        
        // Add default categories
        for (name, keys) in Self::get_default_categories() {
            categories.push((name.to_string(), keys.iter().map(|s| s.to_string()).collect()));
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
        
        let key = key.trim().to_uppercase();
        
        // Check if key already exists in this category
        if let Some(existing_keys) = self.custom_keys.get(&category) {
            if existing_keys.contains(&key) {
                return Err(format!("Key '{}' already exists in category '{}'", key, category));
            }
        }
        
        self.custom_keys
            .entry(category)
            .or_insert_with(Vec::new)
            .push(key);
        
        self.save_to_storage()
    }

    pub fn remove_key(&mut self, category: &str, key: &str) -> Result<(), String> {
        if let Some(keys) = self.custom_keys.get_mut(category) {
            if let Some(pos) = keys.iter().position(|k| k == key) {
                keys.remove(pos);
                if keys.is_empty() {
                    self.custom_keys.remove(category);
                }
                self.save_to_storage()?;
                return Ok(());
            }
        }
        Err(format!("Key '{}' not found in category '{}'", key, category))
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
    pub on_key_select: Callback<String>,
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
        Callback::from(move |key_info: (String, String)| {
            let mut data = (*library_data).clone();
            let (category, key) = key_info;
            
            match data.remove_key(&category, &key) {
                Ok(_) => {
                    library_data.set(data);
                    web_sys::console::log_1(&format!("Key '{}' removed successfully from category '{}'", key, category).into());
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
                                        keys.into_iter().map(|key| {
                                            let on_select = {
                                                let key = key.clone();
                                                let on_key_select = props.on_key_select.clone();
                                                Callback::from(move |_| {
                                                    on_key_select.emit(key.clone());
                                                })
                                            };
                                            
                                            let on_remove = {
                                                let category = category.clone();
                                                let key = key.clone();
                                                let on_remove_key = on_remove_key.clone();
                                                Callback::from(move |e: web_sys::MouseEvent| {
                                                    e.stop_propagation();
                                                    // Extract the actual category name from the display name
                                                    let actual_category = if category.starts_with("Custom: ") {
                                                        category.strip_prefix("Custom: ").unwrap_or(&category).to_string()
                                                    } else {
                                                        category.clone()
                                                    };
                                                    on_remove_key.emit((actual_category, key.clone()));
                                                })
                                            };
                                            
                                            let on_drag_start = {
                                                let key = key.clone();
                                                Callback::from(move |e: DragEvent| {
                                                    // Store key in a data attribute for the drop handler to access
                                                    if let Some(target) = e.target() {
                                                        if let Ok(element) = target.dyn_into::<web_sys::HtmlElement>() {
                                                            let _ = element.set_attribute("data-drag-key", &key);
                                                        }
                                                    }
                                                })
                                            };
                                            
                                            html! {
                                                <div class="library-key-container">
                                                    <button 
                                                        class="library-key"
                                                        onclick={on_select}
                                                        key={key.clone()}
                                                        title={format!("Click to use '{}' or drag to keyboard", key)}
                                                        draggable="true"
                                                        ondragstart={on_drag_start}
                                                    >
                                                        {&key}
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
