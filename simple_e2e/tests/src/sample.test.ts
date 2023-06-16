import {expect, test} from 'vitest';
import {helloRust, helloMotoko} from './actor.js';

test('sample', () => {
  const sum = 1 + 1;
  expect(sum).toBe(2);
});

test('sample with canister', async () => {
  const msgFromRust = await helloRust.greet('Rust');
  expect(msgFromRust).toBe('Hello, Rust! from Rust');
  const msgFromMotoko = await helloMotoko.greet('Motoko');
  expect(msgFromMotoko).toBe('Hello, Motoko! from Motoko');
});
