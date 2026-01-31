<script setup lang="ts">
const dropZone = useTemplateRef("dropZone");

// Upload file handler
function upload(files: File[] | FileList | null) {
  // Assert existence of just one file
  if (files && files.length == 1) {
    const file = files[0];
    // Assert file is of type image
    if (file?.type.startsWith("image/")) {
      // TODO File is now validated.
    } else {
      alert("Incorrect file type.");
    }
  } else {
    alert("Upload failed.");
    // `files` will be an Array if dropped, or a FileList if selected.
    console.error(
      "Couldn't receive file from ",
      files instanceof Array
        ? "drop"
        : files instanceof FileList
          ? "select"
          : "_", // _ would mean files is null
    );
  }
}

// Drop zone init
const { isOverDropZone } = useDropZone(dropZone, {
  dataTypes: ["image"],
  multiple: false,
  onDrop: upload,
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
        <input
          v-show="false"
          type="file"
          accept="image/*"
          @change="upload(($event.currentTarget as HTMLInputElement).files)"
        />
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
