<template>
  <div class="login">
    <form v-on:submit.prevent="login">
      <input 
        type="text" 
        name="username" 
        id="username" 
        v-model="username" 
        placeholder="Username"/>
      <input 
        type="password" 
        name="password" 
        id="password" 
        placeholder="Password"/>
      <input type="submit" value="Log In" />
    </form>
    <hr />
    <p>
      Registration and accounts not implemented on markup. Registration
      interface would otherwise go here. Login can be performed with any
      username and all passwords are automatically accepted. User account
      information is lost on page reload.
    </p>
  </div>
</template>

<script>
export default {
  name: "Login",
  props: {
    user: {
      type: Object,
      required: true
    }
  },
  data() {
    return {
      username: ""
    };
  },
  methods: {
    login() {
      this.$root.$emit("update-user", {
        name: this.name
      });
      this.$router.push({ name: "status" });
    }
  },
  created() {
    if (this.$root.$children[0].isLoggedIn) {
      this.$router.push({ name: "status" });
    }
  }
};
</script>
