<script setup lang="ts">
import UserComponent from "./components/User.vue";
import ButtonSubmit from "./components/HTML/ButtonSubmit.vue";
import Form from "./components/HTML/Form.vue";
import Input from "./components/HTML/Input.vue";
import Modal from "./components/Modal.vue";
//Icons
import AddUserIcon from "./components/Icon/AddUser.vue";
//Type
import type { User } from "./types/User";
//Store
import { useStore } from "./store/store";
// Vue
import { onMounted, reactive, ref, Ref } from "vue";
const user = reactive({ name: "" });
const opened = ref(false);
const users = ref();
const createUser = useStore((store) => store.createUser);
const getAllUser = useStore((store) => store.getAllUser);

onMounted(async () => {
    await getAllUser();
    users.value = useStore((store) => store.Users);
});
const uploadUser = async () => {
    await createUser(user.name);
    user.name = "";
    await getAllUser();
    users.value = useStore((store) => store.Users);
};

const toggleModal = () => {
    opened.value = !opened.value;
};

const updateUsers = (value: Ref<User>[]) => {
    users.value = value;
};
</script>

<template>
    <div class="Container">
        <h1>Usuarios</h1>
        <span class="Icon"
            ><AddUserIcon
                width="24"
                height="24"
                fill="black"
                @click="toggleModal"
        /></span>
        <ul>
            <UserComponent
                v-for="user in users"
                :id="user?.id"
                :key="user?.id"
                :name="user?.name"
                @update:users="updateUsers"
            />
        </ul>
        <Modal
            :opened="opened"
            @update:opened="toggleModal"
        >
            <template v-slot:title>
                <h1>Agregar Usuario</h1>
            </template>
            <Form
                dir="V"
                :action="uploadUser"
            >
                <Input
                    label-text="Nombre"
                    v-model:value="user.name"
                />

                <ButtonSubmit text="Añadir" />
            </Form>
        </Modal>
    </div>
</template>

<style scoped>
.Container {
    padding: 1rem;
    background-color: white;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    width: 50%;

    border-radius: 8px;
    box-shadow: rgba(0, 0, 0, 0.24) 0px 3px 8px;
}
ul {
    list-style: none;
    margin: 1rem 0;
    width: 60%;
}
.Icon {
    cursor: pointer;
    border-radius: 8px;
    transition: 0.3s ease;
    padding: 4px;
    border: none;
}
.Icon:hover {
    background-color: var(--color-opacity);
}
</style>
