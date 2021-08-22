<script setup lang="ts">
import { useWindowSize } from '@vueuse/core'
import SwiperCore, { Navigation } from 'swiper/core'
import { Swiper as VSwiper, SwiperSlide } from 'swiper/vue'
import type { Swiper as SwiperType } from 'swiper'
import 'swiper/swiper.min.css'
import MasonryItem from '~/components/Masonry/Item.vue'
import 'swiper/components/navigation/navigation.min.css'

const { width } = useWindowSize()

SwiperCore.use([Navigation])

type Item = {
  id: string
  title: string
}

const props = defineProps<{
  items: Item[]
}>()

const onSwiper = (swiper: SwiperType) => {
  console.log(swiper)
}
const onSlideChange = () => {
  console.log('slide change')
}

const slidesPerView = computed(() => {
  if (width.value < 747)
    return 2

  if (width.value >= 747 && width.value <= 1100)
    return 3

  if (width.value > 1100 && width.value <= 1480)
    return 4

  if (width.value > 1480 && width.value <= 1700)
    return 5

  if (width.value > 1700 && width.value <= 1900)
    return 6

  if (width.value > 1900)
    return 7
})

const options = {
  navigation: {
    prevEl: '.prevEl',
    nextEl: '.nextEl',
  },
}
</script>

<template>
  <div class="relative overflow-hidden">
    <VSwiper
      class="relative h-full overflow-visible"
      :slides-per-view="slidesPerView"
      :space-between="16"
      :slides-per-group="slidesPerView"
      :navigation="options.navigation"
      :hash-navigation="{
        'watchState': true
      }"
      :style="{'--swiper-navigation-color': '#fff'}"
      @swiper="onSwiper"
      @slideChange="onSlideChange"
    >
      <SwiperSlide v-for="item in props.items" :key="item.id" :data-hash="item.id">
        <MasonryItem class="h-full">
        </MasonryItem>
      </SwiperSlide>

      <div class="prevEl -left-4 button-container group">
        <button class="button">
          <carbon-arrow-left style="font-size: 1.5rem;" />
        </button>
      </div>

      <div class="nextEl -right-4 button-container group">
        <button class="button">
          <carbon-arrow-right style="font-size: 1.5rem;" />
        </button>
      </div>
    </VSwiper>
  </div>
</template>

<style scoped lang="postcss">
.button-container {
  @apply absolute top-0 h-full w-10 z-10 flex justify-center items-center cursor-pointer;
}

.button {
  background-color: #121212;
  @apply h-10 w-10 shadow rounded-full flex justify-center items-center group-hover:bg-gray-600 outline-none;
  transition: background-color 0.2s;
}
</style>
