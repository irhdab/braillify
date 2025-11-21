import { configs } from 'eslint-plugin-devup'

export default [
  ...configs.recommended,
  {
    ignores: ['**/.pytest_cache/**', 'packages/node/pkg/**'],
  },
]
