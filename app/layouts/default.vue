<script setup lang="ts"></script>

<template>
  <SidebarProvider
    class="flex max-h-dvh flex-col [--header-height:calc(--spacing(10))]"
  >
    <!-- Header -->
    <AppHeader />

    <div class="flex flex-1 overflow-hidden">
      <AppSidebar />

      <SidebarInset class="border-border/40 mt-0! border">
        <slot />
      </SidebarInset>
    </div>
  </SidebarProvider>
</template>

<style scoped></style>
