<script setup lang="ts">
const props = defineProps<{
  columns: number
  rows: number
}>()
</script>

<template>
  <div :class="`grid grid-rows-${props.rows} grid-cols-${props.columns} gap-4`">
    <slot></slot>
  </div>
</template>
