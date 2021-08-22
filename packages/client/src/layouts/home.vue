<script setup lang="ts">
// import Header from '~/components/Header/Header.vue'
</script>

<template>
  <main class="px-4 pb-10 text-center text-gray-700 dark:text-gray-200">
    <Header />
    <router-view />
    <Footer class="mt-10" />
    <div class="mt-5 mx-auto text-center opacity-25 text-sm">
      [Home Layout]
    </div>
  </main>
</template>
