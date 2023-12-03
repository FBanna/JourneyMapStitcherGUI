<template>
    <div class="launcher">
        <li :class="{if_selected: check_grand_child_selected('MC')}">Vanilla Launcher</li>

        <li class="world_type" :class="{if_selected: check_child_selected('MC', 'mp')}" @click="MC_multi_player_show = !MC_multi_player_show">MP</li>

        <li class="world" :class="{if_selected: check_selected('MC', 'mp', index)}" @click="selected = ['MC', 'mp', index]" 
        v-if="MC_multi_player_show" v-for="(item, index) in MC_multi_player">

            {{ item.substring(item.lastIndexOf('\\')+1, item.length) }}

        </li>

        <li class="world_type" :class="{if_selected: check_child_selected('MC', 'sp')}" @click="MC_single_player_show = !MC_single_player_show">SP</li>

        <li class="world" :class="{if_selected: check_selected('MC', 'sp', index)}" @click="selected = ['MC', 'sp', index]" 
        v-if="MC_single_player_show" v-for="(item, index) in MC_single_player">

            {{ item.substring(item.lastIndexOf('\\')+1, item.length) }}

        </li>


        <li :class="{if_selected: check_grand_child_selected('Prism')}">Prism Launcher</li>

        <li class="world_type" :class="{if_selected: check_child_selected('Prism', 'mp')}" @click="Prism_multi_player_show = !Prism_multi_player_show">MP</li>

        <li class="world" :class="{if_selected: check_selected('Prism', 'mp', index)}" @click="selected = ['Prism', 'mp', index]" 
        v-if="Prism_multi_player_show" v-for="(item, index) in Prism_multi_player">

            {{ item.substring(item.lastIndexOf('\\')+1, item.length) }}

        </li>

        <li class="world_type" :class="{if_selected: check_child_selected('Prism', 'sp')}" @click="Prism_single_player_show = !Prism_single_player_show">SP</li>

        <li class="world" :class="{if_selected: check_selected('Prism', 'sp', index)}" @click="selected = ['Prism', 'sp', index]" 
        v-if="Prism_single_player_show" v-for="(item, index) in Prism_single_player">

            {{ item.substring(item.lastIndexOf('\\')+1, item.length) }}

        </li>

    </div>


    <div class="refresh">
        <button class="inputsButton" @click="get_worlds">Refresh</button>
    </div>

    <div class="select">
        <button class="inputsButton" @click="set_world">Select</button>
    </div>

    {{ selected }}

    

</template>




<script setup>
    import { ref, computed, onMounted } from 'vue'
    import leaflet from "leaflet";
    import { event, invoke } from '@tauri-apps/api';
    import { WebviewWindow, LogicalSize, PhysicalSize  } from '@tauri-apps/api/window'
    import { window } from "@tauri-apps/api"
    import { TauriEvent, emit } from "@tauri-apps/api/event"
    import { appWindow } from '@tauri-apps/api/window';

    window.getCurrent().listen(TauriEvent.WINDOW_CLOSE_REQUESTED, () => {
        console.log("clicked")
        invoke("store_last_world")
        appWindow.close()
    
        
    });

    var selected = ref(["launcher", "type", 0])

    var MC_multi_player = ref()
    var MC_single_player = ref()
    var Prism_multi_player = ref()
    var Prism_single_player = ref()


    var MC_multi_player_show = ref(true)
    var MC_single_player_show = ref(true)
    var Prism_multi_player_show = ref(true)
    var Prism_single_player_show = ref(true)

    get_worlds()
    
    async function get_worlds() {
        [MC_multi_player.value, MC_single_player.value, 
        Prism_multi_player.value, Prism_single_player.value] = await invoke("get_world")
    }

    function check_selected(launcher, type, index) {
        if(JSON.stringify(selected.value) == JSON.stringify([launcher,type,index])) {
            return true
        } else {
            return false
        }
    }

    function check_child_selected(launcher, type) {
        if(JSON.stringify([selected.value[0], selected.value[1]]) == JSON.stringify([launcher, type])){
            return true
        } else {
            return false
        }
    }

    function check_grand_child_selected(launcher) {
        if( selected.value[0] == launcher){
            return true
        } else {
            return false
        }
    }

    function set_world() {
        //if(selected.value[0] == "MC"){
        //    if(selected.value[1] == "mp"){
        invoke("set_world", {launcher: selected.value[0], serverType: selected.value[1], index: selected.value[2]})

        console.log(selected.value[0], selected.value[1], selected.value[2])
            //} else {
                //invoke("set_world")
            //}
        //} else {
            //console.log("not selected")
        //}
        
    }

    

</script>
    



<style>
    .refresh {
        position: absolute;
        bottom: 20px;
        left: 20px;
    }
    .select {
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
    .if_selected {
        color: rgb(11, 219, 11);
    }
</style>