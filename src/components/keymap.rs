use std::collections::HashMap;
use web_sys::window;
use serde_json;

use super::key::KeyConfig;

#[derive(Clone)]
pub struct Keymap {
    current: HashMap<(usize, usize), KeyConfig>,
    saved: HashMap<(usize, usize), KeyConfig>,
}

impl Keymap {
    pub fn new() -> Self {
        let saved = Self::load_from_storage();
        let current = saved.clone();
        
        web_sys::console::log_1(&format!("Keymap initialized with {} keys", current.len()).into());
        
        Self { current, saved }
    }

    pub fn get_current(&self) -> &HashMap<(usize, usize), KeyConfig> {
        &self.current
    }

    pub fn set_current(&mut self, keymap: HashMap<(usize, usize), KeyConfig>) {
        self.current = keymap;
    }

    pub fn update_key(&mut self, row: usize, col: usize, label: String) {
        if let Some(key_config) = self.current.get_mut(&(row, col)) {
            key_config.label = label.clone();
            key_config.keycode = label.to_lowercase();
        }
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

    fn save_to_storage(keymap: &HashMap<(usize, usize), KeyConfig>) -> Result<(), String> {
        let window = window().ok_or("Window not available")?;
        let storage = window.local_storage()
            .map_err(|_| "Failed to access localStorage")?
            .ok_or("localStorage not available")?;

        // Convert HashMap to a Vec of serializable entries
        let keymap_entries: Vec<((usize, usize), KeyConfig)> = keymap.iter()
            .map(|(&key, value)| (key, value.clone()))
            .collect();

        // Serialize to JSON
        let keymap_json = serde_json::to_string(&keymap_entries)
            .map_err(|e| format!("Serialization failed: {}", e))?;

        // Save to localStorage
        storage.set_item("dactyl_keymap", &keymap_json)
            .map_err(|_| "Failed to save to localStorage".to_string())?;

        Ok(())
    }

    fn load_from_storage() -> HashMap<(usize, usize), KeyConfig> {
        Self::load_from_storage_result().unwrap_or_else(|_| Self::initialize_default())
    }

    fn load_from_storage_result() -> Result<HashMap<(usize, usize), KeyConfig>, String> {
        let window = window().ok_or("Window not available")?;
        let storage = window.local_storage()
            .map_err(|_| "Failed to access localStorage")?
            .ok_or("localStorage not available")?;

        let saved_keymap = storage.get_item("dactyl_keymap")
            .map_err(|_| "Failed to read from localStorage")?
            .ok_or("No saved layout found")?;

        web_sys::console::log_1(&format!("Found saved data, length: {}", saved_keymap.len()).into());

        // Try to deserialize as Vec of entries first
        if let Ok(keymap_entries) = serde_json::from_str::<Vec<((usize, usize), KeyConfig)>>(&saved_keymap) {
            return Ok(keymap_entries.into_iter().collect());
        }

        // Fallback: try to deserialize as HashMap directly (for backward compatibility)
        if let Ok(keymap) = serde_json::from_str::<HashMap<(usize, usize), KeyConfig>>(&saved_keymap) {
            return Ok(keymap);
        }

        Err("Failed to parse saved layout data".to_string())
    }

    fn initialize_default() -> HashMap<(usize, usize), KeyConfig> {
        // Initialize with default Dactyl Manuform 5x7 layout
        let mut map = HashMap::new();
        
        // Left hand regular keys
        // Rows 0-2: 7 keys each (cols 0-6)
        for row in 0..3 {
            for col in 0..7 {
                let default_key = match (row, col) {
                    (0, 0) => "ESC", (0, 1) => "1", (0, 2) => "2", (0, 3) => "3", (0, 4) => "4", (0, 5) => "5", (0, 6) => "6",
                    (1, 0) => "TAB", (1, 1) => "Q", (1, 2) => "W", (1, 3) => "E", (1, 4) => "R", (1, 5) => "T", (1, 6) => "Y",
                    (2, 0) => "CAPS", (2, 1) => "A", (2, 2) => "S", (2, 3) => "D", (2, 4) => "F", (2, 5) => "G", (2, 6) => "H",
                    _ => "",
                };
                
                map.insert((row, col), KeyConfig {
                    label: default_key.to_string(),
                    keycode: default_key.to_lowercase(),
                    layer: 0,
                });
            }
        }
        
        // Row 3: 6 keys (cols 0-5, aligned left)
        for col in 0..6 {
            let default_key = match col {
                0 => "SHIFT", 1 => "Z", 2 => "X", 3 => "C", 4 => "V", 5 => "B",
                _ => "",
            };
            
            map.insert((3, col), KeyConfig {
                label: default_key.to_string(),
                keycode: default_key.to_lowercase(),
                layer: 0,
            });
        }
        
        // Row 4: 4 keys (cols 0-3, aligned left)
        for col in 0..4 {
            let default_key = match col {
                0 => "CTRL", 1 => "ALT", 2 => "GUI", 3 => "LOWER",
                _ => "",
            };
            
            map.insert((4, col), KeyConfig {
                label: default_key.to_string(),
                keycode: default_key.to_lowercase(),
                layer: 0,
            });
        }
        
        // Right hand regular keys
        // Rows 0-2: 7 keys each (cols 7-13)
        for row in 0..3 {
            for col in 7..14 {
                let default_key = match (row, col) {
                    (0, 7) => "7", (0, 8) => "8", (0, 9) => "9", (0, 10) => "0", (0, 11) => "-", (0, 12) => "=", (0, 13) => "BKSP",
                    (1, 7) => "U", (1, 8) => "I", (1, 9) => "O", (1, 10) => "P", (1, 11) => "[", (1, 12) => "]", (1, 13) => "\\",
                    (2, 7) => "J", (2, 8) => "K", (2, 9) => "L", (2, 10) => ";", (2, 11) => "'", (2, 12) => "ENT", (2, 13) => "",
                    _ => "",
                };
                
                map.insert((row, col), KeyConfig {
                    label: default_key.to_string(),
                    keycode: default_key.to_lowercase(),
                    layer: 0,
                });
            }
        }
        
        // Row 3: 6 keys (cols 8-13, aligned right)
        for col in 8..14 {
            let default_key = match col {
                8 => "N", 9 => "M", 10 => ",", 11 => ".", 12 => "/", 13 => "SHIFT",
                _ => "",
            };
            
            map.insert((3, col), KeyConfig {
                label: default_key.to_string(),
                keycode: default_key.to_lowercase(),
                layer: 0,
            });
        }
        
        // Row 4: 4 keys (cols 10-13, aligned right)
        for col in 10..14 {
            let default_key = match col {
                10 => "RAISE", 11 => "GUI", 12 => "ALT", 13 => "CTRL",
                _ => "",
            };
            
            map.insert((4, col), KeyConfig {
                label: default_key.to_string(),
                keycode: default_key.to_lowercase(),
                layer: 0,
            });
        }
        
        // Thumb cluster keys
        // Left thumb cluster
        // Row 1: 2 keys at cols 5-6 (aligned right, empty slot under col 6)
        for col in 5..7 {
            let default_key = match col {
                5 => "HOME", 6 => "END",
                _ => "",
            };
            
            map.insert((5, col), KeyConfig {
                label: default_key.to_string(),
                keycode: default_key.to_lowercase(),
                layer: 0,
            });
        }
        
        // Row 2: 2 keys at cols 5-6
        for col in 5..7 {
            let default_key = match col {
                5 => "PGUP", 6 => "PGDN",
                _ => "",
            };
            
            map.insert((6, col), KeyConfig {
                label: default_key.to_string(),
                keycode: default_key.to_lowercase(),
                layer: 0,
            });
        }
        
        // Row 3: 2 keys at cols 5-6
        for col in 5..7 {
            let default_key = match col {
                5 => "SPC", 6 => "BKSP",
                _ => "",
            };
            
            map.insert((7, col), KeyConfig {
                label: default_key.to_string(),
                keycode: default_key.to_lowercase(),
                layer: 0,
            });
        }
        
        // Right thumb cluster
        // Row 1: 2 keys at cols 8-9 (aligned left, empty slot under col 7)
        for col in 8..10 {
            let default_key = match col {
                8 => "LEFT", 9 => "RIGHT",
                _ => "",
            };
            
            map.insert((5, col), KeyConfig {
                label: default_key.to_string(),
                keycode: default_key.to_lowercase(),
                layer: 0,
            });
        }
        
        // Row 2: 2 keys at cols 7-8 (starting from beginning)
        for col in 7..9 {
            let default_key = match col {
                7 => "UP", 8 => "DOWN",
                _ => "",
            };
            
            map.insert((6, col), KeyConfig {
                label: default_key.to_string(),
                keycode: default_key.to_lowercase(),
                layer: 0,
            });
        }
        
        // Row 3: 2 keys at cols 7-8 (starting from beginning)
        for col in 7..9 {
            let default_key = match col {
                7 => "DEL", 8 => "ENT",
                _ => "",
            };
            
            map.insert((7, col), KeyConfig {
                label: default_key.to_string(),
                keycode: default_key.to_lowercase(),
                layer: 0,
            });
        }
        
        map
    }
}
