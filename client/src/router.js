import Vue from "vue";
import Router from "vue-router";
import Home from "./views/Home.vue";
import Status from "./views/Status.vue";
import Submit from "./views/Submit.vue";
import Login from "./views/Login.vue";

Vue.use(Router);

export default new Router({
  routes: [
    {
      path: "/",
      name: "home",
      component: Home
    },
    {
      path: "/status",
      name: "status",
      component: Status
    },
    {
      path: "/submit",
      name: "submit",
      component: Submit
    },
    {
      path: "/login",
      name: "login",
      component: Login
    }
  ]
});
