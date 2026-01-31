<script setup lang="ts">
const dropZone = useTemplateRef("dropZone");
const imageSource = ref("");
const inputSelect = useTemplateRef("inputSelect");

// Upload file handler
function upload(files: File[] | FileList | null) {
  // Assert existence of just one file
  if (files && files.length == 1) {
    const file = files[0];
    // Assert file is of type image
    if (file?.type.startsWith("image/")) {
      imageSource.value = URL.createObjectURL(file);
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
  <!-- Content -->
  <main class="flex size-full flex-1 overflow-hidden">
    <!-- Image Selection/Viewer -->
    <div class="flex-1 p-4">
      <img
        v-if="imageSource"
        :src="imageSource"
        alt="uploaded image"
        class="bg-muted size-full rounded-xl object-contain"
      />
      <!-- Upload region -->
      <label
        v-else
        ref="dropZone"
        class="bg-muted/60 hover:border-primary/40 hover:bg-muted/80 flex size-full cursor-pointer flex-col items-center-safe justify-center-safe rounded-xl border-2 border-dashed"
        :class="[isOverDropZone && 'border-primary/40 bg-muted/80!']"
      >
        Drag a file
        <input
          v-show="false"
          ref="inputSelect"
          type="file"
          accept="image/*"
          @change="upload(($event.currentTarget as HTMLInputElement).files)"
        />
      </label>
    </div>

    <!-- Options "Sidebar" -->
    <div class="flex w-1/4 flex-col gap-6 border-l p-4">
      <p class="text-2xl font-bold">Image type converter</p>

      <div class="flex flex-col gap-2">
        <p class="text-xl font-semibold">Step 1</p>
        <Button class="self-start" @click="inputSelect?.click()">
          Select Image
        </Button>
      </div>
    </div>
  </main>
</template>
