use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongInfo{
    title: String,
    ogg: String,
    banner: String,
}


#[cfg(feature = "ssr")]
#[derive(Deserialize)]
pub struct Infos{
    songs: Vec<SongInfo>
}

#[server(GetSongs, "/api", "GetJSON")]
async fn get_songs() -> Result<Vec<SongInfo>, ServerFnError> {
    use std::{fs, path};
    use toml;

    let conf = get_configuration(None).await.unwrap();
    let options = conf.leptos_options;

    let file = path::Path::new(&options.site_root).join("songs.toml");

    let file = fs::read_to_string(file)?;
    
    let song_info: Infos = toml::from_str(&file)?;

    Ok(song_info.songs)
}

/// main component
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/music-rs.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// A single song button
#[component]
fn Song(si: SongInfo) -> impl IntoView {
    view! {
        <div class="rounded-sm">
            <span>{si.title}</span>
            <img src=si.banner/>
        </div>
    }
}

/// Main header of the page
#[component]
fn Header() -> impl IntoView {
    view! {
        <header class="w-full flex flex-col items-center">
            <h1 class="text-5xl text-teal-100">"Music"</h1>
            <span class="text-teal-100">"Version - 1.2"</span>
        </header>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    
    let values = create_resource(|| (), |_| get_songs());
    let on_click = move |_| {set_count.update(|count| *count += 1); values.refetch()};

    view! {
        <Header/>
        <Suspense fallback=move || {
            view! { <p>"Loading 1..."</p> }
        }>
            <ErrorBoundary fallback=|errors| view! { <ErrorTemplate errors/> }>
                <div class="flex flex-col">
                    {move || {
                        values
                            .and_then(|v| {
                                v.iter().map(|s| view! { <Song si=s.clone()/> }).collect_view()
                            })
                    }}

                </div>
            </ErrorBoundary>
        </Suspense>

        <button
            class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
            on:click=on_click
        >
            "Click Me: "
            {count}
        </button>
        <div class="flex gap-6 flex-col">
            <div class="rounded-sm bg-red-600 w-32">1</div>
            <div class="rounded-sm bg-blue-600 w-32">2</div>
            <div class="rounded-sm bg-green-600 w-32">3</div>
            <div class="rounded-sm bg-yellow-400 w-32">4</div>
        </div>
    }
}
