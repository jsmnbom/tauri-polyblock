<template>
  <div id="app">
    <h4 class="row text-center">Node: {{ nodeMsg }}</h4>
    <h3 class="row text-center">{{ msg }}</h3>
    <div class="row">
      <button @click="eventToRust()">SEND MSG</button>
    </div>
  </div>
</template>

<script>
import tauri from "tauri/api";

export default {
  name: "App",
  data() {
    return {
      msg: "waiting for rust",
      nodeMsg: "waiting"
    };
  },
  mounted() {
    tauri.listen("reply", res => {
      console.log(JSON.stringify(res));
      this.msg = res.payload.msg;
    });
  },
  methods: {
    // set up an event listener
    eventToRust() {
      tauri.emit("myCustomCommand", JSON.stringify({msg: "hi"}));
    }
  }
};
</script>

<style>
#app {
  font-family: "Avenir", Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
}

#nav {
  padding: 30px;
}

#nav a {
  font-weight: bold;
  color: #2c3e50;
}

#nav a.router-link-exact-active {
  color: #42b983;
}
</style>
