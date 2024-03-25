#[macro_use]
extern crate log;

mod converter;
mod downloader;
mod error;
mod settings;
mod spotify;
mod tag;
mod rest;
mod static_server;

use std::sync::Arc;
use colored::Colorize;
use downloader::Downloader;
use error::SpotifyError;
use settings::Settings;
use spotify::Spotify;

#[cfg(not(windows))]
#[tokio::main]
async fn main() {
	start().await;
}

#[cfg(windows)]
#[tokio::main]
async fn main() {
	use colored::control;

	//backwards compatibility.
	if control::set_virtual_terminal(true).is_ok() {};
	start().await;
}

async fn load_settings() -> Result<Settings, SpotifyError> {
	match Settings::load().await {
		Ok(settings) => {
			println!(
				"{} {}.",
				"Settings successfully loaded.\nContinuing with spotify account:".green(),
				settings.username
			);
			Ok(settings)
		}
		Err(e) => {
			println!(
				"{} {}...",
				"Settings could not be loaded, because of the following error:".red(),
				e
			);
			Err(e)
		}
	}
}

async fn login_spotify(settings: &Settings) -> Result<Arc<Spotify>, SpotifyError> {
	match Spotify::new(
		&settings.username,
		&settings.password,
		&settings.client_id,
		&settings.client_secret,
	)
	.await
	{
		Ok(spotify) => {
			println!("{}", "Login succeeded.".green());
			Ok(Arc::new(spotify))
		}
		Err(e) => {
			println!(
				"{} {}",
				"Login failed, possibly due to invalid credentials or settings:".red(),
				e
			);
			Err(SpotifyError::AuthenticationError)
		}
	}
}

async fn start() {

	env_logger::init();

	let settings = match load_settings().await {
		Ok(settings) => settings,
		Err(e) => {
			error!("{} {}", "Could not load settings:".red(), e);
			return;
		},
	};

	let spotify = match login_spotify(&settings).await {
		Ok(spotify) => spotify,
		Err(e) => {
			error!("{} {}", "Could not login to Spotify:".red(), e);
			return;
		},
	};

	let downloader = Arc::new(Downloader::new(settings.downloader.clone(), spotify.clone()));
	
	tokio::spawn(
		static_server::start_static_server()
	);
	rest::launch_rest(&settings, spotify, downloader).await;
}
