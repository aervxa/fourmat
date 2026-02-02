<script setup lang="ts">
import { Image, Cpu } from "lucide-vue-next";

const dropZone = useTemplateRef("dropZone");
const imageSource = ref("");
const inputSelect = useTemplateRef("inputSelect");

function save() {
  // TODO Save file top output dir
}

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
        class="bg-muted/60 hover:border-primary/40 hover:bg-muted/80 relative flex size-full cursor-pointer flex-col items-center-safe justify-center-safe gap-4 rounded-xl border-2 border-dashed p-4"
        :class="[isOverDropZone && 'border-primary/40 bg-muted/80!']"
      >
        <Image :size="64" />
        <span class="text-2xl font-semibold">Drag and drop any image</span>
        <span
          class="absolute bottom-4 max-w-prose text-center font-mono text-sm leading-relaxed opacity-40"
        >
          Supported files: AVIF, BMP, DDS, OpenEXR, Farbfeld, GIF, HDR, ICO,
          JPEG, PNG, PNM, QOI, TGA, TIFF, WebP
        </span>
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

      <!-- Step 1: Select Image -->
      <div class="flex flex-col gap-2">
        <p class="text-xl font-semibold">Step 1</p>
        <Button
          class="self-start"
          variant="secondary"
          @click="inputSelect?.click()"
        >
          Select Image
        </Button>
      </div>

      <!-- Step 2: Select Format -->
      <div class="flex flex-col gap-2">
        <p class="text-xl font-semibold">Step 2</p>
        <p class="text-sm">Select format to convert to</p>
        <Select>
          <SelectTrigger>
            <SelectValue placeholder="Select Format" />
          </SelectTrigger>
          <SelectContent>
            <SelectGroup>
              <SelectLabel>Formats</SelectLabel>
              <SelectItem value="jpg">JPG</SelectItem>
              <SelectItem value="png">PNG</SelectItem>
              <SelectItem value="webp">WEBP</SelectItem>
            </SelectGroup>
          </SelectContent>
        </Select>
      </div>

      <!-- Step 3: Select Output Directory -->
      <div class="flex flex-col gap-2">
        <p class="text-xl font-semibold">Step 3</p>
        <p class="text-sm">Defaults to image's path</p>
        <!-- TODO Select input for selecting folder -->
      </div>

      <!-- Step 4: Fourmat (Format and Save) -->
      <div class="flex flex-col gap-2">
        <p class="text-xl font-semibold">Step 4</p>
        <Button class="self-start" @click="save">
          <Cpu />
          Fourmat
        </Button>
      </div>
    </div>
  </main>
</template>
