use crate::error_template::{AppError, ErrorTemplate};
use leptos::{html::Audio, *};
use leptos_icons::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongInfo {
    title: String,
    ogg: String,
    banner: String,
}

#[cfg(feature = "ssr")]
#[derive(Deserialize)]
pub struct Assets {
    songs: Vec<SongInfo>,
}

#[server(GetSongs, "/api", "GetJSON")]
async fn get_songs() -> Result<Vec<SongInfo>, ServerFnError> {
    use std::{fs, path};
    use toml;

    let options = use_context::<LeptosOptions>()
        .ok_or(ServerFnError::new("Could not fetch options from context"))?;

    let file = path::Path::new(&options.site_root).join("assets/assets.toml");
    let file = fs::read_to_string(file)?;
    let assets: Assets = toml::from_str(&file)?;

    Ok(assets.songs)
}

/// main component
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/music-rs.css"/>
        <Link rel="apple-touch-icon" href="logo192.png"/>
        <Link rel="manifest" href="manifest.json"/>

        // sets the document title
        <Title text="Music"/>

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
fn Song(
    si: SongInfo,
    #[prop(into)] on_click: Callback<ev::MouseEvent>,
    #[prop(into)] active: Signal<bool>,
) -> impl IntoView {
    use icondata as i;

    let audio = create_node_ref::<Audio>();

    create_effect(move |_| {
        let audio_element = audio.get().expect("audio element should be mounted");

        if active() {
            let _ = audio_element.play();
        } else {
            let _ = audio_element.pause();
            let _ = audio_element.fast_seek(0.0);
        }
    });

    let note_icon = move || active().then(|| view! { <Icon icon=i::BiMusicSolid/> });

    view! {
        <div class="flex flex-col items-center gap-2">
            <button
                on:click=on_click
                class="overflow-hidden rounded-lg shadow-lg shadow-slate-600"
                class="hover:outline-4 hover:outline-double hover:outline-teal-100 active:scale-95"
            >
                <img class=("sepia", move || !active()) src=si.banner/>
            </button>
            <div class="flex items-center gap-3 font-kode text-teal-100">
                {note_icon} <span>{si.title}</span> {note_icon}
            </div>
            <audio _ref=audio loop src=si.ogg></audio>
        </div>
    }
}

/// Main header of the page
#[component]
fn Header() -> impl IntoView {
    view! {
        <header class="font-kode w-full flex flex-col items-center mt-2 mb-5">
            <h1 class="text-7xl text-teal-100">"Music"</h1>
            <span class="text-teal-100">"Version - " {env!("CARGO_PKG_VERSION")}</span>
        </header>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (current_song, set_current_song) = create_signal::<Option<usize>>(None);

    let values = create_resource(|| (), |_| get_songs());

    let handle_click = move |i| {
        set_current_song.update(|v| {
            if *v == Some(i) {
                *v = None;
                return;
            }

            *v = Some(i);
        })
    };

    view! {
        <Header/>
        <Suspense fallback=move || {
            view! { <p>"Loading 1..."</p> }
        }>
            <ErrorBoundary fallback=|errors| view! { <ErrorTemplate errors/> }>
                <div class="flex flex-col gap-4">
                    {move || {
                        values
                            .and_then(|v| {
                                v.iter()
                                    .enumerate()
                                    .map(|(index, s)| {
                                        view! {
                                            <Song
                                                si=s.clone()
                                                on_click=move |_| handle_click(index)
                                                active=Signal::derive(move || Some(index) == current_song())
                                            />
                                        }
                                    })
                                    .collect_view()
                            })
                    }}

                </div>
            </ErrorBoundary>
        </Suspense>
    }
}
