import js from '@eslint/js';
import vitest from '@vitest/eslint-plugin';
import unicorn from 'eslint-plugin-unicorn';
import writeGoodComments from 'eslint-plugin-write-good-comments';
import noSecrets from 'eslint-plugin-no-secrets';
import tseslint from 'typescript-eslint';

export default tseslint.config(
  {
    ignores: ['node_modules/**']
  },
  js.configs.recommended,
  ...tseslint.configs.recommendedTypeChecked,
  {
    files: ['eslint.config.ts', 'vitest.config.ts'],
    rules: {
      '@typescript-eslint/no-unsafe-assignment': 'off'
    }
  },
  {
    languageOptions: {
      parserOptions: {
        projectService: true,
        tsconfigRootDir: import.meta.dirname
      },
      globals: {
        process: 'readonly'
      }
    },
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
    files: ['**/*.test.ts'],
    ...vitest.configs.recommended,
    languageOptions: {
      globals: vitest.environments.env.globals
    }
  }
);
