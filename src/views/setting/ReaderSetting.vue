<template>
    <div class="reader-setting">
        <n-grid :cols="4" :x-gap="16" :y-gap="4">
            <n-gi :span="3">
                <div class="set-item-title">字体大小</div>
                <div class="set-item-desc">设置阅读模式字体大小</div>
            </n-gi>
            <n-gi>
                <n-input-number v-model:value="fontSize" :min="8" :max="72" size="small" />
            </n-gi>
            <n-gi :span="4">
                <n-divider style="margin: 16px 0" />
            </n-gi>
            <n-gi :span="3">
                <div class="set-item-title">窗口阴影</div>
                <div class="set-item-desc">是否显示阅读模式窗口阴影效果</div>
            </n-gi>
            <n-gi>
                <n-switch v-model:value="showShadow" />
            </n-gi>
            <n-gi :span="4">
                <n-divider style="margin: 16px 0" />
            </n-gi>
            <n-gi :span="3">
                <div class="set-item-title">字体颜色</div>
                <div class="set-item-desc">设置阅读模式字体颜色</div>
            </n-gi>
            <n-gi>
                <div class="color-picker-control">
                    <n-color-picker v-model:value="fontColor" show-alpha :modes="['hex']" placement="bottom-end">
                        <template #trigger="{ value, onClick, ref: triggerRef }">
                            <div :ref="triggerRef" class="color-swatch" :style="{ backgroundColor: value || '#000000' }"
                                role="button" tabindex="0" aria-label="选择字体颜色" @click="onClick" @keydown.enter="onClick"
                                @keydown.space.prevent="onClick" />
                        </template>
                    </n-color-picker>
                    <div class="color-values">
                        <span>{{ getColorDetails(fontColor).color }}</span>
                        <small>透明度 {{ getColorDetails(fontColor).opacity }}%</small>
                    </div>
                </div>
            </n-gi>
            <n-gi :span="4">
                <n-divider style="margin: 16px 0" />
            </n-gi>
            <n-gi :span="3">
                <div class="set-item-title">背景颜色/透明度</div>
                <div class="set-item-desc">设置阅读模式背景颜色&透明度</div>
            </n-gi>
            <n-gi>
                <div class="color-picker-control">
                    <n-color-picker v-model:value="backgroundColor" show-alpha :modes="['hex']" placement="bottom-end">
                        <template #trigger="{ value, onClick, ref: triggerRef }">
                            <div :ref="triggerRef" class="color-swatch" :style="{ backgroundColor: value || '#ffffff' }"
                                role="button" tabindex="0" aria-label="选择背景颜色" @click="onClick" @keydown.enter="onClick"
                                @keydown.space.prevent="onClick" />
                        </template>
                    </n-color-picker>
                    <div class="color-values">
                        <span>{{ getColorDetails(backgroundColor).color }}</span>
                        <small>透明度 {{ getColorDetails(backgroundColor).opacity }}%</small>
                    </div>
                </div>
            </n-gi>
            <n-gi :span="4">
                <n-divider style="margin: 16px 0" />
            </n-gi>
            <n-gi :span="3">
                <div class="set-item-title">窗口大小</div>
                <div class="set-item-desc">设置阅读模式窗口大小</div>
            </n-gi>
            <n-gi>
                <n-input-number v-model:value="windowWidth" :min="100" :max="1920" size="small">
                    <template #prefix>
                        宽
                    </template>
                </n-input-number>
                <n-input-number v-model:value="windowHeight" :min="100" :max="1080" size="small">
                    <template #prefix>
                        高
                    </template>
                </n-input-number>
            </n-gi>
        </n-grid>
    </div>
</template>

<script setup lang="ts">
import { useSettingStore } from '@/stores/setting'

const settingStore = useSettingStore()

const fontSize = ref(settingStore.reader.fontSize)
const showShadow = ref(settingStore.reader.showShadow)
const fontColor = ref(settingStore.reader.fontColor)
const backgroundColor = ref(settingStore.reader.backgroundColor)
const windowWidth = ref(settingStore.reader.windowWidth)
const windowHeight = ref(settingStore.reader.windowHeight)

const getColorDetails = (value: string | null) => {
    const color = value || '#000000'
    const match = color.match(/^#([\da-f]{6})([\da-f]{2})$/i)
        || color.match(/^#([\da-f]{3})([\da-f])$/i)
    const alphaMatch = color.match(/^rgba\(\s*\d+\s*,\s*\d+\s*,\s*\d+\s*,\s*([\d.]+)\s*\)$/i)
    const alpha = match
        ? Math.round((parseInt(match[2].length === 1 ? match[2] + match[2] : match[2], 16) / 255) * 100)
        : alphaMatch
            ? Math.round(Number(alphaMatch[1]) * 100)
            : 100

    return {
        color: match ? `#${match[1].toUpperCase()}` : color,
        opacity: Math.min(100, Math.max(0, alpha)),
    }
}

watch(fontSize, (newVal) => {
    settingStore.reader.fontSize = newVal
})
watch(showShadow, (newVal) => {
    settingStore.reader.showShadow = newVal
    //   getCurrentWindow().setShadow(newVal)
})
watch(fontColor, (newVal) => {
    settingStore.reader.fontColor = newVal
})
watch(backgroundColor, (newVal) => {
    settingStore.reader.backgroundColor = newVal
})
watch(windowWidth, (newVal) => {
    settingStore.reader.windowWidth = newVal
})
watch(windowHeight, (newVal) => {
    settingStore.reader.windowHeight = newVal
})
</script>

<style lang="scss" scoped>
.reader-setting {
    padding: 20px;
}

.set-item-title {
    font-size: var(--set-item-title-size);
    font-weight: bold;
}

.set-item-desc {
    font-size: var(--set-item-desc-size);
    color: var(--color-text-secondary);
}

.color-picker-control {
    display: inline-flex;
    flex-direction: column;
    align-items: flex-start;
    gap: 6px;
}

.color-swatch {
    width: 22px;
    height: 22px;
    border: 2px solid #ffffff;
    border-radius: 50%;
    cursor: pointer;
    // background-image: linear-gradient(45deg, #d7d7d7 25%, transparent 25%), linear-gradient(-45deg, #d7d7d7 25%, transparent 25%), linear-gradient(45deg, transparent 75%, #d7d7d7 75%), linear-gradient(-45deg, transparent 75%, #d7d7d7 75%);
    background-position: 0 0, 0 4px, 4px -4px, -4px 0;
    background-size: 8px 8px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);

    &:focus-visible {
        outline: 2px solid var(--color-accent);
        outline-offset: 2px;
    }
}

.color-values {
    display: flex;
    flex-direction: column;
    gap: 1px;
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
    font-size: 12px;
    line-height: 16px;

    small {
        color: var(--color-text-secondary);
        font-family: inherit;
        font-size: 11px;
    }
}
</style>
