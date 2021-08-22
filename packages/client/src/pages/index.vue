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
const items: Item[] = new Array(10).fill({ id: '1', title: 'hello' })
// const items = ref<Item[]>(defaultitems)
const categories = [
  {
    id: 'popular',
    title: 'Popular',
    items,
  },
  {
    id: 'new',
    title: 'New',
    items,
  },
  {
    id: 'movies',
    title: 'Movies',
    items,
  },
  {
    id: 'shows',
    title: 'Shows',
    items,
  },

]
</script>

<template>
  <Hero class="m-0"></Hero>

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
