<script setup lang="ts">
import { X, ChevronLeft, ChevronRight, RotateCcw } from "lucide-vue-next";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { isDev } from "~/lib/utils";

const route = useRoute();
const canGoBackward = ref(false);
const canGoForward = ref(false);
watch(
  () => route.fullPath,
  () => {
    canGoBackward.value = !!window.history.state.back;
    canGoForward.value = !!window.history.state.forward;
  },
  { immediate: true },
);

const router = useRouter();
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
        <Button
          variant="ghost"
          size="icon"
          class="size-7"
          :disabled="!canGoBackward"
          @click="router.back()"
        >
          <ChevronLeft class="size-6" />
          <span class="sr-only">Go back</span>
        </Button>
        <Button
          variant="ghost"
          size="icon"
          class="size-7"
          :disabled="!canGoForward"
          @click="router.forward()"
        >
          <ChevronRight class="size-6" />
          <span class="sr-only">Go forward</span>
        </Button>
        <Button
          v-if="isDev"
          variant="destructive"
          size="icon"
          class="size-7"
          @click="router.go(0)"
        >
          <RotateCcw class="size-4.25" />
          <span class="sr-only">Reload</span>
        </Button>
      </div>
    </div>
    <div class="flex aspect-square items-center justify-center">
      <!-- Close app -->
      <Button
        variant="secondary"
        size="icon"
        class="size-6 rounded-full"
        @click="getCurrentWindow().close()"
      >
        <X class="size-3 stroke-4" />
        <span class="sr-only">Close app</span>
      </Button>
    </div>
  </div>
</template>

<style scoped></style>
