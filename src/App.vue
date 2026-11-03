<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const password = ref("test123");
const names = ref<string[]>([]);
const error = ref("");
const isLoading = ref(false);

async function openDatabase() {
  isLoading.value = true;
  error.value = "";
  names.value = [];

  try {
    // Вызов нашей Rust-команды
    const result = await invoke<string[]>("init_and_test_db", {
      password: password.value,
    });
    names.value = result;
  } catch (e) {
    error.value = String(e);
  } finally {
    isLoading.value = false;
  }
}
</script>

<template>
  <main style="padding: 2rem; font-family: sans-serif">
    <h1>Семейный архив (Vue)</h1>

    <div style="margin-bottom: 1rem">
      <label>
        Пароль для базы:
        <input v-model="password" type="password" style="margin-left: 0.5rem" />
      </label>
    </div>

    <button @click="openDatabase" :disabled="isLoading">
      {{ isLoading ? "Загрузка..." : "Открыть/создать БД" }}
    </button>

    <div v-if="error" style="color: red; margin-top: 1rem">
      Ошибка: {{ error }}
    </div>

    <div v-if="names.length > 0" style="margin-top: 1rem">
      <h3>Люди в базе:</h3>
      <ul>
        <li v-for="name in names" :key="name">{{ name }}</li>
      </ul>
    </div>
  </main>
</template>
