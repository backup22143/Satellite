<script setup lang="ts">
const props = defineProps<{
  title: string
  showMore?: boolean
}>()
</script>

<template>
  <div :class="$style.container">
    <h2 :class="$style.heading">
      {{ props.title }}
    </h2>

    <span v-show="showMore" :class="$style.external">
      Show More
    </span>
  </div>
</template>

<style module lang="postcss">
.container {
  @apply pb-2 px-4 flex justify-between items-end;
}

.heading {
  @apply text-lg text-left font-bold;
}

.external {
  @apply text-xs text-right font-semibold text-gray-400 cursor-pointer hover:text-gray-100 uppercase;
}
</style>
