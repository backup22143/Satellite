<script setup lang="ts">
import SwiperCore, { Navigation } from 'swiper/core'
import { Swiper, SwiperSlide } from 'swiper/vue'
import type { Swiper as SwiperType } from 'swiper'
import 'swiper/swiper.min.css'
import MasonryItem from '~/components/Masonry/Item.vue'
import 'swiper/components/navigation/navigation.min.css'
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
</script>

<template>
  <Swiper
    :slides-per-view="4"
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
      <MasonryItem />
    </SwiperSlide>

    <!-- <div class="swiper-button-prev">
      Yes
    </div>
    <div class="swiper-button-next">
      No
    </div> -->
  </Swiper>
</template>
