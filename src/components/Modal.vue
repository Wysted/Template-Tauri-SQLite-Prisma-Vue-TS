<script setup lang="ts">
//Icon
import Close from "./Icon/Close.vue";
//Vue
import { onUpdated, watch, toRef, ref, Ref, onBeforeUnmount } from "vue";

const props = defineProps<{
    opened: boolean;

    fn?: () => any;
}>();

const opened = toRef(props, "opened");
const scroll = ref(0);
const content: Ref<HTMLElement | null> = ref(null);
const asideEl: Ref<HTMLElement | null> = ref(null);
const emits = defineEmits(["update:opened"]);
watch(opened, (newValue) => {
    if (!newValue) {
        document.body.classList.remove("Modal__open");
    } else {
        document.body.classList.add("Modal__open");
    }
});

onUpdated(() => {
    const B = document.body;
    let D = document.documentElement;
    D = D.clientHeight ? D : B;

    scroll.value = D.scrollTop;

    if (asideEl.value) {
        asideEl.value.style.setProperty("--top", `${scroll.value}px`);
    }
});

onBeforeUnmount(() => {
    document.body.classList.remove("Modal__open");
});

const closeModal = () => {
    if (props.fn) props.fn();
    emits("update:opened", false);
};
</script>

<template>
    <Teleport to="body">
        <aside
            v-if="opened"
            ref="asideEl"
            class="Modal"
        >
            <div
                ref="content"
                class="Modal__container"
            >
                <section>
                    <header class="Modal__header">
                        <slot name="title" />
                        <Close
                            width="32"
                            height="32"
                            fill="black"
                            class="fa-solid fa-xmark Icon"
                            @click="() => closeModal()"
                        />
                    </header>
                    <section class="Modal__content">
                        <slot />
                    </section>
                </section>
            </div>
        </aside>
    </Teleport>
</template>

<style scoped>
.Modal {
    position: absolute;
    width: 100%;
    height: 100%;
    z-index: 99999;
    background: rgba(0, 0, 0, 0.4);
    display: flex;
    justify-content: center;
    left: 0;
    align-items: center;
    top: var(--top);
}

.Modal__container {
    background-color: white;
    max-height: 100%;
    position: absolute;
    top: 30px;
    overflow-y: auto;
    overflow-x: hidden;
    width: 45%;
    border-radius: 4px;
    animation: modal 0.4s;
}

.Modal__header {
    padding: 1rem;
    display: flex;
    justify-content: center;
    align-items: center;
}

.Modal__header i {
    position: absolute;
    right: 20px;
    top: 20px;
    font-size: 1.1rem;
}
.Icon {
    cursor: pointer;
}
.Modal__content {
    display: flex;
    width: 100%;
    flex-direction: column;
    padding: 30px;
    box-sizing: border-box;
}

@keyframes modal {
    0% {
        opacity: 0;
    }
    100% {
        opacity: 1;
    }
}

@media (max-width: 500px) {
    .Modal__container {
        width: 90%;
    }

    .Modal__header {
        padding: 15px 5px;
    }

    .Modal__content {
        padding: 0 20px;
        padding-bottom: 30px;
    }
}

@media (min-width: 500px) and (max-width: 575.98px) {
    .Modal__container {
        width: 80%;
    }

    .Modal__content {
        padding: 20px;
    }
}

@media (min-width: 575.98px) and (max-width: 750px) {
    .Modal__container {
        width: 80%;
    }

    .Modal__content {
        padding: 20px;
    }
}
</style>
