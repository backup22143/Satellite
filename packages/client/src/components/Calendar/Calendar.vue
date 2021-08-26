<script lang="ts">
export default {
  name: 'Calendar',
}
</script>

<script setup lang="ts">
const month = ref(new Array(31).fill(0))

for (const [index] of month.value.entries())
  month.value[index] = index + 1
</script>

<template>
  <div class="aspect-w-12 aspect-h-8">
    <div class="calendar">
      <div v-for="day in month" :key="day" class="day">
        {{ day }}
      </div>
    </div>
  </div>
</template>

<style scoped lang="postcss">
.calendar {
  @apply grid grid-cols-6 grid-flow-row max-w-5xl max-h-3xl border-1;
}

.day {
  @apply border-1;
}
</style>
