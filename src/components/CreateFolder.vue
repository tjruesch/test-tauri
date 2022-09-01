<script setup lang="ts">
import { ref, computed, onMounted, Ref } from 'vue'
import type { config } from "../types/config"
import { invoke } from '@tauri-apps/api/tauri'

const appConfig:Ref<config|null> = ref(null)
const type = ref("ANS")

const folderNamePt1 = computed(() => (
  appConfig.value?.baseDir.endsWith("/") ? 
    appConfig.value?.baseDir + appConfig.value?.currentId + "_" +  type.value + "_" :
    appConfig.value?.baseDir + "/" + appConfig.value?.currentId + "_" +  type.value + "_"
  )
)
const folderNamePt2 = ref("awesomeProject")
const folderNamePt3 = computed(() => "_" + monthString.value + "-" + today.getFullYear())

const showSettings = ref(false)
const showConfirmation = ref(false)
const presetWeb = ref(true)
const subDirs = computed(() => presetWeb.value ? appConfig.value?.presets.web : appConfig.value?.presets.print)

const today = new Date()
const monthString = computed(() => {
  if (today.getMonth() + 1 < 10) {
    return `0${today.getMonth() + 1}`
  } else {
    return `${today.getMonth() + 1}`
  }
})

async function loadConfig() {
  const configString: string = await invoke('read_app_config')
  console.log(configString)
  appConfig.value = JSON.parse(configString)
  console.log("app config: ", appConfig.value)
}

async function updateConfig() {
  if (appConfig.value) {
    const configString: string = await invoke('update_app_config', { config: JSON.stringify(appConfig.value) })
    appConfig.value = JSON.parse(configString)
  }
}

const rootId = ref()
async function updateBaseDir() {
  if (appConfig.value) {
    appConfig.value.baseDir = rootId.value.value
  }
  await updateConfig()
}

async function createFolders() {
  let baseDir = folderNamePt1.value + folderNamePt2.value + folderNamePt3.value
  baseDir = baseDir.endsWith("/") ? baseDir : baseDir + "/"

  const dirs = []
  if (subDirs.value) {
    for (const dir of subDirs.value) {
      if (dir.include) {
        dirs.push(baseDir + dir.displayName)
      }
    }
  }
  
  for (const d of dirs) {
    await invoke('create_folder', {folder: d})
  }

  if (appConfig.value) {
    appConfig.value.currentId += 1
    await updateConfig()
  }
}

onMounted(() => {
  loadConfig()
})
</script>

<template>
  <div>
    <div class="md:grid md:grid-cols-3 md:gap-6">
      <div class="md:col-span-1">
        <div class="px-4 px-0 mt-4">
          <h3 class="text-lg font-medium leading-6 text-gray-900 dark:text-white">Neuen Ordner anlegen</h3>
          <p class="my-1 text-sm text-gray-600 dark:text-gray-300">Diese Funktion erstellt einen neuen Ordner mit der vorgegebenen Struktur in dem angegebenen Root-Ordner.</p>

          <button class="flex justify-start items-center mb-1 mt-5 font-medium text-lg dark:text-white text-gray-900" @click="showSettings = !showSettings">
            Basiseinstellungen
            <span class="mr-2" :class="showSettings ? 'rotate-180' : ''">
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="w-5 h-5">
                <path fill-rule="evenodd" d="M5.23 7.21a.75.75 0 011.06.02L10 11.168l3.71-3.938a.75.75 0 111.08 1.04l-4.25 4.5a.75.75 0 01-1.08 0l-4.25-4.5a.75.75 0 01.02-1.06z" clip-rule="evenodd" />
              </svg>
            </span>
          </button>
          <div v-if="showSettings">
            <!--begin::rootDir-->
            <div class="mt-2 mb-5">
               <label for="root-dir" class="block text-sm font-medium dark:text-white text-gray-700">Basis-Ordner</label>
               <input id="root-dir" ref="rootId" :value="appConfig?.baseDir" type="text" name="root-dir" class="px-2 mt-1 block w-full rounded-md border border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-base text-lg" />
            </div>
            <!--end::rootDir-->
            <div class="flex justify-end">
              <button 
                class="inline-flex justify-center rounded-md border border-gray-300 dark:border-gray-700 shadow-sm bg-white dark:bg-[#616060] py-px px-2 text-sm dark:text-white"
                @click="updateBaseDir"
              >
                Speichern
              </button>
            </div>
          </div>
          
        </div>
      </div>
      <div class="mt-5 md:col-span-2">
          <div class="border-[0.5px] border-gray-300 dark:border-gray-700 overflow-hidden rounded-md">
            <div class="space-y-6 px-4 py-5 sm:p-6 bg-[#e6e6e6] dark:bg-[#2b2a2a]">
              <div class="grid grid-cols-3 gap-6">
                <!--begin::folderName-->
                <div class="col-span-3">
                  <label for="root-dir" class="block text-sm font-medium dark:text-white text-gray-700">Ordner-Name</label>
                  <div class="my-1 flex rounded-md shadow-sm">
                    <span class="inline-flex items-center rounded-l-md border border-r-0 border-gray-300 dark:border-gray-700 bg-gray-50 dark:bg-[#616060] px-3 text-sm text-gray-500 dark:text-white">{{ folderNamePt1 }}</span>
                    <input
                      id="root-dir"
                      v-model="folderNamePt2" 
                      type="text"
                      name="root-dir" 
                      class="px-2 block w-full flex-1 rounded-none border border-gray-300 dark:border-gray-700 focus:border-indigo-500 focus:ring-indigo-500 text-sm dark:bg-[#252424] dark:text-white"
                    />
                    <span class="inline-flex items-center rounded-r-md border border-l-0 border-gray-300 dark:border-gray-700 bg-gray-50 dark:bg-[#616060] px-3 text-sm text-gray-500 dark:text-white">{{ folderNamePt3 }}</span>
                  </div>
                </div>
                <!--end::folderName-->
                <!--begin::projectType-->
                <div class="col-span-1">
                  <label for="type" class="block text-sm font-medium dark:text-white text-gray-700">Projekttyp</label>
                  <select 
                    id="type"
                    v-model="type" 
                    name="type" 
                    class="mt-1 block w-full rounded-md border border-gray-300 bg-white py-2 px-3 shadow-sm focus:border-indigo-500 focus:outline-none focus:ring-indigo-500 sm:text-sm"
                  >
                    <option v-for="v in appConfig?.typeSelections" :key="v" :value="v">{{v}}</option>
                  </select>
                </div>
                <!--end::projectType-->
                <!--begin::preset-->
                <div class="col-span-2 h-full">
                  <p class="block text-sm font-medium dark:text-white text-gray-700 mb-1">Preset</p>
                  <fieldset>
                    <div class="flex justify-start items-end h-full space-x-4">
                      <div class="flex items-center">
                        <input id="preset-web" v-model="presetWeb" :value="true" name="preset" type="radio" class="h-5 w-5 border-gray-300 text-indigo-600 focus:ring-indigo-500">
                        <label for="preset-web" class="ml-2 block text-sm font-medium dark:text-white text-gray-700">Web</label>
                      </div>
                      <div class="flex items-center">
                        <input id="preset-print" v-model="presetWeb" :value="false" name="preset" type="radio" class="h-5 w-5 border-gray-300 text-indigo-600 focus:ring-indigo-500">
                        <label for="preset-print" class="ml-2 block text-sm font-medium dark:text-white text-gray-700">Print</label>
                      </div>
                    </div>
                  </fieldset>
                </div>
                <!--end::preset-->
                <!--begin::subFolders-->
                <div class="col-span-3">
                  <fieldset>
                    <div class="block text-sm font-medium dark:text-white text-gray-700">Unterordner</div>
                    <div class="grid grid-cols-2 mt-4 space-y-2">
                      <div v-for="s in subDirs" :key="s.label" class="flex items-start">
                        <div class="flex h-5 items-center">
                          <input 
                            :id="s.label" 
                            v-model="s.include" 
                            :name="s.label" 
                            type="checkbox"
                            class="h-4 w-4 rounded border-gray-300 text-indigo-600 focus:ring-indigo-500"
                          />
                        </div>
                        <div class="ml-3 text-sm">
                          <label :for="s.label" class="font-medium dark:text-white text-gray-700">{{ s.displayName }}</label>
                          <p class="dark:text-gray-300 text-gray-500">{{ s.description }}</p>
                        </div>
                      </div>
                    </div>
                  </fieldset>
                </div>
                <!--end::subFolders-->
              </div>
            </div>
            <div class="bg-[#e6e6e6] dark:bg-[#2b2a2a] px-4 py-3 text-right sm:px-6">
              <span v-if="showConfirmation" class="text-gray-700 dark:text-white mr-3 text-xs">Ordner erstellt!</span>
              <button 
                class="inline-flex justify-center rounded-md border border-gray-300 dark:border-gray-700 shadow-sm bg-white dark:bg-[#616060] py-px px-2 text-sm dark:text-white"
                @click="createFolders"
              >
                Erstellen
              </button>
            </div>
          </div>
      </div>
    </div>
  </div>
</template>