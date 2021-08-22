<script setup lang="ts">

</script>

<template>
  <header class="pt-2 pb-4 mx-4 flex justify-between items-center">
    <h1 class="text-xl font-bold text-left flex justify-between cursor-pointer pr-32">
      <carbon-satellite style="font-size: 1.75em;" />
    </h1>
    <div>
      <a href="/" class="text-sm font-bold rounded p-4 hover:bg-gray-800">MOVIES</a>
      <a href="/" class="text-sm font-bold rounded p-4 hover:bg-gray-800">SHOWS</a>
      <a href="/" class="text-sm font-bold rounded p-4 hover:bg-gray-800">SCHEDULE</a>
      <a href="/" class="text-sm font-bold rounded p-4 hover:bg-gray-800">COLLECTION</a>
    </div>

    <div>
      <div class="hover:bg-gray-800 rounded-full p-3 icon-btn">
        <a href="/search">
          <carbon-search style="font-size: 1.75em;" />
        </a>
      </div>

      <div class="hover:bg-gray-800 rounded-full p-3 icon-btn relative">
        <carbon-download style="font-size: 1.75em;" />

        <div class="notification bg-green-400">
          <span>6</span>
        </div>
      </div>

      <div class="hover:bg-gray-800 rounded-full p-3 icon-btn relative">
        <carbon-settings style="font-size: 1.75em;" />
        <div class="notification bg-red-500">
          <span>6</span>
        </div>
      </div>
    </div>
  </header>
</template>

<style scoped lang="postcss">
.notification {
  @apply flex justify-center items-center absolute top-1 right-1 w-5 h-5 text-sm text-white font-bold rounded-full;
}
</style>
