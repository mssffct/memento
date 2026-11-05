<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

// Интерфейс, повторяющий Rust-структуру
interface Unit {
  id: number;
  entity_type: string;
  last_name: string | null;
  first_name: string | null;
  middle_name: string | null;
  pet_name: string | null;
  birth_date: string | null;
  gender: string | null;
  photo_path: string | null;
}

const password = ref("test123");
const units = ref<Unit[]>([]);
const selectedId = ref<number | null>(null);
const isLoading = ref(false);
const error = ref("");

// Редактируемая копия выбранного индивида
const editing = ref<Unit | null>(null);

// Вычисляемый выбранный индивид из списка
const selectedUnit = computed(() => {
  return units.value.find(i => i.id === selectedId.value) || null;
});

// Загрузка данных при старте (или по кнопке)
async function loadData() {
  isLoading.value = true;
  error.value = "";
  try {
    // Сначала инициализируем/проверяем схему
    await invoke("init_and_test_db", { password: password.value });
    // Потом получаем полный список
    const list = await invoke<Unit[]>("get_all_units", { password: password.value });
    units.value = list;
    if (list.length > 0 && selectedId.value === null) {
      selectedId.value = list[0].id;
    }
    updateEditingCopy();
  } catch (e) {
    error.value = String(e);
  } finally {
    isLoading.value = false;
  }
}

// При выборе другого индивида — копируем данные в редактируемую форму
function updateEditingCopy() {
  if (selectedUnit.value) {
    // Глубокое клонирование, чтобы изменения не летели сразу в список
    editing.value = JSON.parse(JSON.stringify(selectedUnit.value));
  } else {
    editing.value = null;
  }
}

// Заглушка для фото
const defaultPhoto = computed(() => {
  if (!editing.value) return "";
  if (editing.value.entity_type === "pet") return "/default-pet.png"; // Ваша картинка питомца
  if (editing.value.gender === "female") return "/default-female.png"; // Если есть
  return "/default-male.png"; // Ваша картинка человека
});

// Сохранение (пока просто выводим в консоль, так как команды сохранения ещё нет)
function saveUnit() {
  console.log("Сохранение:", editing.value);
  alert("Сохранение пока не реализовано на бэкенде. Данные в консоли.");
}

// Выбор индивида
function selectUnit(id: number) {
  selectedId.value = id;
  updateEditingCopy();
}

onMounted(() => {
  loadData();
});
</script>

<template>
  <main style="padding: 1rem; font-family: sans-serif; display: flex; gap: 2rem;">
    <div v-if="error" style="color: red; position: absolute; top: 0; left: 0; background: #fee; padding: 0.5rem; width: 100%;">
    Ошибка: {{ error }}
    </div>
    <!-- Левая панель: список -->
    <aside style="width: 250px; border-right: 1px solid #ccc; padding-right: 1rem;">
      <h3>Список</h3>
      <button @click="loadData" :disabled="isLoading">
        {{ isLoading ? "Загрузка..." : "Обновить" }}
      </button>
      <ul style="list-style: none; padding: 0;">
        <li 
          v-for="ind in units" 
          :key="ind.id"
          @click="selectUnit(ind.id)"
          style="cursor: pointer; padding: 0.5rem; border-bottom: 1px solid #eee;"
          :style="{ backgroundColor: ind.id === selectedId ? '#e0e0e0' : 'transparent' }"
        >
          {{ ind.entity_type === 'pet' ? ind.pet_name : `${ind.last_name || ''} ${ind.first_name || ''}` }}
        </li>
      </ul>
    </aside>

    <!-- Правая панель: карточка -->
    <section v-if="editing" style="flex: 1;">
      <h2>Карточка</h2>
      
      <!-- Фото -->
      <div style="margin-bottom: 1rem;">
        <img 
          :src="editing.photo_path || defaultPhoto" 
          alt="Фото"
          style="width: 150px; height: 150px; object-fit: cover; border: 1px solid #ccc; border-radius: 8px;"
        />
        <p><small>Фото: пока заглушка (поле photo_path пустое)</small></p>
      </div>

      <!-- Тип -->
      <div style="margin-bottom: 1rem;">
        <label>Тип: </label>
        <select v-model="editing.entity_type">
          <option value="person">Человек</option>
          <option value="pet">Питомец</option>
        </select>
      </div>

      <!-- Поля для Человека -->
      <div v-if="editing.entity_type === 'person'" style="display: flex; flex-direction: column; gap: 0.5rem; margin-bottom: 1rem;">
        <label>Фамилия: <input v-model="editing.last_name" type="text" /></label>
        <label>Имя: <input v-model="editing.first_name" type="text" /></label>
        <label>Отчество: <input v-model="editing.middle_name" type="text" /></label>
      </div>

      <!-- Поля для Питомца -->
      <div v-if="editing.entity_type === 'pet'" style="margin-bottom: 1rem;">
        <label>Кличка: <input v-model="editing.pet_name" type="text" /></label>
      </div>

      <!-- Общие поля -->
      <div style="display: flex; flex-direction: column; gap: 0.5rem; margin-bottom: 1rem;">
        <label>Дата рождения (ГГГГ-ММ-ДД): 
          <input v-model="editing.birth_date" type="date" />
        </label>
        <label>Пол:
          <select v-model="editing.gender">
            <option value="male">Мужской</option>
            <option value="female">Женский</option>
            <option value="other">Другой</option>
          </select>
        </label>
      </div>

      <button @click="saveUnit" style="padding: 0.5rem 1rem; font-size: 1rem;">
        Сохранить (заглушка)
      </button>
      
      <div v-if="error" style="color: red; margin-top: 1rem;">{{ error }}</div>
    </section>
    <section v-else>
      <p>Выберите запись слева или нажмите «Обновить».</p>
    </section>
  </main>
</template>