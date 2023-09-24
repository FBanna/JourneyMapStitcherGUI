<template>
  <p>JourneyMapStitcherGUI</p>
  <div class="grid">

  </div>
  <div class="inputsBorder">
    <div class="inputs">




      

      <label for="x">Enter X coord:</label><br>
      <input class="inputsForm" type="number" name="x" value="0"><br><br>

      <label for="y">Enter Y coord:</label><br>
      <input class="inputsForm" type="number" name="y" value="0"><br><br>

      <button class="inputsButton">Goto</button>
      <br><br>
      <button class="inputsButton">Stitch</button>
      




    </div>
  </div>



  
</template>


<script setup>
  

  import { ref, computed } from 'vue'
  //import { invoke } from '@tauri-apps/api' 
  //import { convertFileSrc } from '@tauri-apps/api/tauri';
  //import { homeDir, join} from '@tauri-apps/api/path';
  //import { save } from "@tauri-apps/api/dialog";

</script>


<style>

  .inputsButton{
    border: none;
    background-color: #ae00ff;
    color: white;
    border-radius: 3px;
    width: 50px;
    height: 20px;
    font-size: 13px;
    transition-duration: 0.2s;
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
    height: 100vh;
  }


  .grid {
    display: grid;
    grid-template-columns: min-content min-content min-content min-content;
    gap: 1px;
    background-color: #01AFE4;
    user-select: initial;

  }

  .grid > div {
    background-color: rgba(255, 255, 255, 0.8);
    border: 3px solid black;
    text-align: center;
    font-size: 5px;
    user-select: initial;
  }

</style>