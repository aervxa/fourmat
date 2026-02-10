<script setup lang="ts">
import type { ButtonVariants } from "~/components/ui/button";

defineProps({
  src: String,
  alt: String,
  action: Function,
  actionIcon: Function,
  actionVariant: String as PropType<ButtonVariants["variant"]>,
});
</script>

<template>
  <div
    class="group relative aspect-square overflow-clip rounded-xl bg-cover bg-center"
    :style="{
      backgroundImage: `url(&quot;${src}&quot;)`,
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
    <!-- Action button -->
    <Button
      v-if="actionIcon"
      :variant="actionVariant"
      size="icon"
      class="animate-in fade-in-30 absolute top-2 right-2 backdrop-blur-xs not-[:where(.group):hover_*]:hidden"
      @click.stop="action?.()"
    >
      <Component :is="actionIcon" />
    </Button>
  </div>
</template>
