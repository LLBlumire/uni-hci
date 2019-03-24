<template>
  <div class="status">
    <div class="column">
      <FoodSearch v-on:search-resolved="searchResolved" v-bind:user="user" />
      <FoodInfoBox
        v-bind:foodInfoProp="foodInfo"
        v-bind:showAddFood="showAddFood"
        v-bind:user="user"
        v-bind:dateToAdd="date"
      />
    </div>
    <div class="column">
      <input type="date" v-model="selectDate" id="selectDate" />
      <FoodInfoBox
        v-bind:foodInfoProp="user.totalConsumption[dateNumber]"
        v-bind:user="user"
      />
      <hr/>
      <FoodInfoBox
        v-for="food in user.consumed[dateNumber]"
        v-bind:key="food.uuid"
        v-bind:foodInfoProp="food.foodInfo"
        v-bind:mutateByGrams="food.grams"
        v-bind:user="user"
      />
    </div>
  </div>
</template>

<script>
import FoodInfoBox from "../components/FoodInfoBox.vue";
import FoodSearch from "../components/FoodSearch.vue";

export default {
  name: "status",
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
      date: new Date(),
      foodInfo: {}
    };
  },
  computed: {
    showAddFood() {
      return typeof this.foodInfo.shrt_desc !== "undefined";
    },
    selectDate: {
      get() {
        return this.date.toISOString().slice(0, 10);
      },
      set(newValue) {
        this.date = new Date(Date.parse(newValue));
      }
    },
    dateNumber() {
      return this.$root.$children[0].getDateNumber(this.date);
    }
  },
  created() {
    if (!this.$root.$children[0].isLoggedIn) {
      this.$router.push({ name: "home" });
    }
    console.log(this.dateNumber);
    this.$root.$children[0].forceCreateDateTotalConsumption(this.dateNumber);
  },
  watch: {
    dateNumber(newDateNumber) {
      this.$root.$children[0].forceCreateDateTotalConsumption(newDateNumber);
    }
  },
  methods: {
    searchResolved(newFoodInfo) {
      this.foodInfo = newFoodInfo;
    }
  }
};
</script>
