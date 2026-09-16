import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { test } from 'node:test';

const cases = [
  ['plain conventional header', 'feat: add release pipeline', true],
  [
    'ticket and breaking change',
    '[PROJ-1] feat!: change macro syntax\n\nBREAKING CHANGE: callers must update',
    true,
  ],
  ['upper-case subject', 'feat: Add release pipeline', false],
  ['unknown type', 'build: add release pipeline', false],
  ['trailing period', 'fix: handle errors.', false],
  ['overlong header', `feat: ${'x'.repeat(68)}`, false],
  ['overlong body', `feat: add release pipeline\n\n${'x'.repeat(73)}`, false],
  [
    'attribution footer',
    'feat: add release pipeline\n\nCo-Authored-By: robot',
    false,
  ],
];

for (const [name, message, valid] of cases) {
  test(name, () => {
    const result = spawnSync('node_modules/.bin/commitlint', [], {
      encoding: 'utf8',
      input: `${message}\n`,
    });
    assert.equal(result.status === 0, valid, result.stdout + result.stderr);
  });
}
