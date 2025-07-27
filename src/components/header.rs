use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub on_load_layout: Callback<()>,
    pub on_save_layout: Callback<()>,
    pub on_reset_layout: Callback<()>,
    pub on_factory_reset_layout: Callback<()>,
    pub on_export_layout: Callback<()>,
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

    let on_reset = {
        let on_reset_layout = props.on_reset_layout.clone();
        Callback::from(move |_: web_sys::MouseEvent| {
            on_reset_layout.emit(());
        })
    };

    let on_factory_reset = {
        let on_factory_reset_layout = props.on_factory_reset_layout.clone();
        Callback::from(move |_: web_sys::MouseEvent| {
            on_factory_reset_layout.emit(());
        })
    };

    let on_export = {
        let on_export_layout = props.on_export_layout.clone();
        Callback::from(move |_: web_sys::MouseEvent| {
            on_export_layout.emit(());
        })
    };

    html! {
        <header class="header">
            <h1>{"Dactyl Manuform 5x7 Layout Editor"}</h1>
            <div class="header-controls">
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
                    
                    <button class="export-btn" onclick={on_export} title="Export Layout as JSON">
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
                            <polyline points="7 10 12 15 17 10"/>
                            <line x1="12" y1="15" x2="12" y2="3"/>
                        </svg>
                        {"Export"}
                    </button>
                    
                    <button class="reset-btn" onclick={on_reset} title="Reset unsaved changes" disabled={!props.has_unsaved_changes}>
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <polyline points="1 4 1 10 7 10"/>
                            <path d="M3.51 15a9 9 0 1 0 2.13-9.36L1 10"/>
                        </svg>
                        {"Reset"}
                    </button>
                    
                    <button class="factory-reset-btn" onclick={on_factory_reset} title="Factory Reset - Delete all saved changes">
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <polyline points="3 6 5 6 21 6"/>
                            <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/>
                            <line x1="10" y1="11" x2="10" y2="17"/>
                            <line x1="14" y1="11" x2="14" y2="17"/>
                        </svg>
                        {"Factory Reset"}
                    </button>
                </div>
            </div>
        </header>
    }
}
