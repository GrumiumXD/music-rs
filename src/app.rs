use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongInfo{
    song: String,
    banner: String,
}

#[server(ScanSongs, "/api", "GetJSON")]
async fn scan_songs() -> Result<Vec<SongInfo>, ServerFnError> {
    let song_dir = "songs";
    let banner_dir = "banner";

    use std::{fs, path};

    let conf = get_configuration(None).await.unwrap();
    let options = conf.leptos_options;

    let dir = path::Path::new(&options.site_root).join(song_dir);
    
    let mut songs = Vec::new();

    for entry in fs::read_dir(dir.as_path())? {
        let path = entry?.path();

        let stem = path.file_stem().ok_or(ServerFnError::new("no file name"))?.to_str().ok_or(ServerFnError::new("no valid UTF8"))?;

        let song = format!("{song_dir}/{stem}.ogg");
        let banner = format!("{banner_dir}/{stem}.png");

        songs.push(SongInfo { song, banner });
    }

    Ok(songs)
}

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

#[component]
fn Song(si: SongInfo) -> impl IntoView {
    view! {
        <div class="rounded-sm">
            <span>{si.song}</span>
            <img src=si.banner/>
        </div>
    }
}

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
    
    let values = create_resource(|| (), |_| scan_songs());
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
