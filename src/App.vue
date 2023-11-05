<template>

  <div id="mapid" class="map"></div>


  <div class="inputsBorder">
    <div class="inputs">

      <input type="radio" class="radios" name="type" value="center" v-model="type">
      <label for="center" @click="type = 'center'"> center</label><br>
      <input type="radio" class="radios" name="type" value="span" v-model="type">
      <label for="span" @click="type = 'span'"> span</label><br><br>


      <div v-if="type == 'center'">
        <label for="x">Enter X coord:</label><br>
        <input class="inputsForm" type="number" name="x" v-model="x1"><br><br>

        <label for="y">Enter Y coord:</label><br>
        <input class="inputsForm" type="number" name="y" v-model="y1"><br><br>

        <label for="radius">Enter radius:</label><br>
        <input class="inputsForm" type="number" name="radius" max="65024" v-model="radius"><br><br>
        <div v-if="radius > 65024">
          IMPUT VALUE LESS THAN 65024     
        </div>

      </div>

      <div v-if="type == 'span'">

        <label for="x1">Enter X coord 1:</label><br>
        <input class="inputsForm" type="number" name="x1" v-model="x1"><br><br>

        <label for="y1">Enter Y coord 1:</label><br>
        <input class="inputsForm" type="number" name="y1" v-model="y1"><br><br>

        <label for="x2">Enter X coord 2:</label><br>
        <input class="inputsForm" type="number" name="x2" v-model="x2"><br><br>

        <label for="y2">Enter Y coord 2:</label><br>
        <input class="inputsForm" type="number" name="y2" v-model="y2"><br><br>

      </div>



      
      




    </div>

    <div class="calls">
      <button class="inputsButton" @click="goto">Goto</button>
      <br><br>
      <button class="inputsButton" @click="stitch">Stitch</button>
      <br><br>
      <button class="inputsButton" @click="get_tile">get tile TEST</button>
    </div>

  </div>



  
</template>


<script setup>
  

  import { ref, computed, onMounted } from 'vue'
  import leaflet from "leaflet";
  import { invoke } from '@tauri-apps/api';
  //import { invoke } from '@tauri-apps/api' 
  //import { convertFileSrc } from '@tauri-apps/api/tauri';
  //import { homeDir, join} from '@tauri-apps/api/path';
  //import { save } from "@tauri-apps/api/dialog";
  var type = ref("center")
  var radius = ref(1000)
  var map;

  //var x1 = ref(51.505) //stating x y values also used for span and used as x y for center
  //var y1 = ref(-0.09)
  var x1 = ref(0) //stating x y values also used for span and used as x y for center
  var y1 = ref(0)

  var x2 = ref(0)
  var y2 = ref(0)

  onMounted(() => {
    
    map = leaflet.map('mapid').setView([x1.value, y2.value], 13);
    
    //leaflet.tileLayer('https://tile.openstreetmap.org/{z}/{x}/{y}.png', {
    leaflet.tileLayer("http://localhost:3000/{z}/{x}/{y}", {
      maxZoom: 19,
    }).addTo(map);
  })

  function goto(){
    map.setView([x1.value,y1.value],13)
  }

  async function stitch(){
    console.log("stitch")
    await invoke("stitch", {x1: x1.value, y1: y1.value, x2: x2.value, y2: y2.value, radius: radius.value, style: type.value})
  }

  async function get_tile() {
    console.log("get tile")
    await invoke("get_tile", {x: x1.value, y: y1.value})
  }
</script>


<style>
  .leaflet-control-attribution.leaflet-control {
    display: none;
  }
  .map{
    height: 100vh;
    width: calc(100% - 204px);
    margin: -8px;
    left: 0px;
    right: 0px;
    border: 0px;
    padding: 0px;
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

  .radios[type="radio"]{
    -webkit-appearance: none;
    appearance: none;
    background-color: #fff;
    margin: 0;
    width: 11px;
    height: 11px;
    border: 2px solid #ae00ff;
    border-radius: 50%;
    margin-right: 3px;

    transform: translateY(1px);
    /*transition: 0.4s ease-in-out 0s;*/
    background-color: transparent;
  }

  .radios[type="radio"]:checked{
    border: 2px solid #e0abfb;
    box-shadow: 0 0 0 2px #ae00ff;
    background-color: #ae00ff;

  }

  .calls{
    position: absolute;
    right: 20px;
    bottom: 20px;
    width: 180px;
  }

  .inputsButton:hover{
    background-color: #01AFE4;
  }

  .inputs{
    position: absolute;
    right: 20px;
    top: 20px;
    width: 180px;
  }

  .inputsForm{
    border-radius: 3px;
    border: 2px solid #ae00ff;
    -webkit-transition: 0.2s;
    transition: 0.2s;
    outline: none;
  }

  .inputsForm:focus{
    border: 2px solid #01AFE4;
  }

  input::-webkit-outer-spin-button,
  input::-webkit-inner-spin-button {
    -webkit-appearance: none;
    margin: 0;
  }

  .inputsBorder{
    position: absolute;
    right: 0px;
    top: 0px;
    background-color: #e0abfb;
    width: 220px;
    height: 100%;
    font-family: Verdana, sans-serif;
  }

</style>