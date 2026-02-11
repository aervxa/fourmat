<script setup lang="ts">
import { Image } from "lucide-vue-next";

const ROUTES = [
  {
    title: "Image Converter",
    href: "/image-converter",
    icon: Image,
  },
];
</script>

<template>
  <main class="flex size-full flex-col">
    <p class="px-8 pt-6 text-center text-3xl font-bold tracking-wide">
      Fourmat Tools
    </p>
    <!-- Credits lol -->
    <p
      class="mt-1 px-4 pb-3 text-center font-mono text-xs tracking-wider opacity-40"
      :style="{
        wordSpacing: '2px',
      }"
    >
      Built by
      <!-- TODO Open link in native browser -->
      <a href="https://www.github.com/aervxa" target="_blank">zorvi</a>
    </p>
    <!-- Grid of all tools -->
    <div
      class="grid flex-1 auto-rows-min grid-cols-[repeat(auto-fill,minmax(256px,1fr))] gap-4 p-8"
    >
      <LazyNuxtLink
        v-for="route in ROUTES"
        :href="route.href"
        class="flex aspect-square size-full flex-col gap-4 rounded-2xl border-2 p-4"
      >
        <Component :is="route.icon" class="size-full p-4 opacity-40" />
        <p class="text-center text-2xl font-semibold">{{ route.title }}</p>
      </LazyNuxtLink>
    </div>
  </main>
</template>
