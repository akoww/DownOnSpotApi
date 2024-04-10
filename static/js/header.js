import { ref, markRaw } from 'vue';

export function header() {
    return {
  
      setup() {
        const header = ref("login");
        const login = ref("login");
        return { header, login }
      },

      methods: {
        warn(message, event) {

          // get the content of the input field
          const searchText = this.$refs.search_text.value;

          // set the url to the search query
          // change everything after the # to the search query
          window.location.hash = `#search/${searchText}`;
          console.log(searchText);

          // now we have access to both the message and the event
          if (event) {
            event.preventDefault()
          }
        }
      },

      template: ` 
      <div class="header"> 
        <input type="text" placeholder="Search track" ref="search_text">
        <button type="submit" @click="(event) => warn('Form cannot be submitted yet.', event)">Search</button>
        <div></div> <!-- Empty space in the middle -->
        <a href="http://127.0.0.1:3001/static/login.html">
            <button class="loginbutton" type="button">{{ login }}</button> 
        </a>
      </div>`,
    }
}