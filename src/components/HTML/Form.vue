<script lang="ts" setup>
defineProps<{
    action: () => any;
    dir: "V" | "H";
}>();
</script>

<template>
    <form
        :class="dir"
        @submit.prevent="action"
    >
        <slot />
    </form>
</template>

<style scoped>
form {
    width: 100%;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    gap: 1.5rem;
}

.H {
    display: flex;
    flex-direction: row;
}
.V {
    display: flex;
    flex-direction: column;
}
</style>
