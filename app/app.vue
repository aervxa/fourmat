<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";

onMounted(async () => {
  await invoke("show_window");
});
</script>

<template>
  <NuxtLayout>
    <NuxtPage />
  </NuxtLayout>
</template>
