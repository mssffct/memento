<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { Unit } from "./types/unit";
import UnitCard from "./components/UnitCard.vue";

const password = ref("test123");
const units = ref<Unit[]>([]);
const selectedId = ref<number | null>(null);
const isLoading = ref(false);
const error = ref("");

const selectedUnit = computed(
  () => units.value.find(i => i.id === selectedId.value) ?? null
);

async function loadData() {
  isLoading.value = true;
  error.value = "";
  try {
    await invoke("init_and_test_db", { password: password.value });
    const list = await invoke<Unit[]>("get_all_units", { password: password.value });
    units.value = list;
    if (list.length > 0 && selectedId.value == null) {
      selectedId.value = list[0].id;
    }
  } catch (e) {
    error.value = String(e);
  } finally {
    isLoading.value = false;
  }
}

async function handleSave(updated: Unit) {
  try {
    const newId = await invoke<number>("save_unit", {
      password: password.value,
      unit: updated,
    });
    // Обновляем локальный список: если id был 0 — назначаем новый
    const idx = units.value.findIndex(i => i.id === updated.id);
    const saved: Unit = { ...updated, id: newId };
    if (idx >= 0) {
      units.value[idx] = saved;
    } else {
      units.value.push(saved);
    }
    selectedId.value = newId;
  } catch (e) {
    error.value = String(e);
  }
}

function selectUnit(id: number) {
  selectedId.value = id;
}

onMounted(loadData);
</script>

<template>
  <main style="padding: 1rem; font-family: sans-serif; display: flex; gap: 2rem;">
    <div v-if="error" style="color: red; position: absolute; top: 0; left: 0; background: #fee; padding: 0.5rem; width: 100%;">
      Ошибка: {{ error }}
    </div>

    <aside style="width: 250px; border-right: 1px solid #ccc; padding-right: 1rem;">
      <h3>Список</h3>
      <button @click="loadData" :disabled="isLoading">
        {{ isLoading ? "Загрузка..." : "Обновить" }}
      </button>
      <ul style="list-style: none; padding: 0;">
        <li
          v-for="u in units"
          :key="u.id"
          @click="selectUnit(u.id)"
          style="cursor: pointer; padding: 0.5rem; border-bottom: 1px solid #eee;"
          :style="{ backgroundColor: u.id === selectedId ? '#e0e0e0' : 'transparent' }"
        >
          {{ u.entity_type === "pet" ? u.pet_name : `${u.last_name ?? ""} ${u.first_name ?? ""} ${u.id}` }}
        </li>
      </ul>
    </aside>

    <UnitCard
      v-if="selectedUnit"
      :unit="selectedUnit"
      :password="password"
      @save="handleSave"
    />
    <section v-else>
      <p>Выберите запись слева или нажмите «Обновить».</p>
    </section>
    
  </main>
</template>