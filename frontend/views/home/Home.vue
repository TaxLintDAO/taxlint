<template>
  <div class="home-container">
    <Navigator />
    <Login />
  </div>
</template>

<script lang="ts" setup>
import Navigator from "@/components/navigator/Navigator.vue"
import Login from "./modules/Login.vue"
</script>

<style lang="scss">
.home-container {
  /*background-color: rgb(246, 246, 246);*/
  min-height: 100vh;
  /*color: black;*/
  /*width: 100%;*/
  /*height: 100%;*/
}
</style>
