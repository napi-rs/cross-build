import test from 'ava'

import { compressToBase64 } from '../index.js'

const FIXTURE = `const a: number = 1;`

test('compressToBase64 native', (t) => {
  t.is(compressToBase64(FIXTURE), `MYewdgzgLgBAhgLhmArgWwEYFMBOMC8MAjANxA===`)
})
