// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: '2025-05-15',
  devtools: { enabled: true },

  // Configure for Tauri
  ssr: false,
  components: {
    global: true,
    dirs: ['~/components']
  },

  typescript: {
    typeCheck: false
  },

  vite: {
    server: {
      strictPort: true
    },
    envPrefix: ['VITE_', 'TAURI_']
  },

  css: ['@/main.css'],

  modules: ['@nuxt/eslint', '@nuxt/fonts', '@nuxt/icon', '@nuxt/ui', '@pinia/nuxt', 'v-gsap-nuxt'],

  icon: {
    mode: 'svg',
    clientBundle: {
      scan: true,
      sizeLimitKb: 512
    },
    serverBundle: 'local'
  },

  // Enhanced color mode configuration
  colorMode: {
    preference: 'dark',
    fallback: 'dark',
    hid: 'nuxt-color-mode-script',
    globalName: '__NUXT_COLOR_MODE__',
    componentName: 'ColorScheme',
    classSuffix: '',
    classPrefix: '',
    storageKey: 'nuxt-color-mode'
  }
});
