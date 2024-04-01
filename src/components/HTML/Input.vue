<script setup lang="ts">
defineProps<{
    labelText?: string;
    value: string;
    type?: string;
}>();

defineEmits<{
    (e: "update:value", value: any): void;
}>();
</script>
<template>
    <div class="input-container">
        <input
            :value="value"
            id="input"
            :type="type ? type : 'text'"
            required="true"
            @input="
                $emit('update:value', ($event.target as HTMLInputElement).value)
            "
        />
        <label
            for="input"
            class="label"
            >{{ labelText }}</label
        >
        <div class="underline"></div>
    </div>
</template>

<style scoped>
.input-container {
    position: relative;
}

.input-container input[type="text"] {
    font-size: 1rem;
    width: 100%;
    border: none;
    border-bottom: 2px solid #ccc;
    padding: 5px 0;
    background-color: transparent;
    outline: none;
}

.input-container .label {
    position: absolute;
    top: 0;
    left: 0;
    color: #ccc;
    transition: all 0.3s ease;
    pointer-events: none;
}

.input-container input[type="text"]:focus ~ .label,
.input-container input[type="text"]:valid ~ .label {
    top: -20px;
    font-size: 16px;
    color: #333;
}

.input-container .underline {
    position: absolute;
    bottom: 0;
    left: 0;
    height: 2px;
    width: 100%;
    background-color: #333;
    transform: scaleX(0);
    transition: all 0.3s ease;
}

.input-container input[type="text"]:focus ~ .underline,
.input-container input[type="text"]:valid ~ .underline {
    transform: scaleX(1);
}
</style>
