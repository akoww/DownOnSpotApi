use aspotify::{AlbumSimplified, Client, ClientCredentials, ItemType, PlaylistSimplified, Track};
use librespot::core::authentication::Credentials;
use librespot::core::cache::Cache;
use librespot::core::config::SessionConfig;
use librespot::core::session::Session;
use std::fmt;
use std::path::Path;

use crate::error::SpotifyError;

pub struct Spotify {
	// librespotify sessopm
	pub session: Session,
	pub spotify: Client,
}

impl Spotify {
	/// Create new instance
	pub async fn new(
		username: &str,
		password: &str,
		client_id: &str,
		client_secret: &str,
	) -> Result<Spotify, SpotifyError> {
		// librespot
		let credentials = Credentials::with_password(username, password);
		let (session, _) = Session::connect(
			SessionConfig::default(),
			credentials,
			Some(Cache::new(Some(Path::new("credentials_cache")), None, None, None).unwrap()),
			true,
		)
		.await?;

		//aspotify
		let credentials = ClientCredentials {
			id: client_id.to_string(),
			secret: client_secret.to_string(),
		};
		let spotify = Client::new(credentials);

		Ok(Spotify { session, spotify })
	}

	pub async fn track(&self, id: &str) -> Result<aspotify::model::Track, SpotifyError> {
		let track = self.spotify.tracks().get_track(id, None).await?;
		Ok(track.data)
	}

	pub async fn playlist(&self, id: &str) -> Result<aspotify::model::Playlist, SpotifyError> {
		let playlist = self.spotify.playlists().get_playlist(id, None).await?;
		Ok(playlist.data)
	}

	pub async fn album(&self, id: &str) -> Result<aspotify::model::Album, SpotifyError> {
		let album = self.spotify.albums().get_album(id, None).await?;
		Ok(album.data)
	}

	pub async fn artist(&self, id: &str) -> Result<aspotify::model::Artist, SpotifyError> {
		let artist = self.spotify.artists().get_artist(id).await?;
		Ok(artist.data)
	}

	pub async fn user_playlists(&self) -> Result<Vec<aspotify::model::PlaylistSimplified>, SpotifyError> {
		let playlists = self.spotify.playlists().current_users_playlists(20,0).await?;
		Ok(playlists.data.items)
	}

	pub async fn search_tracks(&self, query: &str) -> Result<Vec<Track>, SpotifyError> {
		Ok(self
			.spotify
			.search()
			.search(query, [ItemType::Track], true, 20, 0, None)
			.await?
			.data
			.tracks
			.unwrap()
			.items)
	}

	pub async fn search_albums(&self, query: &str) -> Result<Vec<AlbumSimplified>, SpotifyError> {
		Ok(self
			.spotify
			.search()
			.search(query, [ItemType::Album], true, 10, 0, None)
			.await?
			.data
			.albums
			.unwrap()
			.items)
	}

	pub async fn search_palaylist(
		&self,
		query: &str,
	) -> Result<Vec<PlaylistSimplified>, SpotifyError> {
		Ok(self
			.spotify
			.search()
			.search(query, [ItemType::Playlist], true, 10, 0, None)
			.await?
			.data
			.playlists
			.unwrap()
			.items)
	}
}

impl Clone for Spotify {
	fn clone(&self) -> Self {
		Self {
			session: self.session.clone(),
			spotify: Client::new(self.spotify.credentials.clone()),
		}
	}
}

/// Basic debug implementation so can be used in other structs
impl fmt::Debug for Spotify {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "<Spotify Instance>")
	}
}
