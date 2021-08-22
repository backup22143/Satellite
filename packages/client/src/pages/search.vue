<script setup lang="ts">
type Item = {
  id: string
  title: string
}

const items: Item[] = new Array(10).fill({ title: 'Lorem Ipsum' })
const itemsWithIds = items.map((item, index) => ({ ...item, id: index.toString() }))

const categories = [
  {
    id: 'genres',
    title: 'Genres',
    items: itemsWithIds,
  },
  {
    id: 'movies',
    title: 'Movies',
    items: itemsWithIds,
  },
  {
    id: 'shows',
    title: 'Shows',
    items: itemsWithIds,
  },

]

const searchTerms = ['Parasite', 'Lorem', 'Ipsum', 'Ding Dong']
</script>

<template>
  <div class="page">
    <div class="px-4 mb-10">
      <SearchBar class="w-100 mx-4"></SearchBar>
      <div class="p-4 flex justify-center items-center gap-2">
        <button v-for="(term, index) in searchTerms" :key="index" class="chip">
          {{ term }}
        </button>
      </div>
    </div>
    <CategorySwiper v-for="category in categories" :key="category.id" :title="category.title" :items="category.items" show-more></CategorySwiper>
  </div>
</template>

<style scoped lang="postcss">
.chip {
  @apply rounded-2xl shadow bg-green-500 px-4 py-1 text-sm font-semibold hover:bg-green-300;
}
</style>
