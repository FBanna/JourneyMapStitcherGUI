<template>
  <p>JourneyMapStitcherGUI</p>
  <div class="grid">

  </div>
  <div class="inputsBorder">
    <div class="inputs">
      <label for="x">Enter X coord:</label><br>
      <input type="number" name="x" value="0"><br><br>

      <label for="y">Enter Y coord:</label><br>
      <input type="number" name="y" value="0"><br><br>

      <button>Goto</button>
      <br><br>
      <button>Stitch</button>
      

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
  .inputs{
    position: absolute;
    right: 20px;
    top: 20px;
    width: 180px;
  }

  .inputsBorder{
    position: absolute;
    right: 0px;
    top: 0px;
    background-color: #2196F3;
    width: 220px;
    height: 100vh;
  }


  .grid {
    display: grid;
    grid-template-columns: min-content min-content min-content min-content;
    gap: 1px;
    background-color: #2196F3;
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