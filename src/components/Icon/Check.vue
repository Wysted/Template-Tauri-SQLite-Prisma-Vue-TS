<script setup lang="ts">
withDefaults(
    defineProps<{
        width: string;
        height: string;
        fill: string;
    }>(),
    { width: "24", height: "24", fill: "black" }
);
</script>
<template>
    <svg
        xmlns="http://www.w3.org/2000/svg"
        :width="width"
        :height="height"
        viewBox="0 0 24 24"
    >
        <path
            :fill="fill"
            d="M9 16.17L4.83 12l-1.42 1.41L9 19L21 7l-1.41-1.41z"
        />
    </svg>
</template>
