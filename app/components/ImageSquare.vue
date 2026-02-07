<script setup lang="ts">
import { Trash2 } from "lucide-vue-next";

const props = defineProps({
  deleteFn: Function,
  src: String,
  alt: String,
});
</script>

<template>
  <div
    class="group relative aspect-square overflow-clip rounded-xl bg-cover bg-center"
    :style="{
      backgroundImage: `url(${src})`,
    }"
  >
    <!-- Overlay to blur the background of the container -->
    <div class="absolute inset-0 backdrop-blur-sm backdrop-brightness-60"></div>
    <img
      :src
      :alt
      class="relative size-full object-contain"
      draggable="false"
    />
    <!-- Delete button -->
    <Button
      variant="destructive"
      size="icon"
      class="animate-in fade-in-30 absolute top-2 right-2 hidden backdrop-blur-sm group-hover:flex"
      @click="props.deleteFn?.()"
    >
      <Trash2 />
    </Button>
  </div>
</template>
