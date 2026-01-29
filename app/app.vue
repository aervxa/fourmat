<template>
  <SidebarProvider style="--sidebar-width: 24rem">

    <!-- Content -->
    <main class="flex-1 grid place-content-center">
      <p class="text-2xl">File upload</p>
    </main>

    <!-- Sidebar -->
    <Sidebar side="right">
      <SidebarHeader>header</SidebarHeader>
      <SidebarContent class="bg-muted grid place-content-center">content</SidebarContent>
      <SidebarFooter>footer</SidebarFooter>
    </Sidebar>

  </SidebarProvider>
</template>
