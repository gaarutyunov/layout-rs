use std::collections::HashMap;
use web_sys::window;
use serde_json;
use once_cell::sync::Lazy;
use wasm_bindgen::JsCast;
use serde::{Serialize, Deserialize};
use crate::keycodes::{KeyboardUsage};

static DEFAULT_KEYMAP: Lazy<HashMap<(usize, usize), KeyboardUsage>> = Lazy::new(|| {
    use KeyboardUsage::*;
    
    let mut map = HashMap::new();
    
    // Left hand regular keys
    // Row 0 (cols 0-6)
    map.insert((0, 0), KeyboardEscape);
    map.insert((0, 1), Keyboard1Exclamation);
    map.insert((0, 2), Keyboard2At);
    map.insert((0, 3), Keyboard3Hash);
    map.insert((0, 4), Keyboard4Dollar);
    map.insert((0, 5), Keyboard5Percent);
    map.insert((0, 6), Keyboard6Caret);
    
    // Row 1 (cols 0-6)
    map.insert((1, 0), KeyboardTab);
    map.insert((1, 1), KeyboardQq);
    map.insert((1, 2), KeyboardWw);
    map.insert((1, 3), KeyboardEe);
    map.insert((1, 4), KeyboardRr);
    map.insert((1, 5), KeyboardTt);
    map.insert((1, 6), KeyboardYy);
    
    // Row 2 (cols 0-6)
    map.insert((2, 0), KeyboardCapsLock);
    map.insert((2, 1), KeyboardAa);
    map.insert((2, 2), KeyboardSs);
    map.insert((2, 3), KeyboardDd);
    map.insert((2, 4), KeyboardFf);
    map.insert((2, 5), KeyboardGg);
    map.insert((2, 6), KeyboardHh);
    
    // Row 3 (cols 0-5)
    map.insert((3, 0), KeyboardLeftShift);
    map.insert((3, 1), KeyboardZz);
    map.insert((3, 2), KeyboardXx);
    map.insert((3, 3), KeyboardCc);
    map.insert((3, 4), KeyboardVv);
    map.insert((3, 5), KeyboardBb);
    
    // Row 4 (cols 0-3)
    map.insert((4, 0), KeyboardLeftControl);
    map.insert((4, 1), KeyboardLeftAlt);
    map.insert((4, 2), KeyboardLeftGUI);
    map.insert((4, 3), KeyboardLower);
    
    // Right hand regular keys
    // Row 0 (cols 7-13)
    map.insert((0, 7), Keyboard7Ampersand);
    map.insert((0, 8), Keyboard8Asterisk);
    map.insert((0, 9), Keyboard9OpenParens);
    map.insert((0, 10), Keyboard0CloseParens);
    map.insert((0, 11), KeyboardDashUnderscore);
    map.insert((0, 12), KeyboardEqualPlus);
    map.insert((0, 13), KeyboardBackspace);
    
    // Row 1 (cols 7-13)
    map.insert((1, 7), KeyboardUu);
    map.insert((1, 8), KeyboardIi);
    map.insert((1, 9), KeyboardOo);
    map.insert((1, 10), KeyboardPp);
    map.insert((1, 11), KeyboardOpenBracketBrace);
    map.insert((1, 12), KeyboardCloseBracketBrace);
    map.insert((1, 13), KeyboardBackslashBar);
    
    // Row 2 (cols 7-13)
    map.insert((2, 7), KeyboardJj);
    map.insert((2, 8), KeyboardKk);
    map.insert((2, 9), KeyboardLl);
    map.insert((2, 10), KeyboardSemiColon);
    map.insert((2, 11), KeyboardSingleDoubleQuote);
    map.insert((2, 12), KeyboardEnter);
    map.insert((2, 13), KeyboardEmpty);
    
    // Row 3 (cols 8-13)
    map.insert((3, 8), KeyboardNn);
    map.insert((3, 9), KeyboardMm);
    map.insert((3, 10), KeyboardCommaLess);
    map.insert((3, 11), KeyboardPeriodGreater);
    map.insert((3, 12), KeyboardSlashQuestion);
    map.insert((3, 13), KeyboardRightShift);
    
    // Row 4 (cols 10-13)
    map.insert((4, 10), KeyboardRaise);
    map.insert((4, 11), KeyboardRightGUI);
    map.insert((4, 12), KeyboardRightAlt);
    map.insert((4, 13), KeyboardRightControl);
    
    // Left thumb cluster
    // Row 5 (cols 5-6)
    map.insert((5, 5), KeyboardHome);
    map.insert((5, 6), KeyboardEnd);
    
    // Row 6 (cols 5-6)
    map.insert((6, 5), KeyboardPageUp);
    map.insert((6, 6), KeyboardPageDown);
    
    // Row 7 (cols 5-6)
    map.insert((7, 5), KeyboardSpacebar);
    map.insert((7, 6), KeyboardBackspace);
    
    // Right thumb cluster
    // Row 5 (cols 8-9)
    map.insert((5, 8), KeyboardLeftArrow);
    map.insert((5, 9), KeyboardRightArrow);
    
    // Row 6 (cols 7-8)
    map.insert((6, 7), KeyboardUpArrow);
    map.insert((6, 8), KeyboardDownArrow);
    
    // Row 7 (cols 7-8)
    map.insert((7, 7), KeyboardDelete);
    map.insert((7, 8), KeyboardEnter);
    
    map
});

#[derive(Serialize, Deserialize)]
struct KeymapExport {
    metadata: ExportMetadata,
    keys: Vec<KeymapEntry>,
}

#[derive(Serialize, Deserialize)]
struct ExportMetadata {
    version: String,
    keyboard: String,
    exported_at: String,
    total_keys: usize,
}

#[derive(Serialize, Deserialize)]
struct KeymapEntry {
    position: (usize, usize),
    label: String,
    keycode: u8,
}

#[derive(Clone)]
pub struct Keymap {
    current: HashMap<(usize, usize), KeyboardUsage>,
    saved: HashMap<(usize, usize), KeyboardUsage>,
}

impl Keymap {
    pub fn new() -> Self {
        let saved = Self::load_from_storage();
        let current = saved.clone();
        
        web_sys::console::log_1(&format!("Keymap initialized with {} keys", current.len()).into());
        
        Self { current, saved }
    }

    pub fn current(&self) -> &HashMap<(usize, usize), KeyboardUsage> {
        &self.current
    }

    pub fn update_key(&mut self, row: usize, col: usize, label: String) {
        let keycode: KeyboardUsage = label.into();
        self.current.insert((row, col), keycode);
    }

    pub fn has_unsaved_changes(&self) -> bool {
        self.current != self.saved
    }

    pub fn save(&mut self) -> Result<(), String> {
        match Self::save_to_storage(&self.current) {
            Ok(_) => {
                self.saved = self.current.clone();
                web_sys::console::log_1(&format!("Layout saved! {} keys", self.current.len()).into());
                Ok(())
            }
            Err(e) => {
                let error_msg = format!("Failed to save layout: {}", e);
                web_sys::console::log_1(&error_msg.clone().into());
                Err(error_msg)
            }
        }
    }

    pub fn load(&mut self) -> Result<(), String> {
        match Self::load_from_storage_result() {
            Ok(keymap) => {
                self.current = keymap.clone();
                self.saved = keymap;
                web_sys::console::log_1(&format!("Layout loaded! {} keys", self.current.len()).into());
                Ok(())
            }
            Err(e) => {
                let error_msg = format!("Failed to load layout: {}", e);
                web_sys::console::log_1(&error_msg.clone().into());
                Err(error_msg)
            }
        }
    }

    pub fn reset(&mut self) -> Result<(), String> {
        // Reset current to saved state (discard unsaved changes)
        self.current = self.saved.clone();
        web_sys::console::log_1(&format!("Reset to saved state! {} keys", self.current.len()).into());
        Ok(())
    }

    pub fn factory_reset(&mut self) -> Result<(), String> {
        // Clear localStorage
        let window = window().ok_or("Window not available")?;
        let storage = window.local_storage()
            .map_err(|_| "Failed to access localStorage")?
            .ok_or("localStorage not available")?;

        storage.remove_item("dactyl_keymap")
            .map_err(|_| "Failed to clear localStorage".to_string())?;

        // Reset to default keymap
        let default_keymap = Self::initialize_default();
        self.current = default_keymap.clone();
        self.saved = default_keymap;

        web_sys::console::log_1(&format!("Factory reset! {} keys", self.current.len()).into());
        Ok(())
    }

    pub fn export_json(&self) -> Result<String, String> {
        // Convert HashMap to a Vec of structured entries
        let mut keymap_entries: Vec<KeymapEntry> = self.current.iter()
            .map(|(&position, &keycode)| {
                let label: String = keycode.into();
                KeymapEntry {
                    position,
                    label,
                    keycode: keycode as u8,
                }
            })
            .collect();

        // Sort by position for consistent output
        keymap_entries.sort_by(|a, b| {
            a.position.0.cmp(&b.position.0)
                .then(a.position.1.cmp(&b.position.1))
        });

        // Create export structure with metadata
        let export = KeymapExport {
            metadata: ExportMetadata {
                version: "1.0".to_string(),
                keyboard: "Dactyl Manuform 5x7".to_string(),
                exported_at: js_sys::Date::new_0().to_iso_string().as_string().unwrap_or_default(),
                total_keys: keymap_entries.len(),
            },
            keys: keymap_entries,
        };

        // Serialize to pretty JSON
        serde_json::to_string_pretty(&export)
            .map_err(|e| format!("JSON serialization failed: {}", e))
    }

    pub fn download_json(&self) -> Result<(), String> {
        let json_data = self.export_json()?;
        
        let window = window().ok_or("Window not available")?;
        let document = window.document().ok_or("Document not available")?;
        
        // Create a blob with the JSON data
        let array = js_sys::Array::new();
        array.push(&json_data.into());
        
        let blob = web_sys::Blob::new_with_str_sequence(&array)
            .map_err(|_| "Failed to create blob")?;
        
        // Create a download URL
        let url = web_sys::Url::create_object_url_with_blob(&blob)
            .map_err(|_| "Failed to create object URL")?;
        
        // Create a temporary anchor element for download
        let anchor = document.create_element("a")
            .map_err(|_| "Failed to create anchor element")?
            .dyn_into::<web_sys::HtmlAnchorElement>()
            .map_err(|_| "Failed to cast to anchor element")?;
        
        anchor.set_href(&url);
        anchor.set_download("dactyl_keymap.json");
        
        // Set style using setAttribute
        anchor.set_attribute("style", "display: none")
            .map_err(|_| "Failed to set style")?;
        
        // Append to body, click, and remove
        let body = document.body().ok_or("Body not available")?;
        body.append_child(&anchor)
            .map_err(|_| "Failed to append anchor")?;
        
        anchor.click();
        
        body.remove_child(&anchor)
            .map_err(|_| "Failed to remove anchor")?;
        
        // Clean up the URL
        web_sys::Url::revoke_object_url(&url)
            .map_err(|_| "Failed to revoke object URL")?;
        
        web_sys::console::log_1(&"Layout exported successfully!".into());
        Ok(())
    }

    fn save_to_storage(keymap: &HashMap<(usize, usize), KeyboardUsage>) -> Result<(), String> {
        let window = window().ok_or("Window not available")?;
        let storage = window.local_storage()
            .map_err(|_| "Failed to access localStorage")?
            .ok_or("localStorage not available")?;

        // Convert HashMap to a Vec of serializable entries
        let keymap_entries: Vec<((usize, usize), KeyboardUsage)> = keymap.iter()
            .map(|(&key, &value)| (key, value))
            .collect();

        // Serialize to JSON
        let keymap_json = serde_json::to_string(&keymap_entries)
            .map_err(|e| format!("Serialization failed: {}", e))?;

        // Save to localStorage
        storage.set_item("dactyl_keymap", &keymap_json)
            .map_err(|_| "Failed to save to localStorage".to_string())?;

        Ok(())
    }

    fn load_from_storage() -> HashMap<(usize, usize), KeyboardUsage> {
        Self::load_from_storage_result().unwrap_or_else(|_| Self::initialize_default())
    }

    fn load_from_storage_result() -> Result<HashMap<(usize, usize), KeyboardUsage>, String> {
        let window = window().ok_or("Window not available")?;
        let storage = window.local_storage()
            .map_err(|_| "Failed to access localStorage")?
            .ok_or("localStorage not available")?;

        let saved_keymap = storage.get_item("dactyl_keymap")
            .map_err(|_| "Failed to read from localStorage")?
            .ok_or("No saved layout found")?;

        web_sys::console::log_1(&format!("Found saved data, length: {}", saved_keymap.len()).into());

        // Try to deserialize as Vec of entries first
        if let Ok(keymap_entries) = serde_json::from_str::<Vec<((usize, usize), KeyboardUsage)>>(&saved_keymap) {
            return Ok(keymap_entries.into_iter().collect());
        }

        // Fallback: try to deserialize as HashMap directly (for backward compatibility)
        if let Ok(keymap) = serde_json::from_str::<HashMap<(usize, usize), KeyboardUsage>>(&saved_keymap) {
            return Ok(keymap);
        }

        Err("Failed to parse saved layout data".to_string())
    }

    fn initialize_default() -> HashMap<(usize, usize), KeyboardUsage> {
        DEFAULT_KEYMAP.clone()
    }
}
