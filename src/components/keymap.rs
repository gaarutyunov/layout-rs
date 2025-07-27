use std::collections::HashMap;
use web_sys::window;
use serde_json;
use crate::keycodes::{KeyboardUsage};
use paste::paste;

#[derive(Clone)]
pub struct Keymap {
    current: HashMap<(usize, usize), KeyboardUsage>,
    saved: HashMap<(usize, usize), KeyboardUsage>,
}

macro_rules! k {
    ($name:ident) => {
        paste! {
            KeyboardUsage::[<Keyboard $name>]
        }
    };
}

macro_rules! ku {
    ($name:ident) => {
        KeyboardUsage::$name
    };
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
        // Initialize with default Dactyl Manuform 5x7 layout
        let mut map = HashMap::new();
        
        // Left hand regular keys
        // Rows 0-2: 7 keys each (cols 0-6)
        for row in 0..3 {
            for col in 0..7 {
                let default_key = match (row, col) {
                    (0, 0) => k!(Escape), (0, 1) => ku!(Keyboard1Exclamation), (0, 2) => ku!(Keyboard2At), (0, 3) => ku!(Keyboard3Hash), (0, 4) => ku!(Keyboard4Dollar), (0, 5) => ku!(Keyboard5Percent), (0, 6) => ku!(Keyboard6Caret),
                    (1, 0) => k!(Tab), (1, 1) => k!(Qq), (1, 2) => k!(Ww), (1, 3) => k!(Ee), (1, 4) => k!(Rr), (1, 5) => k!(Tt), (1, 6) => k!(Yy),
                    (2, 0) => k!(CapsLock), (2, 1) => k!(Aa), (2, 2) => k!(Ss), (2, 3) => k!(Dd), (2, 4) => k!(Ff), (2, 5) => k!(Gg), (2, 6) => k!(Hh),
                    _ => k!(ErrorRollOver),
                };
                
                map.insert((row, col), default_key.into());
            }
        }
        
        // Row 3: 6 keys (cols 0-5, aligned left)
        for col in 0..6 {
            let default_key = match col {
                0 => k!(LeftShift), 1 => k!(Zz), 2 => k!(Xx), 3 => k!(Cc), 4 => k!(Vv), 5 => k!(Bb),
                _ => k!(ErrorRollOver),
            };
            
            map.insert((3, col), default_key.into());
        }
        
        // Row 4: 4 keys (cols 0-3, aligned left)
        for col in 0..4 {
            let default_key = match col {
                0 => k!(LeftControl), 1 => k!(LeftAlt), 2 => k!(LeftGUI), 3 => k!(Lower),
                _ => k!(ErrorRollOver),
            };
            
            map.insert((4, col), default_key.into());
        }
        
        // Right hand regular keys
        // Rows 0-2: 7 keys each (cols 7-13)
        for row in 0..3 {
            for col in 7..14 {
                let default_key = match (row, col) {
                    (0, 7) => ku!(Keyboard7Ampersand), (0, 8) => ku!(Keyboard8Asterisk), (0, 9) => ku!(Keyboard9OpenParens), (0, 10) => ku!(Keyboard0CloseParens), (0, 11) => k!(DashUnderscore), (0, 12) => k!(EqualPlus), (0, 13) => k!(Backspace),
                    (1, 7) => k!(Uu), (1, 8) => k!(Ii), (1, 9) => k!(Oo), (1, 10) => k!(Pp), (1, 11) => k!(OpenBracketBrace), (1, 12) => k!(CloseBracketBrace), (1, 13) => k!(BackslashBar),
                    (2, 7) => k!(Jj), (2, 8) => k!(Kk), (2, 9) => k!(Ll), (2, 10) => k!(SemiColon), (2, 11) => k!(SingleDoubleQuote), (2, 12) => k!(Enter), (2, 13) => k!(Empty),
                    _ => k!(ErrorRollOver),
                };
                
                map.insert((row, col), default_key.into());
            }
        }
        
        // Row 3: 6 keys (cols 8-13, aligned right)
        for col in 8..14 {
            let default_key = match col {
                8 => k!(Nn), 9 => k!(Mm), 10 => k!(CommaLess), 11 => k!(PeriodGreater), 12 => k!(SlashQuestion), 13 => k!(RightShift),
                _ => k!(ErrorRollOver),
            };
            
            map.insert((3, col), default_key.into());
        }
        
        // Row 4: 4 keys (cols 10-13, aligned right)
        for col in 10..14 {
            let default_key = match col {
                10 => k!(Raise), 11 => k!(RightGUI), 12 => k!(RightAlt), 13 => k!(RightControl),
                _ => k!(ErrorRollOver),
            };
            
            map.insert((4, col), default_key.into());
        }
        
        // Thumb cluster keys
        // Left thumb cluster
        // Row 1: 2 keys at cols 5-6 (aligned right, empty slot under col 6)
        for col in 5..7 {
            let default_key = match col {
                5 => k!(Home), 6 => k!(End),
                _ => k!(ErrorRollOver),
            };
            
            map.insert((5, col), default_key.into());
        }
        
        // Row 2: 2 keys at cols 5-6
        for col in 5..7 {
            let default_key = match col {
                5 => k!(PageUp), 6 => k!(PageDown),
                _ => k!(ErrorRollOver),
            };
            
            map.insert((6, col), default_key.into());
        }
        
        // Row 3: 2 keys at cols 5-6
        for col in 5..7 {
            let default_key = match col {
                5 => k!(Spacebar), 6 => k!(Backspace),
                _ => k!(ErrorRollOver),
            };
            
            map.insert((7, col), default_key.into());
        }
        
        // Right thumb cluster
        // Row 1: 2 keys at cols 8-9 (aligned left, empty slot under col 7)
        for col in 8..10 {
            let default_key = match col {
                8 => k!(LeftArrow), 9 => k!(RightArrow),
                _ => k!(ErrorRollOver),
            };
            
            map.insert((5, col), default_key.into());
        }
        
        // Row 2: 2 keys at cols 7-8 (starting from beginning)
        for col in 7..9 {
            let default_key = match col {
                7 => k!(UpArrow), 8 => k!(DownArrow),
                _ => k!(ErrorRollOver),
            };
            
            map.insert((6, col), default_key.into());
        }
        
        // Row 3: 2 keys at cols 7-8 (starting from beginning)
        for col in 7..9 {
            let default_key = match col {
                7 => k!(Delete), 8 => k!(Enter),
                _ => k!(ErrorRollOver),
            };
            
            map.insert((7, col), default_key.into());
        }
        
        map
    }
}
