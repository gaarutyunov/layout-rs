use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub current_layer: usize,
    pub on_layer_change: Callback<usize>,
    pub on_load_layout: Callback<()>,
    pub on_save_layout: Callback<()>,
    pub has_unsaved_changes: bool,
}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    let on_save = {
        let on_save_layout = props.on_save_layout.clone();
        Callback::from(move |_| {
            on_save_layout.emit(());
        })
    };

    let on_load = {
        let on_load_layout = props.on_load_layout.clone();
        Callback::from(move |_| {
            on_load_layout.emit(());
        })
    };

    html! {
        <header class="header">
            <h1>{"Dactyl Manuform 5x7 Layout Editor"}</h1>
            <div class="header-controls">
                <div class="layer-selector">
                    <span>{"Layer: "}</span>
                    {for (0..4).map(|layer| {
                        let is_active = layer == props.current_layer;
                        let onclick = {
                            let on_layer_change = props.on_layer_change.clone();
                            move |_| on_layer_change.emit(layer)
                        };
                        html! {
                            <button 
                                class={classes!("layer-btn", is_active.then(|| "active"))}
                                onclick={onclick}
                            >
                                {layer}
                            </button>
                        }
                    })}
                </div>
                
                <div class="layout-controls">
                    <button 
                        class={classes!("save-btn", props.has_unsaved_changes.then(|| "has-changes"))} 
                        onclick={on_save} 
                        title={if props.has_unsaved_changes { "Save Layout (unsaved changes)" } else { "Save Layout" }}
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"/>
                            <polyline points="17,21 17,13 7,13 7,21"/>
                            <polyline points="7,3 7,8 15,8"/>
                        </svg>
                        {if props.has_unsaved_changes { "Save*" } else { "Save" }}
                    </button>
                    
                    <button class="load-btn" onclick={on_load} title="Load Layout">
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
                            <polyline points="14,2 14,8 20,8"/>
                            <path d="M16,13H8"/>
                            <path d="M16,17H8"/>
                            <polyline points="10,9 9,9 8,9"/>
                        </svg>
                        {"Load"}
                    </button>
                </div>
            </div>
        </header>
    }
}
