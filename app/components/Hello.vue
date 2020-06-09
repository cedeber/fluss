<template>
    <p>Hello, {{ label }} - {{ name }}</p>
</template>

<script lang="ts">
    import {ref, computed} from "vue";

    export default {
        props: {
            name: String,
        },
        setup(props) {
            let label = computed(() => props.name);
            return {label}
        }
    }
</script>