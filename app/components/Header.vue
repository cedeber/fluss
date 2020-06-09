<template>
    <header class="page-header--header">
        <div class="page-header--container"></div>
        <a class="page-header--logo swsh has-font-timberline" href="/">
            <div class="page-header--logo--text">
                <span class="ss01">E</span>ukoli<span class="ss02">a</span>&nbsp;
                <span class="ss02">D</span>esig<span class="fina">n</span>
            </div>
            <div class="page-header--subtitle has-thin-font"></div>
        </a>
        <div class="page-header--container"></div>
    </header>
</template>

<script lang="ts">
    export default {
        name: "Header"
    }
</script>

<style scoped>
    .page-header--header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 7px;
        background: rgba(255, 255, 255, 0.03);
        position: absolute;
        z-index: 99;
        top: 0;
        left: 0;
        width: 100%;
        color: white;
    }

    .page-header--logo {
        font-size: 28px;
        line-height: 1;
        text-decoration: none;
        display: flex;
        flex-direction: column;
        text-align: center;
    }

    .page-header--logo--text {
        margin: 8px 0;
    }

    .page-header--container {
        position: relative;
        margin: 8px;
    }

    .page-header--subtitle {
        font-family: UniNeue, sans-serif;
        font-size: 16px;
        text-transform: uppercase;
    }
</style>