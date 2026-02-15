import js from '@eslint/js';
import globals from 'globals';

export default [
  {
    ignores: ['target/**', '**/node_modules/**', 'examples/wasm-js/pkg/**'],
  },
  js.configs.recommended,
  {
    files: ['**/*.{js,mjs,cjs}'],
    languageOptions: {
      ecmaVersion: 'latest',
      sourceType: 'module',
      globals: {
        ...globals.node,
      },
    },
    rules: {
      'no-console': 'off',
    },
  },
];
