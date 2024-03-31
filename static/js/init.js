import { ref, markRaw } from 'vue';
//import axios from 'axios';

export function menu_playlists() {
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
    }
  }
}

function about() {
  return {
    setup() {
      return {}
    },
    template: `<div>about</div>`
  }
}

function contact() {
  return {
    setup() {
      return {}
    },
    template: `<div>contact</div>`
  }
}


export function content_playlists() {
  return {

    setup() {
        const content = ref();
        return { content }
    },

    mounted() {
      // Function to update the current hash
      const updateHash = () => {
        if (window.location.hash === '#about') {
          this.content = about();
        } else if (window.location.hash === '#contact') {
          this.content = contact();
        } else  {
          this.content = window.location.hash;
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