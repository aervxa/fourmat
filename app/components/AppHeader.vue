<script setup lang="ts">
import { useSidebar } from "@/components/ui/sidebar";
import { PanelLeft, X } from "lucide-vue-next";

const { toggleSidebar } = useSidebar();
</script>

<template>
  <div class="flex h-(--header-height) shrink-0 justify-between">
    <div class="flex items-center px-2">
      <Button variant="ghost" size="icon-sm" @click="toggleSidebar">
        <PanelLeft />
        <span class="sr-only">Toggle Sidebar</span>
      </Button>
    </div>
    <div class="flex aspect-square items-center justify-center">
      <!-- TODO: Close app -->
      <Button variant="secondary" size="icon-sm" class="size-6 rounded-full">
        <X class="scale-80 stroke-4" />
        <span class="sr-only">Close app</span>
      </Button>
    </div>
  </div>
</template>

<style scoped></style>
