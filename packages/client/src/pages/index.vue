<script setup lang="ts">
// import { useRouter } from 'vue-router'
// import { useI18n } from 'vue-i18n'

// import MasonryItem from '~/components/Masonry/Item.vue'
// import MasonryGrid from '~/components/Masonry/Grid.vue'
// import { useUserStore } from '~/stores/user'
type Item = {
  id: string
  title: string
}
// const user = useUserStore()
// const name = ref(user.savedName)

// const router = useRouter()
// const go = () => {
//   if (name.value)
//     router.push(`/hi/${encodeURIComponent(name.value)}`)
// }

// const { t } = useI18n()
const items: Item[] = new Array(10).fill({ title: 'Lorem Ipsum' })
const itemsWithIds = items.map((item, index) => ({ ...item, id: index.toString() }))
// const items = ref<Item[]>(defaultitems)
const categories = [
  {
    id: 'popular',
    title: 'Popular',
    items: itemsWithIds,
  },
  {
    id: 'new',
    title: 'New',
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
</script>

<template>
  <Hero class="m-0 mx-4"></Hero>

  <div class="page pt-10">
    <CategorySwiper v-for="category in categories" :key="category.id" :title="category.title" :items="category.items" show-more></CategorySwiper>
    <!-- <MasonryGrid :columns="3" :rows="3">
      <MasonryItem v-for="i of 24" :key="i"></MasonryItem>
    </MasonryGrid> -->
  </div>
</template>

<route lang="yaml">
meta:
  layout: home
</route>
