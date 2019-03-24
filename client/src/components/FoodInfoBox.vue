<template>
  <div class="food-infobox">
    <template v-if="!input">
      <div class="long-desc">
        {{ foodInfo.long_desc }}
        <div v-if="mutateByGrams > 0.0" class="grams">({{ mutateByGrams }} g)</div>
      </div>
      <div class="short-desc">{{ foodInfo.shrt_desc }}</div>
      <div class="group">{{ foodInfo.fd_grp_desc }}</div>
      <div class="record">
        <div class="record-type">Protein</div>
        <div class="record-value">{{ foodInfo.protein_g }} g</div>
      </div>
      <div class="record">
        <div class="record-type">Fat</div>
        <div class="record-value">{{ foodInfo.fat_g }} g</div>
      </div>
      <div class="record">
        <div class="record-type">Carbohydrates</div>
        <div class="record-value">{{ foodInfo.carbs_g }} g</div>
      </div>
      <div class="record">
        <div class="record-type">Energy</div>
        <div class="record-value">{{ foodInfo.energy_kcal }} kcal</div>
      </div>
      <div class="record">
        <div class="record-type">Water</div>
        <div class="record-value">{{ foodInfo.water_g }} g</div>
      </div>
      <div class="record">
        <div class="record-type">Sugars</div>
        <div class="record-value">{{ foodInfo.sugars_g }} g</div>
      </div>
    </template>
    <template v-else>
      <div class="long-desc">
        <input 
          type="text" 
          v-model="foodInfoInput.long_desc" 
          placeholder="Food Long Description"/>
        <div class="grams">
          (<input type="number" v-model="foodInfoInput.inputGrams"> g)
        </div>
      </div>
      <input 
        type="text"
        v-model="foodInfoInput.shrt_desc"
        class="short-desc"
        placeholder="FOOD SHORT DESCRIPTION"/>
      <input 
        type="text"
        v-model="foodInfoInput.fd_grp_desc"
        class="group"
        placeholder="Food Group"/>
      <div class="record">
        <div class="record-type">Protein</div>
        <div class="record-value">
          <input type="number" v-model="foodInfoInput.protein_g"/> g
        </div>
      </div>
      <div class="record">
        <div class="record-type">Fat</div>
        <div class="record-value">
          <input type="number" v-model="foodInfoInput.fat_g"/> g
        </div>
      </div>
      <div class="record">
        <div class="record-type">Carbohydrates</div>
        <div class="record-value">
          <input type="number" v-model="foodInfoInput.carbs_g"/> g
        </div>
      </div>
      <div class="record">
        <div class="record-type">Energy</div>
        <div class="record-value">
          <input type="number" v-model="foodInfoInput.energy_kcal"/> g
        </div>
      </div>
      <div class="record">
        <div class="record-type">Water</div>
        <div class="record-value">
          <input type="number" v-model="foodInfoInput.water_g"/> g
        </div>
      </div>
      <div class="record">
        <div class="record-type">Sugars</div>
        <div class="record-value">
          <input type="number" v-model="foodInfoInput.sugars_g"/> g
        </div>
      </div>
    </template>
    <form v-if="showAddFood" v-on:submit.prevent="addFood">
      <div class="input-group">
        <label for="food-quantity">Grams</label>
        <input type="number" name="quantity" id="food-quantity" v-model="grams">
      </div>
      <input type="submit" value="Add Food">
    </form>
  </div>
</template>

<script>
import uuidv4 from "uuid/v4";
export default {
  name: "FoodInfoBox",
  data() {
    return {
      grams: 100.0,
      foodInfoInput: {
        long_desc: "",
        shrt_desc: "",
        fd_grp_desc: "",
        protein_g: 0.0,
        fat_g: 0.0,
        carbs_g: 0.0,
        energy_kcal: 0.0,
        water_g: 0.0,
        sugars_g: 0.0,
        inputGrams: 100
      }
    };
  },
  props: {
    foodInfoProp: {
      type: Object,
      required: false
    },
    mutateByGrams: {
      type: Number,
      required: false,
      default: 0.0
    },
    showAddFood: {
      type: Boolean,
      required: false,
      default: false
    },
    user: {
      type: Object,
      required: true
    },
    dateToAdd: {
      type: Date,
      required: false,
      default: () => new Date()
    },
  },
  computed: {
    foodInfo() {
      if (this.input) {
        return this.$root.$children[0].gramExpand(
          this.foodInfoInput,
          this.grams,
          this.deepInputGrams
        );
      }
      if (typeof this.foodInfoProp.shrt_desc === "undefined") {
        return {
          long_desc: "Unknown Food Item",
          shrt_desc: "UNKNOWN",
          fd_grp_desc: "Unknown Food Group",
          protein_g: 0.0,
          fat_g: 0.0,
          carbs_g: 0.0,
          energy_kcal: 0.0,
          water_g: 0.0,
          sugars_g: 0.0
        };
      }
      if (this.mutateByGrams > 0.0) {
        return this.$root.$children[0].gramExpand(
          this.foodInfoProp,
          this.mutateByGrams
        );
      } else {
        return this.foodInfoProp;
      }
    },
    input() {
      return typeof this.foodInfoProp === 'undefined';
    },
    deepInputGrams() {
      return this.foodInfoInput.inputGrams;
    }
  },
  methods: {
    addFood() {
      let date = this.$root.$children[0].getDateNumber(this.dateToAdd);
      let newAssignment = Object.assign({}, this.user.consumed);
      if (typeof newAssignment[date] === "undefined") {
        newAssignment[date] = [];
      }
      newAssignment[date].push({
        foodInfo: this.foodInfo,
        grams: 1.0 * this.grams,
        uuid: uuidv4()
      });
      this.$root.$emit("update-user", {
        consumed: Object.assign({}, this.user.consumed, newAssignment)
      });``
    }
  },
  watch: {
    deepInputGrams(newValue, oldValue) {
      if (this.grams == oldValue) {
        this.grams = newValue;
      }
    }
  }
};
</script>
