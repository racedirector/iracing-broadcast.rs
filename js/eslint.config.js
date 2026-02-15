import js from '@eslint/js';
import vitest from '@vitest/eslint-plugin';
import unicorn from 'eslint-plugin-unicorn';
import writeGoodComments from 'eslint-plugin-write-good-comments';
import noSecrets from 'eslint-plugin-no-secrets';

export default [
  {
    ignores: ['node_modules/**'],
    languageOptions: {
      globals: {
        process: 'readonly'
      }
    }
  },
  js.configs.recommended,
  {
    plugins: {
      unicorn,
      'write-good-comments': writeGoodComments,
      'no-secrets': noSecrets
    },
    rules: {
      ...unicorn.configs.recommended.rules,
      'unicorn/prevent-abbreviations': 'off',
      'write-good-comments/write-good-comments': 'warn',
      'no-secrets/no-secrets': 'error'
    }
  },
  {
    files: ['**/*.test.js'],
    ...vitest.configs.recommended,
    languageOptions: {
      globals: vitest.environments.env.globals
    }
  }
];
