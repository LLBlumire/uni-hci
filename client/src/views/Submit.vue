<template>
  <div class="submit">
    <input type="date" v-model="selectDate" id="selectDate" />
    <FoodInfoBox 
      v-bind:user="user"
      v-bind:showAddFood="true"
      v-bind:dateToAdd="date"/>
  </div>
</template>

<script>
import FoodInfoBox from '../components/FoodInfoBox'

export default {
  name: "submit",
  components: {
    FoodInfoBox
  },
  props: {
    user: {
      type: Object,
      required: true
    }
  },
  data () {
    return {
      date: new Date()
    }
  },
  computed: {
    selectDate: {
      get() {
        return this.date.toISOString().slice(0, 10);
      },
      set(newValue) {
        this.date = new Date(Date.parse(newValue));
      }
    }
  }
};
</script>
