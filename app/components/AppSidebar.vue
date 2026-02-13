<script setup lang="ts">
import { Home } from "lucide-vue-next";
import { ROUTES } from "@/tools";
</script>

<template>
  <Sidebar
    class="top-(--header-height) max-h-[calc(100dvh-var(--header-height))] pt-0!"
    variant="inset"
  >
    <Separator class="mb-2" />

    <SidebarHeader>
      <SidebarMenu>
        <SidebarMenuItem>
          <SidebarMenuButton as-child>
            <LazyNuxtLink href="/">
              <Home />
              <span>All Tools</span>
            </LazyNuxtLink>
          </SidebarMenuButton>
        </SidebarMenuItem>
      </SidebarMenu>
    </SidebarHeader>

    <SidebarContent>
      <SidebarGroup>
        <SidebarGroupLabel>Tools</SidebarGroupLabel>
        <SidebarMenuButton as-child v-for="route in ROUTES">
          <LazyNuxtLink :href="route.href">
            <Component :is="route.icon" />
            <span>{{ route.title }}</span>
          </LazyNuxtLink>
        </SidebarMenuButton>
      </SidebarGroup>
    </SidebarContent>
  </Sidebar>
</template>

<style scoped></style>
