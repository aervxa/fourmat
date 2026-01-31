<script setup lang="ts">
const dropZone = useTemplateRef("dropZone");

// Upload file handler
function upload(file: File) {
  /* on file receival */
  console.log(file);
}

// Select file handler
function select(event: Event) {
  // Extract files from target
  const files = (event.currentTarget as HTMLInputElement).files;
  if (files && files.length == 1) {
    // SAFETY: files is File[], TypeScript isn't smart enough to know of the .length check and indexing number
    const file = files[0]!;
    if (file.type.startsWith("image/")) {
      upload(file);
    } else {
      alert("Incorrect file type, please select an image file.");
    }
  } else {
    alert("Couldn't receive file via select, please try again.");
  }
}

// Drop zone init
const { isOverDropZone } = useDropZone(dropZone, {
  dataTypes: ["image"],
  multiple: false,
  onDrop: (files) => {
    if (files && files.length == 1) {
      // SAFETY: files is File[], TypeScript isn't smart enough to know of the .length check and indexing number
      upload(files[0]!);
    } else {
      alert("Couldn't receive file via drag, please try selecting it.");
    }
  },
});
</script>

<template>
  <SidebarProvider style="--sidebar-width: 24rem">
    <!-- Content -->
    <main class="flex-1 p-4">
      <!-- Upload region -->
      <label
        ref="dropZone"
        class="bg-muted/60 hover:border-primary/40 hover:bg-muted/80 flex size-full cursor-pointer flex-col items-center-safe justify-center-safe rounded-xl border-2 border-dashed"
        :class="[isOverDropZone && 'border-primary/40 bg-muted/80!']"
      >
        Drag a file
        <input v-show="false" type="file" accept="image/*" @change="select" />
      </label>
    </main>

    <!-- Sidebar -->
    <Sidebar side="right">
      <SidebarHeader>header</SidebarHeader>
      <SidebarContent class="bg-muted grid place-content-center">
        content
      </SidebarContent>
      <SidebarFooter>footer</SidebarFooter>
    </Sidebar>
  </SidebarProvider>
</template>
