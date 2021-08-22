<script setup lang="ts">
const downloads = [
  {
    id: '1',
    title: 'Parasite',
    description: '',
    time: 241,
  },
]
</script>

<template>
  <div>
    <a href="/">Queue</a>
    <a href="/">History</a>
    <a href="/">Queue</a>

    <div class="download-list">
      <div v-for="download in downloads" :key="download.id" class="download-item">
        <div class="pr-4">
          <h4 class="font-bold text-xl">
            {{ download.title }}
          </h4>
          <p class="font-semibold">
            {{ download.description }}
          </p>
        </div>

        <div class="flex justify-start items-center">
          <div class="progress-bar-container">
            <div class="progress-bar"></div>
          </div>

          <span class="time">{{ download.time }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped lang="postcss">
.download-list {
  @apply w-120 mx-auto;
}

.download-item {
  @apply flex justify-between items-start;
}

.progress-bar-container {
  @apply bg-gray-700 h-2 w-42 rounded-lg overflow-hidden;
}

.progress-bar {
  @apply bg-green-200 h-full w-23 rounded-lg;
}

.time {
  @apply pl-4 text-sm font-semibold;
}
</style>
