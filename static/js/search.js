import { ref, markRaw } from 'vue';

export function search() {
    return {
  
      setup() {
        const search = ref("search content");
        return { search }
      },
      template: ` 
      <div class="search"> 
        {{ search }}
      </div>`,
    }
}