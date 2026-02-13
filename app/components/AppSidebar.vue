<script setup lang="ts">
import { Home } from "lucide-vue-next";
import { ROUTES } from "@/tools";
</script>

<template>
  <Sidebar
    class="top-(--header-height) max-h-[calc(100dvh-var(--header-height))] px-0! pt-0!"
    variant="inset"
    collapsible="icon"
  >
    <Separator
      class="max-w-[calc(100%-(var(--spacing)*4))] self-center opacity-80"
    />

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
    <Separator
      class="max-w-[calc(100%-(var(--spacing)*4))] self-center opacity-80"
    />

    <SidebarContent>
      <SidebarGroup>
        <SidebarGroupLabel>Tools</SidebarGroupLabel>
        <SidebarMenu>
          <SidebarMenuItem v-for="route in ROUTES">
            <SidebarMenuButton as-child>
              <LazyNuxtLink :href="route.href">
                <Component :is="route.icon" />
                <span>{{ route.title }}</span>
              </LazyNuxtLink>
            </SidebarMenuButton>
          </SidebarMenuItem>
        </SidebarMenu>
      </SidebarGroup>
    </SidebarContent>
  </Sidebar>
</template>

<style scoped></style>
