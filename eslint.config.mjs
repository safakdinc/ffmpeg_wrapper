// eslint.config.mjs
import withNuxt from './.nuxt/eslint.config.mjs'; // This is provided by the @nuxt/eslint module
import pluginVue from 'eslint-plugin-vue';
import globals from 'globals';
import typescriptEslintParser from '@typescript-eslint/parser'; // For parsing TypeScript in .vue files
import typescriptEslintPlugin from '@typescript-eslint/eslint-plugin'; // For TypeScript specific rules
import prettierPlugin from 'eslint-plugin-prettier';
import eslintConfigPrettier from 'eslint-config-prettier'; // To disable ESLint rules that conflict with Prettier

export default withNuxt(
  // Your custom flat configs go here
  {
    // Apply Vue recommended rules
    ...pluginVue.configs['flat/recommended']
  },
  {
    files: ['**/*.{js,jsx,mjs,cjs,ts,tsx,vue}'],
    // Add plugins
    plugins: {
      '@typescript-eslint': typescriptEslintPlugin,
      prettier: prettierPlugin // Use the prettier plugin
    },
    rules: {
      // Extend TypeScript recommended rules
      ...typescriptEslintPlugin.configs['recommended'].rules,
      // Formatting rules based on Prettier settings
      'prettier/prettier': [
        'error',
        {
          singleQuote: true,
          printWidth: 100,
          // jsxSingleQuote: false, // Default is false
          // bracketSpacing: true, // Default is true
          // jsxBracketSameLine: false, // Deprecated in Prettier 2.x, use bracketSameLine
          trailingComma: 'none',
          arrowParens: 'avoid',
          endOfLine: 'auto'
          // Consider using bracketSameLine if you need this
          // bracketSameLine: false,
        }
      ],
      // Additional rules can be added here
      '@typescript-eslint/no-explicit-any': 'warn',
      '@typescript-eslint/no-unused-vars': [
        'error',
        {
          argsIgnorePattern: '^_',
          varsIgnorePattern: '^_'
        }
      ],
      // Nuxt-specific adjustments if needed, though @nuxt/eslint usually handles these
      'vue/multi-word-component-names': 'off' // Often useful in Nuxt for pages/layouts
    },
    languageOptions: {
      // Use the TypeScript parser for all relevant files
      parser: typescriptEslintParser,
      parserOptions: {
        ecmaVersion: 'latest',
        sourceType: 'module'
      },
      // Define globals for browser and Node.js environments
      globals: {
        ...globals.browser,
        ...globals.node
      }
    }
  },
  // Add eslint-config-prettier to disable rules that conflict with Prettier
  eslintConfigPrettier
);
