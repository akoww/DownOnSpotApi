import ref from 'vue';
import axios from 'axios';

export function menu_playlists() {
  return {

    setup() {
      const items = ref([{ name: 'Foo' }, { name: 'Bar' }])
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
            this.apiData = response.data;
          })
          .catch(error => {
            // Handle error
            console.error('There was an error!', error);
          });
      }
    }
  }
}


export function test2() {
  return {
    setup() {
      const count = ref(0)
      return { count }
    },
    template: `<div>count is {{ count }}</div>`
  }
}