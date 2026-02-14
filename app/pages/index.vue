<script setup lang="ts">
import { openUrl } from "@tauri-apps/plugin-opener";
import { motion } from "motion-v";
import { ROUTES } from "@/tools";
</script>

<template>
  <main class="flex size-full flex-col overflow-auto">
    <motion.p
      :initial="{
        y: '-100%',
      }"
      :animate="{
        y: 0,
      }"
      class="px-8 pt-6 text-center text-3xl font-bold tracking-wide"
    >
      Fourmat Tools
    </motion.p>
    <!-- Credits lol -->
    <motion.p
      :initial="{
        opacity: 0,
      }"
      :animate="{
        opacity: 1,
      }"
      class="mt-1 px-4 pb-3 text-center font-mono text-xs tracking-wider opacity-40"
      :style="{
        wordSpacing: '2px',
      }"
    >
      Built by
      <Button
        variant="link"
        class="-ml-1 px-2"
        @click="openUrl('https://github.com/aervxa')"
      >
        zorvi
      </Button>
    </motion.p>
    <!-- Grid of all tools -->
    <div
      class="grid flex-1 auto-rows-min grid-cols-[repeat(auto-fill,minmax(192px,1fr))] justify-center gap-4 p-8 sm:grid-cols-[repeat(auto-fill,minmax(224px,1fr))]"
    >
      <LazyNuxtLink v-for="route in ROUTES" :href="route.href" class="contents">
        <motion.div
          :initial="{
            scale: 0.9,
          }"
          :animate="{
            scale: 1,
          }"
          class="flex aspect-square size-full flex-col gap-4 rounded-2xl border-2 p-4"
        >
          <Component :is="route.icon" class="size-full p-1 opacity-40 sm:p-2" />
          <p class="text-center text-lg font-semibold sm:text-xl">
            {{ route.title }}
          </p>
        </motion.div>
      </LazyNuxtLink>
    </div>
  </main>
</template>
