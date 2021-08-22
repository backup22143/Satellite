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

</script>

<template>
  <VSwiper
    :slides-per-view="slidesPerView"
    :space-between="16"
    navigation
    :hash-navigation="{
      'watchState': true
    }"
    :style="{'--swiper-navigation-color': '#fff'}"
    @swiper="onSwiper"
    @slideChange="onSlideChange"
  >
    <SwiperSlide v-for="item in props.items" :key="item.id" :data-hash="item.id">
      <MasonryItem class="h-full" />
    </SwiperSlide>

    <div class="swiper-button-prev">
      <button>
        <carbon-arrow-left />
      </button>
    </div>
    <div class="swiper-button-next">
      <button>
        <carbon-arrow-right />
      </button>
    </div>
  </VSwiper>
</template>
