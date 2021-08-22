<script setup lang="ts">
type Item = {
  id: string
  title: string
}

const props = defineProps<{
  items: Item[]
  title: string
  showMore?: boolean
}>()
</script>
<template>
  <Label :title="title" :show-more="showMore" />
  <Swiper :items="props.items" class="mb-6 h-42 px-4" />
</template>

<style scoped lang="postcss">
.container {
  @apply pb-2 flex justify-between items-end;
}

.heading {
  @apply text-lg text-left font-bold;
}

.view-more {
  @apply text-xs font-semibold text-gray-300 cursor-pointer hover:text-gray-100 uppercase;
}
</style>
