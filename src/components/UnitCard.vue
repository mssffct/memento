<script setup lang="ts">
import { ref, computed, watch } from "vue";
import type { Unit } from "../types/unit";
import { formatFuzzyDate } from "../utils/date";

const props = defineProps<{ unit: Unit; password: string }>();
const emit = defineEmits<{
  (e: "save", value: Unit): void;
}>();

// Локальная копия для редактирования
const editing = ref<Unit>({...props.unit});

// Если родитель выбрал другого Unit — обновляем копию
watch(
  () => props.unit,
  (newUnit) => {
    editing.value = {...newUnit};
  },
  { deep: true }
);

// Отображаемая дата рождения / годы жизни
const birthDisplay = computed(() =>
  formatFuzzyDate(
    editing.value.birth_year, editing.value.birth_year_qual,
    editing.value.birth_month, editing.value.birth_month_qual,
    editing.value.birth_day, editing.value.birth_day_qual,
  )
);

const deathDisplay = computed(() =>
  formatFuzzyDate(
    editing.value.death_year, editing.value.death_year_qual,
    editing.value.death_month, editing.value.death_month_qual,
    editing.value.death_day, editing.value.death_day_qual,
  )
);

const hasDeathDate = computed(
  () => editing.value.death_year != null && editing.value.death_year_qual !== "unknown"
);

// Заглушка фото
const photoSrc = computed(() => {
  if (editing.value.photo_path) return editing.value.photo_path;
  if (editing.value.entity_type === "pet") return "/default-pet.png";
  if (editing.value.gender === "female") return "/default-female.png";
  return "/default-male.png";
});

function onSave() {
  emit("save",{...editing.value});
}
</script>

<template>
  <section style="flex: 1;">
    <h2>Карточка Unit</h2>

    <!-- Фото -->
    <img
      :src="photoSrc"
      alt="Фото"
      style="width: 150px; height: 150px; object-fit: cover; border: 1px solid #ccc; border-radius: 8px;"
    />

    <!-- Тип -->
    <div style="margin: 1rem 0;">
      <label>Тип: </label>
      <select v-model="editing.entity_type">
        <option value="person">Человек</option>
        <option value="pet">Питомец</option>
      </select>
    </div>

    <!-- Человек -->
    <div v-if="editing.entity_type === 'person'" style="display: flex; flex-direction: column; gap: 0.5rem; margin-bottom: 1rem;">
      <label>Фамилия: <input v-model="editing.last_name" type="text" /></label>
      <label>Имя: <input v-model="editing.first_name" type="text" /></label>
      <label>Отчество: <input v-model="editing.middle_name" type="text" /></label>
    </div>

    <!-- Питомец -->
    <div v-if="editing.entity_type === 'pet'" style="margin-bottom: 1rem;">
      <label>Кличка: <input v-model="editing.pet_name" type="text" /></label>
    </div>

    <!-- Пол -->
    <div style="margin-bottom: 1rem;">
      <label>Пол:
        <select v-model="editing.gender">
          <option value="male">Мужской</option>
          <option value="female">Женский</option>
          <option value="other">Другой</option>
        </select>
      </label>
    </div>

    <!-- Дата рождения -->
    <fieldset style="margin-bottom: 1rem;">
      <legend>Дата рождения</legend>
      <div style="display: flex; gap: 0.5rem; align-items: center; margin-bottom: 0.3rem;">
        <label>Год:</label>
        <input v-model.number="editing.birth_year" type="number" placeholder="ГГГГ" style="width: 80px;" />
        <select v-model="editing.birth_year_qual">
          <option value="exact">точно</option>
          <option value="approx">приблизительно</option>
          <option value="unknown">неизвестно</option>
        </select>
      </div>
      <div style="display: flex; gap: 0.5rem; align-items: center; margin-bottom: 0.3rem;">
        <label>Месяц:</label>
        <input v-model.number="editing.birth_month" type="number" min="1" max="12" style="width: 60px;" />
        <select v-model="editing.birth_month_qual">
          <option value="exact">точно</option>
          <option value="approx">приблизительно</option>
          <option value="unknown">неизвестно</option>
        </select>
      </div>
      <div style="display: flex; gap: 0.5rem; align-items: center;">
        <label>День:</label>
        <input v-model.number="editing.birth_day" type="number" min="1" max="31" style="width: 60px;" />
        <select v-model="editing.birth_day_qual">
          <option value="exact">точно</option>
          <option value="approx">приблизительно</option>
          <option value="unknown">неизвестно</option>
        </select>
      </div>
    </fieldset>

    <!-- Дата смерти -->
    <fieldset style="margin-bottom: 1rem;">
      <legend>Дата смерти</legend>
      <div style="display: flex; gap: 0.5rem; align-items: center; margin-bottom: 0.3rem;">
        <label>Год:</label>
        <input v-model.number="editing.death_year" type="number" placeholder="ГГГГ" style="width: 80px;" />
        <select v-model="editing.death_year_qual">
          <option value="exact">точно</option>
          <option value="approx">приблизительно</option>
          <option value="unknown">неизвестно</option>
        </select>
      </div>
      <div style="display: flex; gap: 0.5rem; align-items: center; margin-bottom: 0.3rem;">
        <label>Месяц:</label>
        <input v-model.number="editing.death_month" type="number" min="1" max="12" style="width: 60px;" />
        <select v-model="editing.death_month_qual">
          <option value="exact">точно</option>
          <option value="approx">приблизительно</option>
          <option value="unknown">неизвестно</option>
        </select>
      </div>
      <div style="display: flex; gap: 0.5rem; align-items: center;">
        <label>День:</label>
        <input v-model.number="editing.death_day" type="number" min="1" max="31" style="width: 60px;" />
        <select v-model="editing.death_day_qual">
          <option value="exact">точно</option>
          <option value="approx">приблизительно</option>
          <option value="unknown">неизвестно</option>
        </select>
      </div>
    </fieldset>

    <!-- Отображение -->
    <div style="margin-bottom: 1rem; padding: 0.5rem; background: #f5f5f5;">
      <div v-if="!hasDeathDate"><strong>Дата рождения:</strong> {{ birthDisplay }}</div>
      <div v-else><strong>Годы жизни:</strong> {{ birthDisplay }} — {{ deathDisplay }}</div>
    </div>

    <button @click="onSave" style="padding: 0.5rem 1rem; font-size: 1rem;">Сохранить</button>
  </section>
</template>