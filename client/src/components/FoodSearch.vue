<template>
  <div class="food-search">
    <label for="food-search">Search</label>
    <input id="food-search" type="text" list="food-list" v-model="search" />
    <datalist id="food-list">
      <option v-for="food in foodKeys" :key="food">
        {{ food }}
      </option>
    </datalist>
  </div>
</template>

<script>
import axios from "axios";

export default {
  name: "FoodSearch",
  data() {
    return {
      foods: {},
      search: ""
    };
  },
  props: {
    user: {
      type: Object,
      required: true
    }
  },
  computed: {
    foodKeys() {
      return Object.keys(this.foods).sort();
    }
  },
  created() {
    axios.get("/api/foods").then(response => {
      this.foods = response.data;
    });
  },
  watch: {
    search(newSearch) {
      if (this.foodKeys.includes(newSearch)) {
        axios.get(`/api/food/${this.foods[newSearch]}`).then(response => {
          this.$emit("search-resolved", response.data);
          this.$emit("foobar", "foo, bar");
        });
      }
    }
  }
};
</script>
