<script setup lang="ts">
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { useUserStore } from '~/stores/user'

const user = useUserStore()
const name = ref(user.savedName)

const router = useRouter()
const go = () => {
  if (name.value)
    router.push(`/hi/${encodeURIComponent(name.value)}`)
}

const { t } = useI18n()
</script>

<template>
  <input
    id="input"
    v-model="name"
    :placeholder="t('search.placeholder')"
    :aria-label="t('search.placeholder')"
    type="text"
    autocomplete="false"
    p="x-4 y-2"
    w="full"
    text="center"
    bg="transparent"
    border="~ rounded gray-200 dark:gray-700"
    outline="none active:none"
    @keydown.enter="go"
  >
  <label class="hidden" for="input">{{ t('search.placeholder') }}</label>
</template>
