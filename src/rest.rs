extern crate rocket;

use std::fs;
use std::sync::Arc;

use rocket::catch;
use rocket::catchers;
use rocket::get;
use rocket::routes;
use rocket::State;
use serde_json::json;

use crate::downloader::Downloader;
use crate::settings::Settings;
use crate::spotify::Spotify;

pub async fn launch_rest(settings: &Settings, spotify: Arc<Spotify>, downloader: Arc<Downloader>) {
	rocket::build()
		.manage(settings.clone())
		.manage(spotify)
		.manage(downloader)
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
	];

	serde_json::to_string_pretty(&routes).unwrap()
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
async fn playlist(spotify: &State<Arc<Spotify>>, id: &str) -> String {
	match spotify.playlist(id).await {
		Ok(playlist) => serde_json::to_string(&playlist).unwrap().to_string(),
		Err(e) => {
			format!("{{\"error\": \"{}\"}}", e)
		}
	}
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
async fn search(spotify: &State<Arc<Spotify>>, name: &str) -> String {
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

	serde_json::to_string_pretty(&obj).unwrap()
}

#[get("/spotify/user_playlists")]
async fn user_playlists(spotify: &State<Arc<Spotify>>) -> String {
	match spotify.user_playlists().await {
		Ok(playlists) => serde_json::to_string_pretty(&playlists)
			.unwrap()
			.to_string(),
		Err(e) => {
			format!("{{\"error\": \"{}\"}}", e)
		}
	}
}

#[get("/downloads/queue")]
async fn downloads_queue(downloader: &State<Arc<Downloader>>) -> String {
	let queue = downloader.get_downloads().await;

	serde_json::to_string(&queue).unwrap()
}

#[get("/downloads/add/<id>")]
async fn downloads_add(
	id: &str,
	downloader: &State<Arc<Downloader>>,
	spotify: &State<Arc<Spotify>>,
) -> String {
	// load teh track from aspotify
	let track = match spotify.track(id).await {
		Ok(track) => track,
		Err(e) => {
			return format!("{{\"error\": \"{}\"}}", e);
		}
	};

	// format this pretty
	let obj = json!({
		"status" : "added",
		"track" : track
	});
	let answer = serde_json::to_string(&obj).unwrap();

	downloader.add_to_queue(track.into()).await;

	answer
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
