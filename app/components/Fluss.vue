<template>
    <div id="wasm"/>
</template>

<script lang="ts">
    import init, { Foo } from "/pkg/fluss.js";

    export default {
        name: "Fluss",
        mounted() {
            (async function() {
                await init();
                let bar = new Foo("test");
                console.log(bar.name);
            }());
        },
    };
</script>

<style>
    #wasm canvas {
        display: block;
        cursor: none;
    }
</style>

