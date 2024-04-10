extern crate rocket;

use std::fs;
use std::sync::Arc;
use std::sync::Mutex;
use std::{fmt::Write, num::ParseIntError};

use aspotify::Scope;
use rocket::catch;
use rocket::catchers;
use rocket::get;
use rocket::routes;
use rocket::State;
use serde_json::json;
use rocket::response::Responder;
use rocket::http::Header;

use crate::downloader::Downloader;
use crate::settings::Settings;
use crate::spotify::Spotify;


#[derive(Responder)]
struct MyResponder<T> {
    inner: T,
    my_header: Header<'static>,
}
impl<'r, 'o: 'r, T: Responder<'r, 'o>> MyResponder<T> {
    fn new(inner: T) -> Self {
        MyResponder {
            inner,
            my_header: Header::new("Access-Control-Allow-Origin", "*"),
        }
    }

	fn from(inner: T) -> Self {
		MyResponder {
			inner,
			my_header: Header::new("Access-Control-Allow-Origin", "*"),
		}
	}
}

struct RedirectState {
	state: Mutex<String>,
}


pub async fn launch_rest(settings: &Settings, spotify: Arc<Spotify>, downloader: Arc<Downloader>) {
	rocket::build()
		.manage(settings.clone())
		.manage(spotify)
		.manage(downloader)
		.manage(RedirectState {
			state: Mutex::new("".to_string()),
		})
		.mount("/", routes![version])
		.mount("/", routes![downloads_list])
		.mount("/", routes![downloads_queue])
		.mount("/", routes![downloads_add])
		.mount("/", routes![track])
		.mount("/", routes![album])
		.mount("/", routes![playlist])
		.mount("/", routes![artist])
		.mount("/", routes![user_playlists])
		.mount("/", routes![search])
		.mount("/", routes![login_url])
		.mount("/", routes![login_confirm])
		.register("/", catchers![general_not_found])
		.launch()
		.await
		.unwrap();
}

#[catch(404)]
fn general_not_found() -> String {
	// return a list of all available routes
	let routes = vec![
		"/version",
		"/spotify/track/<id>",
		"/spotify/album/<id>",
		"/spotify/playlist/<id>",
		"/spotify/artist/<id>",
		"/spotify/user_playlists",
		"/spotify/search/<name>",
		"/downloads/list",
		"/downloads/queue",
		"/downloads/add/<id>",
		"/login/url",
		"/login/confirm/<code>",
	];

	serde_json::to_string_pretty(&routes).unwrap()
}

fn decode_hex(s: &str) -> Result<String, ParseIntError> {
    let as_vec = (0..s.len())
	.step_by(2)
	.map(|i| u8::from_str_radix(&s[i..i + 2], 16))
	.collect::<Result<Vec<_>, _>>()?;

	Ok(String::from_utf8(as_vec).unwrap())
}

#[get("/login/confirm/<code>")]
async fn login_confirm(
	spotify: &State<Arc<Spotify>>,
	redirect_state: &State<RedirectState>,
	code: &str,
) -> MyResponder<String> {
	let login_state: String;
	{
		let locked = redirect_state.state.lock().unwrap();
		login_state = locked.clone();
	}

	let redirect_url = match decode_hex(code) {
		Ok(url) => url,
		Err(e) => {
			return MyResponder::new(format!("{{\"error\": \"{}\"}}", e));
		}
	};
	
	match spotify.spotify.redirected(&redirect_url, &login_state).await
	{
		Ok(_) => {
			MyResponder::new("{\"state\": \"redirect success\"}".to_string())
		}
		Err(e) => {
			MyResponder::new(format!("{{ \"error\": \"{}\"}}", e))
		}
	}
}

#[get("/login/url")]
async fn login_url(spotify: &State<Arc<Spotify>>, redirect_state: &State<RedirectState>) -> MyResponder<String> {
	let (url, state) = aspotify::authorization_url(
		&spotify.spotify.credentials.id,
		[
			Scope::UgcImageUpload,
			Scope::UserReadPlaybackState,
			Scope::UserModifyPlaybackState,
			Scope::UserReadCurrentlyPlaying,
			Scope::Streaming,
			Scope::AppRemoteControl,
			Scope::UserReadEmail,
			Scope::UserReadPrivate,
			Scope::PlaylistReadCollaborative,
			Scope::PlaylistModifyPublic,
			Scope::PlaylistReadPrivate,
			Scope::PlaylistModifyPrivate,
			Scope::UserLibraryModify,
			Scope::UserLibraryRead,
			Scope::UserTopRead,
			Scope::UserReadRecentlyPlayed,
			Scope::UserFollowRead,
			Scope::UserFollowModify,
		]
		.iter()
		.copied(),
		false,
		"http://127.0.0.1:3001/static/forward.html",
	);

	let mut locked = redirect_state.state.lock().unwrap();
	*locked = state;

	MyResponder::new(format!("{{\"url\": \"{}\"}}", url))
}

#[get("/version")]
fn version(settings: &State<Settings>) -> String {
	format!("{{\"version\": \"{}\"}}", settings.version)
}

#[get("/spotify/track/<id>")]
async fn track(spotify: &State<Arc<Spotify>>, id: &str) -> String {
	match spotify.track(id).await {
		Ok(track) => serde_json::to_string(&track).unwrap().to_string(),
		Err(e) => {
			format!("{{\"error\": \"{}\"}}", e)
		}
	}
}

#[get("/spotify/album/<id>")]
async fn album(spotify: &State<Arc<Spotify>>, id: &str) -> String {
	match spotify.album(id).await {
		Ok(album) => serde_json::to_string(&album).unwrap().to_string(),
		Err(e) => {
			format!("{{\"error\": \"{}\"}}", e)
		}
	}
}

#[get("/spotify/playlist/<id>")]
async fn playlist(spotify: &State<Arc<Spotify>>, id: &str) -> MyResponder<String> {
	let result = match spotify.playlist(id).await {
		Ok(playlist) => serde_json::to_string(&playlist).unwrap().to_string(),
		Err(e) => {
			format!("{{\"error\": \"{}\"}}", e)
		}
	};

	MyResponder::from(result)
}

#[get("/spotify/artist/<id>")]
async fn artist(spotify: &State<Arc<Spotify>>, id: &str) -> String {
	match spotify.artist(id).await {
		Ok(artist) => serde_json::to_string(&artist).unwrap().to_string(),
		Err(e) => {
			format!("{{\"error\": \"{}\"}}", e)
		}
	}
}

#[get("/spotify/search/<name>")]
async fn search(spotify: &State<Arc<Spotify>>, name: &str) -> MyResponder<String> {
	let tracks = match spotify.search_tracks(name).await {
		Ok(artist) => serde_json::to_string(&artist).unwrap().to_string(),
		Err(e) => format!("{{\"error\": \"{}\"}}", e),
	};

	let albums = match spotify.search_albums(name).await {
		Ok(artist) => serde_json::to_string(&artist).unwrap().to_string(),
		Err(e) => format!("{{\"error\": \"{}\"}}", e),
	};

	let playlists = match spotify.search_palaylist(name).await {
		Ok(artist) => serde_json::to_string(&artist).unwrap().to_string(),
		Err(e) => format!("{{\"error\": \"{}\"}}", e),
	};

	let obj = json!({
			"tracks" : tracks,
			"albums" : albums,
			"playlists" : playlists
	});

	MyResponder::from(serde_json::to_string_pretty(&obj).unwrap())
}

#[get("/spotify/user_playlists")]
async fn user_playlists(spotify: &State<Arc<Spotify>>) -> MyResponder<String> {
	let result = match spotify.user_playlists().await {
		Ok(playlists) => serde_json::to_string_pretty(&playlists)
			.unwrap()
			.to_string(),
		Err(e) => {
			format!("{{\"error\": \"{}\"}}", e)
		}
	};
	MyResponder::from(result)
}

#[get("/downloads/queue")]
async fn downloads_queue(downloader: &State<Arc<Downloader>>) -> MyResponder<String>  {
	let queue = downloader.get_downloads().await;

	MyResponder::from(serde_json::to_string(&queue).unwrap())
}

#[get("/downloads/add/<id>")]
async fn downloads_add(
	id: &str,
	downloader: &State<Arc<Downloader>>,
	spotify: &State<Arc<Spotify>>,
) -> MyResponder<String>  {
	// load teh track from aspotify
	let track = match spotify.track(id).await {
		Ok(track) => track,
		Err(e) => {
			return MyResponder::from(format!("{{\"error\": \"{}\"}}", e));
		}
	};

	// print the found track and id
	println!("Found track: {}", track.name);

	
	// format this pretty
	let obj = json!({
		"status" : "added",
		"track" : track
	});
	let answer = serde_json::to_string(&obj).unwrap();
	
	
	println!("Adding track to queue: {}", track.name);
	downloader.add_to_queue(track.into()).await;
	
	MyResponder::from(answer)
}

#[get("/downloads/list")]
fn downloads_list(settings: &State<Settings>) -> String {
	// function that lists all files with a defined extension within a directory
	let extensions = vec!["flac", "ogg", "mp3", "aac"];

	// create am empty vector to store the files
	let mut files = Vec::new();

	// iterate over the downloadpath and list all files with the defined extensions
	// return the list of files in form of a json object
	let paths = match fs::read_dir(settings.downloader.path.clone()) {
		Ok(paths) => paths,
		Err(e) => {
			println!("Error: {}", e);
			// create a json object with an error message
			return "{'error': 'Could not read directory'}".to_string();
		}
	};

	for file in paths {
		let file = file.unwrap();

		let path = file.path();
		let extension = path.extension().unwrap();
		let extension = extension.to_str().unwrap();

		if path.is_dir() {
			continue;
		}

		println!("Extension: {}", extension);
		if extensions.contains(&extension) {
			println!("File: {:?}", path);
			files.push(path);
		}
	}

	// format
	let obj = json!({
		"files" : files
	});

	serde_json::to_string_pretty(&obj).unwrap()
}
