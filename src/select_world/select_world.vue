<template>

    <div class="tree">
        <ul v-for="(launcher, launchernum) in launchers">
            
            <li :class="{if_selected: check_grand_child_selected(launchernum), collapsed: show[launchernum][2]}" @click="show[launchernum][2] = !show[launchernum][2]">{{launcher}} Launcher</li>
            

            <ul v-if="show[launchernum][2]" v-for="(type, typenum) in types">

                <li :class="{if_selected: check_child_selected(launchernum, typenum), collapsed: show[launchernum][typenum]}" @click="show[launchernum][typenum] = !show[launchernum][typenum]">{{ type }}</li>
            

                <ul v-if="show[launchernum][typenum]" v-for="(item, index) in paths[launchernum][typenum]">

                    <li @click="selected = [launchernum, typenum, index], changed = true" :class="{if_selected: check_selected(launchernum, typenum, index)}">
                    {{ item.substring(item.lastIndexOf('\\')+1, item.length) }}</li>

                </ul>
            </ul>  
        </ul>
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
    var launchers = ["Vanilla", "Prism"]
    var types = ["MP", "SP"]
    // 00 01 20 21
    // 0, 1, 2, 3
    // 0, 0, 1, 1
    // 0, 1, 0, 1

    var show = ref([[true, true, true], [true, true, true]]) // types within launchers

    var selected = ref([0, 0, 0]) // launchers, types, index

    /*var MC_multi_player = ref() //              0
    var MC_single_player = ref() //             1
    var Prism_multi_player = ref() //           2
    var Prism_single_player = ref() //          3*/

    var paths = ref() // [[],[]]

    var changed = ref(false)

    refresh()

    async function refresh(){
        paths.value = await invoke("get_world")
        selected.value = await invoke("get_selected")
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
            console.log("true", launcher)
            return true
        } else {
            return false
        }
    }

    /*function check_selected(list, index) {
        if (selected.value[0] == list && selected.value[1] == [index]){
            return true
        } else {
            return false
        }
    }

    function check_child_selected(launcher, type) {
        var list = listNum(launcher, type)
        console.log(list, launcher, type)
        if (selected.value[0] == list) {
            return true
        } else {
            return false
        }
    } 

    function check_grand_child_selected(list) {
        // 0, 1, 2
        // 
        if (selected.value[0] > 1 && list > 1) {
            return true
        } else {
            return false
        }
    }*/

    async function set_world() {

        invoke("set_world", {launcher: selected.value[0], list: selected.value[1], index: selected.value[2]})

        console.log(selected.value[0], selected.value[1], selected.value[2])

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

    .if_selected {
        color: rgb(11, 219, 11);
        
    }

    .tree {
        font-family: Verdana, sans-serif;
        font-size: 13px;
        user-select: none;
    }

    ul {
        list-style-type: "➤ ";

    }

    ul ul {
        list-style-type: "➤ ";
    }

    ul ul ul {
        list-style-type: disc;
    }

    .collapsed {
        list-style-type: "⮟ ";
    }




</style>