<template>

  <div id="mapid" class="map" v-on:mousedown.ctrl="boxStart"></div>
  <div class="position">
    X: {{ (lng*128).toFixed(0).replace(/\B(?=(\d{3})+(?!\d))/g, ",") }}<br>
    Z: {{ (-lat*128).toFixed(0).replace(/\B(?=(\d{3})+(?!\d))/g, ",") }}
  </div>


  <div class="inputsBorder">
  
    <div class="inputs">

      
      
      <input type="radio" class="radios" name="type" value="center" v-model="type">
      <label for="center" @click="type = 'center'"> center</label><br>
      <input type="radio" class="radios" name="type" value="span" v-model="type">
      <label for="span" @click="type = 'span'"> span</label><br><br>


      <label for="dimension" >Enter dimension:</label><br>
      <select @change="changeDimension" class="inputsForm" name="dimension" v-model="newdimension">
        <option value="overworld">Overworld</option>
        <option value="the_nether">Nether</option>
        <option value="the_end">End</option>
      </select><br><br>

      <div v-if="type == 'center'">
        <label for="x">Enter X coord:</label><br>
        <input class="inputsForm" type="number" name="x" v-model="x1"><br><br>

        <label for="z">Enter Z coord:</label><br>
        <input class="inputsForm" type="number" name="z" v-model="z1"><br><br>

        <label for="radius">Enter radius:</label><br>
        <input class="inputsForm" type="number" name="radius" max="65024" v-model="radius"><br><br>
        <div v-if="radius > 65024">
          IMPUT VALUE LESS THAN 65024     
        </div>

      </div>

      <div v-if="type == 'span'">

        <label for="x1">Enter X coord 1:</label><br>
        <input class="inputsForm" type="number" name="x1" v-model="x1"><br><br>

        <label for="z1">Enter Z coord 1:</label><br>
        <input class="inputsForm" type="number" name="z1" v-model="z1"><br><br>

        <label for="x2">Enter X coord 2:</label><br>
        <input class="inputsForm" type="number" name="x2" v-model="x2"><br><br>

        <label for="z2">Enter Z coord 2:</label><br>
        <input class="inputsForm" type="number" name="z2" v-model="z2"><br><br>

      </div>

    </div>

    <div class="waypointlist">
      <div class="scrollbar">
        <div v-for="(waypoint, i) in waypoints">
          <div class="waypoint" @click="gotowaypoint(i)">
            {{ waypoint.name }}
          </div>
        </div>
      </div>
    </div>
    


    <div class="calls">
      <button class="inputsButton" @click="goto">Goto</button>
      <button class="inputsButton" @click="stitch">Stitch</button>
      <button class="inputsButton" @click="select_world_window">Select world</button>
    </div>

  </div>



</template>


<script setup>
  

  import { ref, computed, onMounted } from 'vue'
  import leaflet from "leaflet";
  import { invoke } from '@tauri-apps/api';
  //import { WebviewWindow, LogicalSize, PhysicalSize, title  } from '@tauri-apps/api/window'
  import { window } from "@tauri-apps/api"
  import { TauriEvent, emit } from "@tauri-apps/api/event"
  import { appWindow } from '@tauri-apps/api/window';
  import { listen } from '@tauri-apps/api/event'
  import { exit } from '@tauri-apps/api/process';



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
  var z1 = ref(0)

  var x2 = ref(0)
  var z2 = ref(0)

  var lat = ref(0)
  var lng = ref(0)

  var bounds;

  var dimension = ref("overworld")

  var newdimension = ref("overworld")

  var tileUrl;

  var ifCtrl = ref(0)

  var marker; //box
  var selection;

  var reloads = 0

  var waypoints = ref();
  var markers = [];
  var marker_options
  var temp_marker;

  get_waypoints()

  window.getCurrent().listen(TauriEvent.WINDOW_CLOSE_REQUESTED, () => {
    console.log("clicked")
    invoke("store_last_world")
    exit(1);
    
        
  });


  const unlisten = listen('REFRESH', (event) => {

    console.log("refreshed!");

    reloads = reloads + 1
    
    console.log("http://localhost:3000/" + reloads + "/" + dimension.value + "/{z}/{x}/{y}")
    tileUrl.setUrl("http://localhost:3000/" + reloads + "/" + dimension.value + "/{z}/{x}/{y}")

    map.setView([0,0],2)

    get_waypoints()

  })

  async function get_waypoints(){
    //console.log("before loading", waypoints.length,markers.length)
    waypoints.value = await invoke("get_waypoints")


    //console.log("after loading", waypoints.length,markers.length)
    /*for (let i = 0; i<markers.length; i++) {
      console.log(markers[i])
    }*/
    for (let k = 0; k<markers.length;k++) {
      //console.log(k)
      map.removeLayer(markers[k])
      
    }
    markers = []
    //console.log("after deleting", waypoints.length,markers.length)



    for (let i = 0; i<waypoints.value.length;i++) {
      //console.log(waypoints[i].dimensions.length)
      for (let j = 0; j<waypoints.value[i].dimensions.length;j++){
        if (dimension.value == waypoints.value[i].dimensions[j]){

          marker_options = {
            title: (waypoints.value[i].name)
          }

          if (dimension.value == "the_nether"){
            temp_marker = new L.Marker(
              [(-((waypoints.value[i].z)/128)/8), (((waypoints.value[i].x)/128)/8)],
              marker_options
            )
          } else {
            temp_marker = new L.Marker(
              [(-(waypoints.value[i].z)/128), ((waypoints.value[i].x)/128)],
              marker_options
            )
          }
          
          map.addLayer(temp_marker);
          markers.push(temp_marker)
          console.log(waypoints.value[i].name, waypoints.value[i].x, waypoints.value[i].z, waypoints.value[i].dimensions[j], "ADDED!")
          //break
        }
      }
    }


  }

  function gotowaypoint(i) {
    console.log(waypoints.value[i].name, newdimension.value)

    

    newdimension.value = waypoints.value[i].dimensions[0]
    
    
    console.log(newdimension.value)
    changeDimension()

    if (newdimension.value == "the_nether") {
      x1.value = waypoints.value[i].x/8
      z1.value = waypoints.value[i].z/8
    } else {
      x1.value = waypoints.value[i].x
      z1.value = waypoints.value[i].z
    }


    goto()


  }
  
  
  onMounted(() => {
    
    map = leaflet.map('mapid', {
      crs: L.CRS.Simple
    }).setView([x1.value, z2.value], 2);
    
    //leaflet.tileLayer('https://tile.openstreetmap.org/{z}/{x}/{y}.png', {
    tileUrl = L.tileLayer("http://localhost:3000/0/overworld/{z}/{x}/{y}", {
    //leaflet.tileLayer(url.value, {
      maxZoom: 8,
      maxNativeZoom: 6,
      noWrap: true
    }).addTo(map);

    

    map.addEventListener('mousemove', function(ev) {
      lat.value = ev.latlng.lat;
      lng.value = ev.latlng.lng;
    });

  })

  function goto(){
    //map.setZoom(6)
    //var coords = L.latLng(0,0)
    lat.value = -z1.value/128
    lng.value = x1.value/128
    map.setView(L.latLng(lat.value,lng.value), 6)
    //map.panTo(L.latLng((x1.value/go.value),(y1.value/go.value)), 6)
  }

  async function stitch(){
    console.log("stitch")
    await invoke("stitch", {x1: x1.value, y1: z1.value, x2: x2.value, y2: z2.value, radius: radius.value, style: type.value, dimension: dimension.value})
  }

  function changeDimension() {
    if (dimension.value == "overworld" && newdimension.value == "the_nether") {
      
      

      //[lat.value, lng.value] = map.target.getCenter()
      var mapcenter = map.getCenter()

      lat.value = mapcenter.lat/8
      lng.value = mapcenter.lng/8

      //console.log(map.getCenter(), lat.value, lng.value)
      map.setView([lat.value, lng.value])
    } else if (dimension.value == "the_nether" && newdimension.value == "overworld") {

      var mapcenter = map.getCenter()

      lat.value = mapcenter.lat*8
      lng.value = mapcenter.lng*8
      map.setView([lat.value, lng.value])
    } else {

      lat.value = 0
      lng.value = 0
      map.setView([lat.value, lng.value])
    }
    dimension.value = newdimension.value

    reloads = 0
    var url = "http://localhost:3000/" + reloads + "/" + dimension.value + "/{z}/{x}/{y}"
    console.log(url)
    tileUrl.setUrl(url)

    get_waypoints()

  }

  /*export const changeUrl = () => {
    //path.value = new_path
    url = path.value + dimension.value + "/{z}/{x}/{y}"
    console.log(url)
    tileUrl.setUrl(url)
  }*/

  function boxStart() {

    ifCtrl.value = 1

    x1.value = lng.value*128
    z1.value = -lat.value*128

    console.log(lat.value, lng.value, x1.value, z1.value)

    map.dragging.disable()
    
    
    
    marker = new L.Marker([lat.value, lng.value]);
    map.addLayer(marker);

    map.removeLayer(selection)

  
  }

  async function select_world_window() {

    invoke("select_world_window")
 
  }

  document.addEventListener("mouseup",() => {

    if (ifCtrl.value == 1) {


      ifCtrl.value = 0

      x2.value = lng.value*128
      z2.value = -lat.value*128

      map.dragging.enable()

      map.removeLayer(marker)

      if (type.value == "span") {
        

        console.log(lat.value, lng.value, x2.value, z2.value)
        



        

        bounds = [[-z1.value/128,x1.value/128],[-z2.value/128,x2.value/128]]
        //var bounds = [[2.5, 56], [6.7,8.7]]
        console.log(bounds)
        
        
      } else {

        console.log(x2.value, z2.value, radius.value)

        if (Math.abs(x2.value-x1.value) > Math.abs(z2.value-z1.value)){
          radius.value = Math.abs(x2.value-x1.value)
        } else {
          radius.value = Math.abs(z2.value-z1.value)
        }

        var difference = radius.value/128

        bounds = [[(-z1.value/128)-difference, (x1.value/128)-difference],
        [(-z1.value/128)+difference, (x1.value/128)+difference]]

      }

      selection = new L.rectangle(bounds, {color: "#ff7800", weight: 1})
      
      map.addLayer(selection)

    }
    

  })






</script>


<style>

  .waypointlist {
    position: relative; 
    width: 180px; 
    left: 20px;

    margin-top: 20px;
    margin-bottom: 20px;

    font-size: 12px;

    background-color: #ae00ff;
    box-sizing: border-box;
    border: 3px solid #ae00ff;
    border-radius: 5px;
     
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }

  .scrollbar{
    position: relative;
    overflow-y: auto;
    min-height: 20px;
    
  }

  .scrollbar::-webkit-scrollbar {
    width: 12px;               /* width of the entire scrollbar */
  }

  .scrollbar::-webkit-scrollbar-track {
    background: #e0abfb;        /* color of the tracking area */
    -webkit-border-radius: 10px;
    border-radius: 5px;
  }

  .scrollbar::-webkit-scrollbar-thumb {
    background-color: #ae00ff;    /* color of the scroll thumb */
    border-radius: 20px;       /* roundness of the scroll thumb */
    border: 3px solid #e0abfb;  /* creates padding around scroll thumb */
  }

  .scrollbar::-webkit-scrollbar-thumb:active {
    background-color: #01AFE4;   /* color of the scroll thumb */
  }



  .waypoint {
    position: relative;
    background-color: #e0abfb;
    min-height: 20px;
    width: 145px;
    border-radius: 5px;

    padding: 2px;
    padding-left: 7px;

    left: -2px;
    margin: 5px;
  }

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
    user-select: none;
  }

  .position {
    font-family: Verdana, sans-serif;
    background-color: #ae00ff;
    left: 0px;
    bottom: 0px;
    width: 150px;
    height: 40px;
    position: absolute;
    z-index: 400;
    user-select: none;
    border-top-right-radius: 10px;
  }


  .inputsButton{
    border: none;
    background-color: #ae00ff;
    color: white;
    border-radius: 3px;
    width: 100px;
    height: 20px;
    font-size: 13px;
    transition-duration: 0.2s;
    font-family: Verdana, sans-serif;
    margin-top: 10px;

  }

  .inputsButton:hover{
    background-color: #01AFE4;
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
    position: relative;
    left: 20px;
    bottom: 10px;
    width: 100px;

  }

  
  .inputs{
    position: relative;
    left: 20px;
    top: 20px;
    width: 180px;
  }

  .inputsForm{
    border-radius: 3px;
    width: 180px;
    border: 2px solid #ae00ff;
    -webkit-transition: 0.2s;
    transition: 0.2s;
    outline: none;
    box-sizing:border-box
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
    display: flex;
    flex-direction: column;
    right: 0px;
    top: 0px;
    background-color: #e0abfb;
    width: 220px;
    height: 100%;
    font-family: Verdana, sans-serif;
  }

</style>