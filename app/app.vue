<script setup lang="ts">
import "vue-sonner/style.css";
import { Image, Cpu } from "lucide-vue-next";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import type { UnlistenFn } from "@tauri-apps/api/event";
import { open } from "@tauri-apps/plugin-dialog";
import { convertFileSrc, invoke } from "@tauri-apps/api/core";
import { toast } from "vue-sonner";

const supportedExtensions = [
  "avif",
  "bmp",
  "gif",
  "jpg",
  "jpeg",
  "png",
  "webp",
];

const imagePath = ref("");
function selectImage() {
  open({
    title: "Select an image",
    filters: [
      {
        name: "Supported Images",
        extensions: supportedExtensions,
      },
    ],
  }).then((path) => {
    // Set image path
    imagePath.value = path || "";
  });
}

const outputDir = ref("");
function setOutputDir() {
  open({
    title: "Select output folder",
    directory: true,
  }).then((path) => {
    // Set image path
    path && (outputDir.value = path);
  });
}

const toFormat = ref("");
const isSaving = ref(false); // bool to disable save button while saving
// Call Rust to modify image accordingly and save it
function save() {
  // Verify required steps are complete
  if (!imagePath.value) {
    toast.error("Please upload a file");
  } else if (!toFormat.value) {
    toast.error("Please select a format to convert to");
  } else {
    isSaving.value = true;

    toast.promise(
      invoke("convert", {
        path: imagePath.value,
        toFormat: toFormat.value,
        outputDir: outputDir.value,
      }),
      {
        loading: "Converting image...",
        success: "Image converted!",
        error: (err: string) => {
          console.error(err);
          return `Failed with error: ${err}`;
        },
        finally: () => {
          isSaving.value = false;
        },
      },
    );
  }
}

/* File drag drop
 * START */
const dragActive = ref(false);
let unlistenDragDropEvent: UnlistenFn;
onMounted(async () => {
  unlistenDragDropEvent = await getCurrentWebviewWindow().onDragDropEvent(
    ({ payload }) => {
      // Keep bool true during enter/over (resets on drop/leave)
      dragActive.value = payload.type == "enter" || payload.type == "over";

      // File(s) dropped
      if (payload.type == "drop") {
        // Asserts that file(s) dropped is 1
        if (payload.paths.length == 1) {
          // SAFETY: paths[0] isn't undefined after the length check, and -1 will always return in this case
          const extension = payload.paths[0]!.split(".").at(-1)!;
          if (supportedExtensions.includes(extension)) {
            imagePath.value = convertFileSrc(payload.paths[0]!);
          } else {
            alert("Format not supported");
          }
        } else {
          alert("Please select only one file");
        }
      }
    },
  );
});
onUnmounted(async () => {
  unlistenDragDropEvent && unlistenDragDropEvent();
});
/* END */
</script>

<template>
  <main class="flex size-full flex-1 overflow-hidden">
    <!-- Image Selection/Viewer -->
    <div class="flex-1 p-4">
      <img
        v-if="imagePath"
        :src="convertFileSrc(imagePath)"
        alt="uploaded image"
        class="bg-muted size-full rounded-xl object-contain"
      />
      <!-- Upload region -->
      <div
        @click="selectImage()"
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
          Supported formats: &nbsp; AVIF, BMP, GIF, JPG, JPEG, PNG, WebP
        </p>
      </div>
    </div>

    <!-- Options "Sidebar" -->
    <div class="flex w-1/4 flex-col gap-6 overflow-auto border-l p-4 pb-8">
      <p class="text-2xl font-bold">Image type converter</p>

      <!-- Step 1: Select Image -->
      <div class="flex flex-col gap-2">
        <p class="text-xl font-semibold">Step 1</p>
        <Button class="self-start" variant="secondary" @click="selectImage()">
          Select Image
        </Button>
      </div>

      <!-- Step 2: Select Format -->
      <div class="flex flex-col gap-2">
        <p class="text-xl font-semibold">Step 2</p>
        <p class="text-sm">Select format to convert to</p>
        <Select v-model="toFormat">
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
        <Button class="self-start" variant="secondary" @click="setOutputDir">
          Set Output Folder
        </Button>
        <p v-if="outputDir" class="font-mono text-sm wrap-anywhere opacity-40">
          {{ outputDir }}
        </p>
      </div>

      <!-- Step 4: Fourmat (Format and Save) -->
      <div class="flex flex-col gap-2">
        <p class="text-xl font-semibold">Step 4</p>
        <Button class="self-start" @click="save" :disabled="isSaving">
          <Cpu />
          Fourmat
        </Button>
      </div>
    </div>
  </main>
  <Toaster richColors :theme="$colorMode.value == 'dark' ? 'dark' : 'light'" />
</template>
