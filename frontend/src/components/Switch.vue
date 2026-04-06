<script lang="ts" setup>
import { NSwitch, type GlobalThemeOverrides } from 'naive-ui';
import type { CSSProperties } from 'vue'

const model = defineModel<boolean>();

const switchOverrides: GlobalThemeOverrides['Switch'] = {
  boxShadowFocus: 'none', 
}

function railStyle({ focused, checked }: { focused: boolean; checked: boolean }): CSSProperties {
  const style: CSSProperties = {
    transition: 'all 0.3s cubic-bezier(0.4, 0, 0.2, 1)',
    backgroundColor: checked ? '#00674F' : '#e5e7eb',
  }

  if (focused) {
    const inner = 'inset 0 0 0 1px rgba(0, 0, 0, 0.05)';
    const outer = checked
      ? '0 0 0 3px rgba(0, 103, 79, 0.2)'
      : '0 0 0 3px rgba(156, 163, 175, 0.2)';
    
    style.boxShadow = `${inner}, ${outer}`;
  }

  return style
}
</script>

<template>
  <n-switch 
    v-model:value="model" 
    :theme-overrides="switchOverrides"
    :rail-style="railStyle" 
  />
</template>

<style scoped>
:deep(.n-switch .n-switch__rail) {
    overflow: visible !important;
    border: none !important;
    outline: none !important;
}

/* 3. Remove any focus ring on the actual switch button if it persists */
:deep(.n-switch:focus) {
    outline: none !important;
    box-shadow: none !important;
}
</style>