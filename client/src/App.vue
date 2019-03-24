<template>
  <div id="app">
    <header>
      <div id="hero">
        <div id="hero-left"></div>
        <div id="logo"></div>
        <div id="hero-right">
          <router-link to="/login" v-if="!isLoggedIn">SIGN IN</router-link>
          <a href="#" v-on:click="logout" v-else>SIGN OUT</a>
        </div>
      </div>
      <div id="nav">
        <router-link to="/">Home</router-link>
        <router-link to="/status" v-if="isLoggedIn">Status</router-link>
        <router-link to="/submit" v-if="isLoggedIn">Submit</router-link>
        <a href="https://hciufit.wordpress.com">About</a>
      </div>
    </header>
    <footer>
      <router-view v-bind:user="user" />
    </footer>
  </div>
</template>

<script>
export default {
  name: "app",
  data() {
    return {
      user: {
        name: "",
        consumed: {},
        totalConsumption: {}
      }
    };
  },
  methods: {
    logout() {
      this.user.name = "";
    },
    getDateNumber(dateObj) {
      dateObj = typeof dateObj === "undefined" ? new Date() : dateObj;
      let year = "" + dateObj.getFullYear();
      let monthPadding = dateObj.getMonth().toString().length == 1 ? "0" : "";
      let month = "" + (dateObj.getMonth());
      let datePadding = dateObj.getDate().toString().length == 1 ? "0" : "";
      let date = "" + dateObj.getDate();
      let fullDateString = year + monthPadding + month + datePadding + date;
      return fullDateString * 1;
    },
    getDateFromNumber(dateNumber) {
      dateNumber = "" + dateNumber;
      return new Date(
        dateNumber.slice(0, 4),
        dateNumber.slice(4, 6),
        dateNumber.slice(6, 8)
      );
    },
    gramExpand(foodInfo, grams, baseGrams = 100) {
      return {
        long_desc: foodInfo.long_desc,
        shrt_desc: foodInfo.shrt_desc,
        fd_grp_desc: foodInfo.fd_grp_desc,
        protein_g: foodInfo.protein_g * (grams / baseGrams),
        fat_g: foodInfo.fat_g * (grams / baseGrams),
        carbs_g: foodInfo.carbs_g * (grams / baseGrams),
        energy_kcal: foodInfo.energy_kcal * (grams / baseGrams),
        water_g: foodInfo.water_g * (grams / baseGrams),
        sugars_g: foodInfo.sugars_g * (grams / baseGrams)
      };
    },
    addFoodInfo(base, add) {
      return {
        long_desc: base.long_desc,
        shrt_desc: base.shrt_desc,
        fd_grp_desc: base.fd_grp_desc,
        protein_g: base.protein_g + add.protein_g,
        fat_g: base.fat_g + add.fat_g,
        carbs_g: base.carbs_g + add.carbs_g,
        energy_kcal: base.energy_kcal + add.energy_kcal,
        water_g: base.water_g + add.water_g,
        sugars_g: base.sugars_g + add.sugars_g
      };
    },
    forceCreateDateTotalConsumption(date) {
      if (typeof this.user.totalConsumption[date] !== 'undefined') {
        return;
      }
      let nicedate = this.getDateFromNumber(date).toISOString().slice(0, 10);
      this.user.totalConsumption[date] = {
        long_desc: `Total Food Eaten: ${nicedate}`,
        shrt_desc: "",
        fd_grp_desc: "",
        protein_g: 0.0,
        fat_g: 0.0,
        carbs_g: 0.0,
        energy_kcal: 0.0,
        water_g: 0.0,
        sugars_g: 0.0
      };
    }
  },
  computed: {
    isLoggedIn() {
      return this.user.name != "";
    },
    innerConsumed() {
      return this.user.consumed;
    }
  },
  watch: {
    innerConsumed(consumed) {
      for (var date of Object.keys(consumed)) {
        this.forceCreateDateTotalConsumption(date);
        for (var fi of consumed[date]) {
          this.user.totalConsumption[date] = this.addFoodInfo(
            this.user.totalConsumption[date], 
            fi.foodInfo
          );
        }
      }
    }
  },
  created() {
    this.$root.$on("update-user", userUpdate => {
      Object.assign(this.user, userUpdate);
    });
  }
};
</script>
