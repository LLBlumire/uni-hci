<template>
  <div class="home">
    <div id="intro">
      <p>
        Quick search for food, and add it if you're logged in! The status page
        can be used to see your food consumption on specific dates, as well as
        to add food on previous days if you forgot to! The Submit page allows
        you to add custom food entries.
      </p>
    </div>
    <div id="food-query">
      <FoodSearch v-on:search-resolved="searchResolved" v-bind:user="user" />
      <FoodInfoBox
        v-bind:foodInfoProp="foodInfo"
        v-bind:showAddFood="showAddFood"
        v-bind:user="user"
      />
    </div>
  </div>
</template>

<script>
import FoodInfoBox from "../components/FoodInfoBox.vue";
import FoodSearch from "../components/FoodSearch.vue";

export default {
  name: "home",
  components: {
    FoodInfoBox,
    FoodSearch
  },
  props: {
    user: {
      type: Object,
      required: true
    }
  },
  data() {
    return {
      foods: {},
      setDate: new Date(),
      foodInfo: {}
    };
  },
  computed: {
    knownFoodItem() {
      return this.foodInfo !== {};
    },
    showAddFood() {
      return this.$root.$children[0].isLoggedIn && this.knownFoodItem;
    }
  },
  methods: {
    searchResolved(newFoodInfo) {
      this.foodInfo = newFoodInfo;
    }
  }
};
</script>
