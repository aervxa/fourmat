<script setup lang="ts">
import { X, ChevronLeft, ChevronRight } from "lucide-vue-next";
</script>

<template>
  <div class="flex h-(--header-height) shrink-0 justify-between">
    <div class="flex items-center">
      <div class="flex aspect-square h-full items-center justify-center">
        <SidebarTrigger />
      </div>

      <Separator orientation="vertical" class="max-h-1/2 opacity-80" />

      <div class="ml-2 flex gap-1">
        <!-- Forward and Back -->
        <!-- TODO: Get it to actually work -->
        <Button variant="ghost" size="icon" class="size-7">
          <ChevronLeft class="size-6" />
          <span class="sr-only">Go back</span>
        </Button>
        <Button variant="ghost" size="icon" class="size-7">
          <ChevronRight class="size-6" />
          <span class="sr-only">Go forward</span>
        </Button>
      </div>
    </div>
    <div class="flex aspect-square items-center justify-center">
      <!-- TODO: Close app -->
      <Button variant="secondary" size="icon" class="size-6 rounded-full">
        <X class="size-3 stroke-4" />
        <span class="sr-only">Close app</span>
      </Button>
    </div>
  </div>
</template>

<style scoped></style>
