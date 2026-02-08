<script setup lang="ts">
import "vue-sonner/style.css";
import {
  Image,
  Cpu,
  Check,
  X,
  FolderInput,
  FolderPen,
  RotateCcw,
  Plus,
  Trash2,
} from "lucide-vue-next";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import type { UnlistenFn } from "@tauri-apps/api/event";
import { open } from "@tauri-apps/plugin-dialog";
import { convertFileSrc, invoke } from "@tauri-apps/api/core";
import { toast } from "vue-sonner";
import { useWindowSize } from "@vueuse/core";
import ImageSquare from "~/components/ImageSquare.vue";

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
      const extension = path.split(".").at(-1)?.toLowerCase() || "";
      if (
        // extension is supported
        EXTENSIONS.some((ext) => ext.toLowerCase() == extension) &&
        // not a duplicate
        !imagePaths.value.includes(path)
      ) {
        newPaths.push(path);
      }
    });

    // Push valid paths if exists
    if (newPaths.length > 0) {
      imagePaths.value.push(...newPaths);
    }
    // Some files are invalid
    if (newPaths.length !== paths.length) {
      toast.error("Some files were skipped!", {
        description:
          "They were either already selected or had an unsupported format.",
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
        extensions: EXTENSIONS.map((ext) => ext.toLowerCase()),
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
      invoke("convert_images", {
        paths: imagePaths.value,
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

function reset() {
  imagePaths.value = [];
  outputDir.value = "";
  toFormat.value = "";
  transitioningImage.value = -1;
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
const EXTENSIONS = ["AVIF", "BMP", "GIF", "JPG", "JPEG", "PNG", "WebP"];
const EXTENSIONS_STR = EXTENSIONS.toSpliced(-1, 0, "and")
  .join(", ")
  .replace("and,", "and");

const transitioningImage = ref(-1);
const showDialog = ref(false);
const imageTransitionName = "transitioning_image";
async function transitionImage(i: number) {
  /**
   * if actual image is being transitioned, otherwise, it's a back transition,
   * which would need to happen backwards, hence the setting after .finished
   */

  if (i >= 0) {
    transitioningImage.value = i;
    // wait for vue to actually update DOM
    await nextTick();
  }
  async function transition() {
    // transition code
    showDialog.value = !showDialog.value;
    // wait for vue to actually update DOM
    await nextTick();
  }
  if (!document.startViewTransition) {
    // Fallback for browsers that don't support the transitioning
    transition();
  } else {
    document.startViewTransition(transition).finished.then(() => {
      if (i < 0) {
        transitioningImage.value = i;
      }
    });
  }
}
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
        <!-- Reset -->
        <Button
          v-if="imagePaths.length"
          variant="secondary"
          size="icon"
          @click="reset()"
          title="Reset"
        >
          <RotateCcw />
        </Button>

        <Button
          variant="secondary"
          size="icon"
          @click="setOutputDir"
          :title="`${outputDir ? 'Change' : 'Set'} Output Folder`"
        >
          <FolderPen v-if="outputDir" />
          <FolderInput v-else />
        </Button>
      </div>
    </div>

    <!-- Image Selection/Viewer -->
    <div class="flex-1 gap-2 overflow-auto p-4">
      <!-- Image preview -->
      <div
        v-if="imagePaths.length"
        class="bg-card relative grid size-full auto-rows-min grid-cols-[repeat(auto-fill,minmax(192px,1fr))] gap-4 overflow-auto rounded-xl p-4"
      >
        <!-- Image zoomed-in preview -->
        <div
          v-if="showDialog"
          @click="transitionImage(-1)"
          class="bg-background/40 absolute inset-0 z-10 flex cursor-pointer items-center-safe justify-center-safe backdrop-blur-xs"
        >
          <ImageSquare
            :src="imagePathsSrc[transitioningImage]"
            :alt="`uploaded_image_${transitioningImage}`"
            :delete-fn="
              () => {
                transitionImage(-1);
              }
            "
            :action-icon="X"
            action-variant="outline"
            :style="{
              viewTransitionName: imageTransitionName,
            }"
            class="max-w-96"
          />
        </div>

        <!-- Image grids -->
        <ImageSquare
          v-for="(src, i) in imagePathsSrc"
          :key="src"
          :src
          :alt="`uploaded_image_${i}`"
          :delete-fn="
            () => {
              imagePaths.splice(i, 1);
            }
          "
          :action-icon="Trash2"
          action-variant="destructive"
          @click="transitionImage(i)"
          class="cursor-pointer"
          :class="[transitioningImage === i && showDialog && 'opacity-0']"
          :style="[
            transitioningImage === i &&
              !showDialog && {
                viewTransitionName: imageTransitionName,
              },
          ]"
        />
        <!-- Select more images -->
        <div
          @click="selectImage()"
          class="flex aspect-square cursor-pointer flex-col items-center-safe justify-center-safe gap-4 rounded-xl border-2 border-dashed"
        >
          <Plus :size="64" />
          <p class="px-4 text-center text-sm font-extralight">
            Drop or select more images
          </p>
        </div>
      </div>
      <!-- Upload region -->
      <div
        v-else
        @click="selectImage()"
        class="bg-muted/60 hover:border-primary/40 hover:bg-muted/80 relative flex size-full cursor-pointer flex-col items-center-safe justify-center-safe gap-4 rounded-xl border-2 border-dashed p-4"
        :class="[dragActive && 'border-primary/40 bg-muted/80!']"
      >
        <Image :size="64" />
        <p class="text-center text-2xl font-semibold">Drop or select images</p>
        <p
          class="absolute inset-x-0 bottom-3 mx-auto max-w-prose px-6 text-center font-mono text-sm leading-relaxed opacity-40"
        >
          Supported EXTENSIONS: &nbsp; {{ EXTENSIONS_STR }}
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
      <p class="text-xl font-bold">{{ TITLE }}</p>

      <!-- Step 1: Select Image -->
      <div class="flex flex-col gap-2">
        <p
          class="text-xl font-semibold"
          :class="[imagePaths.length && 'flex items-center-safe gap-2']"
        >
          Step 1
          <Check v-if="imagePaths.length" :size="18" class="text-primary" />
        </p>
        <div class="flex gap-2">
          <Button class="self-start" variant="secondary" @click="selectImage()">
            Select Images
          </Button>
          <Button
            v-if="imagePaths.length"
            variant="secondary"
            size="icon"
            @click="reset"
          >
            <RotateCcw />
          </Button>
        </div>
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
            <template v-if="outputDir">Change</template>
            <template v-else>Set</template> Output Folder
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
    :position="BREAKPOINT ? 'top-center' : 'bottom-right'"
    richColors
    :theme="$colorMode.value == 'dark' ? 'dark' : 'light'"
  />
</template>
