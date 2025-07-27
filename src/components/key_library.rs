use yew::prelude::*;
use once_cell::sync::Lazy;
use crate::keycodes::KeyboardUsage;

static KEY_CATEGORIES: Lazy<Vec<(&'static str, Vec<KeyboardUsage>)>> = Lazy::new(|| {
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
            KeyboardMute, KeyboardVolumeUp, KeyboardVolumeDown, KeyboardEmpty
        ]),
        ("Numpad", vec![
            Keypad0Insert, Keypad1End, Keypad2DownArrow, Keypad3PageDown, Keypad4LeftArrow, 
            Keypad5, Keypad6RightArrow, Keypad7Home, Keypad8UpArrow, Keypad9PageUp, 
            KeypadPeriodDelete, KeypadEnter, KeypadPlus, KeypadMinus, KeypadMultiply, 
            KeypadDivide, KeypadEqual
        ]),
    ]
});

pub fn get_all_categories() -> Vec<(String, Vec<KeyboardUsage>)> {
    KEY_CATEGORIES
        .iter()
        .map(|(name, keys)| (name.to_string(), keys.clone()))
        .collect()
}

#[derive(Properties, PartialEq)]
pub struct KeyLibraryProps {
    pub on_key_select: Callback<KeyboardUsage>,
}

#[function_component(KeyLibrary)]
pub fn key_library(props: &KeyLibraryProps) -> Html {
    let categories = get_all_categories();

    html! {
        <div class="key-library">
            <div class="library-header">
                <h3>{"Key Library"}</h3>
            </div>
            
            <div class="library-content">
                {
                    categories.into_iter().map(|(category, keys)| {
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
                                            
                                            html! {
                                                <button 
                                                    class="library-key"
                                                    onclick={on_select}
                                                    key={label}
                                                    title={format!("Click to use '{}'", label)}
                                                >
                                                    {label}
                                                </button>
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
