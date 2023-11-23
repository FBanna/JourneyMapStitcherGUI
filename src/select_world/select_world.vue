<template>
    <div class="launcher">
        <li>Vanilla Launcher</li>
        <li class="world_type" @click="MC_multi_player_show = !MC_multi_player_show">MP</li>
        <li class="world" v-if="MC_multi_player_show" v-for="(item, index) in MC_multi_player">
            <div class="items" @click="select(index)">{{ item }}</div>
        </li>
        <li class="world_type" @click="MC_single_player_show = !MC_single_player_show">SP</li>
        <li class="world" v-if="MC_single_player_show" v-for="item in MC_single_player">
            <div class="items">{{ item }}</div>
        </li>
    </div>

    <div class="buttons">
        <button class="inputsButton" @click="get_worlds">Refresh</button>
    </div>
    

</template>




<script setup>
    import { ref, computed, onMounted } from 'vue'
    import leaflet from "leaflet";
    import { invoke } from '@tauri-apps/api';
    import { WebviewWindow, LogicalSize, PhysicalSize  } from '@tauri-apps/api/window'
    import { appWindow } from '@tauri-apps/api/window';

    //appWindow.setTitle("hello")
    var MC_multi_player = ref()
    var MC_single_player = ref()

    var MC_multi_player_show = ref(true)
    var MC_single_player_show = ref(true)

    get_worlds()
    
    async function get_worlds() {
        [MC_multi_player.value, MC_single_player.value] = await invoke("get_world")
    }

    function select(index) {
        console.log(index)
    }


</script>
    



<style>
    .buttons {
        position: absolute;
        bottom: 20px;
        right: 20px;
    }


    .inputsButton{
        border: none;
        background-color: #ae00ff;
        color: white;
        border-radius: 3px;
        width: 60px;
        height: 20px;
        font-size: 13px;
        transition-duration: 0.2s;
        font-family: Verdana, sans-serif;
    }

    .inputsButton:hover{
        background-color: #01AFE4;
    }  

    .launcher {
        font-family: Verdana, sans-serif;
        user-select: none;
    }

    .world_type {
        text-indent: 20px;
    }
    .world {
        text-indent: 40px;
    }
    .items {
        position: relative;
    }
</style>