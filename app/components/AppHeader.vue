<script setup lang="ts">
import { useSidebar } from "~/components/ui/sidebar";
import { PanelLeft, X } from "lucide-vue-next";

const { toggleSidebar } = useSidebar();
</script>

<template>
  <div class="flex h-(--header-height) shrink-0 justify-between px-2">
    <div class="flex items-center">
      <Button variant="ghost" size="icon-sm" @click="toggleSidebar">
        <PanelLeft />
        <span class="sr-only">Toggle Sidebar</span>
      </Button>
    </div>
    <div class="flex items-center">
      <!-- TODO: Close app -->
      <Button variant="ghost" size="icon-sm">
        <X />
        <span class="sr-only">Close app</span>
      </Button>
    </div>
  </div>
</template>

<style scoped></style>
