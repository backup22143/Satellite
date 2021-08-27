<script setup lang="ts">
const props = defineProps<{
  message: string
}>()
</script>

<template>
  <div class="notification">
    <p class="message">
      {{ props.message }}
    </p>

    <button class="close-button text-black">
      <carbon-close-filled class="text-black" style="font-size: 1.5em;">
      </carbon-close-filled>
    </button>
  </div>
</template>

<style scoped lang="postcss">
.notification {
  @apply flex justify-between items-center bg-light-100 h-16 p-4 rounded-xl;
}

.message {
  @apply text-black font-bold text-xl uppercase;
}
</style>
