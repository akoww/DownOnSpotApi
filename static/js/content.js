import { ref, markRaw } from 'vue';
//import axios from 'axios';


function getFirstImage(images) {
  // get the first image with at least 192x192px or the first image if no image has been found
  var suited_images = images.filter(image => image.height >= 192 || image.width >= 192);

  if (suited_images.length > 0) {
    return suited_images[0].url;
  } else if (images.length > 0) {
    return images[0].url;
  } else {
    return "placeholder.jpg";
  }
}

function formatDurationMs(duration) {
  var minutes = Math.floor(duration / 60000);
  var seconds = ((duration % 60000) / 1000).toFixed(0);
  return minutes + ":" + (seconds < 10 ? '0' : '') + seconds;
}

function about() {
  return {
    setup() {
      return {}
    },
    template: `<div>about</div>`
  }
}

function home() {
  return {
    setup() {
      return {}
    },
    template: `<div>home</div>`
  }
}

function content_playlist(params) {
  return {
    setup() {
      const tracks = ref([]);
      const meta = ref({
        name: "",
        owner: ""

      });
      const image = ref("placeholder.jpg");
      return { tracks, meta, image }
    },

    mounted() {
      this.fetchData();
    },

    methods: {

      handleDownloadTrack(track_id) {

        axios.get('http://127.0.0.1:8000/downloads/add/' + track_id)
          .then(response => {
            console.log(response.data);

            // display short notification
            Toastify({
              text: "Download Started",
              duration: 3000
            }).showToast();


          })
          .catch(error => {
            // Handle error
            console.error('There was an error!', error);
            Toastify({
              text: "Download failed: " + error.response.error,
              duration: 3000
            }).showToast();
          });

      },


      handleDownloadPlaylist() {

        // collect all track ids and concat them
        var track_ids = this.tracks.map(track => track.track.id);
        var track_ids_str = track_ids.join(',');

        axios.get('http://127.0.0.1:8000/downloads/add_multi/' + track_ids_str)
          .then(response => {
            console.log(response.data);

            // display short notification
            Toastify({
              text: "Download Started",
              duration: 3000
            }).showToast();
          })
          .catch(error => {
            // Handle error
            console.error('There was an error!', error);
            Toastify({
              text: "Download failed: " + error.response.error,
              duration: 3000
            }).showToast();
          });
      },

      fetchData() {
        var playlist_id = params[0];

        axios.get('http://127.0.0.1:8000/spotify/playlist/' + playlist_id)
          .then(response => {


            // loop through all tracks and format the duration and date
            response.data.tracks.items.forEach(track => {
              track.track.duration = formatDurationMs(track.track.duration_ms);
              track.track.artist_id = track.track.artists[0].id;
            });

            // get all tracks
            this.tracks = response.data.tracks.items;


            // get best url for image
            this.image = getFirstImage(response.data.images);
            this.meta = {
              name: response.data.name,
              owner: response.data.owner.display_name
            };
          })
          .catch(error => {
            // Handle error
            console.error('There was an error!', error);
          });
      }
    },

    template:
      `<div class="playlist_content">
        <div class="playlist_header">
          <div class="image">
            <img :src="image" width="192px" height="192px">
          </div>
          <div class="text">
            <h1>Playlist: "{{ meta.name }}"</h1>
            <p>{{ meta.owner }}</p>
          </div>
          <div id="download_playlist" >
            <span class="material-symbols-outlined" @click="handleDownloadPlaylist()">download</span>
          </div>
        </div>
      </div>
      <table id="track_list" class="striped">
        <thead>
          <tr>
            <th class="table_id">#</th>
            <th>Title</th>
            <th>Album</th>
            <th>Length</th>
            <th>Date</th>
            <th>Artist</th>
            <th>...</th> <!-- Additional columns placeholder -->
          </tr>
        </thead>
        <tbody>
          <!-- Sample row -->
          <tr v-for="(track, index) in tracks" :key="track.track.id, index">
            <td class="table_id">{{ index+1 }}</td>
            <td>{{ track.track.name }}</td>
            <td><a :href='"#album/" + track.track.album.id'>{{ track.track.album.name }}</a></td>
            <td>{{ track.track.duration }}</td>
            <td>{{ track.track.album.release_date }}</td>
            <td><a :href='"#artist/" + track.track.artist_id'>{{ track.track.album.artists[0].name }}</a></td>
            <td><span class="material-symbols-outlined" @click="handleDownloadTrack(track.track.id)">
            download
            </span></td> <!-- Additional columns placeholder -->
          </tr>
          <!-- More rows can be added here -->
        </tbody>
      </table>`
  }
}


function artist(params) {
  return {
    setup() {
      const tracks = ref([]);
      const albums = ref([]);
      const image = ref("placeholder.jpg");
      const meta = ref({
        name: "...",
        followers: {
          total: 0
        },
        popularity: 0
      });
      return { tracks, meta, albums, image }
    },

    mounted() {
      this.fetchData();
    },

    methods: {

      handleDownloadTrack(track_id) {

        axios.get('http://127.0.0.1:8000/downloads/add/' + track_id)
          .then(response => {
            console.log(response.data);

            // display short notification
            Toastify({
              text: "Download Started",
              duration: 3000
            }).showToast();
          })
          .catch(error => {
            // Handle error
            console.error('There was an error!', error);
            Toastify({
              text: "Download failed: " + error.response.error,
              duration: 3000
            }).showToast();
          });

      },

      fetchData() {
        var artist_id = params[0];

        axios.get('http://127.0.0.1:8000/spotify/artist/' + artist_id)
          .then(response => {

            response.data.tracks.forEach(track => {
              track.duration = formatDurationMs(track.duration_ms);
            });

            this.meta = response.data.meta;
            this.albums = response.data.albums.items;
            this.tracks = response.data.tracks;
            this.image = getFirstImage(response.data.meta.images);

          })
          .catch(error => {
            // Handle error
            console.error('There was an error!', error);
          });
      }
    },
    template: `
  <div>
    <div class="artist_header">
      <div class="image">
        <img :src="image" width="192px" height="192px">
      </div>
      <div class="text">
        <h1>Artist: "{{ meta.name }}"</h1>
        <p>{{ meta.followers.total }} listener</p>
        <p>{{ meta.popularity }} popularity</p>
      </div>
    </div>
    <div class="content">
      <div class="tracks_small">
        <h2>Top Tracks</h2>
        <table id="track_list" class="striped">
          <thead>
            <tr>
              <th class="table_id">#</th>
              <th> Title </th>
              <th> Album </th>
              <th> Popularity </th>
              <th> Length </th>
              <th>...</th> <!-- Additional columns placeholder -->
            </tr>
          </thead>
          <tbody>
            <tr v-for="(track, index) in tracks" :key="track.id, index">
              <td class="table_id">{{ index+1 }}</td>
              <td>{{ track.name }}</td>
              <td><a :href='"#album/" + track.album.id'>{{ track.album.name }}</a></td>
              <td>{{ track.popularity }}</td>
              <td>{{ track.duration }}</td>
              <td><span class="material-symbols-outlined" @click="handleDownloadTrack(track.track.id)">
                  download
                </span></td> <!-- Additional columns placeholder -->
            </tr>
          </tbody>
        </table>
      </div>
      <div class="albums">
        <h2>Albums</h2>
        <div class="row">
          <div v-for="album in albums" :key="album.id" class="card">
          <a :href='"#album/" + album.id'>
          <img :src="album.images[1].url" alt="album cover"> </img>
          <div class="container">
            <h4><b>{{ album.name }}</b></h4>
            <p>{{ album.release_date }}</p>
          </div>
          </a>
          </div>
        </div>
      </div>
    </div>
  </div>

    `
  }

}


function album(params) {
  return {
    setup() {
      const tracks = ref([]);
      const image = ref("placeholder.jpg");
      const meta = ref({
        name: "...",
        release_data: "",
        artist: ""
      });
      return { tracks, image, meta }
    },

    mounted() {
      this.fetchData();
    },

    methods: {

      handleDownloadTrack(track_id) {

        axios.get('http://127.0.0.1:8000/downloads/add/' + track_id)
          .then(response => {
            console.log(response.data);

            // display short notification
            Toastify({
              text: "Download Started",
              duration: 3000
            }).showToast();
          })
          .catch(error => {
            // Handle error
            console.error('There was an error!', error);
            Toastify({
              text: "Download failed: " + error.response.error,
              duration: 3000
            }).showToast();
          });

      },

      fetchData() {
        var album_id = params[0];

        axios.get('http://127.0.0.1:8000/spotify/album/' + album_id)
          .then(response => {

            response.data.tracks.items.forEach(track => {
              track.duration = formatDurationMs(track.duration_ms);
            });

            this.meta = {
              name: response.data.name,
              release_date: response.data.release_date,
              artist: response.data.artists[0].name,
              artist_id: response.data.artists[0].id
            };
            this.tracks = response.data.tracks.items;
            this.image = getFirstImage(response.data.images);

          })
          .catch(error => {
            // Handle error
            console.error('There was an error!', error);
          });
      }
    },
    template: `
  <div>
    <div class="album_header">
      <div class="image">
        <img :src="image" width="192px" height="192px">
      </div>
      <div class="text">
        <h1>Album: "{{ meta.name }}"</h1>
        <p><a :href='"#artist/" + meta.artist_id'> {{ meta.artist }}</a></p>
        <p>{{ meta.release_date }}</p>
      </div>
    </div>
    <div class="content">
      <div class="tracks_small">
        <h2>Tracks</h2>
        <table id="track_list" class="striped">
          <thead>
            <tr>
              <th class="table_id">#</th>
              <th> Title </th>
              <th> Length </th>
              <th>...</th> <!-- Additional columns placeholder -->
            </tr>
          </thead>
          <tbody>
            <tr v-for="(track, index) in tracks" :key="track.id, index">
              <td class="table_id">{{ index+1 }}</td>
              <td>{{ track.name }}</td>
              <td>{{ track.duration }}</td>
              <td><span class="material-symbols-outlined" @click="handleDownloadTrack(track.track.id)">
                  download
                </span></td> <!-- Additional columns placeholder -->
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>

    `
  }

}


function search(params) {
  return {
    setup() {
      const tracks = ref([]);
      const meta = ref({
        name: "..."
      });
      return { tracks, meta }
    },

    mounted() {
      this.fetchData();
    },

    methods: {

      handleDownloadTrack(track_id) {

        axios.get('http://127.0.0.1:8000/downloads/add/' + track_id)
          .then(response => {
            console.log(response.data);

            // display short notification
            Toastify({
              text: "Download Started",
              duration: 3000
            }).showToast();
          })
          .catch(error => {
            // Handle error
            console.error('There was an error!', error);
            Toastify({
              text: "Download failed: " + error.response.error,
              duration: 3000
            }).showToast();
          });

      },

      fetchData() {
        var search_term = decodeURIComponent(params[0]);
        this.meta.name = search_term;

        function formatDurationMs(duration) {
          var minutes = Math.floor(duration / 60000);
          var seconds = ((duration % 60000) / 1000).toFixed(0);
          return minutes + ":" + (seconds < 10 ? '0' : '') + seconds;
        }

        axios.get('http://127.0.0.1:8000/spotify/search/' + search_term)
          .then(response => {
            // loop through all tracks and format the duration and date

            // parse json into object
            var tracks = response.data.tracks;

            tracks.forEach(track => {
              track.duration = formatDurationMs(track.duration_ms);
              track.artist_id = track.artists[0].id;
            });

            this.tracks = tracks;
          })
          .catch(error => {
            // Handle error
            console.error('There was an error!', error);
          });
      }
    },

    template:
      ` 
      <div class="text">
        <h1>Search results for: "{{ meta.name }}"</h1>
      </div>
      <table id="track_list" class="striped">
        <thead>
          <tr>
            <th class="table_id">#</th>
            <th>Title</th>
            <th>Album</th>
            <th>Length</th>
            <th>Date</th>
            <th>Artist</th>
            <th>...</th> <!-- Additional columns placeholder -->
          </tr>
        </thead>
        <tbody>
          <!-- Sample row -->
          <tr v-for="(track, index) in tracks" :key="track.id, index">
            <td class="table_id">{{ index+1 }}</td>
            <td>{{ track.name }}</td>
            <td>{{ track.album.name }}</td>
            <td>{{ track.duration }}</td>
            <td>{{ track.album.release_date }}</td>
            <td><a :href='"#artist/" + track.artist_id'>{{ track.album.artists[0].name }}</a></td>
            <td><span class="material-symbols-outlined" @click="handleDownloadTrack(track.track.id)">
            download
            </span></td> <!-- Additional columns placeholder -->
          </tr>
          <!-- More rows can be added here -->
        </tbody>
      </table>`
  }
}

function download_list() {
  return {
    setup() {
      const downloads = ref([]);
      return { downloads }
    },

    mounted() {
      this.fetchData();
    },

    methods: {
      fetchData() {
        axios.get('http://127.0.0.1:8000/downloads/list')
          .then(response => {
            this.downloads = response.data.files;
          })
          .catch(error => {
            // Handle error
            console.error('There was an error!', error);
          });
      }
    },

    template:
      `
      <div class="download_list">
        <h1>Downloads</h1>
        <ul>
          <li v-for="download in downloads">
            {{ download }}
          </li>
        </ul>
      </div>
      `
  }
}


export function content() {
  return {

    setup() {
      const content = ref();
      return { content }
    },

    mounted() {
      // Function to update the current hash
      const updateHash = () => {

        // create map for endpoints and components
        const endpoints = {
          '#home': home,
          '#about': about,
          '#search': search,
          '#playlist': content_playlist,
          '#artist': artist,
          '#album': album,
          '#downloads': download_list
        };

        // calculate the hash it its always something like "#endpoint/param1;param2;param3"
        const endpoint = window.location.hash.split('/')[0];

        // if params exist split them by ;
        const params = window.location.hash.split('/')[1] ? window.location.hash.split('/')[1].split(';') : [];

        // check if endpoint exists
        if (endpoints[endpoint]) {
          // set the content to the endpoint
          this.content = endpoints[endpoint](params);
        } else {
          // if not set it to home
          this.content = home();
        }

      };


      // Event listener to track hash changes
      const hashChangeListener = () => {
        updateHash();
      };

      window.addEventListener('hashchange', hashChangeListener);
      hashChangeListener();
    },
    template: `
      <component :is="content"></component>`
  }
}
