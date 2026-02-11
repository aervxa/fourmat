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
  <!-- Grid of all tools -->
  <main
    class="grid size-full auto-rows-min grid-cols-[repeat(auto-fill,minmax(256px,1fr))] gap-4 p-8"
  >
    <a
      v-for="route in ROUTES"
      :href="route.href"
      class="flex aspect-square size-full flex-col gap-4 rounded-2xl border-2 p-4"
    >
      <Component :is="route.icon" class="size-full p-4 opacity-40" />
      <p class="text-center text-2xl font-semibold">{{ route.title }}</p>
    </a>
  </main>
</template>
