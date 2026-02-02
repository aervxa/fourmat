<script setup lang="ts">
import { Image, Cpu } from "lucide-vue-next";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import type { UnlistenFn } from "@tauri-apps/api/event";
import { open } from "@tauri-apps/plugin-dialog";
import { convertFileSrc } from "@tauri-apps/api/core";

const dragActive = ref(false);
let unlistenDragDropEvent: UnlistenFn;

onMounted(async () => {
  unlistenDragDropEvent = await getCurrentWebviewWindow().onDragDropEvent(
    (event) => {
      // Keep bool true during enter/over (resets on drop/leave)
      dragActive.value =
        event.payload.type == "enter" || event.payload.type == "over";

      // File(s) dropped
      if (event.payload.type == "drop") {
        // TODO file dropped
        console.log(event.payload.paths);
      }
    },
  );
});
onUnmounted(async () => {
  unlistenDragDropEvent();
});

function save() {
  // TODO Save file top output dir
}

const imagePath = ref("");

function select() {
  open({
    title: "Select an image",
  }).then((path) => {
    // Set image path
    path && (imagePath.value = convertFileSrc(path));
  });
}
</script>

<template>
  <main class="flex size-full flex-1 overflow-hidden">
    <!-- Image Selection/Viewer -->
    <div class="flex-1 p-4">
      <img
        v-if="imagePath"
        :src="imagePath"
        alt="uploaded image"
        class="bg-muted size-full rounded-xl object-contain"
      />
      <!-- Upload region -->
      <div
        @click="select()"
        v-else
        ref="dropZone"
        class="bg-muted/60 hover:border-primary/40 hover:bg-muted/80 relative flex size-full cursor-pointer flex-col items-center-safe justify-center-safe gap-4 rounded-xl border-2 border-dashed p-4"
        :class="[dragActive && 'border-primary/40 bg-muted/80!']"
      >
        <Image :size="64" />
        <p class="text-2xl font-semibold">Drag and drop any image</p>
        <p
          class="absolute bottom-4 max-w-prose text-center font-mono text-sm leading-relaxed opacity-40"
        >
          Supported files: AVIF, BMP, DDS, OpenEXR, Farbfeld, GIF, HDR, ICO,
          JPEG, PNG, PNM, QOI, TGA, TIFF, WebP
        </p>
      </div>
    </div>

    <!-- Options "Sidebar" -->
    <div class="flex w-1/4 flex-col gap-6 border-l p-4">
      <p class="text-2xl font-bold">Image type converter</p>

      <!-- Step 1: Select Image -->
      <div class="flex flex-col gap-2">
        <p class="text-xl font-semibold">Step 1</p>
        <Button class="self-start" variant="secondary" @click="select()">
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
        <Button class="self-start" variant="secondary" @click="setDir">
          Set Output Folder
        </Button>
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
