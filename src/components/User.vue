<script setup lang="ts">
//Type
import type { User } from "../types/User";
import type { Ref } from "vue";
//Icon
import DeleteIcon from "./Icon/Delete.vue";
import CheckIcon from "./Icon/Check.vue";
import UpdateIcon from "./Icon/Update.vue";
import CancelIcon from "./Icon/Cancel.vue";
//Store
import { useStore } from "../store/store";
// Vue
import { reactive, ref } from "vue";
//Component
import Form from "./HTML/Form.vue";
import Input from "./HTML/Input.vue";

const props = defineProps<{
    id: number;
    name: string;
}>();

const deleteUser = useStore((store) => store.deleteUser);
const updateUser = useStore((store) => store.updateUser);
const getAllUser = useStore((store) => store.getAllUser);

const user = reactive({ name: "", id: "" });
const update = ref(false);

const emit = defineEmits<{
    (e: "update:users", value: Ref<User>[]): void;
}>();

const delUser = async () => {
    await deleteUser(props.id);
    await getAllUser();
    const data = useStore((store) => store.Users);
    emit("update:users", data);
};

const updUser = () => {
    update.value = !update.value;
};

const submit = async () => {
    await updateUser(props.id, user.name);
    await getAllUser();
    const data = useStore((store) => store.Users);
    emit("update:users", data);
};
</script>
<template>
    <li v-if="id">
        <div
            v-if="!update"
            class="Container"
        >
            <header>
                <span> {{ name }} </span>
            </header>
            <div class="Icons">
                <span class="Icons_item"
                    ><UpdateIcon
                        width="24"
                        height="24"
                        fill="black"
                        @click="() => updUser()"
                    /> </span
                ><span class="Icons_item"
                    ><DeleteIcon
                        width="24"
                        height="24"
                        fill="black"
                        @click="() => delUser()"
                /></span>
            </div>
        </div>
        <Form
            dir="H"
            v-else
            :action="submit"
        >
            <Input
                class="Input"
                label-text="Nombre"
                v-model:value="user.name"
            />
            <button type="submit">
                <CheckIcon
                    class="Icons_item"
                    width="24"
                    height="24"
                    fill="black"
                />
                <CancelIcon
                    class="Icons_item"
                    width="24"
                    height="24"
                    fill="black"
                    @click="() => updUser()"
                />
            </button>
        </Form>
    </li>
</template>

<style scoped>
button {
    background: transparent;
    border: none;
    cursor: pointer;
    border-radius: 8px;
    transition: 0.3s ease;
    display: flex;
}

.Input {
    font-size: 1rem;
}
li {
    display: flex;
    justify-self: center;
    align-items: center;
    justify-content: space-around;
    font-size: 2em;
}
span {
    display: flex;
    justify-content: center;
    align-items: center;
}
.Icons {
    display: flex;
    gap: 4px;
}
.Icons_item {
    cursor: pointer;
    border-radius: 8px;
    transition: 0.3s ease;
    border: none;
}
.Icons_item:hover {
    background-color: var(--color-opacity);
}
.Container {
    width: 100%;
    height: 100%;
    display: flex;
    justify-content: space-around;
}
</style>
