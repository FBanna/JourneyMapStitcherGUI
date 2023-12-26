<template>
    <div class="launcher">
        <li :class="{if_selected: check_grand_child_selected(0)}">Vanilla Launcher</li>

        <li class="world_type" :class="{if_selected: check_child_selected(0)}" @click="MC_multi_player_show = !MC_multi_player_show">MP</li>

        <li class="world" :class="{if_selected: check_selected(0, index)}" @click="selected = [0, index], changed = true" 
        v-if="MC_multi_player_show" v-for="(item, index) in paths[0]">

            {{ item.substring(item.lastIndexOf('\\')+1, item.length) }}

        </li>

        <li class="world_type" :class="{if_selected: check_child_selected(1)}" @click="MC_single_player_show = !MC_single_player_show">SP</li>

        <li class="world" :class="{if_selected: check_selected(1, index)}" @click="selected = [1, index], changed = true" 
        v-if="MC_single_player_show" v-for="(item, index) in paths[1]">

            {{ item.substring(item.lastIndexOf('\\')+1, item.length) }}

        </li>


        <li :class="{if_selected: check_grand_child_selected(2)}">Prism Launcher</li>

        <li class="world_type" :class="{if_selected: check_child_selected(2)}" @click="Prism_multi_player_show = !Prism_multi_player_show">MP</li>

        <li class="world" :class="{if_selected: check_selected(2, index)}" @click="selected = [2, index], changed = true" 
        v-if="Prism_multi_player_show" v-for="(item, index) in paths[2]">

            {{ item.substring(item.lastIndexOf('\\')+1, item.length) }}

        </li>

        <li class="world_type" :class="{if_selected: check_child_selected(3)}" @click="Prism_single_player_show = !Prism_single_player_show">SP</li>

        <li class="world" :class="{if_selected: check_selected(3, index)}" @click="selected = [3, index], changed = true" 
        v-if="Prism_single_player_show" v-for="(item, index) in paths[3]">

            {{ item.substring(item.lastIndexOf('\\')+1, item.length) }}

        </li>

    </div>


    <div class="refresh">
        <button class="inputsButton" @click="refresh">Refresh</button>
    </div>

    <div class="select">
        <button class="inputsButton" @click="set_world">Select</button>
    </div>
   

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
        //invoke("store_last_world")

        if (changed.value == true) {
            set_world()
        }
        
        appWindow.close()
    
        
    });

    //var selected = ref(["launcher", "type", 0])

    var selected = ref([0, 0]) // 0..3 lists, 0... items in list

    /*var MC_multi_player = ref() //              0
    var MC_single_player = ref() //             1
    var Prism_multi_player = ref() //           2
    var Prism_single_player = ref() //          3*/

    var paths = ref()

    var changed = ref(false)


    var MC_multi_player_show = ref(true) //     0
    var MC_single_player_show = ref(true) //    1
    var Prism_multi_player_show = ref(true) //  2
    var Prism_single_player_show = ref(true) // 3

    refresh()

    async function refresh(){
        paths.value = await invoke("get_world")
        selected.value = await invoke("get_selected")
    }

    /*function check_selected(launcher, type, index) {
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
        }       0123  3
    }*/

    function check_selected(list, index) {
        if (selected.value[0] == list && selected.value[1] == [index]){
            return true
        } else {
            return false
        }
    }

    function check_child_selected(list) {
        if (selected.value[0] == list) {
            return true
        } else {
            return false
        }
    } 

    function check_grand_child_selected(list) {
        if (selected.value[0] > 1 && list > 1) {
            return true
        } else {
            return false
        }
    }

    async function set_world() {

        invoke("set_world", {list: selected.value[0], index: selected.value[1]})

        console.log(selected.value[0], selected.value[1])

        changed.value = false

        emit('REFRESH')

    }

    

</script>
    



<style>
    .refresh {
        position: fixed;
        bottom: 20px;
        left: 20px;
    }
    .select {
        position: fixed;
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