<template>
    good bye
</template>




<script setup>
    import { ref, computed, onMounted } from 'vue'
    import leaflet from "leaflet";
    import { invoke } from '@tauri-apps/api';
    import { WebviewWindow, LogicalSize, PhysicalSize  } from '@tauri-apps/api/window'
    import { appWindow } from '@tauri-apps/api/window';

    appWindow.setTitle("hello")


</script>
    



<style>

</style>