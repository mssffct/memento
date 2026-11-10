<div>
    <!-- Год рождения -->
    <div style="display: flex; gap: 0.5rem; align-items: center; margin-bottom: 0.5rem;">
    <label>Год рождения:</label>
    <input v-model.number="editing.birth_year" type="number" placeholder="ГГГГ" style="width: 80px;" />
    <select v-model="editing.birth_year_qual">
        <option value="exact">точно</option>
        <option value="approx">приблизительно</option>
        <option value="unknown">неизвестно</option>
    </select>
    </div>
    <!-- Месяц рождения -->
    <div style="display: flex; gap: 0.5rem; align-items: center; margin-bottom: 0.5rem;">
    <label>Месяц:</label>
    <input v-model.number="editing.birth_month" type="number" min="1" max="12" style="width: 60px;" />
    <select v-model="editing.birth_month_qual">
        <option value="exact">точно</option>
        <option value="approx">приблизительно</option>
        <option value="unknown">неизвестно</option>
    </select>
    </div>
    <!-- День рождения -->
    <div style="display: flex; gap: 0.5rem; align-items: center; margin-bottom: 0.5rem;">
    <label>День:</label>
    <input v-model.number="editing.birth_day" type="number" min="1" max="31" style="width: 60px;" />
    <select v-model="editing.birth_day_qual">
        <option value="exact">точно</option>
        <option value="approx">приблизительно</option>
        <option value="unknown">неизвестно</option>
    </select>
    </div>
</div>