import { ref, markRaw } from 'vue';


export function menu() {
    return {
  
      setup() {
        const items = ref([])
        return { items }
      },
  
      mounted() {
        this.fetchData();
      },
  
      methods: {
        fetchData() {
          axios.get('http://127.0.0.1:8000/spotify/user_playlists/')
            .then(response => {
              // Handle success
              this.items = response.data;
            })
            .catch(error => {
              // Handle error
              console.error('There was an error!', error);
            });
        }
      },

      template: `
        <h1>Spotiy</h1>
        <h2>Discover</h2>
        <ul>
            <li><a href="#home"><span class="icon-home"></span>Home</a></li>
            <li><a href="#browse"><span class="icon-rss"></span>Browse</a></li>
            <li><a href="#search"><span class="icon-search"></span>Search</a></li>
            <li><a href="#downloads"><span class="icon-download"></span>Downloads</a></li>
        </ul>
        <h2>Library</h2>
        <ul id="menu">
            <li><a href="#favorites"><span class="icon-bookmark"></span>Favorites</a></li>
            <li><a href="#following"><span class="icon-share"></span>Following</a></li>
            <li v-for="item in items" :key="item.id"><a :href="'#playlist/' + item.id">{{ item.name }}</a></li>    
        </ul>`
    }
  }