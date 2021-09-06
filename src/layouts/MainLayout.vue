<template>
  <q-layout view="lHh Lpr lFf">
    <q-header elevated>
      <q-toolbar>
        <q-toolbar-title>
          Wikit App
        </q-toolbar-title>
        <div class="q-pa-md q-gutter-sm">
          <q-btn color="white" text-color="black" @click="callRust" label="Call Rust" />
          <q-btn color="white" text-color="black" @click="openFile" label="Open File" />
        </div>
      </q-toolbar>
    </q-header>

    <q-page-container>
      <router-view />
    </q-page-container>
  </q-layout>
</template>

<script lang="ts">

import { defineComponent, ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { open } from '@tauri-apps/api/dialog'
import { app, dialog, event, fs, globalShortcut } from '@tauri-apps/api'

export default defineComponent({
  name: 'MainLayout',

  components: {},

  // const tauri = window.__TAURI__;
  setup () {
    return {
      callRust: function() {
        invoke('call_rust');
      },
      openFile: function() {
        dialog.open().then((r) => {
          console.log(r);
          invoke("parse_file", { path: r })
            .then((ret) => {
                console.log(`receive: ${ret}`);
            });
        });
      }
    }
  },
})
</script>
