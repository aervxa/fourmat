<script setup lang="ts">
import "vue-sonner/style.css";
import { Image, Cpu } from "lucide-vue-next";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import type { UnlistenFn } from "@tauri-apps/api/event";
import { open } from "@tauri-apps/plugin-dialog";
import { convertFileSrc, invoke } from "@tauri-apps/api/core";
import { toast } from "vue-sonner";
import {
  Check,
  X,
  FolderInput,
  ImagePlus,
  ImageUp,
  Trash2,
} from "lucide-vue-next";
import { useWindowSize } from "@vueuse/core";

const imagePaths = ref<string[]>([]);
const imagePathsSrc = computed(() =>
  imagePaths.value.map((path) => convertFileSrc(path)),
);

function pushImagePaths(paths: string[] | null) {
  // if paths exist
  if (paths && paths.length > 0) {
    const newPaths: string[] = [];
    // Feed valid paths into newPath
    paths.forEach((path) => {
      // SAFETY: index -1 will always return in this case
      const extension = path.split(".").at(-1)!;
      if (
        SUPPORTED_EXTENSIONS.map((ext) => ext.toLowerCase()).includes(
          extension.toLowerCase(),
        )
      ) {
        newPaths.push(path);
      }
    });

    // Push valid paths if exists
    if (newPaths.length > 0) {
      imagePaths.value = newPaths;
    }
    // Some files are invalid
    if (newPaths.length !== paths.length) {
      setTimeout(() => {
        toast.error("Couldn't upload every selected file!", {
          description: `Supported formats: ${SUPPORTED_EXTENSIONS_STR}`,
        });
      });
    }
  } else {
    toast.error("Please select at least one file");
  }
}

function selectImage() {
  open({
    title: "Select an image",
    filters: [
      {
        name: "Supported Images",
        extensions: SUPPORTED_EXTENSIONS.map((ext) => ext.toLowerCase()),
      },
    ],
    multiple: true,
  }).then(pushImagePaths);
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
  if (!imagePaths.value.length) {
    toast.error("Please upload a file");
  } else if (!toFormat.value) {
    toast.error("Please select a format to convert to");
  } else {
    isSaving.value = true;

    toast.promise(
      invoke("convert", {
        path: imagePaths.value[0], // FIXME whole array of paths should be sent
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
        // Set image paths
        pushImagePaths(payload.paths);
      }
    },
  );
});
onUnmounted(async () => {
  unlistenDragDropEvent && unlistenDragDropEvent();
});
/* END */

// CONSTANTS
const TITLE = "Image format converter";
const { width } = useWindowSize();
const BREAKPOINT = computed(() => width.value <= 768);
const SUPPORTED_EXTENSIONS = [
  "AVIF",
  "BMP",
  "GIF",
  "JPG",
  "JPEG",
  "PNG",
  "WebP",
];
const SUPPORTED_EXTENSIONS_STR = SUPPORTED_EXTENSIONS.toSpliced(-1, 0, "and")
  .join(", ")
  .replace("and,", "and");
</script>

<template>
  <main class="flex size-full flex-1 overflow-hidden max-md:flex-col">
    <!-- Title + Options Header (for small screens without sidebar) -->
    <div
      v-show="BREAKPOINT"
      class="flex flex-wrap items-center-safe justify-between gap-4 p-4"
    >
      <p class="text-xl font-semibold">{{ TITLE }}</p>

      <!-- Options (image select/change & output directory selection) -->
      <div class="flex gap-2">
        <!-- Select Image -->
        <Button
          variant="secondary"
          @click="selectImage()"
          :title="`${imagePaths.length ? 'Change' : 'Select'} Image`"
        >
          <ImageUp v-if="imagePaths.length" />
          <ImagePlus v-else />
        </Button>

        <Button
          variant="secondary"
          size="icon"
          @click="setOutputDir"
          :title="`${outputDir ? 'Change' : 'Set'} Output Folder`"
        >
          <FolderInput />
        </Button>
      </div>
    </div>

    <!-- Image Selection/Viewer -->
    <div class="flex-1 gap-2 overflow-auto p-4">
      <!-- Image preview -->
      <div
        v-if="imagePaths.length"
        class="bg-card grid size-full auto-rows-min grid-cols-[repeat(auto-fill,minmax(192px,1fr))] gap-4 overflow-auto rounded-xl p-4"
      >
        <div
          v-for="(src, i) in imagePathsSrc"
          :key="src"
          class="group relative aspect-square overflow-clip rounded-xl bg-cover bg-center"
          :style="{ backgroundImage: `url(${src})` }"
        >
          <!-- Overlay to blur the background of the container -->
          <div
            class="absolute inset-0 backdrop-blur-sm backdrop-brightness-60"
          ></div>
          <img
            :src="src"
            :alt="`uploaded image ${i}`"
            class="relative size-full object-contain"
            draggable="false"
          />
          <!-- Delete button -->
          <Button
            variant="destructive"
            size="icon"
            class="animate-in fade-in-30 absolute top-2 right-2 hidden backdrop-blur-sm group-hover:flex"
            @click="imagePaths.splice(i, 1)"
          >
            <Trash2 />
          </Button>
        </div>
      </div>
      <!-- Upload region -->
      <div
        v-else
        @click="selectImage()"
        ref="dropZone"
        class="bg-muted/60 hover:border-primary/40 hover:bg-muted/80 relative flex size-full cursor-pointer flex-col items-center-safe justify-center-safe gap-4 rounded-xl border-2 border-dashed p-4"
        :class="[dragActive && 'border-primary/40 bg-muted/80!']"
      >
        <Image :size="64" />
        <p class="text-2xl font-semibold">Drag and drop any image</p>
        <p
          class="absolute bottom-4 max-w-prose text-center font-mono text-sm leading-relaxed opacity-40"
        >
          Supported formats: &nbsp; {{ SUPPORTED_EXTENSIONS_STR }}
        </p>
      </div>
    </div>

    <!-- Options "Bottom bar" (for small screens without sidebar) -->
    <div v-show="BREAKPOINT" class="flex justify-between border-t p-4">
      <!-- Select Format -->
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

      <!-- Fourmat (Format and Save) -->
      <Button class="self-start" @click="save" :disabled="isSaving">
        <Cpu />
        Fourmat
      </Button>
    </div>

    <!-- Options "Sidebar" -->
    <div
      class="flex w-2xs flex-col gap-6 overflow-auto border-l p-4 pb-8 max-md:hidden lg:w-xs xl:w-sm"
    >
      <p class="text-2xl font-bold">{{ TITLE }}</p>

      <!-- Step 1: Select Image -->
      <div class="flex flex-col gap-2">
        <p
          class="text-xl font-semibold"
          :class="[imagePaths.length && 'flex items-center-safe gap-2']"
        >
          Step 1
          <Check v-if="imagePaths.length" :size="18" class="text-primary" />
        </p>
        <Button class="self-start" variant="secondary" @click="selectImage()">
          <template v-if="imagePaths.length">Change</template>
          <template v-else>Select</template> Image
        </Button>
      </div>

      <!-- Step 2: Select Format -->
      <div class="flex flex-col gap-2">
        <p
          class="text-xl font-semibold"
          :class="[toFormat && 'flex items-center-safe gap-2']"
        >
          Step 2
          <Check v-if="toFormat" :size="18" class="text-primary" />
        </p>
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

      <!-- Step 3: Set Output Directory -->
      <div class="flex flex-col gap-2">
        <p
          class="text-xl font-semibold"
          :class="[outputDir && 'flex items-center-safe gap-2']"
        >
          Step 3
          <Check v-if="outputDir" :size="18" class="text-primary" />
        </p>
        <p v-if="!outputDir" class="text-sm">Defaults to image's path</p>
        <div class="flex gap-2">
          <Button variant="secondary" @click="setOutputDir">
            <template v-if="outputDir">Change Folder</template>
            <template v-else>Set Output Folder</template>
          </Button>
          <Button
            v-if="outputDir"
            variant="secondary"
            size="icon"
            @click="outputDir = ''"
          >
            <X />
          </Button>
        </div>
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
  <Toaster
    :position="BREAKPOINT ? 'top-center' : 'bottom-left'"
    richColors
    :theme="$colorMode.value == 'dark' ? 'dark' : 'light'"
  />
</template>
