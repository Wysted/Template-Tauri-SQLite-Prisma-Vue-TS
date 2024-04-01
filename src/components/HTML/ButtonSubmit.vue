<script setup lang="ts">
defineProps<{
    text?: string;
}>();
</script>
<template>
    <button type="submit">
        {{ text }}
    </button>
</template>

<style scoped>
button {
    font-size: 1rem;
    align-items: center;
    min-width: 150px;
    height: 2.5rem;
    border-radius: 0.4rem;
    font-weight: 600;
    padding: 0 1.2rem;
    color: white;
    border: none;
    cursor: pointer;
    background: #000000;
}

button:hover {
    background: #2b2a2a;
}
</style>
